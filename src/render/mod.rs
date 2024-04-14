use crate::{
    assets::{Assets, Texture},
    editor::{Editor, EditorState},
    model::*,
};

use evenio::{prelude::*, query};
use generational_arena::Index;
use geng::prelude::*;

#[derive(Event)]
pub struct Draw {
    pub framebuffer: &'static mut ugli::Framebuffer<'static>,
}

pub struct ModelPart {
    pub mesh: Rc<ugli::VertexBuffer<Vertex>>,
    pub draw_mode: ugli::DrawMode,
    pub texture: Texture,
    pub transform: mat4<f32>,
    pub billboard: bool,
}

#[derive(Component)]
pub struct Object {
    pub parts: Vec<ModelPart>,
    pub transform: mat4<f32>,
    pub replace_color: Option<Rgba<f32>>,
}

fn clear(mut receiver: ReceiverMut<Draw>) {
    let framebuffer = &mut *receiver.event.framebuffer;
    ugli::clear(framebuffer, Some(Rgba::BLUE), Some(1.0), None);
}

#[derive(ugli::Vertex, Clone, Copy)]
pub struct Vertex {
    pub a_pos: vec3<f32>,
    pub a_uv: vec2<f32>,
}

#[derive(Deserialize)]
struct CameraConfig {
    distance: f32,
    fov: f32,
    default_rotation: f32,
    attack_angle: f32,
    offset: f32,
    predict: f32,
    speed: f32,
    auto_rotate: bool,
}

#[derive(Deserialize)]
struct Config {
    pixels_per_unit: f32,
    camera: CameraConfig,
}

#[derive(Component)]
pub struct Global {
    pub geng: Geng,
    pub timer: Timer,
    pub config: Rc<Config>,
    pub assets: Rc<Assets>,
    pub quad: Rc<ugli::VertexBuffer<Vertex>>,
    pub editor: bool,
}

#[derive(Component)]
pub struct Camera {
    pub position: vec3<f32>,
    pub rotation: Angle,
    pub attack_angle: Angle,
    pub distance: f32,
    pub fov: Angle,
}

impl geng::camera::AbstractCamera3d for Camera {
    fn view_matrix(&self) -> mat4<f32> {
        mat4::translate(vec3(0.0, 0.0, -self.distance))
            * mat4::rotate_x(self.attack_angle - Angle::from_degrees(90.0))
            * mat4::rotate_z(-self.rotation)
            * mat4::translate(-self.position)
    }
    fn projection_matrix(&self, framebuffer_size: vec2<f32>) -> mat4<f32> {
        mat4::perspective(self.fov.as_radians(), framebuffer_size.aspect(), 0.1, 100.0)
    }
}

fn draw_sprites(
    mut receiver: ReceiverMut<Draw>,
    objects: Fetcher<&Object>,
    global: Single<&Global>,
    camera: Single<&Camera>,
) {
    let match_color: Rgba<f32> = "#ff10e3".try_into().unwrap();
    let framebuffer = &mut *receiver.event.framebuffer;
    // TODO instancing
    for object in objects {
        for part in &object.parts {
            let mut transform = object.transform;
            if part.billboard {
                transform *= mat4::rotate_z(camera.rotation);
            }
            transform *= part.transform;
            ugli::draw(
                framebuffer,
                &global.assets.shaders.main,
                part.draw_mode,
                &*part.mesh,
                (
                    ugli::uniforms! {
                        u_texture: part.texture.ugli(),
                        u_model_matrix: transform,
                        u_match_color: match_color,
                        u_replace_color: object.replace_color.unwrap_or(match_color),
                    },
                    camera.uniforms(framebuffer.size().map(|x| x as f32)),
                ),
                ugli::DrawParameters {
                    depth_func: Some(ugli::DepthFunc::Less),
                    ..default()
                },
            );
        }
    }
}

