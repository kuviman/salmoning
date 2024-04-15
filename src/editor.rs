use crate::{
    controls::GengEvent,
    model::*,
    render::{self, Camera, Draw, Object},
};

use evenio::{prelude::*, world};
use generational_arena::Index;
use geng::prelude::*;

#[derive(Component)]
pub struct Editor {
    pub state: EditorState,
    pub level: Level,
    pub building_kind: i32,
    pub building_kind_small: i32,
    pub building_small: bool,
    pub tree_kind: i32,
    pub road_history: Vec<RoadGraph>,
    pub road_redo: Vec<RoadGraph>,
}

pub enum EditorState {
    Roads,
    ExtendRoad(Index),
    MoveRoad(Index),
    Trees,
    EditTree(usize, EntityId),
    MoveTree(usize, EntityId),
    Buildings,
    EditBuilding(usize, EntityId),
    MoveBuilding(usize, EntityId),
    Waypoints,
    EditWaypoint(usize, EntityId),
    MoveWaypoint(usize, EntityId),
    Leaderboards,
    EditLeaderboard(usize, EntityId),
    MoveLeaderboard(usize, EntityId),
}

#[derive(Component)]
struct Global {
    geng: Geng,
    framebuffer_size: vec2<f32>,
}

pub async fn init(world: &mut World, geng: &Geng, level: Level) {
    let global = world.spawn();
    world.insert(
        global,
        Global {
            geng: geng.clone(),
            framebuffer_size: vec2::splat(1.0),
        },
    );

    let editor = world.spawn();
    world.insert(
        editor,
        Editor {
            state: EditorState::Roads,
            level,
            tree_kind: 0,
            building_kind: 0,
            road_history: Vec::new(),
            road_redo: Vec::new(),
            building_kind_small: 0,
            building_small: false,
        },
    );

    world.add_handler(update_framebuffer_size);
    world.add_handler(update_graph);

    world.add_handler(move_stuff);
    world.add_handler(scroll);

    world.add_handler(click_road);
    world.add_handler(click_tree);
    world.add_handler(click_waypoint);
    world.add_handler(click_building);
    world.add_handler(click_leaderboard);
    world.add_handler(event_handler);
}

fn update_framebuffer_size(receiver: Receiver<Draw>, mut global: Single<&mut Global>) {
    global.framebuffer_size = receiver.event.framebuffer.size().map(|x| x as f32);
}

fn update_graph(receiver: Receiver<Insert<RoadGraph>, ()>, mut editor: Single<&mut Editor>) {
    editor.level.graph = receiver.event.component.clone()
}

#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
fn move_stuff(
    receiver: Receiver<Update>,
    global: Single<&Global>,
    camera: Single<&Camera>,
    buildings: Fetcher<&Building>,
    trees: Fetcher<&Tree>,
    boards: Fetcher<&LeaderboardBillboard>,
    graph: Single<(EntityId, &RoadGraph)>,
    mut editor: Single<&mut Editor>,
    mut sender: Sender<(
        Insert<Building>,
        Insert<Waypoint>,
        Insert<Tree>,
        Insert<LeaderboardBillboard>,
        Insert<RoadGraph>,
    )>,
) {
    let Some(cursor_pos) = global.geng.window().cursor_position() else {
        return;
    };
    let cursor_pos = cursor_pos.map(|x| x as f32);
    let click_world_pos = {
        let ray = camera.pixel_ray(global.framebuffer_size, cursor_pos);
        // ray.from + ray.dir * t = 0
        let t = -ray.from.z / ray.dir.z;
        ray.from.xy() + ray.dir.xy() * t
    };

    match editor.state {
        EditorState::MoveRoad(idx) => {
            let (graph_entity, graph) = *graph;
            let mut graph = graph.clone();
            graph.roads[idx].position = click_world_pos;
            editor.road_history.push(graph.clone());
            sender.insert(graph_entity, graph);
        }
        EditorState::MoveWaypoint(idx, entity_id) => {
            editor.level.waypoints[idx].pos = click_world_pos;
            sender.insert(
                entity_id,
                Waypoint {
                    pos: click_world_pos,
                },
            );
        }
        EditorState::MoveBuilding(idx, entity_id) => {
            editor.level.buildings[idx].pos = click_world_pos;
            let stuff = buildings.get(entity_id).unwrap();
            sender.insert(
                entity_id,
                Building {
                    half_size: stuff.half_size,
                    pos: click_world_pos,
                    rotation: stuff.rotation,
                    kind: stuff.kind,
                    small: stuff.small,
                },
            );
        }
        EditorState::MoveTree(idx, entity_id) => {
            editor.level.trees[idx].pos = click_world_pos;
            let stuff = trees.get(entity_id).unwrap();
            sender.insert(
                entity_id,
                Tree {
                    pos: click_world_pos,
                    rotation: stuff.rotation,
                    kind: stuff.kind,
                },
            );
        }
        EditorState::MoveLeaderboard(idx, entity_id) => {
            editor.level.leaderboards[idx].pos = click_world_pos;
            let stuff = boards.get(entity_id).unwrap();
            sender.insert(
                entity_id,
                LeaderboardBillboard {
                    kind: stuff.kind,
                    pos: click_world_pos,
                    rotation: stuff.rotation,
                },
            );
        }
        _ => {}
    };
}

