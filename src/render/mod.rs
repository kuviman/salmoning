use std::f32::consts::PI;

use crate::{
    assets::{Assets, Texture},
    editor::{Editor, EditorState},
    model::*,
};

use evenio::{prelude::*, query};
use generational_arena::Index;
use geng::prelude::*;

mod roads;

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

#[derive(ugli::Vertex, Clone, Copy, Debug)]
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

fn draw_waypoints(
    mut receiver: ReceiverMut<Draw>,
    quests: Single<&Quests>,
    waypoints: Fetcher<&Waypoint>,
    global: Single<&Global>,
    camera: Single<&Camera>,
) {
    let framebuffer = &mut *receiver.event.framebuffer;
    for &quest in &quests.active {
        let waypoint = waypoints.get(quests.index_to_entity[&quest]).unwrap();
        let assets = &global.assets.buildings[0];
        // sides
        const SIDES: i32 = 10;
        for i in 0..SIDES {
            let part = ModelPart {
                mesh: global.quad.clone(),
                draw_mode: ugli::DrawMode::TriangleFan,
                texture: assets.sides[0].clone(),
                transform: mat4::rotate_z(Angle::from_degrees(360.0 / (SIDES as f32)) * i as f32)
                    * mat4::translate(vec3(0.0, 1.0, 0.0))
                    * mat4::scale(vec3((PI / SIDES as f32).tan(), 1.0, 1.0))
                    * mat4::rotate_x(Angle::from_degrees(90.0))
                    * mat4::translate(vec3(0.0, 1.0, 0.0)),
                billboard: false,
            };
            let mut transform = mat4::translate(waypoint.pos.extend(0.0));

            if part.billboard {
                transform *= mat4::rotate_z(camera.rotation);
            }
            transform *= part.transform;
            ugli::draw(
                framebuffer,
                &global.assets.shaders.waypoint,
                part.draw_mode,
                &*part.mesh,
                (
                    ugli::uniforms! {
                        u_texture: part.texture.ugli(),
                        u_model_matrix: transform,
                    },
                    camera.uniforms(framebuffer.size().map(|x| x as f32)),
                ),
                ugli::DrawParameters {
                    depth_func: Some(ugli::DepthFunc::Less),
                    write_depth: false,
                    blend_mode: Some(ugli::BlendMode::straight_alpha()),
                    ..default()
                },
            );
        }

        // sender.insert(
        //     receiver.event.entity,
        //     Object {
        //         parts,
        //         transform: mat4::translate(waypoint.pos.extend(0.0)),
        //         replace_color: None,
        //     },
        // );
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
            global.geng.draw2d().circle(
                framebuffer,
                &geng::PixelPerfectCamera,
                pos,
                if data.small { 4.0 } else { 10.0 },
                color,
            );
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
        | EditorState::MoveBuilding(_, _) => {
            if editor.building_small {
                "Decorations"
            } else {
                "Buildings"
            }
        }
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

    world.add_handler(roads::setup_road_graphics);
    world.add_handler(setup_buildings);
    world.add_handler(setup_trees);

    world.add_handler(setup_bike_graphics);
    world.add_handler(setup_car_graphics);
    world.add_handler(update_vehicle_transforms);

    world.add_handler(clear);
    world.add_handler(draw_sprites);
    world.add_handler(draw_waypoints);
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

    assert_eq!(building.half_size.x, building.half_size.y);

    if building.small {
        let assets = &global.assets.small_items[building.kind as usize];
        let height = 2.0 * building.half_size.x / assets.side_a.size().map(|x| x as f32).aspect();
        // top
        parts.push(ModelPart {
            mesh: global.quad.clone(),
            draw_mode: ugli::DrawMode::TriangleFan,
            texture: assets.top.clone(),
            transform: mat4::translate(vec3(0.0, 0.0, height))
                * mat4::scale(building.half_size.extend(1.0)),
            billboard: false,
        });

        // sides
        for i in 0..4 {
            parts.push(ModelPart {
                mesh: global.quad.clone(),
                draw_mode: ugli::DrawMode::TriangleFan,
                texture: if i == 0 {
                    assets.side_a.clone()
                } else {
                    assets.side_b.clone()
                },
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
    } else {
        let assets = &global.assets.buildings[building.kind as usize];
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
    };

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
