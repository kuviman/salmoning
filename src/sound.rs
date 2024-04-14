use crate::{
    assets::{Assets, Sounds},
    controls::GengEvent,
    interop::{ClientMessage, ServerMessage},
    model::*,
};
use evenio::prelude::*;
use geng::prelude::*;

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
}

#[derive(Component)]
struct GlobalSounds {
    geng: Geng,
    sounds: Rc<Sounds>,
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
        GlobalSounds {
            geng: geng.clone(),
            sounds: sounds.clone(),
        },
    );
    world.insert(radio, config);
    world.add_handler(toggle_radio);
    world.add_handler(ring_bell);
    world.add_handler(ring_bell_event);
    world.add_handler(quest_sounds);
}

fn quest_sounds(receiver: Receiver<QuestEvent>, global: Single<&GlobalSounds>) {
    match receiver.event {
        QuestEvent::Start => {
            global.sounds.bell.play();
        }
        QuestEvent::Complete => {
            global.sounds.bell.play();
        }
    }
}

fn toggle_radio(
    receiver: Receiver<GengEvent>,
    config: Single<&Config>,
    global: Single<&GlobalSounds>,
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

#[allow(clippy::type_complexity)]
fn ring_bell(
    receiver: Receiver<GengEvent>,
    global: Single<&mut GlobalSounds>,
    mut sender: Sender<(ClientMessage, Spawn, Despawn)>,
) {
    if let geng::Event::KeyPress { key: geng::Key::B } = receiver.event.0 {
        let mut effect = global.sounds.bell.effect();
        effect.set_volume(0.1);
        effect.play();
        sender.send(ClientMessage::RingBell);
    }
}

#[allow(clippy::type_complexity)]
fn ring_bell_event(receiver: Receiver<ServerMessage>, global: Single<&mut GlobalSounds>) {
    if let ServerMessage::RingBell(_id) = receiver.event {
        //TODO: spacial audio
        let mut effect = global.sounds.bell.effect();
        effect.set_volume(0.1);
        effect.play();
    }
}
