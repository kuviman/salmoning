mod logic;
mod net;

use evenio::prelude::*;
use geng::prelude::*;

#[derive(Event)]
pub struct Update {
    pub delta_time: time::Duration,
}

#[derive(Component)]
pub struct BikeProperties {
    pub max_speed: f32,
    pub max_backward_speed: f32,
    pub acceleration: f32,
    pub brake_deceleration: f32,
    pub max_rotation_speed: Angle,
    pub rotation_accel: Angle,
}

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Bike {
    pub pos: vec2<f32>,
    pub rotation: Angle,
    pub rotation_speed: Angle,
    pub speed: f32,
}

#[derive(Debug, Component)]
pub struct BikeController {
    // -1 for left, +1 for right
    pub rotate: f32,
    // -1..1
    pub accelerate: f32,
    pub brakes: bool,
}

#[derive(Component)]
pub struct Player;

#[derive(Component, Clone)]
pub struct Road {
    pub half_width: f32,
    pub waypoints: Vec<vec2<f32>>,
}

#[derive(Component)]
pub struct Building {
    pub half_size: vec2<f32>,
    pub pos: vec2<f32>,
    pub rotation: Angle,
}

pub fn init(world: &mut World) {
    logic::init(world);
    net::init(world);
    world.add_handler(startup);
}

#[derive(Event)]
pub struct Startup;

#[allow(clippy::type_complexity)]
fn startup(
    _receiver: Receiver<Startup>,
    mut sender: Sender<(
        Spawn,
        Insert<Bike>,
        Insert<BikeController>,
        Insert<BikeProperties>,
        Insert<Player>,
        Insert<Road>,
        Insert<Building>,
    )>,
) {
    let player = sender.spawn();
    sender.insert(
        player,
        Bike {
            pos: vec2::ZERO,
            rotation: Angle::ZERO,
            rotation_speed: Angle::ZERO,
            speed: 0.0,
        },
    );
    sender.insert(
        player,
        BikeController {
            rotate: 0.0,
            accelerate: 0.0,
            brakes: false,
        },
    );
    sender.insert(
        player,
        BikeProperties {
            max_speed: 10.0,
            max_backward_speed: 1.0,
            acceleration: 10.0,
            brake_deceleration: 30.0,
            max_rotation_speed: Angle::from_degrees(360.0),
            rotation_accel: Angle::from_degrees(1500.0),
        },
    );
    sender.insert(player, Player);

    let road = sender.spawn();
    sender.insert(
        road,
        Road {
            half_width: 2.0,
            waypoints: vec![
                vec2(0.0, 0.0),
                vec2(10.0, 0.0),
                vec2(15.0, 2.0),
                vec2(20.0, 5.0),
            ],
        },
    );

    for _ in 0..10 {
        let building = sender.spawn();
        sender.insert(
            building,
            Building {
                half_size: vec2::splat(4.0),
                pos: thread_rng().gen_circle(vec2::ZERO, 50.0),
                rotation: thread_rng().gen(),
            },
        );
    }
}
