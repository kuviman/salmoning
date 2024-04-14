use geng::prelude::itertools::Itertools;

use super::*;

pub fn setup_road_graphics(
    receiver: Receiver<Insert<RoadGraph>, ()>,
    global: Single<&Global>,
    mut sender: Sender<Insert<Object>>,
) {
    let graph = &receiver.event.component;
    let texture = &global.assets.road.road;
    sender.insert(
        receiver.event.entity,
        Object {
            parts: vec![ModelPart {
                mesh: Rc::new(ugli::VertexBuffer::new_static(
                    global.geng.ugli(),
                    generate_road_mesh(graph, texture),
                )),
                draw_mode: ugli::DrawMode::Triangles,
                texture: texture.clone(),
                transform: mat4::translate(vec3(0.0, 0.0, 0.1)),
                billboard: false,
            }],
            transform: mat4::identity(),
            replace_color: None,
        },
    );
}

fn generate_road_mesh(graph: &RoadGraph, texture: &ugli::Texture) -> Vec<Vertex> {
    let mut vertex_data: Vec<Vertex> = Vec::new();

    let mut g: HashMap<Index, Vec<Index>> = HashMap::new();
    for edge in &graph.connections {
        g.entry(edge[0]).or_default().push(edge[1]);
        g.entry(edge[1]).or_default().push(edge[0]);
    }
    for (&v, out) in &mut g {
        let v = &graph.roads[v];
        out.sort_by_key(|&u| {
            let u = &graph.roads[u];
            (u.position - v.position).arg().map(r32)
        });
        out.dedup();
    }

    let intersect = |a, b, c| {
        let a = &graph.roads[a];
        let b = &graph.roads[b];
        let c = &graph.roads[c];

        // For now
        assert_eq!(a.half_width, b.half_width);
        assert_eq!(a.half_width, c.half_width);

        let from =
            a.position - (b.position - a.position).normalize_or_zero().rotate_90() * a.half_width;
        let dir = b.position - a.position;
        let intersect_from =
            c.position - (c.position - b.position).normalize_or_zero().rotate_90() * c.half_width;
        let intersect_dir = c.position - b.position;

        // skew(from + dir * t - intersect_from, intersect_dir) = 0
        let t = vec2::skew(intersect_from - from, intersect_dir) / vec2::skew(dir, intersect_dir);
        from + dir * t
    };

    let make_points = |u, v| {
        let u_index = g[&v].iter().find_position(|&&thing| thing == u).unwrap().0;
        let u_prev = g[&v][(u_index + g[&v].len() - 1) % g[&v].len()];
        let u_next = g[&v][(u_index + 1) % g[&v].len()];
        if u_prev == u {
            let u = &graph.roads[u];
            let v = &graph.roads[v];
            let normal = (u.position - v.position).normalize_or_zero().rotate_90();
            [
                v.position - normal * v.half_width,
                v.position,
                v.position + normal * v.half_width,
            ]
        } else {
            [
                intersect(u_prev, v, u),
                graph.roads[v].position,
                intersect(u, v, u_next),
            ]
        }
    };

    let mut visited = HashSet::new();
    for &v in g.keys() {
        for &u in &g[&v] {
            if !visited.contains(&(u, v)) {
                let mut points = Vec::new();
                points.extend(make_points(v, u));
                points.extend(make_points(u, v));
                visited.insert((u, v));
                visited.insert((v, u));

                let v = &graph.roads[v];
                let u = &graph.roads[u];
                let dir = (u.position - v.position).normalize_or_zero();

                let points: Vec<Vertex> = points
                    .into_iter()
                    .map(|point| Vertex {
                        a_pos: point.extend(0.0),
                        a_uv: vec2(
                            vec2::skew(point - v.position, dir) / v.half_width * 0.5 + 0.5,
                            vec2::dot(point - v.position, dir) / v.half_width / 2.0
                                * texture.size().map(|x| x as f32).aspect(),
                        ),
                    })
                    .collect();

                for window in points[1..].windows(2) {
                    vertex_data.push(points[0]);
                    vertex_data.push(window[0]);
                    vertex_data.push(window[1]);
                }
            }
        }
    }

    vertex_data
}
