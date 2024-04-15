use evenio::prelude::*;
use geng::prelude::*;

use wasm_bindgen::prelude::*;

use crate::model::{LocalPlayer, Money};

#[wasm_bindgen]
extern "C" {
    fn bridge_init();
    fn bridge_sync_money(amount: i32);
}

// #[wasm_bindgen]
// pub fn greet(name: &str) {
//     alert(&format!("Hello, {}!", name));
// }

#[allow(dead_code)]
#[derive(Component)]
pub struct Ui {}

pub async fn init(world: &mut World, geng: &Geng) {
    let ui = world.spawn();
    world.insert(ui, Ui {});
    bridge_init();
    world.add_handler(sync_money);
}

fn sync_money(receiver: Receiver<Insert<Money>, With<&LocalPlayer>>) {
    bridge_sync_money(receiver.event.component.0 as i32);
}
