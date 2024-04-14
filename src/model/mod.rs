mod logic;
mod net;

use std::path::Path;

use evenio::prelude::*;
use generational_arena::{Arena, Index};
use geng::prelude::*;

#[derive(Event)]
pub struct Update {
    pub delta_time: time::Duration,
}

// #[derive(Event)]
// pub struct UpdateGraph {
//     pub new_roads: Vec<Index>,
//     pub new_connections: Vec<(Index, Index)>,
// }

#[derive(Component)]
pub struct VehicleProperties {
    pub max_speed: f32,
    pub max_backward_speed: f32,
    pub acceleration: f32,
    pub brake_deceleration: f32,
    pub max_rotation_speed: Angle,
    pub rotation_accel: Angle,
}

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Vehicle {
    pub pos: vec2<f32>,
    pub rotation: Angle,
    pub rotation_speed: Angle,
    pub speed: f32,
    pub jump: Option<f32>,
}

#[derive(Debug, Component)]
pub struct VehicleController {
    // -1 for left, +1 for right
    pub rotate: f32,
    // -1..1
    pub accelerate: f32,
    pub brakes: bool,
}

#[derive(Component)]
pub struct Player;

#[derive(Default, Serialize, Deserialize, Component, Clone)]
pub struct RoadGraph {
    pub roads: Arena<Road>,
    pub connections: Vec<[Index; 2]>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Road {
    pub half_width: f32,
    pub position: vec2<f32>,
}

#[derive(Component)]
pub struct Bike;

#[derive(Component)]
pub struct Car {
    pub color: Rgba<f32>,
}

#[derive(Serialize, Deserialize, Clone, Component)]
pub struct Building {
    pub half_size: vec2<f32>,
    pub pos: vec2<f32>,
    pub rotation: Angle,
    pub kind: i32,
}

#[derive(Serialize, Deserialize, Clone, Component)]
pub struct Waypoint {
    pub pos: vec2<f32>,
}

pub fn init(world: &mut World) {
    logic::init(world);
    net::init(world);
    world.add_handler(startup);
}

#[derive(Event)]
pub struct Startup {
    pub level: Level,
}

#[derive(Serialize, Deserialize, Clone, Component)]
pub struct Tree {
    pub pos: vec2<f32>,
    pub rotation: Angle,
    pub kind: i32,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Level {
    pub graph: RoadGraph,
    pub trees: Vec<Tree>,
    pub buildings: Vec<Building>,
    pub waypoints: Vec<Waypoint>,
}

impl Level {
    pub async fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let level = file::load_bytes(path).await?;
        let level = bincode::deserialize(&level)?;
        Ok(level)
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct RngStuff {
    pub seed: u64,
    #[deref]
    #[deref_mut]
    pub gen: StdRng,
}

#[allow(clippy::type_complexity)]
fn startup(
    receiver: Receiver<Startup>,
    mut rng: Single<&mut RngStuff>,
    mut sender: Sender<(
        Spawn,
        Insert<Vehicle>,
        Insert<VehicleController>,
        Insert<VehicleProperties>,
        Insert<Player>,
        Insert<RoadGraph>,
        Insert<Building>,
        Insert<Car>,
        Insert<Bike>,
        Insert<Waypoint>,
    )>,
) {
    let startup = receiver.event;
    let level = &startup.level;

    let player = sender.spawn();
    sender.insert(player, Bike);
    sender.insert(
        player,
        Vehicle {
            pos: vec2::ZERO,
            rotation: Angle::ZERO,
            rotation_speed: Angle::ZERO,
            speed: 0.0,
            jump: None,
        },
    );
    sender.insert(
        player,
        VehicleController {
            rotate: 0.0,
            accelerate: 0.0,
            brakes: false,
        },
    );
    sender.insert(
        player,
        VehicleProperties {
            max_speed: 10.0,
            max_backward_speed: 1.0,
            acceleration: 10.0,
            brake_deceleration: 30.0,
            max_rotation_speed: Angle::from_degrees(360.0),
            rotation_accel: Angle::from_degrees(1500.0),
        },
    );
    sender.insert(player, Player);

    let graph = sender.spawn();
    sender.insert(graph, level.graph.clone());

    for data in &level.buildings {
        let building = sender.spawn();
        sender.insert(
            building,
            Building {
                kind: data.kind,
                half_size: vec2::splat(4.0),
                pos: data.pos,
                rotation: data.rotation,
            },
        );
    }

    for data in &level.waypoints {
        let waypoint = sender.spawn();
        sender.insert(waypoint, Waypoint { pos: data.pos });
    }

    /*
    for _ in 0..10 {
        let car = sender.spawn();
        sender.insert(
            car,
            Car {
                color: color::Hsla::new(rng.gen(), 0.5, 0.5, 1.0).into(),
            },
        );
        sender.insert(
            car,
            Vehicle {
                pos: rng.gen_circle(vec2::ZERO, 30.0),
                rotation: rng.gen(),
                rotation_speed: Angle::ZERO,
                speed: 0.0,
                jump: None,
            },
        );
    }
    */
}