fn draw_road_editor(
    mut receiver: ReceiverMut<Draw>,
    graphs: Fetcher<&RoadGraph>,
    global: Single<&Global>,
    camera: Single<&Camera>,
    editor: Single<&Editor>,
) {
    let framebuffer = &mut *receiver.event.framebuffer;
    for graph in graphs {
        for (idx, road) in &graph.roads {
            if let Some(pos) = camera.world_to_screen(
                framebuffer.size().map(|x| x as f32),
                road.position.extend(0.0),
            ) {
                let mut color = Rgba::BLUE;
                if let EditorState::ExtendRoad(extend) = editor.state {
                    if extend == idx {
                        color = Rgba::RED;
                    }
                }
                global.geng.draw2d().circle(
                    framebuffer,
                    &geng::PixelPerfectCamera,
                    pos,
                    10.0,
                    color,
                );
            }
        }
    }
    for (idx, data) in editor.level.trees.iter().enumerate() {
        if let Some(pos) =
            camera.world_to_screen(framebuffer.size().map(|x| x as f32), data.pos.extend(0.0))
        {
            let mut color = Rgba::GREEN;
            if let EditorState::EditTree(extend, _) = editor.state {
                if extend == idx {
                    color = Rgba::RED;
                }
            }
            global
                .geng
                .draw2d()
                .circle(framebuffer, &geng::PixelPerfectCamera, pos, 10.0, color);
        }
    }
    for (idx, data) in editor.level.buildings.iter().enumerate() {
        if let Some(pos) =
            camera.world_to_screen(framebuffer.size().map(|x| x as f32), data.pos.extend(0.0))
        {
            let mut color = Rgba::YELLOW;
            if let EditorState::EditBuilding(extend, _) = editor.state {
                if extend == idx {
                    color = Rgba::RED;
                }
            }
            global
                .geng
                .draw2d()
                .circle(framebuffer, &geng::PixelPerfectCamera, pos, 10.0, color);
        }
    }

    for (idx, data) in editor.level.waypoints.iter().enumerate() {
        if let Some(pos) =
            camera.world_to_screen(framebuffer.size().map(|x| x as f32), data.pos.extend(0.0))
        {
            let mut color = Rgba::CYAN;
            if let EditorState::EditWaypoint(extend, _) = editor.state {
                if extend == idx {
                    color = Rgba::RED;
                }
            }
            global
                .geng
                .draw2d()
                .circle(framebuffer, &geng::PixelPerfectCamera, pos, 10.0, color);
        }
    }

    // Mode
    let text = match editor.state {
        EditorState::Roads | EditorState::ExtendRoad(_) | EditorState::MoveRoad(_) => "Roads",
        EditorState::Trees | EditorState::EditTree(_, _) | EditorState::MoveTree(_, _) => "Trees",
        EditorState::Buildings
        | EditorState::EditBuilding(_, _)
        | EditorState::MoveBuilding(_, _) => "Buildings",
        EditorState::Waypoints
        | EditorState::EditWaypoint(_, _)
        | EditorState::MoveWaypoint(_, _) => "Waypoints",
    };
    global.geng.default_font().draw(
        framebuffer,
        &geng::PixelPerfectCamera,
        text,
        vec2::splat(geng::TextAlign::LEFT),
        mat3::translate(vec2::splat(30.0)) * mat3::scale_uniform(50.0),
        Rgba::WHITE,
    );
}

