mod logic;
pub mod net;

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
pub struct Fish {
    pub local: bool,
    pub bike: EntityId,
}

#[derive(Component, Deserialize, Clone, Serialize, Debug)]
pub struct VehicleProperties {
    pub max_speed: f32,
    pub max_offroad_speed: f32,
    pub max_backward_speed: f32,
    pub acceleration: f32,
    pub auto_deceleration: f32,
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
}

impl Default for Vehicle {
    fn default() -> Self {
        Self {
            pos: vec2::ZERO,
            rotation: Angle::ZERO,
            rotation_speed: Angle::ZERO,
            speed: 0.0,
        }
    }
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
pub struct LocalPlayer;

#[derive(Default, Serialize, Deserialize, Component, Clone)]
pub struct RoadGraph {
    pub roads: Arena<Road>,
    pub connections: Vec<[Index; 2]>,
}

impl RoadGraph {
    pub fn out(&self, v: Index) -> impl Iterator<Item = Index> + '_ {
        self.connections.iter().filter_map(move |&[a, b]| {
            if a == v {
                return Some(b);
            }
            if b == v {
                return Some(a);
            }
            None
        })
    }
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
    #[serde(default)]
    pub small: bool,
}

#[derive(Serialize, Deserialize, Clone, Component)]
pub struct Shop {
    pub pos: vec2<f32>,
    pub rotation: Angle,
    pub door_time: f32,
}

#[derive(Serialize, Deserialize, Clone, Component)]
pub struct Waypoint {
    pub pos: vec2<f32>,
}

#[derive(Event)]
pub enum QuestEvent {
    Start,
    Complete,
}

#[derive(Component)]
pub struct Money(pub i64);