#[allow(clippy::type_complexity)]
fn scroll(
    receiver: Receiver<GengEvent>,
    render_global: Single<&render::Global>,
    buildings: Fetcher<&Building>,
    trees: Fetcher<&Tree>,
    boards: Fetcher<&LeaderboardBillboard>,
    mut editor: Single<&mut Editor>,
    mut sender: Sender<(Insert<Building>, Insert<Tree>, Insert<LeaderboardBillboard>)>,
) {
    let geng::Event::Wheel { delta } = receiver.event.0 else {
        return;
    };

    match editor.state {
        EditorState::MoveBuilding(idx, entity_id) => {
            let stuff = buildings.get(entity_id).unwrap();
            let new_rot = stuff.rotation + Angle::from_degrees(delta as f32 / 20.0);
            editor.level.buildings[idx].rotation = new_rot;
            sender.insert(
                entity_id,
                Building {
                    half_size: stuff.half_size,
                    pos: stuff.pos,
                    rotation: new_rot,
                    kind: stuff.kind,
                    small: stuff.small,
                },
            );
        }
        EditorState::MoveTree(idx, entity_id) => {
            let stuff = trees.get(entity_id).unwrap();
            let new_rot = stuff.rotation + Angle::from_degrees(delta as f32 / 20.0);
            editor.level.trees[idx].rotation = new_rot;
            sender.insert(
                entity_id,
                Tree {
                    pos: stuff.pos,
                    rotation: new_rot,
                    kind: stuff.kind,
                },
            );
        }
        EditorState::MoveLeaderboard(idx, entity_id) => {
            let stuff = boards.get(entity_id).unwrap();
            let new_rot = stuff.rotation + Angle::from_degrees(delta as f32 / 20.0);
            editor.level.leaderboards[idx].rotation = new_rot;
            sender.insert(
                entity_id,
                LeaderboardBillboard {
                    kind: stuff.kind,
                    pos: stuff.pos,
                    rotation: new_rot,
                },
            );
        }
        EditorState::EditLeaderboard(idx, entity_id) => {
            let stuff = boards.get(entity_id).unwrap();
            let new_kind = (stuff.kind.unwrap_or(render_global.assets.billboards.len()) + 1)
                % (render_global.assets.billboards.len() + 1);
            let new_kind = if new_kind < render_global.assets.billboards.len() {
                Some(new_kind)
            } else {
                None
            };
            editor.level.leaderboards[idx].kind = new_kind;
            sender.insert(
                entity_id,
                LeaderboardBillboard {
                    kind: new_kind,
                    pos: stuff.pos,
                    rotation: stuff.rotation,
                },
            );
        }
        EditorState::EditBuilding(idx, entity_id) => {
            let count: i32 = if editor.building_small {
                render_global.0.assets.small_items.len().try_into().unwrap()
            } else {
                render_global.0.assets.buildings.len().try_into().unwrap()
            };
            let stuff = buildings.get(entity_id).unwrap();
            let diff = if delta < 0.0 { -1 } else { 1 };
            let mut new_kind = stuff.kind + diff;
            if new_kind < 0 {
                new_kind = count - 1;
            } else if new_kind >= count {
                new_kind = 0;
            }
            editor.level.buildings[idx].kind = new_kind;
            if editor.building_small {
                editor.building_kind_small = new_kind;
            } else {
                editor.building_kind = new_kind;
            }
            sender.insert(
                entity_id,
                Building {
                    half_size: stuff.half_size,
                    pos: stuff.pos,
                    rotation: stuff.rotation,
                    kind: new_kind,
                    small: stuff.small,
                },
            )
        }
        EditorState::EditTree(idx, entity_id) => {
            let assets = &render_global.0.assets;
            let count: i32 = assets.flora.len().try_into().unwrap();
            let stuff = trees.get(entity_id).unwrap();
            let diff = if delta < 0.0 { -1 } else { 1 };
            let mut new_kind = stuff.kind + diff;
            if new_kind < 0 {
                new_kind = count - 1;
            } else if new_kind >= count {
                new_kind = 0;
            }
            editor.level.trees[idx].kind = new_kind;
            editor.tree_kind = new_kind;
            sender.insert(
                entity_id,
                Tree {
                    pos: stuff.pos,
                    rotation: stuff.rotation,
                    kind: new_kind,
                },
            )
        }
        _ => {}
    };
}

