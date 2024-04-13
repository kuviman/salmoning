use crate::{
    controls::GengEvent,
    model::*,
    render::{Camera, Draw},
};
use evenio::prelude::*;
use geng::prelude::*;

#[derive(Component)]
pub struct Editor {}

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
    world.add_handler(update_framebuffer_size);

    world.add_handler(click_to_append_to_road);
}

fn update_framebuffer_size(receiver: Receiver<Draw>, mut global: Single<&mut Global>) {
    global.framebuffer_size = receiver.event.framebuffer.size().map(|x| x as f32);
}

fn click_to_append_to_road(
    receiver: Receiver<GengEvent>,
    global: Single<&Global>,
    camera: Single<&Camera>,
    graph: Fetcher<(EntityId, &RoadGraph)>,
    mut sender: Sender<Insert<RoadGraph>>, // this way mesh is updated
) {
    let Some(cursor_pos) = global.geng.window().cursor_position() else {
        return;
    };
    let cursor_pos = cursor_pos.map(|x| x as f32);
    let geng::Event::MousePress {
        button: geng::MouseButton::Left,
    } = receiver.event.0
    else {
        return;
    };
    let click_world_pos = {
        let ray = camera.pixel_ray(global.framebuffer_size, cursor_pos);
        // ray.from + ray.dir * t = 0
        let t = -ray.from.z / ray.dir.z;
        ray.from.xy() + ray.dir.xy() * t
    };
    for (graph_entity, graph) in graph {
        let mut graph = graph.clone();
        if let Some((closest_road, _)) = graph
            .roads
            .iter()
            .min_by_key(|(_, road)| r32((road.position - click_world_pos).len()))
        {
            let new_road = graph.roads.insert(Road {
                half_width: 2.0,
                position: click_world_pos,
            });
            graph.connections.push([closest_road, new_road]);

            sender.insert(graph_entity, graph);
        }
    }
}
