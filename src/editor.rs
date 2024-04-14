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
    pub tree_kind: i32,
}

pub enum EditorState {
    Roads,
    ExtendRoad(Index),
    Trees,
    EditTree(usize, EntityId),
    MoveTree(usize, EntityId),
    Buildings,
    EditBuilding(usize, EntityId),
    MoveBuilding(usize, EntityId),
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
        },
    );

    world.add_handler(update_framebuffer_size);
    world.add_handler(update_graph);

    world.add_handler(move_stuff);
    world.add_handler(scroll);

    world.add_handler(click_road);
    world.add_handler(click_tree);
    world.add_handler(click_building);
    world.add_handler(event_handler);
}

fn update_framebuffer_size(receiver: Receiver<Draw>, mut global: Single<&mut Global>) {
    global.framebuffer_size = receiver.event.framebuffer.size().map(|x| x as f32);
}

fn update_graph(receiver: Receiver<Insert<RoadGraph>, ()>, mut editor: Single<&mut Editor>) {
    editor.level.graph = receiver.event.component.clone();
}

#[allow(clippy::type_complexity)]
fn move_stuff(
    receiver: Receiver<GengEvent>,
    global: Single<&Global>,
    camera: Single<&Camera>,
    buildings: Fetcher<&Building>,
    trees: Fetcher<&Tree>,
    mut editor: Single<&mut Editor>,
    mut sender: Sender<(Insert<Building>, Insert<Tree>)>,
) {
    let geng::Event::CursorMove { position } = receiver.event.0 else {
        return;
    };

    let cursor_pos = position.map(|x| x as f32);
    let click_world_pos = {
        let ray = camera.pixel_ray(global.framebuffer_size, cursor_pos);
        // ray.from + ray.dir * t = 0
        let t = -ray.from.z / ray.dir.z;
        ray.from.xy() + ray.dir.xy() * t
    };

    match editor.state {
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
        _ => {}
    };
}

#[allow(clippy::type_complexity)]
fn scroll(
    receiver: Receiver<GengEvent>,
    render_global: Single<&render::Global>,
    buildings: Fetcher<&Building>,
    trees: Fetcher<&Tree>,
    mut editor: Single<&mut Editor>,
    mut sender: Sender<(Insert<Building>, Insert<Tree>)>,
) {
    let geng::Event::Wheel { delta } = receiver.event.0 else {
        return;
    };

    match editor.state {
        EditorState::EditBuilding(_, entity_id) => {
            let assets = &render_global.0.assets;
            let count: i32 = assets.buildings.len().try_into().unwrap();
            let stuff = buildings.get(entity_id).unwrap();
            let diff = if delta < 0.0 { -1 } else { 1 };
            let mut new_kind = stuff.kind + diff;
            if new_kind < 0 {
                new_kind = count - 1;
            } else if new_kind >= count {
                new_kind = 0;
            }
            editor.building_kind = new_kind;
            sender.insert(
                entity_id,
                Building {
                    half_size: stuff.half_size,
                    pos: stuff.pos,
                    rotation: stuff.rotation,
                    kind: new_kind,
                },
            )
        }
        EditorState::EditTree(_, entity_id) => {
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
            let kind = editor.building_kind;
            let data = Building {
                half_size: vec2::splat(4.0),
                rotation: Angle::ZERO,
                kind,
                pos: click_world_pos,
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
                            half_width: 2.0,
                            position: click_world_pos,
                        })
                    });
                    graph.connections.push([idx, connect_idx]);
                    editor.state = EditorState::ExtendRoad(connect_idx);

                    sender.insert(graph_entity, graph);
                }
                _ => {}
            }
        }
        geng::MouseButton::Middle => {
            // Spawn an independent node
            let mut graph = graph.clone();

            let new_road = graph.roads.insert(Road {
                half_width: 2.0,
                position: click_world_pos,
            });
            editor.state = EditorState::ExtendRoad(new_road);

            sender.insert(graph_entity, graph);
        }
    };
}

fn event_handler(
    receiver: Receiver<GengEvent>,
    global: Single<&Global>,
    mut editor: Single<&mut Editor>,
) {
    if let geng::Event::KeyRelease { key } = receiver.event.0 {
        match key {
            geng::Key::AltLeft => {
                if let EditorState::MoveBuilding(a, b) = editor.state {
                    editor.state = EditorState::EditBuilding(a, b);
                }
                if let EditorState::MoveTree(a, b) = editor.state {
                    editor.state = EditorState::EditTree(a, b);
                }
            }
            _ => {}
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
            }
            geng::Key::S if global.geng.window().is_key_pressed(geng::Key::ControlLeft) => {
                editor.save();
            }
            geng::Key::Digit1 => {
                editor.state = EditorState::Roads;
            }
            geng::Key::Digit2 => {
                editor.state = EditorState::Trees;
            }
            geng::Key::Digit3 => {
                editor.state = EditorState::Buildings;
            }
            _ => {}
        }
    }
}

impl Editor {
    pub fn save(&self) {
        let path = run_dir().join("assets").join("level");
        #[cfg(not(target = "wasm32"))]
        {
            let func = || {
                let level = bincode::serialize(&self.level)?;
                std::fs::write(&path, level)?;
                log::info!("Save the level");
                anyhow::Ok(())
            };
            if let Err(err) = func() {
                log::error!("Failed to save the level: {:?}", err);
            }
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