#[allow(clippy::type_complexity)]
fn click_tree(
    receiver: Receiver<GengEvent>,
    global: Single<&Global>,
    mut rng: Single<&mut RngStuff>,
    fetcher: Fetcher<(&Tree, EntityId)>,
    camera: Single<&Camera>,
    mut editor: Single<&mut Editor>,
    mut sender: Sender<(Spawn, Despawn, Insert<Object>, Insert<Tree>)>,
) {
    let geng::Event::MousePress { button } = receiver.event.0 else {
        return;
    };

    let Some(cursor_pos) = global.geng.window().cursor_position() else {
        return;
    };

    match editor.state {
        EditorState::Trees | EditorState::EditTree(_, _) => {}
        _ => {
            return;
        }
    }

    let cursor_pos = cursor_pos.map(|x| x as f32);

    let click_world_pos = {
        let ray = camera.pixel_ray(global.framebuffer_size, cursor_pos);
        // ray.from + ray.dir * t = 0
        let t = -ray.from.z / ray.dir.z;
        ray.from.xy() + ray.dir.xy() * t
    };

    match button {
        geng::MouseButton::Right => {
            if let Some((i, data)) = hover_item(
                click_world_pos,
                editor.level.trees.iter().enumerate(),
                |(_, data)| data.pos,
            ) {
                if let Some((_, tree)) =
                    hover_item(data.pos, fetcher.iter().enumerate(), |(_, tree)| tree.0.pos)
                {
                    sender.despawn(tree.1)
                }
                editor.level.trees.swap_remove(i);
                editor.state = EditorState::Trees;
            }
        }
        geng::MouseButton::Left => {
            // Select a node
            if let Some((idx, data)) = hover_item(
                click_world_pos,
                editor.level.trees.iter().enumerate(),
                |(_, data)| data.pos,
            ) {
                if let Some((_, tree)) =
                    hover_item(data.pos, fetcher.iter().enumerate(), |(_, tree)| tree.0.pos)
                {
                    editor.state = EditorState::EditTree(idx, tree.1);
                }
            }
        }
        geng::MouseButton::Middle => {
            let tree = sender.spawn();
            let data = Tree {
                rotation: Angle::from_degrees(rng.gen_range(0.0..360.0)),
                kind: editor.tree_kind,
                pos: click_world_pos,
            };
            sender.insert(tree, data.clone());
            editor.level.trees.push(data);
            editor.state = EditorState::EditTree(editor.level.trees.len() - 1, tree);
        }
    };
}

