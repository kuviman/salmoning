use crate::{
    controls::{GengEvent, TeamLeader},
    interop::ServerMessage,
    model::LocalPlayer,
    render::{Camera, Draw},
    ui::{OutboundUiMessage, Race},
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
    world.add_handler(pending_race);
    world.add_handler(start_race);
    world.add_handler(race_progress);
    world.add_handler(sync_team_leader_remove);
    world.add_handler(race_finish);
}

fn update_framebuffer_size(receiver: Receiver<Draw>, mut global: Single<&mut Global>) {
    global.framebuffer_size = receiver.event.framebuffer.size().map(|x| x as f32);
}

#[derive(Component)]
pub struct PendingRace {
    pub race: Race,
}

#[derive(Component)]
pub struct ActiveRace {
    pub index: usize,
}

fn race_finish(
    receiver: Receiver<ServerMessage>,
    global: Single<(
        EntityId,
        With<&Global>,
        Option<&PendingRace>,
        Option<&ActiveRace>,
    )>,
    mut sender: Sender<(Remove<ActiveRace>, Remove<PendingRace>, OutboundUiMessage)>,
) {
    let ServerMessage::RaceFinished = receiver.event else {
        return;
    };
    if let Some(pending) = global.2 {
        if let Some(active) = global.3 {
            if pending.race.track.len() > active.index {
                // lol noob, DNF
                sender.send(OutboundUiMessage::ShowRaceSummary);
            }
        }
    }
    if global.2.is_some() {
        sender.remove::<PendingRace>(global.0 .0);
    }
    if global.3.is_some() {
        sender.remove::<ActiveRace>(global.0 .0);
    }
}

fn sync_team_leader_remove(
    _: Receiver<Remove<TeamLeader>, With<&LocalPlayer>>,
    global: Single<(EntityId, With<&Global>, Has<&PendingRace>, Has<&ActiveRace>)>,
    mut sender: Sender<(Remove<ActiveRace>, Remove<PendingRace>)>,
) {
    if *global.2 {
        sender.remove::<PendingRace>(global.0 .0);
    }
    if *global.3 {
        sender.remove::<ActiveRace>(global.0 .0);
    }
}

fn race_progress(
    receiver: Receiver<ServerMessage>,
    global: Single<(EntityId, With<&Global>, Option<&PendingRace>)>,
    mut sender: Sender<(
        Insert<PendingRace>,
        Remove<PendingRace>,
        Insert<ActiveRace>,
        OutboundUiMessage,
    )>,
) {
    let ServerMessage::RaceProgress(index) = receiver.event else {
        return;
    };
    sender.insert(global.0 .0, ActiveRace { index: *index });
    if let Some(pending) = global.2 {
        if pending.race.track.len() == *index {
            // we have completed the race! let's show some UI
            sender.send(OutboundUiMessage::ShowRaceSummary);
        }
    }
}

fn start_race(
    receiver: Receiver<ServerMessage>,
    global: Single<(EntityId, With<&Global>, Has<&PendingRace>)>,
    mut sender: Sender<(Insert<PendingRace>, Remove<PendingRace>, Insert<ActiveRace>)>,
) {
    let ServerMessage::StartRace(included) = receiver.event else {
        return;
    };
    if !included {
        if *global.0 .2 {
            sender.remove::<PendingRace>(global.0 .0);
        }
        return;
    }
    sender.insert(global.0 .0, ActiveRace { index: 1 });
}

fn pending_race(
    receiver: Receiver<ServerMessage>,
    global: Single<(EntityId, With<&Global>)>,
    mut sender: Sender<(Insert<PendingRace>, OutboundUiMessage)>,
) {
    if let ServerMessage::SetPendingRace(race) = receiver.event {
        sender.insert(global.0 .0, PendingRace { race: race.clone() });
        sender.send(OutboundUiMessage::ClearRaceSummary);
    }
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
        geng::Event::KeyPress { key } => {
            if key == geng::Key::Z && global.geng.window().is_key_pressed(geng::Key::ControlLeft) {
                editor.track.pop();
            }
        }
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
