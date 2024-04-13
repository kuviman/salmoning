use crate::{
    model::*,
    render::{Camera, Draw},
};
use evenio::prelude::*;
use geng::prelude::*;

#[derive(Event)]
pub struct GengEvent(pub geng::Event);

#[derive(Component)]
struct Global {
    geng: Geng,
    framebuffer_size: vec2<f32>,
}

pub fn init(world: &mut World, geng: &Geng) {
    let global = world.spawn();
    world.insert(
        global,
        Global {
            geng: geng.clone(),
            framebuffer_size: vec2::splat(1.0),
        },
    );
    world.add_handler(update_framebuffer_size);

    world.add_handler(player_controls);

    init_debug_camera_controls(world);
    world.add_handler(click_to_append_to_road);
}

fn update_framebuffer_size(receiver: Receiver<Draw>, mut global: Single<&mut Global>) {
    global.framebuffer_size = receiver.event.framebuffer.size().map(|x| x as f32);
}

fn player_controls(
    receiver: Receiver<Update>,
    global: Single<&Global>,
    players: Fetcher<(&mut BikeController, With<&Player>)>,
) {
    for (controller, _) in players {
        controller.accelerate = 0.0;
        if global.geng.window().is_key_pressed(geng::Key::ArrowUp) {
            controller.accelerate += 1.0;
        }
        if global.geng.window().is_key_pressed(geng::Key::ArrowDown) {
            controller.accelerate += -1.0;
        }
        controller.rotate = 0.0;
        if global.geng.window().is_key_pressed(geng::Key::ArrowLeft) {
            controller.rotate += 1.0;
        }
        if global.geng.window().is_key_pressed(geng::Key::ArrowRight) {
            controller.rotate -= 1.0;
        }

        controller.brakes = global.geng.window().is_key_pressed(geng::Key::Space);
    }
}

fn click_to_append_to_road(
    receiver: Receiver<GengEvent>,
    global: Single<&Global>,
    camera: Single<&Camera>,
    road: Fetcher<(EntityId, &Road)>,
    mut sender: Sender<Insert<Road>>, // this way mesh is updated
) {
    let geng::Event::MousePress {
        button: geng::MouseButton::Left,
    } = receiver.event.0
    else {
        return;
    };
    let click_world_pos = {
        let ray = camera.pixel_ray(
            global.framebuffer_size,
            global
                .geng
                .window()
                .cursor_position()
                .unwrap()
                .map(|x| x as f32),
        );
        // ray.from + ray.dir * t = 0
        let t = -ray.from.z / ray.dir.z;
        ray.from.xy() + ray.dir.xy() * t
    };
    for (road_entity, road) in road {
        let mut road = road.clone();
        road.waypoints.push(click_world_pos);
        sender.insert(road_entity, road);
    }
}

fn init_debug_camera_controls(world: &mut World) {
    fn zoom(receiver: Receiver<GengEvent>, mut camera: Single<&mut Camera>) {
        if let geng::Event::Wheel { delta } = receiver.event.0 {
            camera.distance += delta as f32 / 10.0;
            camera.distance = camera.distance.clamp(1.0, 100.0);
        }
    }
    fn controls(
        receiver: Receiver<Update>,
        mut camera: Single<&mut Camera>,
        global: Single<&Global>,
    ) {
        let camera = &mut **camera;
        let delta_time = receiver.event.delta_time;
        let rotation_speed = Angle::from_degrees(90.0);
        let movement_speed = 1.0;
        if global.geng.window().is_key_pressed(geng::Key::Q) {
            camera.rotation += rotation_speed * delta_time.as_secs_f64() as f32;
        }
        if global.geng.window().is_key_pressed(geng::Key::E) {
            camera.rotation -= rotation_speed * delta_time.as_secs_f64() as f32;
        }
        if global.geng.window().is_key_pressed(geng::Key::PageUp) {
            camera.attack_angle += rotation_speed * delta_time.as_secs_f64() as f32;
        }
        if global.geng.window().is_key_pressed(geng::Key::PageDown) {
            camera.attack_angle -= rotation_speed * delta_time.as_secs_f64() as f32;
        }
        let mut move_dir = vec2(0.0, 0.0);
        if global.geng.window().is_key_pressed(geng::Key::W) {
            move_dir.y += 1.0;
        }
        if global.geng.window().is_key_pressed(geng::Key::A) {
            move_dir.x -= 1.0;
        }
        if global.geng.window().is_key_pressed(geng::Key::S) {
            move_dir.y -= 1.0;
        }
        if global.geng.window().is_key_pressed(geng::Key::D) {
            move_dir.x += 1.0;
        }
        camera.position += movement_speed
            * move_dir.rotate(camera.rotation).extend(0.0)
            * delta_time.as_secs_f64() as f32;
        camera.rotation = camera.rotation.normalized_2pi();
    }
    world.add_handler(controls);
    world.add_handler(zoom);
}
