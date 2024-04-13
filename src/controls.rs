use crate::model::*;
use evenio::prelude::*;
use geng::prelude::*;

#[derive(Event)]
pub struct GengEvent(pub geng::Event);

pub fn init(world: &mut World, _geng: &Geng) {}
