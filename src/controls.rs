use crate::{
    interop::{ClientMessage, EmoteType, Id},
    model::*,
    render::{BikeJump, Camera, Draw, Wheelie},
    ui::{InboundUiMessage, OutboundUiMessage},
};
use evenio::prelude::*;
use geng::prelude::*;

use self::net::{Invitation, NetId};

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
    invite_distance: f32,
    invite_angle: f32,
    invite: Vec<geng::Key>,
    accept: Vec<geng::Key>,
    reject: Vec<geng::Key>,
    phone_interact: Vec<geng::Key>,
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

    world.add_handler(can_invite);
    world.add_handler(invite);
    world.add_handler(phone_interact);

    world.add_handler(invitation);
    world.add_handler(invitation_accept);
    // init_debug_camera_controls(world);
}

#[derive(Component, Clone, Copy)]
pub struct InviteTarget {
    pub entity: EntityId,
    pub net_id: Id,
}

fn can_invite(
    _receiver: Receiver<Update>,
    global: Single<&Global>,
    player: TrySingle<(
        EntityId,
        &Vehicle,
        With<&LocalPlayer>,
        Has<&InviteTarget>,
        Option<&TeamLeader>,
    )>,
    others: Fetcher<(EntityId, &Vehicle, &NetId, Has<&TeamLeader>)>,
    mut sender: Sender<(Insert<InviteTarget>, Remove<InviteTarget>)>,
) {
    let Ok((player_entity, player, _, _has_invite_target, player_leader)) = &*player else {
        return;
    };

    let can_invite = match player_leader {
        None => true,
        Some(leader) => leader.0 == *player_entity,
    };

    if let Some((entity_id, _, net_id, _)) = others.iter().find(|(_, vehicle, _, has_leader)| {
        let dv = vehicle.pos - player.pos;
        can_invite
            && !has_leader.get()
            && dv.len() < global.controls.invite_distance
            && (dv.arg() - player.rotation)
                .normalized_pi()
                .abs()
                .as_degrees()
                < global.controls.invite_angle
    }) {
        sender.insert(
            *player_entity,
            InviteTarget {
                entity: entity_id,
                net_id: net_id.0,
            },
        );
    } else {
        sender.remove::<InviteTarget>(*player_entity);
    }
}

#[derive(Event)]
pub struct SendInvite(pub InviteTarget);

#[derive(Event)]
pub struct JoinTeam(pub EntityId);

#[derive(Component, PartialEq)]
pub struct TeamLeader(pub EntityId);

fn invitation(
    receiver: Receiver<GengEvent>,
    global: Single<&Global>,
    invitation: TrySingle<(EntityId, &Invitation)>,
    mut sender: Sender<(
        Remove<Invitation>,
        JoinTeam,
        Insert<TeamLeader>,
        OutboundUiMessage,
    )>,
) {
    if let geng::Event::KeyPress { key } = receiver.event.0 {
        if let Ok((invitation_entity, invitation)) = invitation.0 {
            if global.controls.accept.iter().any(|&c| c == key) {
                sender.send(JoinTeam(invitation.entity_id));
                sender.send(OutboundUiMessage::PhoneAcceptInvite);
                sender.remove::<Invitation>(invitation_entity);
                sender.insert(invitation_entity, TeamLeader(invitation.entity_id));
            }
            if global.controls.reject.iter().any(|&c| c == key) {
                sender.remove::<Invitation>(invitation_entity);
                sender.send(OutboundUiMessage::PhoneRejectInvite);
            }
        }
    }
}

fn invitation_accept(
    receiver: Receiver<InboundUiMessage>,
    invitation: TrySingle<(EntityId, &Invitation)>,
    mut sender: Sender<(Remove<Invitation>, JoinTeam, Insert<TeamLeader>)>,
) {
    if let Ok((invitation_entity, invitation)) = invitation.0 {
        match receiver.event {
            InboundUiMessage::AcceptInvite => {
                sender.send(JoinTeam(invitation.entity_id));
                sender.remove::<Invitation>(invitation_entity);
                sender.insert(invitation_entity, TeamLeader(invitation.entity_id));
            }
            InboundUiMessage::DeclineInvite => {
                sender.remove::<Invitation>(invitation_entity);
            }
            _ => {}
        }
    }
}

fn phone_interact(
    receiver: Receiver<GengEvent>,
    global: Single<&Global>,
    mut sender: Sender<OutboundUiMessage>,
) {
    if let geng::Event::KeyPress { key } = receiver.event.0 {
        if global.controls.phone_interact.iter().any(|&c| c == key) {
            sender.send(OutboundUiMessage::PhoneInteractKey { mouse: false });
        }
    }
}

fn invite(
    receiver: Receiver<GengEvent>,
    global: Single<&Global>,
    target: TrySingle<&InviteTarget>,
    mut sender: Sender<SendInvite>,
) {
    if let geng::Event::KeyPress { key } = receiver.event.0 {
        if global.controls.invite.iter().any(|&c| c == key) {
            if let Ok(target) = target.0 {
                sender.send(SendInvite(*target));
            }
        }
    }
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
