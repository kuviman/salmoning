use crate::{assets::Assets, model::*};

use evenio::{prelude::*, query};
use geng::prelude::*;

#[derive(Event)]
pub struct Draw {
    pub framebuffer: &'static mut ugli::Framebuffer<'static>,
}

#[derive(Component)]
pub struct Sprite {
    pub texture: Rc<ugli::Texture>,
    pub transform: mat3<f32>,
}

fn clear(mut receiver: ReceiverMut<Draw>) {
    let framebuffer = &mut *receiver.event.framebuffer;
    ugli::clear(framebuffer, Some(Rgba::BLUE), None, None);
}

#[derive(Component)]
pub struct Global {
    geng: Geng,
    assets: Rc<Assets>,
}

#[derive(Component)]
pub struct Camera {
    camera: Camera2d,
}

fn draw_sprites(
    mut receiver: ReceiverMut<Draw>,
    sprites: Fetcher<&Sprite>,
    global: Single<&Global>,
    camera: Single<&Camera>,
) {
    let framebuffer = &mut *receiver.event.framebuffer;
    for sprite in sprites {
        global.geng.draw2d().draw2d(
            framebuffer,
            &camera.camera,
            &draw2d::TexturedQuad::unit(&*sprite.texture).transform(sprite.transform),
        );
    }
}

pub fn init(world: &mut World, geng: &Geng, assets: &Rc<Assets>) {
    let global = world.spawn();
    world.insert(
        global,
        Global {
            geng: geng.clone(),
            assets: assets.clone(),
        },
    );
    world.insert(
        global,
        Camera {
            camera: Camera2d {
                center: vec2::ZERO,
                rotation: Angle::ZERO,
                fov: 20.0,
            },
        },
    );
    world.spawn();
    world.add_handler(clear);
    world.add_handler(draw_sprites);
}