#[allow(clippy::type_complexity)]
fn click_leaderboard(
    receiver: Receiver<GengEvent>,
    global: Single<&Global>,
    mut rng: Single<&mut RngStuff>,
    fetcher: Fetcher<(&LeaderboardBillboard, EntityId)>,
    camera: Single<&Camera>,
    mut editor: Single<&mut Editor>,
    mut sender: Sender<(Spawn, Despawn, Insert<Object>, Insert<LeaderboardBillboard>)>,
) {
    let geng::Event::MousePress { button } = receiver.event.0 else {
        return;
    };

    let Some(cursor_pos) = global.geng.window().cursor_position() else {
        return;
    };

    match editor.state {
        EditorState::Leaderboards | EditorState::EditLeaderboard(_, _) => {}
        _ => {
            return;
        }
    }

    let cursor_pos = cursor_pos.map(|x| x as f32);

    let click_world_pos = {
        let ray = camera.pixel_ray(global.framebuffer_size, cursor_pos);
        // ray.from + ray.dir * t = 0
        let t = -ray.from.z / ray.dir.z;
        ray.from.xy() + ray.dir.xy() * t
    };

    match button {
        geng::MouseButton::Right => {
            if let Some((i, data)) = hover_item(
                click_world_pos,
                editor.level.leaderboards.iter().enumerate(),
                |(_, data)| data.pos,
            ) {
                if let Some((_, board)) =
                    hover_item(data.pos, fetcher.iter().enumerate(), |(_, board)| {
                        board.0.pos
                    })
                {
                    sender.despawn(board.1)
                }
                editor.level.leaderboards.swap_remove(i);
                editor.state = EditorState::Leaderboards;
            }
        }
        geng::MouseButton::Left => {
            // Select a node
            if let Some((idx, data)) = hover_item(
                click_world_pos,
                editor.level.leaderboards.iter().enumerate(),
                |(_, data)| data.pos,
            ) {
                if let Some((_, board)) =
                    hover_item(data.pos, fetcher.iter().enumerate(), |(_, board)| {
                        board.0.pos
                    })
                {
                    editor.state = EditorState::EditLeaderboard(idx, board.1);
                }
            }
        }
        geng::MouseButton::Middle => {
            let board = sender.spawn();
            let data = LeaderboardBillboard {
                kind: None,
                rotation: Angle::from_degrees(rng.gen_range(0.0..360.0)),
                pos: click_world_pos,
            };
            sender.insert(board, data.clone());
            editor.level.leaderboards.push(data);
            editor.state = EditorState::EditLeaderboard(editor.level.leaderboards.len() - 1, board);
        }
    };
}

#[allow(clippy::type_complexity)]
fn click_waypoint(
    receiver: Receiver<GengEvent>,
    global: Single<&Global>,
    fetcher: Fetcher<(&Waypoint, EntityId)>,
    camera: Single<&Camera>,
    mut editor: Single<&mut Editor>,
    mut sender: Sender<(Spawn, Despawn, Insert<Object>, Insert<Waypoint>)>,
) {
    let geng::Event::MousePress { button } = receiver.event.0 else {
        return;
    };

    let Some(cursor_pos) = global.geng.window().cursor_position() else {
        return;
    };

    match editor.state {
        EditorState::Waypoints | EditorState::EditWaypoint(_, _) => {}
        _ => {
            return;
        }
    }

    let cursor_pos = cursor_pos.map(|x| x as f32);

    let click_world_pos = {
        let ray = camera.pixel_ray(global.framebuffer_size, cursor_pos);
        // ray.from + ray.dir * t = 0
        let t = -ray.from.z / ray.dir.z;
        ray.from.xy() + ray.dir.xy() * t
    };

    match button {
        geng::MouseButton::Right => {
            if let Some((i, data)) = hover_item(
                click_world_pos,
                editor.level.waypoints.iter().enumerate(),
                |(_, data)| data.pos,
            ) {
                if let Some((_, waypoint)) =
                    hover_item(data.pos, fetcher.iter().enumerate(), |(_, waypoint)| {
                        waypoint.0.pos
                    })
                {
                    sender.despawn(waypoint.1)
                }
                editor.level.waypoints.swap_remove(i);
                editor.state = EditorState::Waypoints;
            }
        }
        geng::MouseButton::Left => {
            // Select a node
            if let Some((idx, data)) = hover_item(
                click_world_pos,
                editor.level.waypoints.iter().enumerate(),
                |(_, data)| data.pos,
            ) {
                if let Some((_, waypoint)) =
                    hover_item(data.pos, fetcher.iter().enumerate(), |(_, waypoint)| {
                        waypoint.0.pos
                    })
                {
                    editor.state = EditorState::EditWaypoint(idx, waypoint.1);
                }
            }
        }
        geng::MouseButton::Middle => {
            let waypoint = sender.spawn();
            let data = Waypoint {
                pos: click_world_pos,
            };
            sender.insert(waypoint, data.clone());
            editor.level.waypoints.push(data);
            editor.state = EditorState::EditWaypoint(editor.level.waypoints.len() - 1, waypoint);
        }
    };
}