pub async fn init(
    world: &mut World,
    geng: &Geng,
    assets: &Rc<Assets>,
    rng: &mut dyn RngCore,
    editor: bool,
    startup: &Startup,
) {
    let mk_quad = |size: f32, texture_repeats: f32| -> Rc<ugli::VertexBuffer<Vertex>> {
        Rc::new(ugli::VertexBuffer::new_static(
            geng.ugli(),
            vec![
                Vertex {
                    a_pos: vec3(-size, -size, 0.0),
                    a_uv: vec2(0.0, 0.0),
                },
                Vertex {
                    a_pos: vec3(size, -size, 0.0),
                    a_uv: vec2(texture_repeats, 0.0),
                },
                Vertex {
                    a_pos: vec3(size, size, 0.0),
                    a_uv: vec2(texture_repeats, texture_repeats),
                },
                Vertex {
                    a_pos: vec3(-size, size, 0.0),
                    a_uv: vec2(0.0, texture_repeats),
                },
            ],
        ))
    };
    let quad = mk_quad(1.0, 1.0);

    let config: Rc<Config> = file::load_detect(run_dir().join("assets").join("render.toml"))
        .await
        .unwrap();

    let global = world.spawn();
    world.insert(
        global,
        Global {
            timer: Timer::new(),
            geng: geng.clone(),
            assets: assets.clone(),
            config: config.clone(),
            quad: quad.clone(),
            editor,
        },
    );
    world.insert(
        global,
        Camera {
            attack_angle: Angle::from_degrees(config.camera.attack_angle),
            rotation: Angle::from_degrees(config.camera.default_rotation),
            position: vec3(0.0, 0.0, 0.0),
            distance: config.camera.distance,
            fov: Angle::from_degrees(config.camera.fov),
        },
    );

    // ground
    let ground = world.spawn();
    world.insert(
        ground,
        Object {
            parts: vec![ModelPart {
                draw_mode: ugli::DrawMode::TriangleFan,
                mesh: mk_quad(100.0, 100.0),
                texture: assets.ground.clone(),
                transform: mat4::identity(),
                billboard: false,
            }],
            transform: mat4::identity(),
            replace_color: None,
        },
    );

    world.add_handler(setup_road_graphics);
    world.add_handler(setup_buildings);
    world.add_handler(setup_trees);

    world.add_handler(setup_bike_graphics);
    world.add_handler(setup_car_graphics);
    world.add_handler(update_vehicle_transforms);

    world.add_handler(clear);
    world.add_handler(draw_sprites);
    if editor {
        world.add_handler(draw_road_editor);
    }

    world.add_handler(camera_follow);

    for data in &startup.level.trees {
        let entity = world.spawn();
        world.insert(entity, data.clone());
    }
}

fn setup_buildings(
    receiver: Receiver<Insert<Building>, ()>,
    mut rng: Single<&mut RngStuff>,
    global: Single<&Global>,
    mut sender: Sender<Insert<Object>>,
) {
    let building = &receiver.event.component;
    let mut parts = Vec::new();

    let assets = &global.assets.buildings[building.kind as usize];

    assert_eq!(building.half_size.x, building.half_size.y);

    let height = 2.0 * building.half_size.x / assets.sides[0].size().map(|x| x as f32).aspect();

    // top
    parts.push(ModelPart {
        mesh: global.quad.clone(),
        draw_mode: ugli::DrawMode::TriangleFan,
        texture: assets.tops.choose(&mut rng.gen).unwrap().clone(),
        transform: mat4::translate(vec3(0.0, 0.0, height))
            * mat4::scale(building.half_size.extend(1.0)),
        billboard: false,
    });

    // sides
    for i in 0..4 {
        parts.push(ModelPart {
            mesh: global.quad.clone(),
            draw_mode: ugli::DrawMode::TriangleFan,
            texture: assets.sides.choose(&mut rng.gen).unwrap().clone(),
            transform: mat4::rotate_z(Angle::from_degrees(90.0) * i as f32)
                * mat4::translate(vec3(
                    0.0,
                    if i % 2 == 0 {
                        building.half_size.y
                    } else {
                        building.half_size.x
                    },
                    0.0,
                ))
                * mat4::scale(vec3(
                    if i % 2 == 0 {
                        building.half_size.x
                    } else {
                        building.half_size.y
                    },
                    1.0,
                    height / 2.0,
                ))
                * mat4::rotate_x(Angle::from_degrees(90.0))
                * mat4::translate(vec3(0.0, 1.0, 0.0)),
            billboard: false,
        });
    }

    sender.insert(
        receiver.event.entity,
        Object {
            parts,
            transform: mat4::translate(building.pos.extend(0.0))
                * mat4::rotate_z(building.rotation),
            replace_color: None,
        },
    );
}

