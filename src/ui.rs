use std::collections::VecDeque;

use evenio::prelude::*;
use geng::prelude::{futures::executor::Enter, once_cell::sync::Lazy, *};

use wasm_bindgen::prelude::*;

use crate::{
    interop::{ClientMessage, ServerMessage},
    model::{Fish, LocalPlayer, Money, QuestEvent},
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
struct HatStats {
    name: &'static str,
    cost: i64,
}

#[derive(Serialize, Clone)]
struct BikeStats {
    name: &'static str,
    cost: i64,
}

#[allow(dead_code)]
#[derive(Component, Serialize, Clone)]
pub struct CustomizationInfo {
    hat_names: Vec<HatStats>,
    bike_names: Vec<BikeStats>,
}

#[derive(Component)]
struct Unlocks {
    hats: HashSet<usize>,
    bikes: HashSet<usize>,
}

pub async fn init(world: &mut World, geng: &Geng) {
    let ui = world.spawn();
    let customs = CustomizationInfo {
        bike_names: vec![
            BikeStats {
                name: "Bicycle",
                cost: 0,
            },
            BikeStats {
                name: "Unicycle",
                cost: 1000,
            },
        ],
        hat_names: vec![
            HatStats {
                name: "Bobblehat",
                cost: 10,
            },
            HatStats {
                name: "Cap",
                cost: 10,
            },
            HatStats {
                name: "Cat",
                cost: 10,
            },
            HatStats {
                name: "Cop",
                cost: 10,
            },
            HatStats {
                name: "Crab",
                cost: 10,
            },
            HatStats {
                name: "Crown 1",
                cost: 500,
            },
            HatStats {
                name: "Crown 2",
                cost: 5000,
            },
            HatStats {
                name: "Drill",
                cost: 10,
            },
            HatStats {
                name: "Fish 1",
                cost: 10,
            },
            HatStats {
                name: "Fish 2",
                cost: 10,
            },
            HatStats {
                name: "Halo",
                cost: 1000,
            },
            HatStats {
                name: "Heart",
                cost: 10,
            },
            HatStats {
                name: "Numberone",
                cost: 10,
            },
            HatStats {
                name: "Star",
                cost: 10,
            },
            HatStats {
                name: "Top Hat",
                cost: 10,
            },
        ],
    };

    bridge_init();
    bridge_send_customizations(serde_wasm_bindgen::to_value(&customs.clone()).unwrap());
    world.add_handler(unlock_hats);
    world.add_handler(unlock_bikes);
    world.add_handler(sync_money);
    world.add_handler(sync_shop);
    world.add_handler(handle_events);
    world.add_handler(phone_quest);
    bridge_add_task("choose_name");
    world.insert(ui, customs);
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

fn sync_shop(receiver: Receiver<Shopping>) {
    bridge_show_shop(match receiver.event {
        Shopping::Enter => true,
        Shopping::Exit => false,
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
    info: Single<&CustomizationInfo>,
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
                Customization::Bike => info.bike_names[*index].cost,
                Customization::Hat => info.hat_names[*index].cost,
            };
            if cost <= money.0 .0 {
                money.0 .0 -= cost;
            }
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
        UiMessage::AcceptQuest => {}
        UiMessage::ChangeName { name } => {
            sender.send(ClientMessage::SetName(name.to_string()));
        }
        UiMessage::PreviewCosmetic { kind, index } => match kind {
            Customization::Hat => {
                for fish in fish {
                    if fish.local {
                        sender.send(crate::render::SetHatType {
                            bike_id: fish.bike,
                            hat_type: Some(*index),
                        });
                        sender.send(ClientMessage::SetHatType(Some(*index)));
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
