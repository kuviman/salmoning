use std::collections::VecDeque;

use bomboni_wasm::Wasm;
use bomboni_wasm_derive::Wasm;
use evenio::prelude::*;
use geng::prelude::{futures::executor::Enter, once_cell::sync::Lazy, *};

use wasm_bindgen::prelude::*;

use crate::{
    interop::{ClientMessage, ServerMessage},
    model::{
        net::{Invitation, Name},
        Bike, Fish, LocalPlayer, Money, QuestEvent,
    },
    render::Shopping,
};

// these are how we go rust -> JS
#[derive(evenio::event::Event, Deserialize, Serialize, Wasm, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
#[wasm(wasm_abi)]
pub enum OutboundUiMessage {
    SyncMoney { amount: i32 },
    ShowShop { visible: bool },
    PhoneShowInvite { from: String },
    PhoneNewJob { prompt: String },
    PhoneChangeName,
    PhoneDismissNotification,
    CustomizationInfo(Box<CustomizationInfo>),
    Unlocks(Unlocks),
    PhoneAcceptInvite,
    PhoneRejectInvite,
}

#[wasm_bindgen]
extern "C" {
    fn bridge_init();
    fn bridge_send(msg: OutboundUiMessage);
}

// this is how we go JS -> rust
#[derive(evenio::event::Event, Deserialize, Serialize, Wasm)]
#[serde(tag = "type", rename_all = "snake_case")]
#[wasm(wasm_abi)]
pub enum InboundUiMessage {
    ChangeName { name: String },
    AcceptQuest,
    AcceptInvite,
    DeclineInvite,
    PreviewCosmetic { kind: Customization, index: usize },
    EquipAndBuy { kind: Customization, index: usize },
}

#[derive(Deserialize, Serialize, Wasm)]
#[serde(rename_all = "snake_case")]
#[wasm(wasm_abi)]
pub enum Customization {
    Bike,
    Hat,
}

pub static CUSTOMIZATIONS: Lazy<CustomizationInfo> = Lazy::new(|| CustomizationInfo {
    bike_names: [
        BikeStats {
            name: "Bicycle".to_string(),
            cost: 0,
        },
        BikeStats {
            name: "Unicycle".to_string(),
            cost: 9999,
        },
    ],
    hat_names: [
        None,
        Some(HatStats {
            name: "Bobblehat".to_string(),
            cost: 100,
        }),
        Some(HatStats {
            name: "Cap".to_string(),
            cost: 20,
        }),
        Some(HatStats {
            name: "Cat".to_string(),
            cost: 50,
        }),
        Some(HatStats {
            name: "Cop".to_string(),
            cost: 100,
        }),
        Some(HatStats {
            name: "Crab".to_string(),
            cost: 200,
        }),
        Some(HatStats {
            name: "Crown 1".to_string(),
            cost: 2500,
        }),
        Some(HatStats {
            name: "Crown 2".to_string(),
            cost: 5000,
        }),
        Some(HatStats {
            name: "Drill".to_string(),
            cost: 1000,
        }),
        Some(HatStats {
            name: "Fish 1".to_string(),
            cost: 250,
        }),
        Some(HatStats {
            name: "Fish 2".to_string(),
            cost: 500,
        }),
        Some(HatStats {
            name: "Halo".to_string(),
            cost: 1000,
        }),
        Some(HatStats {
            name: "Heart".to_string(),
            cost: 500,
        }),
        Some(HatStats {
            name: "Numberone".to_string(),
            cost: 100,
        }),
        Some(HatStats {
            name: "Star".to_string(),
            cost: 400,
        }),
        Some(HatStats {
            name: "Top Hat".to_string(),
            cost: 1200,
        }),
    ],
});

static MESSAGE_QUEUE: Lazy<Mutex<VecDeque<InboundUiMessage>>> = Lazy::new(default);

#[wasm_bindgen]
pub fn bridge_reply(message: InboundUiMessage) {
    MESSAGE_QUEUE.lock().unwrap().push_back(message);
}

pub fn new_messages() -> impl Iterator<Item = InboundUiMessage> {
    std::mem::take(&mut *MESSAGE_QUEUE.lock().unwrap()).into_iter()
}

#[derive(Serialize, Deserialize, Clone, Wasm)]
#[wasm(wasm_abi)]
pub struct HatStats {
    pub name: String,
    pub cost: i64,
}

#[derive(Serialize, Deserialize, Clone, Wasm)]
#[wasm(wasm_abi)]
pub struct BikeStats {
    pub name: String,
    pub cost: i64,
}

#[allow(dead_code)]
#[derive(Serialize, Clone, Deserialize, Wasm)]
#[wasm(wasm_abi)]
pub struct CustomizationInfo {
    pub hat_names: [Option<HatStats>; 16],
    pub bike_names: [BikeStats; 2],
}

#[derive(Component, Serialize, Clone, Deserialize, Wasm)]
#[wasm(wasm_abi)]
pub struct Unlocks {
    pub hats: HashSet<usize>,
    pub bikes: HashSet<usize>,
    pub loaded: bool,
}

pub async fn init(world: &mut World, geng: &Geng) {
    let ui = world.spawn();

    bridge_init();
    bridge_send(OutboundUiMessage::CustomizationInfo(Box::new(
        CUSTOMIZATIONS.clone(),
    )));
    world.add_handler(bridge_forwarder);
    world.add_handler(unlock_hats);
    world.add_handler(unlock_bikes);
    world.add_handler(sync_money);
    world.add_handler(sync_shop);
    world.add_handler(handle_events);
    world.add_handler(phone_quest);
    world.add_handler(receive_invitation);
    bridge_send(OutboundUiMessage::PhoneChangeName);
    world.insert(
        ui,
        Unlocks {
            hats: HashSet::new(),
            bikes: HashSet::new(),
            loaded: false,
        },
    );
}