fn setup_trees(
    receiver: Receiver<Insert<Tree>, ()>,
    global: Single<&Global>,
    mut sender: Sender<Insert<Object>>,
) {
    let tree = &receiver.event.component;

    let texture = &global.assets.flora[tree.kind as usize];

    sender.insert(
        receiver.event.entity,
        Object {
            parts: (0..=1)
                .map(|i| ModelPart {
                    mesh: global.quad.clone(),
                    draw_mode: ugli::DrawMode::TriangleFan,
                    texture: texture.clone(),
                    transform: mat4::rotate_z(Angle::from_degrees(90.0 * i as f32) + tree.rotation)
                        * mat4::rotate_x(Angle::from_degrees(90.0))
                        * mat4::scale(
                            texture
                                .size()
                                .map(|x| x as f32 / 2.0 / global.config.pixels_per_unit)
                                .extend(1.0),
                        )
                        * mat4::translate(vec3(0.0, 1.0, 0.0)),
                    // billboard: true,
                    billboard: false,
                })
                .collect(),
            transform: mat4::translate(tree.pos.extend(0.0)),
            replace_color: None,
        },
    );
}

fn setup_road_graphics(
    receiver: Receiver<Insert<RoadGraph>, ()>,
    global: Single<&Global>,
    mut sender: Sender<Insert<Object>>,
) {
    let graph = &receiver.event.component;
    let texture = &global.assets.road.asphalt;

    /// DFS to build a connected road object.
    #[allow(clippy::too_many_arguments)]
    fn handle_road(
        graph: &RoadGraph,
        texture: &Texture,
        prev_id: Index,
        prev_road: &Road,
        id: Index,
        road: &Road,
        uv_y: f32,
        vertices: &mut Vec<Vertex>,
        visited: &mut HashSet<Index>,
        done: &mut HashSet<Index>,
    ) {
        if done.contains(&id) || !visited.insert(id) {
            return;
        }

        let Some(&prev_b) = vertices.get(vertices.len().saturating_sub(2)) else {
            return;
        };
        let Some(&prev_a) = vertices.last() else {
            return;
        };

        let connections: Vec<_> = graph
            .connections
            .iter()
            .filter_map(|&[a, b]| {
                let i = if a == id {
                    Some(b)
                } else if b == id {
                    Some(a)
                } else {
                    None
                };
                i.and_then(|i| graph.roads.get(i).map(|road| (i, road)))
            })
            .filter(|(idx, _)| *idx != prev_id)
            .collect();

        if connections.is_empty() {
            // Last road in the chain
            let prev = prev_road.position;
            let pos = road.position;

            let back = prev - pos;
            let forward = -back;

            let normal = (-back.normalize_or_zero() + forward.normalize())
                .rotate_90()
                .normalize();
            let a = Vertex {
                a_pos: (pos + normal * road.half_width).extend(thread_rng().gen()),
                a_uv: vec2(0.0, uv_y),
            };
            let b = Vertex {
                a_pos: (pos - normal * road.half_width).extend(thread_rng().gen()),
                a_uv: vec2(1.0, uv_y),
            };
            vertices.extend([prev_a, prev_b, b, prev_a, b, a]);
        }

        for (next_id, next_road) in connections {
            if next_id == prev_id {
                continue;
            }

            let prev = prev_road.position;
            let pos = road.position;
            let next = next_road.position;

            // let back = if i == 0 { pos - next } else { prev - pos };
            // let forward = if i + 1 < road.waypoints.len() {
            //     next - pos
            // } else {
            //     pos - prev
            // };

            let back = prev - pos;
            let forward = next - pos;

            let normal = (-back.normalize_or_zero() + forward.normalize())
                .rotate_90()
                .normalize();
            let a = Vertex {
                a_pos: (pos + normal * road.half_width).extend(thread_rng().gen()),
                a_uv: vec2(0.0, uv_y),
            };
            let b = Vertex {
                a_pos: (pos - normal * road.half_width).extend(thread_rng().gen()),
                a_uv: vec2(1.0, uv_y),
            };
            vertices.extend([prev_a, prev_b, b, prev_a, b, a]);
            let uv_y = uv_y
                + forward.len() / texture.size().map(|x| x as f32).aspect() / road.half_width / 2.0;

            if !visited.contains(&next_id) {
                handle_road(
                    graph, texture, id, road, next_id, next_road, uv_y, vertices, visited, done,
                );
            } else {
                // Last road in the loop
                let prev = road.position;
                let pos = next_road.position;

                let prev_a = a;
                let prev_b = b;

                let back = prev - pos;
                let forward = -back;

                let normal = (-back.normalize_or_zero() + forward.normalize())
                    .rotate_90()
                    .normalize();
                let a = Vertex {
                    a_pos: (pos + normal * road.half_width).extend(thread_rng().gen()),
                    a_uv: vec2(0.0, uv_y),
                };
                let b = Vertex {
                    a_pos: (pos - normal * road.half_width).extend(thread_rng().gen()),
                    a_uv: vec2(1.0, uv_y),
                };
                vertices.extend([prev_a, prev_b, b, prev_a, b, a]);
            }
        }

        done.insert(id);
    }

    let mut vertices = Vec::new();
    let mut visited = HashSet::new();
    let mut done = HashSet::new();
    for (id, prev) in &graph.roads {
        if !visited.insert(id) {
            continue;
        }

        let connections = graph.connections.iter().filter_map(|&[a, b]| {
            let i = if a == id {
                Some(b)
            } else if b == id {
                Some(a)
            } else {
                None
            };
            i.and_then(|i| graph.roads.get(i).map(|road| (i, road)))
        });
        for (road_id, road) in connections {
            // Connect first part
            let back = prev.position - road.position;
            let forward = -back;

            let uv_y = 0.0;

            let normal = (-back.normalize_or_zero() + forward.normalize())
                .rotate_90()
                .normalize();
            let a = Vertex {
                a_pos: (prev.position + normal * road.half_width).extend(thread_rng().gen()),
                a_uv: vec2(0.0, uv_y),
            };
            let b = Vertex {
                a_pos: (prev.position - normal * road.half_width).extend(thread_rng().gen()),
                a_uv: vec2(1.0, uv_y),
            };
            vertices.extend([a, b, a]); // Just because im lazy TODO: remove NOTE: `handle_road` assumes the last two vertices in the vec

            let uv_y = uv_y
                + forward.len() / texture.size().map(|x| x as f32).aspect() / road.half_width / 2.0;

            // Recursively connect all the trails
            handle_road(
                graph,
                texture,
                id,
                prev,
                road_id,
                road,
                uv_y,
                &mut vertices,
                &mut visited,
                &mut done,
            );
        }
    }

    let mesh = Rc::new(ugli::VertexBuffer::new_static(global.geng.ugli(), vertices));

    let parts = vec![
        ModelPart {
            mesh: mesh.clone(),
            draw_mode: ugli::DrawMode::Triangles,
            texture: texture.clone(),
            transform: mat4::translate(vec3(0.0, 0.0, 0.2)) * mat4::scale(vec3(1.0, 1.0, 0.1)),
            billboard: false,
        },
        ModelPart {
            mesh: mesh.clone(),
            draw_mode: ugli::DrawMode::Triangles,
            texture: global.assets.road.border.clone(),
            transform: mat4::translate(vec3(0.0, 0.0, 0.1)) * mat4::scale(vec3(1.0, 1.0, 0.1)),
            billboard: false,
        },
    ];

    sender.insert(
        receiver.event.entity,
        Object {
            parts,
            transform: mat4::identity(),
            replace_color: None,
        },
    );
}