#[allow(clippy::type_complexity)]
fn click_building(
    receiver: Receiver<GengEvent>,
    global: Single<&Global>,
    camera: Single<&Camera>,
    fetcher: Fetcher<(&Building, EntityId)>,
    mut editor: Single<&mut Editor>,
    mut sender: Sender<(Spawn, Insert<Building>, Despawn)>,
) {
    let geng::Event::MousePress { button } = receiver.event.0 else {
        return;
    };

    let Some(cursor_pos) = global.geng.window().cursor_position() else {
        return;
    };

    match editor.state {
        EditorState::Buildings | EditorState::EditBuilding(_, _) => {}
        _ => {
            return;
        }
    }
    let cursor_pos = cursor_pos.map(|x| x as f32);

    let click_world_pos = {
        let ray = camera.pixel_ray(global.framebuffer_size, cursor_pos);
        // ray.from + ray.dir * t = 0
        let t = -ray.from.z / ray.dir.z;
        ray.from.xy() + ray.dir.xy() * t
    };

    match button {
        geng::MouseButton::Right => {
            if let Some((i, data)) = hover_item(
                click_world_pos,
                editor.level.buildings.iter().enumerate(),
                |(_, data)| data.pos,
            ) {
                if data.small != editor.building_small {
                    return;
                }
                if let Some((_, building)) =
                    hover_item(data.pos, fetcher.iter().enumerate(), |(_, data)| data.0.pos)
                {
                    sender.despawn(building.1)
                }
                editor.level.buildings.swap_remove(i);
                editor.state = EditorState::Buildings;
            }
        }
        geng::MouseButton::Left => {
            // Select a node
            if let Some((idx, data)) = hover_item(
                click_world_pos,
                editor.level.buildings.iter().enumerate(),
                |(_, data)| data.pos,
            ) {
                if data.small != editor.building_small {
                    return;
                }
                if let Some((_, building)) =
                    hover_item(data.pos, fetcher.iter().enumerate(), |(_, building)| {
                        building.0.pos
                    })
                {
                    editor.state = EditorState::EditBuilding(idx, building.1);
                }
            }
        }
        geng::MouseButton::Middle => {
            let building = sender.spawn();
            let kind = if editor.building_small {
                editor.building_kind_small
            } else {
                editor.building_kind
            };
            let data = Building {
                half_size: vec2::splat(if editor.building_small { 0.8 } else { 4.0 }),
                rotation: Angle::ZERO,
                kind,
                pos: click_world_pos,
                small: editor.building_small,
            };
            sender.insert(building, data.clone());
            editor.level.buildings.push(data);
            editor.state = EditorState::EditBuilding(editor.level.buildings.len() - 1, building);
        }
    };
}

