use crate::{
    assets::{Assets, Texture},
    model::*,
};

use evenio::{prelude::*, query};
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
}

#[derive(Component)]
pub struct Object {
    pub parts: Vec<ModelPart>,
    pub transform: mat4<f32>,
}

fn clear(mut receiver: ReceiverMut<Draw>) {
    let framebuffer = &mut *receiver.event.framebuffer;
    ugli::clear(framebuffer, Some(Rgba::BLUE), Some(1.0), None);
}

#[derive(ugli::Vertex)]
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
    camera: CameraConfig,
}

#[derive(Component)]
pub struct Global {
    geng: Geng,
    config: Rc<Config>,
    assets: Rc<Assets>,
    quad: Rc<ugli::VertexBuffer<Vertex>>,
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
    let framebuffer = &mut *receiver.event.framebuffer;
    // TODO instancing
    for object in objects {
        for part in &object.parts {
            let transform = object.transform * part.transform;
            ugli::draw(
                framebuffer,
                &global.assets.shaders.main,
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
                    ..default()
                },
            );
        }
    }
}

pub async fn init(world: &mut World, geng: &Geng, assets: &Rc<Assets>) {
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
            geng: geng.clone(),
            assets: assets.clone(),
            config: config.clone(),
            quad: quad.clone(),
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
            }],
            transform: mat4::identity(),
        },
    );

    world.add_handler(setup_road_graphics);
    world.add_handler(setup_buildings);

    world.add_handler(setup_bike_graphics);
    world.add_handler(update_bike_transform);

    world.add_handler(clear);
    world.add_handler(draw_sprites);

    world.add_handler(camera_follow);
}

fn setup_buildings(
    receiver: Receiver<Insert<Building>, ()>,
    global: Single<&Global>,
    mut sender: Sender<Insert<Object>>,
) {
    let building = &receiver.event.component;
    let mut parts = Vec::new();

    let assets = global.assets.buildings.choose(&mut thread_rng()).unwrap();

    assert_eq!(building.half_size.x, building.half_size.y);

    let height = 2.0 * building.half_size.x / assets.sides[0].size().map(|x| x as f32).aspect();

    // top
    parts.push(ModelPart {
        mesh: global.quad.clone(),
        draw_mode: ugli::DrawMode::TriangleFan,
        texture: assets.tops.choose(&mut thread_rng()).unwrap().clone(),
        transform: mat4::translate(vec3(0.0, 0.0, height))
            * mat4::scale(building.half_size.extend(1.0)),
    });

    // sides
    for i in 0..4 {
        parts.push(ModelPart {
            mesh: global.quad.clone(),
            draw_mode: ugli::DrawMode::TriangleFan,
            texture: assets.sides.choose(&mut thread_rng()).unwrap().clone(),
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
        });
    }

    sender.insert(
        receiver.event.entity,
        Object {
            parts,
            transform: mat4::translate(building.pos.extend(0.0))
                * mat4::rotate_z(building.rotation),
        },
    );
}

fn setup_road_graphics(
    receiver: Receiver<Insert<Road>, ()>,
    global: Single<&Global>,
    mut sender: Sender<Insert<Object>>,
) {
    let road = &receiver.event.component;
    let texture = &global.assets.road.straight;
    if road.waypoints.len() < 2 {
        return;
    }
    let mut vertices = Vec::new();

    let mut uv_y = 0.0;
    for i in 0..road.waypoints.len() {
        let back = if i == 0 {
            road.waypoints[i] - road.waypoints[i + 1]
        } else {
            road.waypoints[i - 1] - road.waypoints[i]
        };
        let forward = if i + 1 < road.waypoints.len() {
            road.waypoints[i + 1] - road.waypoints[i]
        } else {
            road.waypoints[i] - road.waypoints[i - 1]
        };
        let normal = (-back.normalize_or_zero() + forward.normalize()).rotate_90();
        vertices.push(Vertex {
            a_pos: (road.waypoints[i] + normal * road.half_width).extend(0.0),
            a_uv: vec2(0.0, uv_y),
        });
        vertices.push(Vertex {
            a_pos: (road.waypoints[i] - normal * road.half_width).extend(0.0),
            a_uv: vec2(1.0, uv_y),
        });
        uv_y += forward.len() / texture.size().map(|x| x as f32).aspect() / road.half_width / 2.0;
    }

    sender.insert(
        receiver.event.entity,
        Object {
            parts: vec![ModelPart {
                mesh: Rc::new(ugli::VertexBuffer::new_static(global.geng.ugli(), vertices)),
                draw_mode: ugli::DrawMode::TriangleStrip,
                texture: texture.clone(),
                transform: mat4::translate(vec3(0.0, 0.0, 0.1)),
            }],
            transform: mat4::identity(),
        },
    );
}

fn camera_follow(
    receiver: Receiver<Update>,
    mut camera: Single<&mut Camera>,
    global: Single<&Global>,
    player: TrySingle<(&Bike, With<&Player>)>,
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

fn update_bike_transform(_receiver: Receiver<Draw>, bikes: Fetcher<(&Bike, &mut Object)>) {
    for (bike, object) in bikes {
        object.transform = mat4::translate(bike.pos.extend(0.0))
            * mat4::rotate_z(bike.rotation + Angle::from_degrees(180.0));
    }
}

fn setup_bike_graphics(
    receiver: Receiver<Insert<Bike>, ()>,
    global: Single<&Global>,
    mut sender: Sender<Insert<Object>>,
) {
    let bike = receiver.event.entity;
    sender.insert(
        bike,
        Object {
            parts: vec![
                ModelPart {
                    draw_mode: ugli::DrawMode::TriangleFan,
                    mesh: global.quad.clone(),
                    texture: global.assets.bike.top.clone(),
                    transform: mat4::translate(vec3(0.0, 0.0, 1.1)),
                },
                ModelPart {
                    draw_mode: ugli::DrawMode::TriangleFan,
                    mesh: global.quad.clone(),
                    texture: global.assets.bike.top_handle.clone(),
                    transform: mat4::translate(vec3(0.0, 0.0, 1.4)),
                },
                ModelPart {
                    draw_mode: ugli::DrawMode::TriangleFan,
                    mesh: global.quad.clone(),
                    texture: global.assets.bike.side.clone(),
                    transform: mat4::translate(vec3(0.0, 0.0, 1.0))
                        * mat4::rotate_x(Angle::from_degrees(90.0)),
                },
                ModelPart {
                    draw_mode: ugli::DrawMode::TriangleFan,
                    mesh: global.quad.clone(),
                    texture: global.assets.salmon.clone(),
                    transform: mat4::translate(vec3(0.0, 0.0, 1.5)) * mat4::scale_uniform(0.75),
                    // * mat4::rotate_x(Angle::from_degrees(90.0)),
                },
            ],
            transform: mat4::identity(),
        },
    );
}
