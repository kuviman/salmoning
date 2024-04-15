use std::collections::VecDeque;

use evenio::prelude::*;
use geng::prelude::{futures::executor::Enter, once_cell::sync::Lazy, *};

use wasm_bindgen::prelude::*;

use crate::{
    model::{LocalPlayer, Money},
    render::Shopping,
};

#[wasm_bindgen]
extern "C" {
    fn bridge_init();
    fn bridge_sync_money(amount: i32);
    fn bridge_show_shop(visible: bool);
}

#[derive(evenio::event::Event, Deserialize)]
pub enum UiMessage {}

static MESSAGE_QUEUE: Lazy<Mutex<VecDeque<UiMessage>>> = Lazy::new(|| default());

#[wasm_bindgen]
pub fn send_message_to_world(message: JsValue) {
    // TODO: convert message into rust message
    // MESSAGE_QUEUE.lock().unwrap().push_back(message);
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