fn camera_follow(
    receiver: Receiver<Update>,
    mut camera: Single<&mut Camera>,
    global: Single<&Global>,
    player: TrySingle<(&Vehicle, With<&Player>)>,
) {
    let camera: &mut Camera = &mut camera;
    let delta_time = receiver.event.delta_time.as_secs_f64() as f32;
    let Ok((player, _)) = player.0 else {
        return;
    };
    let k = (global.config.camera.speed * delta_time).min(1.0);
    camera.position += (player.pos.extend(0.0)
        + vec2(player.speed, 0.0).rotate(player.rotation).extend(0.0)
            * global.config.camera.predict
        + vec2(0.0, global.config.camera.offset)
            .rotate(player.rotation)
            .extend(0.0)
        - camera.position)
        * k;
    if global.config.camera.auto_rotate {
        camera.rotation = (camera.rotation
            + (player.rotation - Angle::from_degrees(90.0) - camera.rotation).normalized_pi() * k)
            .normalized_2pi();
    }
}

fn update_vehicle_transforms(
    _receiver: Receiver<Draw>,
    global: Single<&Global>,
    bikes: Fetcher<(&Vehicle, &mut Object, Has<&Car>)>,
) {
    for (bike, object, car) in bikes {
        object.transform =
            mat4::translate(bike.pos.extend((bike.jump.unwrap_or(0.0) * f32::PI).sin()))
                * mat4::rotate_z(bike.rotation + Angle::from_degrees(180.0));
        if car.get() {
            object.transform *= mat4::scale(vec3(
                1.0,
                1.0,
                (global.timer.elapsed().as_secs_f64() as f32 * 10.0).sin() * 0.03 + 1.0,
            ));
        }
    }
}