#[allow(clippy::type_complexity)]
fn click_road(
    receiver: Receiver<GengEvent>,
    global: Single<&Global>,
    camera: Single<&Camera>,
    mut editor: Single<&mut Editor>,
    graph: Single<(EntityId, &RoadGraph)>,
    mut sender: Sender<(Spawn, Insert<RoadGraph>)>,
) {
    let geng::Event::MousePress { button } = receiver.event.0 else {
        return;
    };

    let Some(cursor_pos) = global.geng.window().cursor_position() else {
        return;
    };
    match editor.state {
        EditorState::Roads | EditorState::ExtendRoad(_) => {}
        _ => {
            return;
        }
    }

    let cursor_pos = cursor_pos.map(|x| x as f32);

    let click_world_pos = {
        let ray = camera.pixel_ray(global.framebuffer_size, cursor_pos);
        // ray.from + ray.dir * t = 0
        let t = -ray.from.z / ray.dir.z;
        ray.from.xy() + ray.dir.xy() * t
    };

    let (graph_entity, graph) = *graph;

    match button {
        geng::MouseButton::Right => {
            // Remove a node
            if let Some((idx, _)) = hover_item(click_world_pos, graph.roads.iter(), |(_, road)| {
                road.position
            }) {
                let mut graph = graph.clone();

                graph.roads.remove(idx);
                graph.connections.retain(|ids| !ids.contains(&idx));
                editor.state = EditorState::Roads;
                editor.road_history.push(graph.clone());

                sender.insert(graph_entity, graph);
            }
        }
        geng::MouseButton::Left => {
            match &mut editor.state {
                EditorState::Roads => {
                    // Select a node
                    if let Some((idx, _)) =
                        hover_item(click_world_pos, graph.roads.iter(), |(_, road)| {
                            road.position
                        })
                    {
                        editor.state = EditorState::ExtendRoad(idx);
                    }
                }
                &mut EditorState::ExtendRoad(idx) => {
                    // Extend road
                    let mut graph = graph.clone();

                    let connect = hover_item(click_world_pos, graph.roads.iter(), |(_, road)| {
                        road.position
                    })
                    .map(|(idx, _)| idx);

                    let connect_idx = connect.unwrap_or_else(|| {
                        graph.roads.insert(Road {
                            half_width: 3.0,
                            position: click_world_pos,
                        })
                    });
                    graph.connections.push([idx, connect_idx]);
                    editor.state = EditorState::ExtendRoad(connect_idx);
                    editor.road_history.push(graph.clone());

                    sender.insert(graph_entity, graph);
                }
                _ => {}
            }
        }
        geng::MouseButton::Middle => {
            // Spawn an independent node
            let mut graph = graph.clone();

            let new_road = graph.roads.insert(Road {
                half_width: 3.0,
                position: click_world_pos,
            });
            editor.state = EditorState::ExtendRoad(new_road);
            editor.road_history.push(graph.clone());

            sender.insert(graph_entity, graph);
        }
    };
}

