use crate::{assets::Assets, model::*};
use evenio::prelude::*;
use geng::prelude::*;

pub fn init(world: &mut World, geng: &Geng, assets: &Rc<Assets>) {
    assets.music.play();
}