#[derive(Component, Clone, Serialize, Deserialize, Debug)]
pub struct Leaderboard {
    pub rows: Vec<(String, i64)>,
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct LeaderboardBillboard {
    pub pos: vec2<f32>,
    pub rotation: Angle,
}

#[derive(Component, Deserialize)]
struct Config {
    cars: usize,
    car_radius: f32,
    wall_speed_hack: f32,
    car_half_size: vec2<f32>,
    car_loop_length: f32,
    car_speed: f32,
    vehicle: VehicleProperties,
}

pub async fn init(world: &mut World) {
    let global = world.spawn();
    world.insert(
        global,
        file::load_detect::<Config>(run_dir().join("assets").join("config.toml"))
            .await
            .unwrap(),
    );
    world.insert(
        global,
        Quests {
            active: default(),
            index_to_entity: default(),
            deliver: None,
        },
    );
    logic::init(world);
    world.add_handler(startup);
}

pub async fn post_init(world: &mut World) {
    net::init(world);
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

#[derive(Component)]
pub struct Quests {
    pub deliver: Option<usize>,
    pub active: HashSet<usize>,
    pub index_to_entity: HashMap<usize, EntityId>,
}

#[derive(Default, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Level {
    pub graph: RoadGraph,
    pub trees: Vec<Tree>,
    pub buildings: Vec<Building>,
    pub waypoints: Vec<Waypoint>,
    pub leaderboards: Vec<LeaderboardBillboard>,
}

impl Level {
    pub async fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let level = file::load_json::<Level>(path).await?;
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

#[derive(Component)]
pub struct CarPath {
    nodes: Vec<(f32, vec2<f32>)>,
    current_pos: f32,
}

#[allow(clippy::type_complexity)]
fn startup(
    receiver: Receiver<Startup>,
    config: Single<&Config>,
    mut rng: Single<&mut RngStuff>,
    mut quests: Single<&mut Quests>,
    mut sender: Sender<(
        Spawn,
        Insert<Vehicle>,
        Insert<VehicleController>,
        Insert<VehicleProperties>,
        Insert<LocalPlayer>,
        Insert<RoadGraph>,
        Insert<Building>,
        Insert<Car>,
        Insert<Bike>,
        Insert<Waypoint>,
        Insert<CarPath>,
        Insert<Shop>,
        Insert<LeaderboardBillboard>,
        Insert<Leaderboard>,
        Insert<Fish>,
    )>,
) {
    let startup = receiver.event;
    let level = &startup.level;

    let player = sender.spawn();
    sender.insert(player, LocalPlayer);
    sender.insert(player, Bike);
    sender.insert(
        player,
        Vehicle {
            pos: vec2(36.99582, 44.50808),
            ..Default::default()
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
    sender.insert(player, config.vehicle.clone());

    let fish = sender.spawn();
    sender.insert(
        fish,
        Fish {
            bike: player,
            local: true,
        },
    );

    let graph = sender.spawn();
    sender.insert(graph, level.graph.clone());

    for data in &level.buildings {
        let building = sender.spawn();
        sender.insert(
            building,
            Building {
                kind: data.kind,
                half_size: vec2::splat(if data.small { 0.8 } else { 4.0 }),
                pos: data.pos,
                rotation: data.rotation,
                small: data.small,
            },
        );
    }
    let shop = sender.spawn();
    sender.insert(
        shop,
        Shop {
            door_time: 0.0,
            pos: vec2(36.99582, 44.50808),
            rotation: Angle::from_degrees(60.0),
        },
    );

    for (index, data) in level.waypoints.iter().enumerate() {
        let waypoint = sender.spawn();
        quests.index_to_entity.insert(index, waypoint);
        sender.insert(waypoint, Waypoint { pos: data.pos });
    }

    for data in &level.leaderboards {
        let board = sender.spawn();
        sender.insert(board, data.clone());
    }

    for _ in 0..config.cars {
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
                speed: 6.0, // lmao ignore this, it makes our particles work kekw
            },
        );
        sender.insert(
            car,
            VehicleProperties {
                max_speed: 1.0,
                max_offroad_speed: 1.0,
                max_backward_speed: 1.0,
                acceleration: 1.0,
                auto_deceleration: 1.0,
                brake_deceleration: 1.0,
                max_rotation_speed: Angle::from_degrees(1.0),
                rotation_accel: Angle::from_degrees(1.0),
            },
        );
        let mut nodes = Vec::new();
        nodes.push({
            let start = level
                .graph
                .roads
                .iter()
                .map(|node| node.0)
                .choose(&mut rng.gen)
                .unwrap();
            let next = level.graph.out(start).choose(&mut rng.gen).unwrap();
            (start, next)
        });
        loop {
            let (prev, current) = *nodes.last().unwrap();
            if current == nodes.first().unwrap().0 && prev != nodes.first().unwrap().1 {
                break;
            }
            let next = level
                .graph
                .out(current)
                .filter(|&u| u != prev)
                .choose(&mut rng.gen)
                .unwrap_or(prev);
            nodes.push((current, next));
        }
        let mut nodes: Vec<_> = nodes
            .iter()
            .cycle()
            .copied()
            .take(nodes.len() + 1)
            .map(|(from, to)| {
                let from = &level.graph.roads[from];
                let to = &level.graph.roads[to];
                from.position
                    + (to.position - from.position)
                        .normalize_or_zero()
                        .rotate_90()
                        * from.half_width
                        / 2.0
            })
            .scan(None, |state, pos| match state {
                Some((dist, prev)) => {
                    *dist += (pos - *prev).len();
                    *prev = pos;
                    Some((*dist, pos))
                }
                None => {
                    *state = Some((0.0, pos));
                    Some((0.0, pos))
                }
            })
            .collect();
        // let path_len = nodes.last().unwrap().0;
        // let mut loops = (path_len / config.car_loop_length).round();
        // if loops <= 0.0 {
        //     loops = 1.0;
        // }
        // log::info!("loops = {loops:.0}");
        // let multiplier = loops * config.car_loop_length / path_len;
        // for node in &mut nodes {
        //     node.0 *= multiplier;
        // }
        sender.insert(
            car,
            CarPath {
                nodes,
                current_pos: 0.0,
            },
        )
    }
}