fn bridge_forwarder(receiver: Receiver<OutboundUiMessage>) {
    bridge_send(receiver.event.clone());
}

fn receive_invitation(
    receiver: Receiver<Insert<Invitation>, ()>,
    names: Fetcher<&Name>,
    mut sender: Sender<OutboundUiMessage>,
) {
    let Ok(team_name) = names.get(receiver.event.component.entity_id) else {
        return;
    };
    sender.send(OutboundUiMessage::PhoneShowInvite {
        from: team_name.0.clone(),
    });
}

fn unlock_bikes(
    receiver: Receiver<ServerMessage>,
    mut unlocks: Single<&mut Unlocks>,
    mut sender: Sender<OutboundUiMessage>,
) {
    let ServerMessage::YourUnlockedBikes(bikes) = receiver.event else {
        return;
    };
    unlocks.bikes = bikes.clone();
    sender.send(OutboundUiMessage::Unlocks(unlocks.clone()));
}

fn unlock_hats(
    receiver: Receiver<ServerMessage>,
    mut unlocks: Single<&mut Unlocks>,
    mut sender: Sender<OutboundUiMessage>,
) {
    let ServerMessage::YourUnlockedHats(hats) = receiver.event else {
        return;
    };
    unlocks.hats = hats.clone();
    unlocks.loaded = true;
    sender.send(OutboundUiMessage::Unlocks(unlocks.clone()));
}

fn sync_money(
    receiver: Receiver<Insert<Money>, With<&LocalPlayer>>,
    mut sender: Sender<OutboundUiMessage>,
) {
    sender.send(OutboundUiMessage::SyncMoney {
        amount: receiver.event.component.0 as i32,
    });
}

fn sync_shop(
    receiver: Receiver<Shopping>,
    unlocks: Single<&Unlocks>,
    bike: Single<(&Bike, EntityId, With<&LocalPlayer>)>,
    mut sender: Sender<(
        crate::render::SetHatType,
        crate::render::SetBikeType,
        ClientMessage,
        OutboundUiMessage,
    )>,
) {
    let event = OutboundUiMessage::ShowShop {
        visible: match receiver.event {
            Shopping::Enter => true,
            Shopping::Exit => {
                log::info!("is unlocked? {}", unlocks.loaded);
                if let Some(hat) = bike.0 .0.hat_type {
                    if !unlocks.hats.contains(&hat) {
                        sender.send(ClientMessage::SetHatType(None));
                        sender.send(crate::render::SetHatType {
                            bike_id: bike.1,
                            hat_type: None,
                        })
                    }
                }
                if !unlocks.bikes.contains(&bike.0 .0.bike_type) {
                    sender.send(ClientMessage::SetBikeType(0));
                    sender.send(crate::render::SetBikeType {
                        bike_id: bike.1,
                        bike_type: 0,
                    })
                }
                false
            }
        },
    };
    sender.send(event);
}

fn phone_quest(receiver: Receiver<QuestEvent>, mut sender: Sender<OutboundUiMessage>) {
    if let QuestEvent::Start = receiver.event {
        sender.send(OutboundUiMessage::PhoneNewJob {
            prompt: "".to_string(),
        });
    }
}

fn handle_events(
    receiver: Receiver<InboundUiMessage>,
    fish: Fetcher<&Fish>,
    money: Single<&mut Money>,
    mut unlocks: Single<&mut Unlocks>,
    mut sender: Sender<(
        crate::render::SetHatType,
        crate::render::SetBikeType,
        ClientMessage,
        QuestEvent,
        OutboundUiMessage,
    )>,
) {
    match receiver.event {
        InboundUiMessage::EquipAndBuy { kind, index } => {
            let cost = match kind {
                Customization::Bike => CUSTOMIZATIONS.bike_names[*index].cost,
                Customization::Hat => CUSTOMIZATIONS.hat_names[*index]
                    .as_ref()
                    .map_or(0, |x| x.cost),
            };
            if cost <= money.0 .0 {
                match kind {
                    Customization::Bike => {
                        unlocks.bikes.insert(*index);
                        sender.send(OutboundUiMessage::Unlocks(unlocks.clone()));
                        sender.send(ClientMessage::UnlockBike(*index));
                    }
                    Customization::Hat => {
                        unlocks.hats.insert(*index);
                        sender.send(OutboundUiMessage::Unlocks(unlocks.clone()));
                        sender.send(ClientMessage::UnlockHat(*index))
                    }
                }
            }
        }
        InboundUiMessage::AcceptQuest => {}
        InboundUiMessage::ChangeName { name } => {
            sender.send(ClientMessage::SetName(name.to_string()));
        }
        InboundUiMessage::PreviewCosmetic { kind, index } => match kind {
            Customization::Hat => {
                for fish in fish {
                    if fish.local {
                        if *index == 0 {
                            sender.send(ClientMessage::SetHatType(None));
                            sender.send(crate::render::SetHatType {
                                bike_id: fish.bike,
                                hat_type: None,
                            });
                        } else {
                            sender.send(ClientMessage::SetHatType(Some(*index)));
                            sender.send(crate::render::SetHatType {
                                bike_id: fish.bike,
                                hat_type: Some(*index),
                            });
                        }
                    }
                }
            }
            Customization::Bike => {
                for fish in fish {
                    if fish.local {
                        sender.send(crate::render::SetBikeType {
                            bike_id: fish.bike,
                            bike_type: *index,
                        });
                        sender.send(ClientMessage::SetBikeType(*index));
                    }
                }
            }
        },
        _ => {}
    }
}
