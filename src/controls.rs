use crate::{
    interop::{ClientMessage, EmoteType},
    model::*,
    render::{BikeJump, Camera, Draw, Wheelie},
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
    jump: Vec<geng::Key>,
    wheelie: Vec<geng::Key>,
    wheelie_front: Vec<geng::Key>,
}

#[derive(Deserialize)]
struct Controls {
    toggle_camera: Vec<geng::Key>,
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
    world.add_handler(camera);
    world.add_handler(jump);

    // init_debug_camera_controls(world);
}

fn camera(receiver: Receiver<GengEvent>, global: Single<&Global>, mut camera: Single<&mut Camera>) {
    if let geng::Event::KeyPress { key } = receiver.event.0 {
        if global.controls.toggle_camera.iter().any(|&c| c == key) {
            camera.preset += 1;
        }
    }
}

fn update_framebuffer_size(receiver: Receiver<Draw>, mut global: Single<&mut Global>) {
    global.framebuffer_size = receiver.event.framebuffer.size().map(|x| x as f32);
}

fn jump(
    receiver: Receiver<GengEvent>,
    global: Single<&Global>,
    players: Fetcher<(EntityId, Has<&BikeJump>, Has<&Wheelie>, With<&LocalPlayer>)>,
    mut sender: Sender<(
        Insert<Wheelie>,
        Insert<crate::render::BikeJump>,
        ClientMessage,
    )>,
) {
    if let geng::Event::KeyPress { key } = receiver.event.0 {
        if global.controls.player.jump.contains(&key) {
            for (entity, jumping, _, _) in &players {
                if !jumping.get() {
                    sender.insert(entity, BikeJump::default());
                    sender.send(ClientMessage::Emote(EmoteType::Jump));
                }
            }
        }
        if global.controls.player.wheelie.contains(&key) {
            for (entity, _, wheeling, _) in &players {
                if !wheeling.get() {
                    sender.insert(entity, Wheelie::new(false));
                    sender.send(ClientMessage::Emote(EmoteType::Wheelie(false)));
                }
            }
        }
        if global.controls.player.wheelie_front.contains(&key) {
            for (entity, _, wheeling, _) in &players {
                if !wheeling.get() {
                    sender.insert(entity, Wheelie::new(true));
                    sender.send(ClientMessage::Emote(EmoteType::Wheelie(true)));
                }
            }
        }
    }
}

fn player_controls(
    receiver: Receiver<Update>,
    global: Single<&Global>,
    players: Fetcher<(&mut VehicleController, With<&LocalPlayer>)>,
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
