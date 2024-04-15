use std::collections::VecDeque;

use evenio::prelude::*;
use geng::prelude::{futures::executor::Enter, once_cell::sync::Lazy, *};

use wasm_bindgen::prelude::*;

use crate::{
    interop::ClientMessage,
    model::{LocalPlayer, Money},
    render::Shopping,
};

#[wasm_bindgen]
extern "C" {
    fn bridge_init();
    fn bridge_sync_money(amount: i32);
    fn bridge_show_shop(visible: bool);
    fn bridge_show_phone(visible: bool);
    fn alert(s: &str);
}

#[derive(evenio::event::Event, Deserialize)]
pub enum UiMessage {
    ChangeName { name: String },
}

#[derive(Component)]
pub struct Phone {
    visible: bool,
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
    world.add_handler(sync_phone);
    world.add_handler(handle_events);
    world.insert(ui, Phone { visible: true });
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

fn sync_phone(receiver: Receiver<Insert<Phone>, ()>) {
    bridge_show_phone(receiver.event.component.visible);
}

fn handle_events(
    receiver: Receiver<UiMessage>,
    mut phone: Single<&mut Phone>,
    mut sender: Sender<ClientMessage>,
) {
    match receiver.event {
        UiMessage::ChangeName { name } => {
            sender.send(ClientMessage::SetName(name.to_string()));
            phone.visible = false;
        }
    }
}
