use std::any::Any;

use crate::{
    assets::{Assets, Sounds},
    controls::GengEvent,
    interop::{ClientMessage, ServerMessage},
    model::*,
    render::Camera,
};
use evenio::{entity, prelude::*};
use geng::{prelude::*, Audio};

#[derive(Component)]
struct RadioState {
    on: bool,
    music: Option<geng::SoundEffect>,
    radio: Option<geng::SoundEffect>,
}

#[derive(Component, Deserialize)]
struct Config {
    music_volume: f64,
    radio_volume: f64,
    sfx_volume: f64,
}

#[derive(Component)]
struct Global {
    geng: Geng,
    sounds: Rc<Sounds>,
    pedaling: Option<geng::SoundEffect>,
}

#[derive(Event)]
pub struct RingBell {
    #[event(target)]
    pub entity: EntityId,
}

pub async fn init(world: &mut World, geng: &Geng, sounds: &Rc<Sounds>) {
    let radio = world.spawn();
    let config: Config = file::load_detect(run_dir().join("assets").join("audio.toml"))
        .await
        .unwrap();
    world.insert(
        radio,
        RadioState {
            on: false,
            music: Some({
                let mut music = sounds.music.play();
                music.set_volume(config.music_volume);
                music
            }),
            radio: None,
        },
    );
    world.insert(
        radio,
        Global {
            geng: geng.clone(),
            sounds: sounds.clone(),
            pedaling: Some({
                let mut pedaling = sounds.pedaling.play();
                pedaling.set_volume(0.0);
                pedaling
            }),
        },
    );
    world.insert(radio, config);
    world.add_handler(toggle_radio);
    world.add_handler(update_listener_position);
    world.add_handler(ring_bell);
    world.add_handler(ring_bell_event);
    world.add_handler(quest_sounds);
    world.add_handler(pedaling);
}

fn quest_sounds(receiver: Receiver<QuestEvent>, global: Single<&Global>) {
    match receiver.event {
        QuestEvent::Start => {
            global.sounds.quest_start.play();
        }
        QuestEvent::Complete => {
            global.sounds.quest_complete.play();
        }
    }
}

fn toggle_radio(
    receiver: Receiver<GengEvent>,
    config: Single<&Config>,
    global: Single<&Global>,
    mut state: Single<&mut RadioState>,
) {
    if let geng::Event::KeyPress { key: geng::Key::R } = receiver.event.0 {
        if state.on {
            state.radio.take().unwrap().stop();
            state.music = Some(global.sounds.music.play());
            state
                .music
                .as_mut()
                .unwrap()
                .set_volume(config.music_volume);
            state.on = false;
        } else {
            state.music.take().unwrap().stop();
            state.radio = Some({
                let mut effect = global.sounds.salmon_radio.effect();
                effect.set_volume(config.radio_volume);
                effect
                    .play_from(time::Duration::from_secs_f64(thread_rng().gen_range(
                        0.0..global.sounds.salmon_radio.duration().as_secs_f64(),
                    )));
                effect
            });
            state.on = true;
        }
    }
}

fn update_listener_position(
    _receiver: Receiver<Update>,
    global: Single<&mut Global>,
    camera: Single<&Camera>,
) {
    let rot = camera.rotation.unit_vec();

    // prob dont care bout z rotation
    // FIXME: sound comes from the wrong x and y direction
    global
        .geng
        .audio()
        .set_listener_orientation(vec3(rot.x as f64, rot.y as f64, 0.), vec3(0.0, 0.0, 1.0));
    global.geng.audio().set_listener_position(vec3(
        camera.position.x as f64,
        camera.position.y as f64,
        camera.position.z as f64,
    ));
}

fn ring_bell(
    receiver: Receiver<GengEvent>,
    player: Single<(EntityId, With<&LocalPlayer>)>,
    mut sender: Sender<(ClientMessage, RingBell)>,
) {
    if let geng::Event::KeyPress { key: geng::Key::B } = receiver.event.0 {
        sender.send(ClientMessage::RingBell);
        sender.send(RingBell {
            entity: player.0 .0,
        });
    }
}

fn ring_bell_event(
    // why cant you omit the tuple for targetted events?
    receiver: Receiver<RingBell, ()>,
    global: Single<&mut Global>,
    bikes: Fetcher<&Vehicle>,
    config: Single<&Config>,
) {
    let mut effect = global.sounds.bell.effect();
    effect.set_volume(config.sfx_volume * 0.2);
    let pos = bikes.get(receiver.event.entity).unwrap().pos;
    effect.set_position(vec3(pos.x as f64, pos.y as f64, 0.0));
    effect.play();
}

fn pedaling(
    _receiver: Receiver<Update>,
    mut global: Single<&mut Global>,
    config: Single<&Config>,

    bike: Single<(&Vehicle, With<&LocalPlayer>)>,
) {
    if bike.0 .0.speed > 4.0 {
        global
            .pedaling
            .as_mut()
            .unwrap()
            .set_volume(config.sfx_volume * 0.05);
    } else {
        global.pedaling.as_mut().unwrap().set_volume(0.0);
    }
}