fn setup_car_graphics(
    receiver: Receiver<Insert<Vehicle>, &Car>,
    global: Single<&Global>,
    mut sender: Sender<Insert<Object>>,
) {
    let bike = receiver.event.entity;
    let textures = &global.assets.car;
    let unit_scale = mat4::scale_uniform(1.0 / 16.0);
    sender.insert(
        bike,
        Object {
            replace_color: Some(receiver.query.color),
            parts: {
                let mut parts = Vec::new();
                parts.push(ModelPart {
                    draw_mode: ugli::DrawMode::TriangleFan,
                    mesh: global.quad.clone(),
                    texture: textures.toptop.clone(),
                    transform: unit_scale
                        * mat4::translate(vec3(
                            0.0,
                            0.0,
                            textures.bottomside.size().y as f32 + textures.topside.size().y as f32,
                        ))
                        * mat4::scale(textures.toptop.size().map(|x| x as f32 / 2.0).extend(1.0)),
                    billboard: false,
                });
                for r in 0..4 {
                    parts.push(ModelPart {
                        draw_mode: ugli::DrawMode::TriangleFan,
                        mesh: global.quad.clone(),
                        texture: textures.topside.clone(),
                        transform: unit_scale
                            * mat4::rotate_z(Angle::from_degrees(90.0 * r as f32))
                            * mat4::translate(vec3(
                                0.0,
                                textures.toptop.size().y as f32 / 2.0,
                                textures.bottomside.size().y as f32,
                            ))
                            * mat4::rotate_x(Angle::from_degrees(90.0))
                            * mat4::scale(
                                textures.topside.size().map(|x| x as f32 / 2.0).extend(1.0),
                            )
                            * mat4::translate(vec3(0.0, 1.0, 0.0)),
                        billboard: false,
                    });
                }
                for r in 0..2 {
                    parts.push(ModelPart {
                        draw_mode: ugli::DrawMode::TriangleFan,
                        mesh: global.quad.clone(),
                        texture: textures.bottomside.clone(),
                        transform: unit_scale
                            * mat4::rotate_z(Angle::from_degrees(180.0 * r as f32))
                            * mat4::translate(vec3(
                                0.0,
                                textures.toptop.size().y as f32 / 2.0,
                                0.0,
                            ))
                            * mat4::rotate_x(Angle::from_degrees(90.0))
                            * mat4::scale(
                                textures
                                    .bottomside
                                    .size()
                                    .map(|x| x as f32 / 2.0)
                                    .extend(1.0),
                            )
                            * mat4::translate(vec3(0.0, 1.0, 0.0)),
                        billboard: false,
                    });
                }
                for r in 0..2 {
                    parts.push(ModelPart {
                        draw_mode: ugli::DrawMode::TriangleFan,
                        mesh: global.quad.clone(),
                        texture: textures.bottomfront.clone(),
                        transform: unit_scale
                            * mat4::rotate_z(Angle::from_degrees(180.0 * r as f32 + 90.0))
                            * mat4::translate(vec3(
                                0.0,
                                textures.bottomtop.size().x as f32 / 2.0,
                                0.0,
                            ))
                            * mat4::rotate_x(Angle::from_degrees(90.0))
                            * mat4::scale(
                                textures
                                    .bottomfront
                                    .size()
                                    .map(|x| x as f32 / 2.0)
                                    .extend(1.0),
                            )
                            * mat4::translate(vec3(0.0, 1.0, 0.0)),
                        billboard: false,
                    });
                }
                parts.push(ModelPart {
                    draw_mode: ugli::DrawMode::TriangleFan,
                    mesh: global.quad.clone(),
                    texture: textures.bottomtop.clone(),
                    transform: unit_scale
                        * mat4::translate(vec3(0.0, 0.0, textures.bottomside.size().y as f32))
                        * mat4::scale(
                            textures
                                .bottomtop
                                .size()
                                .map(|x| x as f32 / 2.0)
                                .extend(1.0),
                        ),
                    billboard: false,
                });
                parts
            },
            transform: mat4::identity(),
        },
    );
}

