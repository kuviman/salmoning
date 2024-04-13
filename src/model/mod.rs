mod logic;

use evenio::prelude::*;
use geng::prelude::*;

#[derive(Event)]
pub struct Update {
    pub delta_time: time::Duration,
}

pub fn init(world: &mut World) {
    logic::init(world);
}
