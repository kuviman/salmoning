use crate::{assets::Assets, controls::GengEvent, model::*};
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
struct Global {
    geng: Geng,
    assets: Rc<Assets>,
}

pub async fn init(world: &mut World, geng: &Geng, assets: &Rc<Assets>) {
    let radio = world.spawn();
    let config: Config = file::load_detect(run_dir().join("assets").join("audio.toml"))
        .await
        .unwrap();
    world.insert(
        radio,
        RadioState {
            on: false,
            music: Some({
                let mut music = assets.music.play();
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
            assets: assets.clone(),
        },
    );
    world.insert(radio, config);
    world.add_handler(toggle_radio);
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
            state.music = Some(global.assets.music.play());
            state
                .music
                .as_mut()
                .unwrap()
                .set_volume(config.music_volume);
            state.on = false;
        } else {
            state.music.take().unwrap().stop();
            state.radio = Some({
                let mut effect = global.assets.salmon_radio.effect();
                effect.set_volume(config.radio_volume);
                effect
                    .play_from(time::Duration::from_secs_f64(thread_rng().gen_range(
                        0.0..global.assets.salmon_radio.duration().as_secs_f64(),
                    )));
                effect
            });
            state.on = true;
        }
    }
}