fn setup_bike_graphics(
    receiver: Receiver<Insert<Vehicle>, With<&Bike>>,
    global: Single<&Global>,
    mut sender: Sender<Insert<Object>>,
) {
    let bike = receiver.event.entity;
    sender.insert(
        bike,
        Object {
            replace_color: None,
            parts: vec![
                ModelPart {
                    draw_mode: ugli::DrawMode::TriangleFan,
                    mesh: global.quad.clone(),
                    texture: global.assets.bike.top.clone(),
                    transform: mat4::translate(vec3(0.0, 0.0, 1.1)),
                    billboard: false,
                },
                ModelPart {
                    draw_mode: ugli::DrawMode::TriangleFan,
                    mesh: global.quad.clone(),
                    texture: global.assets.bike.top_handle.clone(),
                    transform: mat4::translate(vec3(0.0, 0.0, 1.4)),
                    billboard: false,
                },
                ModelPart {
                    draw_mode: ugli::DrawMode::TriangleFan,
                    mesh: global.quad.clone(),
                    texture: global.assets.bike.side.clone(),
                    transform: mat4::translate(vec3(0.0, 0.0, 1.0))
                        * mat4::rotate_x(Angle::from_degrees(90.0)),
                    billboard: false,
                },
                ModelPart {
                    draw_mode: ugli::DrawMode::TriangleFan,
                    mesh: global.quad.clone(),
                    texture: global.assets.salmon.clone(),
                    transform: mat4::translate(vec3(0.0, 0.0, 1.5)) * mat4::scale_uniform(0.75),
                    // * mat4::rotat_x(Angle::from_degrees(90.0)),
                    billboard: false,
                },
            ],
            transform: mat4::identity(),
        },
    );
}
