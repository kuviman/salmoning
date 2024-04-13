use crate::{
    model::*,
    render::{Camera, Draw},
};
use evenio::prelude::*;
use geng::prelude::*;

#[derive(Event)]
pub struct GengEvent(pub geng::Event);

#[derive(Deserialize)]
struct PlayerControls {
    accelerate: Vec<geng::Key>,
    back: Vec<geng::Key>,
    stop: Vec<geng::Key>,
    left: Vec<geng::Key>,
    right: Vec<geng::Key>,
}

#[derive(Deserialize)]
struct Controls {
    player: PlayerControls,
}

#[derive(Component)]
struct Global {
    geng: Geng,
    controls: Controls,
    framebuffer_size: vec2<f32>,
}

pub async fn init(world: &mut World, geng: &Geng) {
    let controls: Controls = file::load_detect(run_dir().join("assets").join("controls.toml"))
        .await
        .unwrap();
    let global = world.spawn();
    world.insert(
        global,
        Global {
            controls,
            geng: geng.clone(),
            framebuffer_size: vec2::splat(1.0),
        },
    );
    world.add_handler(update_framebuffer_size);

    world.add_handler(player_controls);

    // init_debug_camera_controls(world);
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
    let controls = &global.controls.player;
    for (controller, _) in players {
        controller.accelerate = 0.0;
        if controls
            .accelerate
            .iter()
            .any(|&key| global.geng.window().is_key_pressed(key))
        {
            controller.accelerate += 1.0;
        }
        if controls
            .back
            .iter()
            .any(|&key| global.geng.window().is_key_pressed(key))
        {
            controller.accelerate += -1.0;
        }
        controller.rotate = 0.0;
        if controls
            .left
            .iter()
            .any(|&key| global.geng.window().is_key_pressed(key))
        {
            controller.rotate += 1.0;
        }
        if controls
            .right
            .iter()
            .any(|&key| global.geng.window().is_key_pressed(key))
        {
            controller.rotate -= 1.0;
        }

        controller.brakes = controls
            .stop
            .iter()
            .any(|&key| global.geng.window().is_key_pressed(key));
    }
}

fn click_to_append_to_road(
    receiver: Receiver<GengEvent>,
    global: Single<&Global>,
    camera: Single<&Camera>,
    graph: Fetcher<(EntityId, &RoadGraph)>,
    mut sender: Sender<Insert<RoadGraph>>, // this way mesh is updated
) {
    let Some(cursor_pos) = global.geng.window().cursor_position() else {
        return;
    };
    let cursor_pos = cursor_pos.map(|x| x as f32);
    let geng::Event::MousePress {
        button: geng::MouseButton::Left,
    } = receiver.event.0
    else {
        return;
    };
    let click_world_pos = {
        let ray = camera.pixel_ray(global.framebuffer_size, cursor_pos);
        // ray.from + ray.dir * t = 0
        let t = -ray.from.z / ray.dir.z;
        ray.from.xy() + ray.dir.xy() * t
    };
    for (graph_entity, graph) in graph {
        let mut graph = graph.clone();
        if let Some((closest_road, _)) = graph
            .roads
            .iter()
            .min_by_key(|(_, road)| r32((road.position - click_world_pos).len()))
        {
            let new_road = graph.roads.insert(Road {
                half_width: 2.0,
                position: click_world_pos,
            });
            graph.connections.push([closest_road, new_road]);

            sender.insert(graph_entity, graph);
        }
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
