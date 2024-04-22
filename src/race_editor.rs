use crate::{
    controls::GengEvent,
    render::{Camera, Draw},
};

use evenio::prelude::*;
use geng::prelude::*;

#[derive(Component)]
pub struct RaceEditor {
    pub pos: vec2<f32>,
    pub track: Vec<vec2<f32>>,
}

#[derive(Component)]
struct Global {
    geng: Geng,
    framebuffer_size: vec2<f32>,
    pub dragging: bool,
    pub drag_start: vec2<f32>,
    pub drag_offset: vec2<f32>,
}

pub async fn init(world: &mut World, geng: &Geng) {
    let global = world.spawn();
    world.insert(
        global,
        Global {
            geng: geng.clone(),
            framebuffer_size: vec2::splat(1.0),
            dragging: false,
            drag_start: vec2::ZERO,
            drag_offset: vec2::ZERO,
        },
    );

    world.add_handler(update_framebuffer_size);
    world.add_handler(handle_mouse);
}

fn update_framebuffer_size(receiver: Receiver<Draw>, mut global: Single<&mut Global>) {
    global.framebuffer_size = receiver.event.framebuffer.size().map(|x| x as f32);
}

fn handle_mouse(
    receiver: Receiver<GengEvent>,
    mut global: Single<&mut Global>,
    editor: TrySingle<&mut RaceEditor>,
    camera: Single<&Camera>,
) {
    let Ok(editor) = editor.0 else {
        return;
    };
    let Some(cursor_pos) = global.geng.window().cursor_position() else {
        return;
    };
    let cursor_pos = cursor_pos.map(|x| x as f32);
    match receiver.event.0 {
        geng::Event::CursorMove { .. } => {
            if global.dragging {
                let delta = (cursor_pos - global.drag_offset) / 10.0;
                editor.pos = global.drag_start - delta;
            }
        }
        geng::Event::MousePress { button } => match button {
            geng::MouseButton::Left => {
                global.drag_offset = cursor_pos;
                global.drag_start = editor.pos;
                global.dragging = true;
            }
            geng::MouseButton::Right => {
                let click_world_pos = {
                    let ray = camera.pixel_ray(global.framebuffer_size, cursor_pos);
                    // ray.from + ray.dir * t = 0
                    let t = -ray.from.z / ray.dir.z;
                    ray.from.xy() + ray.dir.xy() * t
                };
                editor.track.push(click_world_pos);
            }
            _ => {}
        },
        geng::Event::MouseRelease { button } => match button {
            geng::MouseButton::Left => {
                global.dragging = false;
            }
            _ => {}
        },
        _ => {}
    }
}
