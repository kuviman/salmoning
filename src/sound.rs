use crate::{
    assets::Assets, controls::GengEvent, interop::ClientMessage, model::*, render::Camera,
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
    music_volume: f32,
    radio_volume: f32,
    sfx_volume: f32,
}

#[derive(Component)]
struct Global {
    geng: Geng,
    assets: Rc<Assets>,
    pedaling: Option<geng::SoundEffect>,
}

#[derive(Event)]
pub struct RingBell {
    #[event(target)]
    pub entity: EntityId,
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
                let mut music = assets.sounds.music.play();
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
            pedaling: Some({
                let mut pedaling = assets.sounds.pedaling.play();
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

fn quest_sounds(receiver: Receiver<QuestEvent>, global: Single<&Global>, config: Single<&Config>) {
    match receiver.event {
        QuestEvent::Start => {
            global
                .assets
                .sounds
                .quest_start
                .play()
                .set_volume(config.sfx_volume * 0.4);
        }
        QuestEvent::Complete => {
            global
                .assets
                .sounds
                .quest_complete
                .play()
                .set_volume(config.sfx_volume * 4.);
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
            state.music = Some(global.assets.sounds.music.play());
            state
                .music
                .as_mut()
                .unwrap()
                .set_volume(config.music_volume);
            state.on = false;
        } else {
            state.music.take().unwrap().stop();
            state.radio = Some({
                let mut effect = global.assets.sounds.salmon_radio.effect();
                effect.set_volume(config.radio_volume);
                effect.play_from(time::Duration::from_secs_f64(thread_rng().gen_range(
                    0.0..global.assets.sounds.salmon_radio.duration().as_secs_f64(),
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
    global.geng.audio().listener().set_position(camera.position);
    global
        .geng
        .audio()
        .listener()
        .set_orientation(vec3(rot.x, rot.y, 0.), vec3(0.0, 0.0, 1.0));
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
    let mut effect = global.assets.sounds.bell.effect();
    effect.set_volume(config.sfx_volume * 0.2);
    let pos = bikes.get(receiver.event.entity).unwrap().pos;
    effect.set_position(vec3(pos.x, pos.y, 0.0));
    effect.play();
}

fn pedaling(
    _receiver: Receiver<Update>,
    mut global: Single<&mut Global>,
    config: Single<&Config>,

    bike: Single<(&Vehicle, With<&LocalPlayer>)>,
) {
    let speed = bike.0 .0.speed;
    let volume = (speed * 0.01).min(config.sfx_volume * 0.5);
    global.pedaling.as_mut().unwrap().set_volume(volume as f32);

    let rate = (speed as f64 * 0.05) + 0.6;
    global.pedaling.as_mut().unwrap().set_speed(rate as f32);
}
