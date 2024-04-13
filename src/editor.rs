use crate::{
    controls::GengEvent,
    model::*,
    render::{Camera, Draw},
};

use evenio::prelude::*;
use generational_arena::Index;
use geng::prelude::*;

#[derive(Component)]
pub struct Editor {
    pub state: EditorState,
}

pub enum EditorState {
    Idle,
    ExtendRoad(Index),
}

#[derive(Component)]
struct Global {
    geng: Geng,
    framebuffer_size: vec2<f32>,
}

pub async fn init(world: &mut World, geng: &Geng) {
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
            state: EditorState::Idle,
        },
    );

    world.add_handler(update_framebuffer_size);

    world.add_handler(click);
    world.add_handler(event_handler);
}

fn update_framebuffer_size(receiver: Receiver<Draw>, mut global: Single<&mut Global>) {
    global.framebuffer_size = receiver.event.framebuffer.size().map(|x| x as f32);
}

fn click(
    receiver: Receiver<GengEvent>,
    global: Single<&Global>,
    camera: Single<&Camera>,
    mut editor: Single<&mut Editor>,
    graph: Single<(EntityId, &RoadGraph)>,
    mut sender: Sender<Insert<RoadGraph>>, // this way mesh is updated
) {
    let geng::Event::MousePress {
        button: geng::MouseButton::Left,
    } = receiver.event.0
    else {
        return;
    };

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

    let (graph_entity, graph) = *graph;

    match &mut editor.state {
        EditorState::Idle => {
            // Select a node
            let dist = |road: &Road| (road.position - click_world_pos).len();
            if let Some((idx, road)) = graph.roads.iter().min_by_key(|(_, road)| r32(dist(road))) {
                if dist(road) < 1.0 {
                    editor.state = EditorState::ExtendRoad(idx);
                }
            }
        }
        &mut EditorState::ExtendRoad(idx) => {
            // Extend road
            let mut graph = graph.clone();

            let dist = |road: &Road| (road.position - click_world_pos).len();
            let mut connect = None;
            if let Some((idx, road)) = graph.roads.iter().min_by_key(|(_, road)| r32(dist(road))) {
                if dist(road) < 1.0 {
                    connect = Some(idx);
                }
            }

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
    }
}

fn event_handler(
    receiver: Receiver<GengEvent>,
    global: Single<&Global>,
    graph: Single<&RoadGraph>,
    mut editor: Single<&mut Editor>,
) {
    if let geng::Event::KeyPress { key } = receiver.event.0 {
        match key {
            geng::Key::Escape => {
                if let EditorState::ExtendRoad(_) = editor.state {
                    editor.state = EditorState::Idle;
                }
            }
            geng::Key::S if global.geng.window().is_key_pressed(geng::Key::ControlLeft) => {
                editor.save(&graph);
            }
            _ => {}
        }
    }
}

impl Editor {
    pub fn save(&self, graph: &RoadGraph) {
        let path = run_dir().join("assets").join("level");
        #[cfg(not(target = "wasm32"))]
        {
            let func = || {
                let level = bincode::serialize(graph)?;
                std::fs::write(&path, level)?;
                anyhow::Ok(())
            };
            if let Err(err) = func() {
                log::error!("Failed to save the level: {:?}", err);
            }
        }
    }
}
