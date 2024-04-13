mod logic;

use evenio::prelude::*;
use geng::prelude::*;

pub type Time = R32;

#[derive(Event)]
pub struct Update {
    pub delta_time: Time,
}

pub fn init(world: &mut World) {
    logic::init(world);
}