fn event_handler(
    receiver: Receiver<GengEvent>,
    global: Single<&Global>,
    graph: Single<(EntityId, &RoadGraph)>,
    mut editor: Single<&mut Editor>,
    mut sender: Sender<(Spawn, Insert<RoadGraph>)>,
) {
    if let geng::Event::KeyRelease { key } = receiver.event.0 {
        if let geng::Key::AltLeft = key {
            if let EditorState::MoveBuilding(a, b) = editor.state {
                editor.state = EditorState::EditBuilding(a, b);
            }
            if let EditorState::MoveTree(a, b) = editor.state {
                editor.state = EditorState::EditTree(a, b);
            }
            if let EditorState::MoveRoad(a) = editor.state {
                editor.state = EditorState::ExtendRoad(a);
            }
            if let EditorState::MoveWaypoint(a, b) = editor.state {
                editor.state = EditorState::EditWaypoint(a, b);
            }
            if let EditorState::MoveLeaderboard(a, b) = editor.state {
                editor.state = EditorState::EditLeaderboard(a, b);
            }
        };
    } else if let geng::Event::KeyPress { key } = receiver.event.0 {
        match key {
            geng::Key::AltLeft => {
                if let EditorState::EditBuilding(a, b) = editor.state {
                    editor.state = EditorState::MoveBuilding(a, b);
                }
                if let EditorState::EditTree(a, b) = editor.state {
                    editor.state = EditorState::MoveTree(a, b);
                }
                if let EditorState::ExtendRoad(a) = editor.state {
                    editor.state = EditorState::MoveRoad(a);
                }
                if let EditorState::EditWaypoint(a, b) = editor.state {
                    editor.state = EditorState::MoveWaypoint(a, b);
                }
                if let EditorState::EditLeaderboard(a, b) = editor.state {
                    editor.state = EditorState::MoveLeaderboard(a, b);
                }
            }
            geng::Key::Escape => {
                if let EditorState::ExtendRoad(_) = editor.state {
                    editor.state = EditorState::Roads;
                }
                if let EditorState::EditBuilding(_, _) = editor.state {
                    editor.state = EditorState::Buildings;
                }
                if let EditorState::EditTree(_, _) = editor.state {
                    editor.state = EditorState::Trees;
                }
                if let EditorState::EditWaypoint(_, _) = editor.state {
                    editor.state = EditorState::Waypoints;
                }
                if let EditorState::EditLeaderboard(_, _) = editor.state {
                    editor.state = EditorState::Leaderboards;
                }
            }
            geng::Key::S if global.geng.window().is_key_pressed(geng::Key::ControlLeft) => {
                editor.save();
            }
            geng::Key::Z if global.geng.window().is_key_pressed(geng::Key::ControlLeft) => {
                if let EditorState::Roads | EditorState::ExtendRoad(_) = editor.state {
                    if global.geng.window().is_key_pressed(geng::Key::ShiftLeft) {
                        editor.redo(graph.0 .0, sender);
                    } else {
                        editor.undo(graph.0 .0, sender);
                    }
                }
            }
            geng::Key::Digit1 => {
                editor.state = EditorState::Roads;
            }
            geng::Key::Digit2 => {
                editor.state = EditorState::Trees;
            }
            geng::Key::Digit3 => {
                editor.state = EditorState::Buildings;
                editor.building_small = false;
            }
            geng::Key::Digit4 => {
                editor.state = EditorState::Waypoints;
            }
            geng::Key::Digit5 => {
                editor.state = EditorState::Buildings;
                editor.building_small = true;
            }
            geng::Key::Digit6 => {
                editor.state = EditorState::Leaderboards;
            }
            _ => {}
        }
    }
}

impl Editor {
    pub fn save(&self) {
        let path = run_dir().join("assets").join("level.json");
        #[cfg(not(target = "wasm32"))]
        {
            let func = || {
                let level = serde_json::to_string(&self.level)?;
                std::fs::write(&path, level)?;
                log::info!("Save the level");
                anyhow::Ok(())
            };
            if let Err(err) = func() {
                log::error!("Failed to save the level: {:?}", err);
            }
        }
    }

    pub fn undo(&mut self, graph_id: EntityId, mut sender: Sender<(Spawn, Insert<RoadGraph>)>) {
        if let Some(state) = self.road_history.pop() {
            self.road_redo.push(self.level.graph.clone());
            self.level.graph = state.clone();
            sender.insert(graph_id, state);
        }
    }

    pub fn redo(&mut self, graph_id: EntityId, mut sender: Sender<(Spawn, Insert<RoadGraph>)>) {
        if let Some(state) = self.road_redo.pop() {
            self.road_history.push(self.level.graph.clone());
            self.level.graph = state.clone();
            sender.insert(graph_id, state);
        }
    }
}

fn hover_item<T>(
    pos: vec2<f32>,
    items: impl Iterator<Item = T>,
    f: impl Fn(&T) -> vec2<f32>,
) -> Option<T> {
    if let Some(item) = items.min_by_key(|item| r32((f(item) - pos).len())) {
        if (f(&item) - pos).len() < 1.0 {
            return Some(item);
        }
    }
    None
}
