use std::collections::VecDeque;

use evenio::prelude::*;
use geng::prelude::{futures::executor::Enter, once_cell::sync::Lazy, *};

use wasm_bindgen::prelude::*;

use crate::{
    interop::{ClientMessage, ServerMessage},
    model::{Bike, Fish, LocalPlayer, Money, QuestEvent},
    render::Shopping,
};

// these are how we go rust -> JS
#[wasm_bindgen]
extern "C" {
    fn bridge_init();
    fn bridge_sync_money(amount: i32);
    fn bridge_show_shop(visible: bool);
    fn bridge_add_task(task: &str);
    fn bridge_quest(s: &str);
    fn bridge_send_customizations(data: JsValue);
    fn alert(s: &str);
}

// this is how we go JS -> rust
#[derive(evenio::event::Event, Deserialize)]
#[serde(tag = "type")]
pub enum UiMessage {
    ChangeName { name: String },
    AcceptQuest,
    PreviewCosmetic { kind: Customization, index: usize },
    EquipAndBuy { kind: Customization, index: usize },
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Customization {
    Bike,
    Hat,
}

pub const CUSTOMIZATIONS: CustomizationInfo = CustomizationInfo {
    bike_names: [
        BikeStats {
            name: "Bicycle",
            cost: 0,
        },
        BikeStats {
            name: "Unicycle",
            cost: 1000,
        },
    ],
    hat_names: [
        None,
        Some(HatStats {
            name: "Bobblehat",
            cost: 10,
        }),
        Some(HatStats {
            name: "Cap",
            cost: 10,
        }),
        Some(HatStats {
            name: "Cat",
            cost: 10,
        }),
        Some(HatStats {
            name: "Cop",
            cost: 10,
        }),
        Some(HatStats {
            name: "Crab",
            cost: 10,
        }),
        Some(HatStats {
            name: "Crown 1",
            cost: 500,
        }),
        Some(HatStats {
            name: "Crown 2",
            cost: 5000,
        }),
        Some(HatStats {
            name: "Drill",
            cost: 10,
        }),
        Some(HatStats {
            name: "Fish 1",
            cost: 10,
        }),
        Some(HatStats {
            name: "Fish 2",
            cost: 10,
        }),
        Some(HatStats {
            name: "Halo",
            cost: 1000,
        }),
        Some(HatStats {
            name: "Heart",
            cost: 10,
        }),
        Some(HatStats {
            name: "Numberone",
            cost: 10,
        }),
        Some(HatStats {
            name: "Star",
            cost: 10,
        }),
        Some(HatStats {
            name: "Top Hat",
            cost: 10,
        }),
    ],
};

static MESSAGE_QUEUE: Lazy<Mutex<VecDeque<UiMessage>>> = Lazy::new(default);

#[wasm_bindgen]
pub fn send_message_to_world(message: JsValue) {
    MESSAGE_QUEUE
        .lock()
        .unwrap()
        .push_back(serde_wasm_bindgen::from_value(message).unwrap());
}

pub fn new_messages() -> impl Iterator<Item = UiMessage> {
    std::mem::take(&mut *MESSAGE_QUEUE.lock().unwrap()).into_iter()
}

#[derive(Serialize, Clone)]
pub struct HatStats {
    pub name: &'static str,
    pub cost: i64,
}

#[derive(Serialize, Clone)]
pub struct BikeStats {
    pub name: &'static str,
    pub cost: i64,
}

#[allow(dead_code)]
#[derive(Serialize, Clone)]
pub struct CustomizationInfo {
    pub hat_names: [Option<HatStats>; 16],
    pub bike_names: [BikeStats; 2],
}

#[derive(Component)]
struct Unlocks {
    hats: HashSet<usize>,
    bikes: HashSet<usize>,
}

pub async fn init(world: &mut World, geng: &Geng) {
    let ui = world.spawn();

    bridge_init();
    bridge_send_customizations(serde_wasm_bindgen::to_value(&CUSTOMIZATIONS.clone()).unwrap());
    world.add_handler(unlock_hats);
    world.add_handler(unlock_bikes);
    world.add_handler(sync_money);
    world.add_handler(sync_shop);
    world.add_handler(handle_events);
    world.add_handler(phone_quest);
    bridge_add_task("choose_name");
    world.insert(
        ui,
        Unlocks {
            hats: HashSet::new(),
            bikes: HashSet::new(),
        },
    );
}

fn unlock_bikes(receiver: Receiver<ServerMessage>, mut unlocks: Single<&mut Unlocks>) {
    let ServerMessage::YourUnlockedBikes(bikes) = receiver.event else {
        return;
    };
    unlocks.bikes = bikes.clone();
}

fn unlock_hats(receiver: Receiver<ServerMessage>, mut unlocks: Single<&mut Unlocks>) {
    let ServerMessage::YourUnlockedHats(hats) = receiver.event else {
        return;
    };
    unlocks.hats = hats.clone();
}

fn sync_money(receiver: Receiver<Insert<Money>, With<&LocalPlayer>>) {
    bridge_sync_money(receiver.event.component.0 as i32);
}

fn sync_shop(
    receiver: Receiver<Shopping>,
    unlocks: Single<&Unlocks>,
    bike: Single<(&Bike, EntityId, With<&LocalPlayer>)>,
    mut sender: Sender<(crate::render::SetHatType, crate::render::SetBikeType)>,
) {
    bridge_show_shop(match receiver.event {
        Shopping::Enter => true,
        Shopping::Exit => {
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
    });
}

fn phone_quest(receiver: Receiver<QuestEvent>) {
    if let QuestEvent::Start = receiver.event {
        bridge_quest("hello world");
    }
}

fn handle_events(
    receiver: Receiver<UiMessage>,
    fish: Fetcher<&Fish>,
    money: Single<&mut Money>,
    mut unlocks: Single<&mut Unlocks>,
    mut sender: Sender<(
        crate::render::SetHatType,
        crate::render::SetBikeType,
        ClientMessage,
        QuestEvent,
    )>,
) {
    match receiver.event {
        UiMessage::EquipAndBuy { kind, index } => {
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
                        sender.send(ClientMessage::UnlockBike(*index));
                    }
                    Customization::Hat => {
                        unlocks.hats.insert(*index);
                        sender.send(ClientMessage::UnlockHat(*index))
                    }
                }
            }
        }
        UiMessage::AcceptQuest => {}
        UiMessage::ChangeName { name } => {
            sender.send(ClientMessage::SetName(name.to_string()));
        }
        UiMessage::PreviewCosmetic { kind, index } => match kind {
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
                            sender.send(ClientMessage::SetHatType(Some(*index - 1)));
                            sender.send(crate::render::SetHatType {
                                bike_id: fish.bike,
                                hat_type: Some(*index - 1),
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
    }
}
