use std::collections::VecDeque;

use evenio::prelude::*;
use geng::prelude::{futures::executor::Enter, once_cell::sync::Lazy, *};

use wasm_bindgen::prelude::*;

use crate::{
    interop::ClientMessage,
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
    fn alert(s: &str);
}

// this is how we go JS -> rust
#[derive(evenio::event::Event, Deserialize)]
#[serde(tag = "type")]
pub enum UiMessage {
    ChangeName { name: String },
    AcceptQuest,
    PreviewCosmetic { kind: Customization, index: usize },
}

#[derive(Deserialize)]
#[serde(tag = "type")]
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

#[allow(dead_code)]
#[derive(Component)]
pub struct Ui {}

pub async fn init(world: &mut World, geng: &Geng) {
    let ui = world.spawn();
    world.insert(ui, Ui {});

    bridge_init();
    world.add_handler(sync_money);
    world.add_handler(sync_shop);
    world.add_handler(handle_events);
    world.add_handler(phone_quest);
    bridge_add_task("choose_name");
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
    mut sender: Sender<(crate::render::SetHatType, ClientMessage, QuestEvent)>,
) {
    match receiver.event {
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
