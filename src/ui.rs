use evenio::prelude::*;
use geng::prelude::{futures::executor::Enter, *};

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
