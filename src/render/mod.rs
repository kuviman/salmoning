use std::{f32::consts::PI, sync::atomic::AtomicBool};

use crate::{
    assets::{Assets, Texture},
    controls::{InviteTarget, TeamLeader},
    editor::{Editor, EditorState},
    interop::ServerMessage,
    model::*,
};

use evenio::{prelude::*, query};
use generational_arena::Index;
use geng::{draw2d::ColoredVertex, prelude::*};
use parry2d::na::ComplexField;
use pathfinding::directed::astar;

use self::{
    net::{CanDoQuests, Invitation, Name},
    particle::{emit_particles, update_particles},
};

mod instancing;
pub mod obj;
pub mod particle;
mod roads;

use instancing::*;

#[derive(Event)]
pub struct Draw {
    pub framebuffer: &'static mut ugli::Framebuffer<'static>,
}

#[derive(Event)]
pub struct MinimapDraw {
    pub framebuffer: &'static mut ugli::Framebuffer<'static>,
}

#[derive(Event)]
pub struct SetBikeType {
    pub bike_id: EntityId,
    pub bike_type: usize,
}

#[derive(Event)]
pub struct SetHatType {
    pub bike_id: EntityId,
    pub hat_type: Option<usize>,
}

#[derive(Clone)]
pub struct ModelPart {
    pub mesh: Rc<ugli::VertexBuffer<Vertex>>,
    pub draw_mode: DrawMode,
    pub texture: Texture,
    pub transform: mat4<f32>,
    pub billboard: bool,
    pub is_self: bool,
}

#[derive(Component, Clone)]
pub struct Object {
    pub parts: Vec<ModelPart>,
    pub transform: mat4<f32>,
    pub replace_color: Option<Rgba<f32>>,
}

fn clear(mut receiver: ReceiverMut<Draw>, global: Single<&Global>, camera: Single<&Camera>) {
    let framebuffer = &mut *receiver.event.framebuffer;
    ugli::clear(framebuffer, Some(Rgba::BLUE), Some(1.0), None);
    ugli::draw(
        framebuffer,
        &global.assets.shaders.sky,
        ugli::DrawMode::TriangleFan,
        &*global.quad,
        (
            ugli::uniforms! {
                u_time: global.timer.elapsed().as_secs_f64() as f32,
            },
            camera.uniforms(framebuffer.size().map(|x| x as f32)),
        ),
        ugli::DrawParameters {
            write_depth: false,
            ..default()
        },
    );
}

#[derive(ugli::Vertex, Clone, Copy, Debug)]
pub struct Vertex {
    pub a_pos: vec3<f32>,
    pub a_uv: vec2<f32>,
    pub a_color: Rgba<f32>,
}

#[derive(Deserialize)]
struct CameraConfig {
    distance: f32,
    fov: f32,
    default_rotation: f32,
    attack_angle: f32,
    offset: vec3<f32>,
    predict: f32,
    show_self: bool,
    speed: f32,
    auto_rotate: bool,
}
#[derive(Deserialize)]
struct MinimapConfig {
    fov: f32,
}

#[derive(Deserialize)]
struct WaypointsConfig {
    quest_color: Rgba<f32>,
    deliver_color: Rgba<f32>,
}

#[derive(Deserialize)]
struct Config {
    pixels_per_unit: f32,
    camera: Vec<CameraConfig>,
    shop_camera: CameraConfig,
    minimap: MinimapConfig,
    waypoints: WaypointsConfig,
}

#[derive(Component)]
pub struct Meshes {
    salmon_mesh: Rc<ugli::VertexBuffer<Vertex>>,
    hats: Vec<Rc<ugli::VertexBuffer<Vertex>>>,
}

#[derive(Component)]
pub struct LeaderboardTexture {
    texture: ugli::Texture,
}

#[derive(Component, Clone)]
pub struct VehicleWheels {
    wheels: Vec<Wheel>,
    rotation: Angle,
}

#[derive(Clone)]
pub struct Wheel {
    pub model_part: usize,
    pub transform: mat4<f32>,
}

#[derive(Component)]
pub struct Global {
    pub geng: Geng,
    white_texture: Texture,
    pub timer: Timer,
    pub config: Rc<Config>,
    pub assets: Rc<Assets>,
    pub quad: Rc<ugli::VertexBuffer<Vertex>>,
    pub editor: bool,
    pub bikes: Vec<(Object, VehicleWheels)>,
}

#[derive(Component)]
pub struct Camera {
    pub preset: usize,
    pub show_self: bool,
    pub position: vec3<f32>,
    pub rotation: Angle,
    pub attack_angle: Angle,
    pub distance: f32,
    pub fov: Angle,
    pub fish_eye_transform: Option<mat4<f32>>,
}

impl geng::camera::AbstractCamera3d for Camera {
    fn view_matrix(&self) -> mat4<f32> {
        self.fish_eye_transform
            .filter(|_| !self.show_self)
            .unwrap_or(
                mat4::translate(vec3(0.0, 0.0, -self.distance))
                    * mat4::rotate_x(self.attack_angle - Angle::from_degrees(90.0))
                    * mat4::rotate_z(-self.rotation)
                    * mat4::translate(-self.position),
            )
    }
    fn projection_matrix(&self, framebuffer_size: vec2<f32>) -> mat4<f32> {
        mat4::perspective(self.fov.as_radians(), framebuffer_size.aspect(), 0.1, 100.0)
    }
}

#[derive(Component)]
pub struct MinimapCamera {
    pub position: vec3<f32>,
    pub rotation: Angle,
    pub attack_angle: Angle,
    pub fov: f32,
}

impl geng::camera::AbstractCamera3d for MinimapCamera {
    fn view_matrix(&self) -> mat4<f32> {
        mat4::rotate_x(self.attack_angle - Angle::from_degrees(90.0))
            * mat4::rotate_z(-self.rotation)
            * mat4::translate(-self.position)
    }
    fn projection_matrix(&self, framebuffer_size: vec2<f32>) -> mat4<f32> {
        // Orthographic projection
        let t = self.fov;
        let b = -t;
        let r = framebuffer_size.aspect() * self.fov;
        let l = -r;
        let f = 100.0;
        let n = 10.0;
        mat4::new([
            [2.0 / (r - l), 0.0, 0.0, -(r + l) / (r - l)],
            [0.0, 2.0 / (t - b), 0.0, -(t + b) / (t - b)],
            [0.0, 0.0, -2.0 / (f - n), -(f + n) / (f - n)],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}

fn draw_minimap(
    mut receiver: ReceiverMut<MinimapDraw>,
    objects: Fetcher<(&Object, Option<&Building>, Option<&RoadGraph>)>,
    quests: Single<&Quests>,
    waypoints: Fetcher<&Waypoint>,
    player: Single<(EntityId, &Vehicle, Option<&TeamLeader>, With<&LocalPlayer>)>,
    global: Single<&Global>,
    can_do_quests: Fetcher<&CanDoQuests>,
    camera: Single<&MinimapCamera>,
) {
    let framebuffer = &mut *receiver.event.framebuffer;

    #[derive(PartialEq, Eq, Hash)]
    pub struct Key<'a> {
        pub draw_mode: DrawMode,
        pub program: Pointer<'a, ugli::Program>,
        pub mesh: Pointer<'a, ugli::VertexBuffer<Vertex>>,
        pub uniforms: Uniforms,
    }

    #[derive(ugli::Vertex)]
    pub struct Instance {
        pub i_color: Rgba<f32>,
        pub i_model_matrix: mat4<f32>,
    }
    #[derive(ugli::Uniforms, PartialEq, Eq, Hash)]
    pub struct Uniforms {}

    let mut instances = HashMap::<Key, Vec<Instance>>::new();
    for (object, building, road) in objects {
        let color = if building.is_some() {
            Rgba::try_from("#c5522b").unwrap()
        } else if road.is_some() {
            Rgba::try_from("#858684").unwrap()
        } else {
            continue;
        };
        for part in &object.parts {
            let mut transform = object.transform;
            if part.billboard {
                continue;
            }
            transform *= part.transform;
            instances
                .entry(Key {
                    draw_mode: part.draw_mode,
                    program: Pointer(&global.assets.shaders.minimap),
                    mesh: Pointer(&part.mesh),
                    uniforms: Uniforms {},
                })
                .or_default()
                .push(Instance {
                    i_color: color,
                    i_model_matrix: transform,
                });
        }
    }

    for (key, instances) in instances {
        let instances = ugli::VertexBuffer::new_dynamic(global.geng.ugli(), instances);
        ugli::draw(
            framebuffer,
            key.program.0,
            key.draw_mode.into(),
            ugli::instanced(key.mesh.0, &instances),
            (
                key.uniforms,
                camera.uniforms(framebuffer.size().map(|x| x as f32)),
            ),
            ugli::DrawParameters {
                depth_func: Some(ugli::DepthFunc::Less),
                ..default()
            },
        );
    }

    let mut draw_circle = |pos: vec2<f32>, color: Rgba<f32>| {
        let framebuffer_size = framebuffer.size().map(|x| x as f32);
        let pos = (camera.projection_matrix(framebuffer_size) * camera.view_matrix())
            * pos.extend(0.0).extend(1.0);
        let pos = pos.xyz() / pos.w;
        let pos = pos.map(|x| x.clamp_abs(1.0));
        let pos = vec2(
            (pos.x + 1.0) / 2.0 * framebuffer_size.x,
            (pos.y + 1.0) / 2.0 * framebuffer_size.y,
        );

        global
            .geng
            .draw2d()
            .circle(framebuffer, &geng::PixelPerfectCamera, pos, 5.0, color);
    };
    let (player_entity, player, player_leader, _) = &*player;
    let can_do_quests = can_do_quests
        .get(
            player_leader
                .as_ref()
                .map_or(*player_entity, |leader| leader.0),
        )
        .is_ok();

    if let Some(i) = quests.deliver {
        draw_circle(
            waypoints.get(quests.index_to_entity[&i]).unwrap().pos,
            global.config.waypoints.deliver_color,
        );
    } else if can_do_quests {
        for &i in &quests.active {
            draw_circle(
                waypoints.get(quests.index_to_entity[&i]).unwrap().pos,
                global.config.waypoints.quest_color,
            );
        }
    }

    if let Some(pos) =
        camera.world_to_screen(framebuffer.size().map(|x| x as f32), player.pos.extend(0.0))
    {
        global
            .geng
            .draw2d()
            .circle(framebuffer, &geng::PixelPerfectCamera, pos, 5.0, Rgba::BLUE);
    }
}

#[allow(clippy::type_complexity)]
fn draw_objects(
    mut receiver: ReceiverMut<Draw>,
    objects: Fetcher<(&Object, Option<&Tree>, Has<&LocalPlayer>, Option<&Fish>)>,
    global: Single<&Global>,
    camera: Single<&Camera>,
) {
    let match_color: Rgba<f32> = "#ff10e3".try_into().unwrap();
    let framebuffer = &mut *receiver.event.framebuffer;

    #[derive(PartialEq, Eq, Hash)]
    pub struct Key<'a> {
        draw_mode: DrawMode,
        program: Pointer<'a, ugli::Program>,
        mesh: Pointer<'a, ugli::VertexBuffer<Vertex>>,
        texture: Pointer<'a, ugli::Texture>,
        wiggle: bool,
    }

    #[derive(ugli::Vertex)]
    pub struct Instance {
        pub i_replace_color: Rgba<f32>,
        pub i_model_matrix: mat4<f32>,
    }

    let mut instances = HashMap::<Key, Vec<Instance>>::new();

    for (object, tree, local, fish) in objects {
        for part in &object.parts {
            if (local.get() || fish.map_or(false, |fish| fish.local))
                && !camera.show_self
                && part.is_self
            {
                continue;
            }
            let mut transform = object.transform;
            transform *= part.transform;
            instances
                .entry(Key {
                    draw_mode: part.draw_mode,
                    program: Pointer(if part.billboard {
                        &global.assets.shaders.billboard
                    } else {
                        &global.assets.shaders.main_instancing
                    }),
                    mesh: Pointer(&part.mesh),
                    wiggle: tree.is_some(),
                    texture: Pointer(&part.texture),
                })
                .or_default()
                .push(Instance {
                    i_model_matrix: transform,
                    i_replace_color: object.replace_color.unwrap_or(match_color),
                });
        }
    }

    for (key, instances) in instances {
        let instances = ugli::VertexBuffer::new_dynamic(global.geng.ugli(), instances);
        ugli::draw(
            framebuffer,
            key.program.0,
            key.draw_mode.into(),
            ugli::instanced(key.mesh.0, &instances),
            (
                ugli::uniforms! {
                    u_time: global.timer.elapsed().as_secs_f64() as f32,
                    u_wiggle: if key.wiggle { 1.0 } else { 0.0 },
                    u_texture: key.texture.0,
                    u_match_color: match_color,
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

fn draw_waypoints(
    mut receiver: ReceiverMut<Draw>,
    quests: Single<&Quests>,
    waypoints: Fetcher<&Waypoint>,
    global: Single<&Global>,
    camera: Single<&Camera>,
) {
    let framebuffer = &mut *receiver.event.framebuffer;

    let mut draw_waypoint = |waypoint: &Waypoint, color: Rgba<f32>| {
        let assets = &global.assets.buildings[0];
        // sides
        const SIDES: i32 = 10;
        for i in 0..SIDES {
            let part = ModelPart {
                mesh: global.quad.clone(),
                draw_mode: DrawMode::TriangleFan,
                texture: assets.sides[0].clone(),
                transform: mat4::rotate_z(Angle::from_degrees(360.0 / (SIDES as f32)) * i as f32)
                    * mat4::translate(vec3(0.0, 1.0, 0.0))
                    * mat4::scale(vec3((PI / SIDES as f32).tan(), 1.0, 1.0))
                    * mat4::rotate_x(Angle::from_degrees(90.0))
                    * mat4::translate(vec3(0.0, 1.0, 0.0)),
                billboard: false,
                is_self: false,
            };
            let mut transform = mat4::translate(waypoint.pos.extend(0.0));

            if part.billboard {
                transform *= mat4::rotate_z(camera.rotation);
            }
            transform *= part.transform;
            ugli::draw(
                framebuffer,
                &global.assets.shaders.waypoint,
                part.draw_mode.into(),
                &*part.mesh,
                (
                    ugli::uniforms! {
                        u_model_matrix: transform,
                        u_color: color,
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
    };

    if let Some(delivery) = quests.deliver {
        draw_waypoint(
            waypoints.get(quests.index_to_entity[&delivery]).unwrap(),
            global.config.waypoints.deliver_color,
        );
    } else {
        for &quest in &quests.active {
            draw_waypoint(
                waypoints.get(quests.index_to_entity[&quest]).unwrap(),
                global.config.waypoints.quest_color,
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
            global.geng.draw2d().circle(
                framebuffer,
                &geng::PixelPerfectCamera,
                pos,
                if data.small { 4.0 } else { 10.0 },
                color,
            );
        }
    }

    for (idx, data) in editor.level.leaderboards.iter().enumerate() {
        if let Some(pos) =
            camera.world_to_screen(framebuffer.size().map(|x| x as f32), data.pos.extend(0.0))
        {
            let mut color = Rgba::MAGENTA;
            if let EditorState::EditLeaderboard(extend, _) = editor.state {
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
        EditorState::Leaderboards
        | EditorState::EditLeaderboard(_, _)
        | EditorState::MoveLeaderboard(_, _) => "Leaderboards",
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

fn draw_gps_line(
    mut receiver: ReceiverMut<MinimapDraw>,
    graphs: Fetcher<&RoadGraph>,
    global: Single<&Global>,
    vehicle: Single<(&Vehicle, With<&LocalPlayer>)>,
    waypoints: Fetcher<&Waypoint>,
    quests: Single<&Quests>,
    camera: Single<&MinimapCamera>,
) {
    let framebuffer = &mut *receiver.event.framebuffer;
    let questination = quests.deliver;
    let Some(dest) = questination else {
        return;
    };
    let waypoint_id = quests.index_to_entity[&dest];
    let waypoint = waypoints.get(waypoint_id).unwrap();
    let vehicle = vehicle.0 .0;
    for graph in graphs {
        let winner = graph
            .roads
            .iter()
            .min_by_key(|(_, asdf)| r32((asdf.position - vehicle.pos).len()));
        let Some((road_start, _)) = winner else {
            return;
        };
        let dinner = graph
            .roads
            .iter()
            .min_by_key(|(_, asdf)| r32((asdf.position - waypoint.pos).len()));

        let Some((road_end, _)) = dinner else {
            return;
        };
        let mut g: HashMap<Index, Vec<Index>> = HashMap::new();
        for edge in &graph.connections {
            g.entry(edge[0]).or_default().push(edge[1]);
            g.entry(edge[1]).or_default().push(edge[0]);
        }
        let Some((path, _)) = astar::astar(
            &road_start,
            |&idx| {
                g[&idx].iter().map(move |a| {
                    (
                        *a,
                        (graph.roads[idx].position - graph.roads[*a].position).len() as i32,
                    )
                })
            },
            |_| 0,
            |idx| *idx == road_end,
        ) else {
            return;
        };
        for thing in path.windows(2) {
            let from = thing[0];
            let to = thing[1];
            let a = graph.roads[from].position;
            let b = graph.roads[to].position;
            if let Some(pos_a) =
                camera.world_to_screen(framebuffer.size().map(|x| x as f32), vec3(a.x, a.y, 0.0))
            {
                if let Some(pos_b) = camera
                    .world_to_screen(framebuffer.size().map(|x| x as f32), vec3(b.x, b.y, 0.0))
                {
                    let color = Rgba::MAGENTA;
                    global.geng.draw2d().draw2d(
                        framebuffer,
                        &geng::PixelPerfectCamera,
                        &draw2d::Segment::new(Segment(pos_a, pos_b), 5.0, color),
                    )
                }
            }
        }
    }
}

fn emotes(receiver: Receiver<ServerMessage>, mut sender: Sender<Insert<BikeJump>>) {
    if let ServerMessage::Emote(id, typ) = receiver.event {}
}

pub async fn init(
    world: &mut World,
    geng: &Geng,
    assets: &Rc<Assets>,
    rng: &mut dyn RngCore,
    editor: bool,
    startup: &Startup,
) {
    world.add_handler(emotes);
    world.add_handler(bike_jump);
    let mk_quad = |size: f32, texture_repeats: f32| -> Rc<ugli::VertexBuffer<Vertex>> {
        Rc::new(ugli::VertexBuffer::new_static(
            geng.ugli(),
            vec![
                Vertex {
                    a_pos: vec3(-size, -size, 0.0),
                    a_uv: vec2(0.0, 0.0),
                    a_color: Rgba::WHITE,
                },
                Vertex {
                    a_pos: vec3(size, -size, 0.0),
                    a_uv: vec2(texture_repeats, 0.0),
                    a_color: Rgba::WHITE,
                },
                Vertex {
                    a_pos: vec3(size, size, 0.0),
                    a_uv: vec2(texture_repeats, texture_repeats),
                    a_color: Rgba::WHITE,
                },
                Vertex {
                    a_pos: vec3(-size, size, 0.0),
                    a_uv: vec2(0.0, texture_repeats),
                    a_color: Rgba::WHITE,
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

            white_texture: Texture(Rc::new(ugli::Texture::new_with(
                geng.ugli(),
                vec2(1, 1),
                |_| Rgba::WHITE,
            ))),
            editor,

            bikes: vec![bike_normal(&quad, assets), bike_unicycle(&quad, assets)],
        },
    );
    world.insert(
        global,
        Meshes {
            salmon_mesh: Rc::new(assets.models.salmon.to_vertex_buffer(geng.ugli())),
            hats: assets
                .models
                .hats
                .iter()
                .map(|hat| Rc::new(hat.to_vertex_buffer(geng.ugli())))
                .collect(),
        },
    );
    world.insert(
        global,
        Camera {
            preset: 0,
            attack_angle: Angle::from_degrees(1.0),
            rotation: Angle::from_degrees(1.0),
            position: vec3(0.0, 0.0, 0.0),
            distance: 1.0,
            show_self: true,
            fov: Angle::from_degrees(1.0),
            fish_eye_transform: None,
        },
    );
    world.insert(
        global,
        MinimapCamera {
            attack_angle: Angle::from_degrees(90.0),
            rotation: Angle::from_degrees(0.0),
            position: vec3(0.0, 0.0, 0.0),
            fov: config.minimap.fov,
        },
    );
    world.insert(
        global,
        LeaderboardTexture {
            texture: {
                let mut texture = ugli::Texture::new_with(geng.ugli(), vec2(1024, 512), |_| {
                    Rgba::TRANSPARENT_BLACK
                });
                // texture.set_filter(ugli::Filter::Nearest);
                texture
            },
        },
    );

    // ground
    let ground = world.spawn();
    world.insert(
        ground,
        Object {
            parts: vec![ModelPart {
                draw_mode: DrawMode::TriangleFan,
                mesh: mk_quad(100.0, 100.0),
                texture: assets.ground.clone(),
                transform: mat4::identity(),
                billboard: false,
                is_self: false,
            }],
            transform: mat4::identity(),
            replace_color: None,
        },
    );

    world.add_handler(roads::setup_road_graphics);
    world.add_handler(setup_buildings);
    world.add_handler(setup_trees);
    world.add_handler(setup_shops);

    world.add_handler(setup_bike_graphics);
    world.add_handler(setup_fish_graphics);
    world.add_handler(setup_car_graphics);
    world.add_handler(set_bike_variant);
    world.add_handler(set_hat_variant);
    world.add_handler(update_camera);
    world.add_handler(rotate_wheels);
    world.add_handler(update_vehicle_transforms);
    world.add_handler(update_fish);
    world.add_handler(render_leaderboard);

    world.add_handler(clear);
    world.add_handler(draw_objects);
    world.add_handler(draw_hats);
    world.add_handler(draw_leaderboards);
    world.add_handler(draw_waypoints);
    if editor {
        world.add_handler(draw_road_editor);
    }
    world.add_handler(draw_minimap);
    world.add_handler(draw_gps_line);
    world.add_handler(camera_follow);
    if editor {
        world.add_handler(camera_editor);
    }
    world.add_handler(minimap_follow);

    world.add_handler(update_shop);

    for data in &startup.level.trees {
        let entity = world.spawn();
        world.insert(entity, data.clone());
    }

    world.add_handler(draw_leaderboard);

    world.add_handler(emit_particles);
    world.add_handler(update_particles);

    world.add_handler(draw_invitation);
    world.add_handler(draw_invite_target);
    world.add_handler(draw_team_leader);
    world.add_handler(draw_names);
}

fn draw_names(
    mut receiver: ReceiverMut<Draw>,
    global: Single<&Global>,
    vehicles: Fetcher<(&Vehicle, &Name)>,
    camera: Single<&Camera>,
) {
    let framebuffer = &mut *receiver.event.framebuffer;
    let font = global.geng.default_font();
    for (vehicle, name) in vehicles {
        let Some(pos) = camera.world_to_screen(
            framebuffer.size().map(|x| x as f32),
            vehicle.pos.extend(2.5),
        ) else {
            continue;
        };
        let ui_cam = Camera2d {
            center: vec2::ZERO,
            rotation: Angle::ZERO,
            fov: 50.0,
        };
        font.draw(
            framebuffer,
            &ui_cam,
            &name.0,
            vec2::splat(geng::TextAlign::CENTER),
            mat3::translate(ui_cam.screen_to_world(framebuffer.size().map(|x| x as f32), pos)),
            Rgba::BLACK,
        );
    }
}

fn draw_team_leader(
    mut receiver: ReceiverMut<Draw>,
    global: Single<&Global>,
    vehicles: Fetcher<(&Vehicle, &TeamLeader)>,
    names: Fetcher<&Name>,
    camera: Single<&Camera>,
) {
    let framebuffer = &mut *receiver.event.framebuffer;
    let font = global.geng.default_font();
    for (vehicle, leader) in vehicles {
        let Ok(leader_name) = names.get(leader.0) else {
            continue;
        };
        let Some(pos) = camera.world_to_screen(
            framebuffer.size().map(|x| x as f32),
            vehicle.pos.extend(3.0),
        ) else {
            continue;
        };
        let ui_cam = Camera2d {
            center: vec2::ZERO,
            rotation: Angle::ZERO,
            fov: 50.0,
        };
        font.draw(
            framebuffer,
            &ui_cam,
            &format!("leader: {}", &leader_name.0),
            vec2::splat(geng::TextAlign::CENTER),
            mat3::translate(ui_cam.screen_to_world(framebuffer.size().map(|x| x as f32), pos)),
            Rgba::BLACK,
        );
    }
}

fn draw_invite_target(
    mut receiver: ReceiverMut<Draw>,
    global: Single<&Global>,
    vehicles: Fetcher<&Vehicle>,
    target: TrySingle<&InviteTarget>,
    camera: Single<&Camera>,
) {
    #[derive(ugli::Vertex)]
    struct Instance {
        i_model_matrix: mat4<f32>,
        i_replace_color: Rgba<f32>,
    }
    let framebuffer = &mut *receiver.event.framebuffer;
    if let Ok(target) = target.0 {
        if let Ok(vehicle) = vehicles.get(target.entity) {
            let transform = mat4::translate(vehicle.pos.extend(3.0));
            ugli::draw(
                framebuffer,
                &global.assets.shaders.billboard,
                ugli::DrawMode::TriangleFan,
                ugli::instanced(
                    &*global.quad,
                    &ugli::VertexBuffer::new_dynamic(
                        global.geng.ugli(),
                        vec![Instance {
                            i_model_matrix: transform,
                            i_replace_color: Rgba::WHITE,
                        }],
                    ),
                ),
                (
                    ugli::uniforms! {
                        u_time: global.timer.elapsed().as_secs_f64() as f32,
                        u_wiggle: 0.0,
                        u_texture: global.white_texture.ugli(), // tODO
                        u_match_color: Rgba::WHITE,
                    },
                    camera.uniforms(framebuffer.size().map(|x| x as f32)),
                ),
                ugli::DrawParameters { ..default() },
            );
        }
    }
}

fn draw_invitation(
    mut receiver: ReceiverMut<Draw>,
    global: Single<&Global>,
    names: Fetcher<&Name>,
    invitation: TrySingle<&Invitation>,
) {
    let framebuffer = &mut *receiver.event.framebuffer;
    if let Ok(invitation) = invitation.0 {
        let Ok(team_name) = names.get(invitation.entity_id) else {
            return;
        };
        let font = global.geng.default_font();
        font.draw(
            framebuffer,
            &Camera2d {
                center: vec2::ZERO,
                rotation: Angle::ZERO,
                fov: 20.0,
            },
            &format!("You were invited to team {}\npress Y/N", &team_name.0),
            vec2::splat(geng::TextAlign::CENTER),
            mat3::identity(),
            Rgba::BLACK,
        );
    }
}

fn draw_hats(
    mut receiver: ReceiverMut<Draw>,
    players: Fetcher<(&Object, &Bike, Has<&LocalPlayer>)>,
    global: Single<&Global>,
    meshes: Single<&Meshes>,
    camera: Single<&Camera>,
) {
    let framebuffer = &mut *receiver.event.framebuffer;
    let match_color = Rgba::BLACK;

    for (object, bike, local) in players {
        if local.get() && !camera.show_self {
            continue;
        }
        if let Some(mesh) = bike.hat_type.and_then(|hat| meshes.hats.get(hat)) {
            let transform = object.transform
                * mat4::translate(vec3(-0.8, 0.00, 2.2))
                * mat4::scale_uniform(1.0 / 24.0)
                * mat4::scale(vec3(1.0, 1.0, -1.0));
            ugli::draw(
                framebuffer,
                &global.assets.shaders.main_no_instancing,
                ugli::DrawMode::Triangles,
                &**mesh,
                (
                    ugli::uniforms! {
                        u_time: global.timer.elapsed().as_secs_f64() as f32,
                        u_wiggle: 0.0,
                        u_texture: global.white_texture.ugli(),
                        u_model_matrix: transform,
                        u_match_color: match_color,
                        u_replace_color: match_color,
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

fn draw_leaderboard(
    mut receiver: ReceiverMut<Draw>,
    leaders: TrySingle<&Leaderboard>,
    global: Single<&Global>,
) {
    if global.geng.window().is_key_pressed(geng::Key::Tab) {
        if let Ok(board) = leaders.0 {
            let mut text = String::new();
            for (index, row) in board.rows.iter().enumerate() {
                if index != 0 {
                    text += "\n";
                }
                text += &format!("{}. {} - {}", index + 1, row.0, row.1);
            }
            let framebuffer = &mut *receiver.event.framebuffer;
            let font = global.geng.default_font(); // TODO: assets.font?
            font.draw(
                framebuffer,
                &Camera2d {
                    center: vec2::ZERO,
                    rotation: Angle::ZERO,
                    fov: 20.0,
                },
                &text,
                vec2::splat(geng::TextAlign::CENTER),
                mat3::identity(),
                Rgba::BLACK,
            );
        }
    }
}

fn draw_leaderboards(
    mut receiver: ReceiverMut<Draw>,
    boards: Fetcher<&LeaderboardBillboard>,
    texture: Single<&LeaderboardTexture>,
    global: Single<&Global>,
    camera: Single<&Camera>,
) {
    let framebuffer = &mut *receiver.event.framebuffer;

    let match_color = Rgba::BLACK;
    for board in boards {
        let scale = 2.0;
        let transform = mat4::translate(board.pos.extend(0.0))
            * mat4::rotate_z(board.rotation)
            * mat4::rotate_x(Angle::from_degrees(90.0))
            * mat4::scale(vec3(1.0, 1.0, 0.5))
            * mat4::scale_uniform(scale);

        ugli::draw(
            framebuffer,
            &global.assets.shaders.main_no_instancing,
            ugli::DrawMode::TriangleFan,
            &*global.quad,
            (
                ugli::uniforms! {
                    u_time: global.timer.elapsed().as_secs_f64() as f32,
                    u_wiggle: 0.0,
                    u_texture: global.assets.billboard_legs.ugli(),
                    u_model_matrix: transform,
                    u_match_color: match_color,
                    u_replace_color: match_color,
                },
                camera.uniforms(framebuffer.size().map(|x| x as f32)),
            ),
            ugli::DrawParameters {
                depth_func: Some(ugli::DepthFunc::Less),
                ..default()
            },
        );

        let transform = mat4::translate(vec3(0.0, 0.0, 2.0 * scale)) * transform;
        ugli::draw(
            framebuffer,
            &global.assets.shaders.main_no_instancing,
            ugli::DrawMode::TriangleFan,
            &*global.quad,
            (
                ugli::uniforms! {
                    u_time: global.timer.elapsed().as_secs_f64() as f32,
                    u_wiggle: 0.0,
                    u_texture: global.assets.billboard_top.ugli(),
                    u_model_matrix: transform,
                    u_match_color: match_color,
                    u_replace_color: match_color,
                },
                camera.uniforms(framebuffer.size().map(|x| x as f32)),
            ),
            ugli::DrawParameters {
                depth_func: Some(ugli::DepthFunc::Less),
                ..default()
            },
        );

        {
            let transform = transform * mat4::translate(vec3(0.0, 0.0, 0.01));
            ugli::draw(
                framebuffer,
                &global.assets.shaders.main_no_instancing,
                ugli::DrawMode::TriangleFan,
                &*global.quad,
                (
                    ugli::uniforms! {
                        u_time: global.timer.elapsed().as_secs_f64() as f32,
                        u_wiggle: 0.0,
                        u_texture: &texture.texture,
                        u_model_matrix: transform,
                        u_match_color: match_color,
                        u_replace_color: match_color,
                    },
                    camera.uniforms(framebuffer.size().map(|x| x as f32)),
                ),
                ugli::DrawParameters {
                    depth_func: Some(ugli::DepthFunc::Less),
                    ..default()
                },
            );
        }

        let transform =
            transform * mat4::scale(vec3(-1.0, 1.0, 1.0)) * mat4::translate(vec3(0.0, 0.0, -0.01));
        ugli::draw(
            framebuffer,
            &global.assets.shaders.main_no_instancing,
            ugli::DrawMode::TriangleFan,
            &*global.quad,
            (
                ugli::uniforms! {
                    u_time: global.timer.elapsed().as_secs_f64() as f32,
                    u_wiggle: 0.0,
                    u_texture: &texture.texture,
                    u_model_matrix: transform,
                    u_match_color: match_color,
                    u_replace_color: match_color,
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

fn update_camera(
    _receiver: Receiver<Update>,
    mut camera: Single<&mut Camera>,
    player: Single<(&Object, With<&LocalPlayer>)>,
    global: Single<&Global>,
) {
    let preset = &global.config.camera[camera.preset % global.config.camera.len()];
    if !preset.auto_rotate {
        camera.rotation = Angle::from_degrees(preset.default_rotation);
    }
    camera.attack_angle = Angle::from_degrees(preset.attack_angle);
    camera.fov = Angle::from_degrees(preset.fov);
    camera.distance = preset.distance;
    camera.show_self = preset.show_self;
    camera.fish_eye_transform = (!camera.show_self).then_some(
        mat4::translate(preset.offset)
            * mat4::rotate_x(Angle::from_degrees(-90.0))
            * mat4::rotate_z(Angle::from_degrees(-90.0))
            * player.0 .0.transform.inverse(),
    );
}

fn update_shop(
    receiver: Receiver<Update>,
    mut shop: Single<(&mut Object, EntityId, &mut Shop)>,
    me: Single<(&Vehicle, With<&LocalPlayer>)>,
    sender: Sender<Insert<Object>>,
) {
    let was = shop.2.door_time;
    if (shop.2.pos - me.0 .0.pos).len() < 10.0 {
        shop.2.door_time += receiver.event.delta_time.as_secs_f64() as f32;
    } else {
        shop.2.door_time -= receiver.event.delta_time.as_secs_f64() as f32;
    }
    shop.2.door_time = shop.2.door_time.clamp(0.0, 1.0);
    let t = shop.2.door_time;
    let t = 3.0 * t * t - 2.0 * t * t * t;
    if was != t {
        let height = 6.0;
        shop.0 .0.parts[0].transform = mat4::translate(vec3(-1.6 * t, 0.0, 1.6 * t))
            * mat4::rotate_y(Angle::from_degrees(90.0) * t)
            * mat4::rotate_z(Angle::from_degrees(90.0))
            * mat4::translate(vec3(0.0, 3.0, 0.0))
            * mat4::scale(vec3(6.0, 1.0, height / 2.0))
            * mat4::rotate_x(Angle::from_degrees(90.0))
            * mat4::translate(vec3(0.0, 1.0, 0.0));
    }
}

fn setup_shops(
    receiver: Receiver<Insert<Shop>, ()>,
    global: Single<&Global>,
    mut sender: Sender<Insert<Object>>,
) {
    let shop = &receiver.event.component;
    let mut parts = Vec::new();

    let half_size = vec2(3.0, 6.0);

    let assets = &global.assets.garage;
    let height = 4.0 * half_size.x / assets.back.size().map(|x| x as f32).aspect();
    // door
    parts.push(ModelPart {
        mesh: global.quad.clone(),
        draw_mode: DrawMode::TriangleFan,
        texture: assets.door.clone(),
        transform: mat4::rotate_z(Angle::from_degrees(90.0))
            * mat4::translate(vec3(0.0, half_size.x, 0.0))
            * mat4::scale(vec3(half_size.y, 1.0, height / 2.0))
            * mat4::rotate_x(Angle::from_degrees(90.0))
            * mat4::translate(vec3(0.0, 1.0, 0.0)),
        billboard: false,
        is_self: false,
    });
    // top
    parts.push(ModelPart {
        mesh: global.quad.clone(),
        draw_mode: DrawMode::TriangleFan,
        texture: assets.top.clone(),
        transform: mat4::translate(vec3(0.0, 0.0, height)) * mat4::scale(half_size.extend(1.0)),
        billboard: false,
        is_self: false,
    });
    // awning
    parts.push(ModelPart {
        mesh: global.quad.clone(),
        draw_mode: DrawMode::TriangleFan,
        texture: assets.awning.clone(),
        transform: mat4::translate(vec3(-4.0, 0.0, height * 0.8))
            * mat4::rotate_y(Angle::from_degrees(-12.0))
            * mat4::scale(vec3(1.0, 6.0, 1.0))
            * mat4::rotate_z(Angle::from_degrees(90.0)),
        billboard: false,
        is_self: false,
    });

    // sides
    for (i, side) in [&assets.side_a, &assets.front, &assets.side_b, &assets.back]
        .iter()
        .enumerate()
    {
        parts.push(ModelPart {
            is_self: false,
            mesh: global.quad.clone(),
            draw_mode: DrawMode::TriangleFan,
            texture: (*side).clone(),
            transform: mat4::rotate_z(Angle::from_degrees(90.0) * i as f32)
                * mat4::translate(vec3(
                    0.0,
                    if i % 2 == 0 { half_size.y } else { half_size.x },
                    0.0,
                ))
                * mat4::scale(vec3(
                    if i % 2 == 0 { half_size.x } else { half_size.y },
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
            transform: mat4::translate(shop.pos.extend(0.0)) * mat4::rotate_z(shop.rotation),
            replace_color: None,
        },
    );
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
            draw_mode: DrawMode::TriangleFan,
            texture: assets.top.clone(),
            transform: mat4::translate(vec3(0.0, 0.0, height))
                * mat4::scale(building.half_size.extend(1.0)),
            billboard: false,
            is_self: false,
        });

        // sides
        for i in 0..4 {
            parts.push(ModelPart {
                mesh: global.quad.clone(),
                draw_mode: DrawMode::TriangleFan,
                texture: if i == 0 {
                    assets.side_a.clone()
                } else {
                    assets.side_b.clone()
                },
                is_self: false,
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
            draw_mode: DrawMode::TriangleFan,
            texture: assets.tops.choose(&mut rng.gen).unwrap().clone(),
            transform: mat4::translate(vec3(0.0, 0.0, height))
                * mat4::scale(building.half_size.extend(1.0)),
            billboard: false,
            is_self: false,
        });

        // sides
        for i in 0..4 {
            parts.push(ModelPart {
                is_self: false,
                mesh: global.quad.clone(),
                draw_mode: DrawMode::TriangleFan,
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
                    is_self: false,
                    mesh: global.quad.clone(),
                    draw_mode: DrawMode::TriangleFan,
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

#[derive(Event)]
pub enum Shopping {
    Enter,
    Exit,
}

fn camera_editor(
    receiver: Receiver<Update>,
    mut camera: Single<&mut Camera>,
    global: Single<&Global>,
    controller: Single<&VehicleController>,
) {
    if !global.editor {
        return;
    }
    let move_dir = vec2(-controller.rotate, controller.accelerate).rotate(camera.rotation) * 50.0;
    camera.position += move_dir.extend(0.0) * receiver.event.delta_time.as_secs_f64() as f32;
}

fn camera_follow(
    receiver: Receiver<Update>,
    mut camera: Single<&mut Camera>,
    global: Single<&Global>,
    player: TrySingle<(&Vehicle, With<&LocalPlayer>)>,
    mut sender: Sender<Shopping>,
    shop: Single<&Shop>,
) {
    if global.editor {
        return;
    }

    let preset = &global.config.camera[camera.preset % global.config.camera.len()];
    let camera: &mut Camera = &mut camera;
    let delta_time = receiver.event.delta_time.as_secs_f64() as f32;
    let Ok((player, _)) = player.0 else {
        return;
    };
    let k = (preset.speed * delta_time).min(1.0);
    let mut target_position = player.pos.extend(0.0)
        + vec2(player.speed, 0.0).rotate(player.rotation).extend(0.0) * preset.predict
        + (mat4::rotate_z(player.rotation) * preset.offset.extend(1.0)).xyz();
    let mut target_rotation = player.rotation - Angle::from_degrees(90.0);
    let inside_shop = {
        let half_size = vec2(3.0, 6.0);
        let position_inside_shop = (player.pos - shop.pos).rotate(-shop.rotation);
        position_inside_shop.x.abs() < half_size.x && position_inside_shop.y.abs() < half_size.y
    };
    static INSIDE: AtomicBool = AtomicBool::new(false);
    let was_inside = INSIDE.swap(inside_shop, std::sync::atomic::Ordering::SeqCst);
    if inside_shop != was_inside {
        sender.send(if inside_shop {
            Shopping::Enter
        } else {
            Shopping::Exit
        });
    }
    if inside_shop {
        let settings = &global.config.shop_camera;
        target_position = shop.pos.extend(0.0) + settings.offset;
        target_rotation = Angle::from_degrees(settings.default_rotation);
        camera.attack_angle = Angle::from_degrees(settings.attack_angle);
        camera.fov = Angle::from_degrees(settings.fov);
        camera.distance = settings.distance;
        camera.show_self = settings.show_self;
    }
    camera.position += (target_position - camera.position) * k;
    if preset.auto_rotate {
        camera.rotation = (camera.rotation
            + (target_rotation - camera.rotation).normalized_pi() * k)
            .normalized_2pi();
    }
}
fn minimap_follow(
    _receiver: Receiver<Update>,
    camera: Single<&Camera>,
    mut minimap: Single<&mut MinimapCamera>,
) {
    let minimap: &mut MinimapCamera = &mut minimap;
    minimap.position = camera.position + vec3(0.0, 0.0, 50.0);
    minimap.rotation = camera.rotation;
}

#[derive(Component)]
pub struct Wheelie {
    front: bool,
    t: f32,
}
impl Wheelie {
    pub fn new(front: bool) -> Self {
        Self { front, t: 0.0 }
    }
}

#[derive(Component, Default)]
pub struct BikeJump {
    t: f32,
}

fn bike_jump(
    receiver: Receiver<Update>,
    bikes: Fetcher<(EntityId, &mut BikeJump)>,
    wheelies: Fetcher<(EntityId, &mut Wheelie)>,
    mut sender: Sender<(Remove<BikeJump>, Remove<Wheelie>)>,
) {
    let delta_time = receiver.event.delta_time.as_secs_f64() as f32;
    for (entity, bike) in bikes {
        bike.t += delta_time * 3.0;
        if bike.t > 1.0 {
            sender.remove::<BikeJump>(entity);
        }
    }
    for (entity, bike) in wheelies {
        bike.t += delta_time * 1.0;
        if bike.t > 1.0 {
            sender.remove::<Wheelie>(entity);
        }
    }
}

fn rotate_wheels(receiver: Receiver<Update>, vehicles: Fetcher<(&Vehicle, &mut VehicleWheels)>) {
    for (vehicle, wheels) in vehicles {
        let radius = 0.5;
        wheels.rotation += Angle::from_radians(
            std::f32::consts::PI
                * vehicle.speed
                * radius
                * receiver.event.delta_time.as_secs_f64() as f32,
        );
    }
}

#[allow(clippy::type_complexity)]
fn update_fish(
    _receiver: Receiver<Draw>,
    fish: Fetcher<(&Fish, &mut Object, Not<With<&Vehicle>>)>,
    bikes: Fetcher<(&Object, With<&Vehicle>)>,
) {
    for (fish, fish_object, _) in fish {
        if let Ok((bike_object, _)) = bikes.get(fish.bike) {
            fish_object.transform = bike_object.transform;
        }
    }
}

#[allow(clippy::type_complexity)]
fn update_vehicle_transforms(
    _receiver: Receiver<Draw>,
    global: Single<&Global>,
    bikes: Fetcher<(
        &Vehicle,
        Option<&mut VehicleWheels>,
        Option<&BikeJump>,
        Option<&Wheelie>,
        &VehicleProperties,
        &mut Object,
        Has<&Car>,
    )>,
) {
    for (bike, wheels, bike_jump, wheelie, props, object, car) in bikes {
        if let Some(wheels) = wheels {
            for wheel in &wheels.wheels {
                let part = &mut object.parts[wheel.model_part];
                part.transform = wheel.transform * mat4::rotate_z(wheels.rotation);
            }
        }
        object.transform = mat4::translate(
            bike.pos
                .extend((bike_jump.map_or(0.0, |jump| jump.t) * f32::PI).sin()),
        ) * mat4::rotate_z(bike.rotation + Angle::from_degrees(180.0))
            * mat4::rotate_x(bike.rotation_speed * 0.1 * bike.speed / props.max_speed)
            * wheelie.map_or(mat4::identity(), |w| {
                let pos = if w.front { -0.7 } else { 0.7 };
                mat4::translate(vec3(pos, 0.0, 0.0))
                    * mat4::rotate_y(
                        Angle::from_degrees(40.0)
                            * (w.t * f32::PI).sin()
                            * if w.front { -1.0 } else { 1.0 },
                    )
                    * mat4::translate(vec3(-pos, 0.0, 0.0))
            });
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
                    is_self: false,
                    draw_mode: DrawMode::TriangleFan,
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
                        is_self: false,
                        draw_mode: DrawMode::TriangleFan,
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
                        is_self: false,
                        draw_mode: DrawMode::TriangleFan,
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
                        is_self: false,
                        draw_mode: DrawMode::TriangleFan,
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
                    is_self: false,
                    draw_mode: DrawMode::TriangleFan,
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

fn setup_fish_graphics(
    receiver: Receiver<Insert<Fish>, ()>,
    global: Single<&Global>,
    meshes: Single<&Meshes>,
    mut sender: Sender<Insert<Object>>,
) {
    let fish = receiver.event.entity;
    sender.insert(
        fish,
        Object {
            parts: vec![
                ModelPart {
                    draw_mode: DrawMode::Triangles,
                    mesh: meshes.salmon_mesh.clone(),
                    texture: global.white_texture.clone(),
                    transform: mat4::translate(vec3(-0.8, 0.00, 1.0))
                        * mat4::scale_uniform(1.0 / 24.0)
                        * mat4::scale(vec3(1.0, 1.0, -1.0)),
                    billboard: false,
                    is_self: true,
                },
                ModelPart {
                    draw_mode: DrawMode::TriangleFan,
                    mesh: global.quad.clone(),
                    texture: global.assets.salmonfin.clone(),
                    transform: mat4::translate(vec3(-0.35, 0.00, 1.5))
                        // * mat4::scale_uniform(1.5)
                        * mat4::rotate_x(Angle::from_degrees(90.0)),
                    billboard: false,
                    is_self: true,
                },
                // ModelPart {
                //     draw_mode: DrawMode::TriangleFan,
                //     mesh: global.quad.clone(),
                //     texture: global.assets.salmon2.clone(),
                //     transform: mat4::translate(vec3(-0.3, -0.02, 1.6))
                //         * mat4::scale_uniform(0.75)
                //         * mat4::rotate_x(Angle::from_degrees(90.0)),
                //     // * mat4::rotat_x(Angle::from_degrees(90.0)),
                //     billboard: false,
                // },
            ],
            transform: mat4::identity(),
            replace_color: None,
        },
    );
}

#[allow(clippy::type_complexity)]
fn setup_bike_graphics(
    receiver: Receiver<Insert<Vehicle>, With<&Bike>>,
    global: Single<&Global>,
    mut sender: Sender<(Spawn, Insert<Object>, Insert<VehicleWheels>, Insert<Fish>)>,
) {
    let bike = receiver.event.entity;
    let (object, wheels) = bike_normal(&global.quad, &global.assets);
    sender.insert(bike, object);
    sender.insert(bike, wheels);
}

fn render_leaderboard(
    receiver: Receiver<Insert<Leaderboard>, ()>,
    mut texture: Single<&mut LeaderboardTexture>,
    global: Single<&Global>,
) {
    let board = &receiver.event.component;

    let mut framebuffer = ugli::Framebuffer::new_color(
        global.geng.ugli(),
        ugli::ColorAttachment::Texture(&mut texture.texture),
    );
    ugli::clear(&mut framebuffer, Some(Rgba::TRANSPARENT_BLACK), None, None);
    let framebuffer_size = framebuffer.size().map(|x| x as f32);

    let font = global.geng.default_font();
    let font_size = framebuffer_size.y / 7.0;

    let mut y = framebuffer_size.y - font_size * 1.2;
    for (index, row) in board.rows.iter().enumerate() {
        let text = format!("{}. {} - {}", index + 1, row.0, row.1);
        font.draw(
            &mut framebuffer,
            &geng::PixelPerfectCamera,
            &text,
            vec2::splat(geng::TextAlign::CENTER),
            mat3::translate(vec2(framebuffer_size.x / 2.0, y)) * mat3::scale_uniform(font_size),
            Rgba::BLACK,
        );
        y -= font_size;
    }
}

fn set_bike_variant(
    receiver: Receiver<SetBikeType>,
    mut bikes: Fetcher<(&mut Object, &mut VehicleWheels)>,
    global: Single<&Global>,
) {
    if let Ok((object, wheels)) = bikes.get_mut(receiver.event.bike_id) {
        let i = receiver.event.bike_type;
        if let Some((new_object, new_wheels)) = global.bikes.get(i) {
            let transform = object.transform;
            *object = new_object.clone();
            object.transform = transform;
            *wheels = new_wheels.clone();
        }
    }
}

fn set_hat_variant(receiver: Receiver<SetHatType>, mut bikes: Fetcher<&mut Bike>) {
    if let Ok(bike) = bikes.get_mut(receiver.event.bike_id) {
        bike.hat_type = receiver.event.hat_type;
    }
}

fn bike_normal(quad: &Rc<ugli::VertexBuffer<Vertex>>, assets: &Assets) -> (Object, VehicleWheels) {
    (
        Object {
            replace_color: None,
            parts: vec![
                ModelPart {
                    draw_mode: DrawMode::TriangleFan,
                    mesh: quad.clone(),
                    texture: assets.bike.top.clone(),
                    transform: mat4::translate(vec3(0.0, 0.0, 1.1)),
                    billboard: false,
                    is_self: false,
                },
                ModelPart {
                    draw_mode: DrawMode::TriangleFan,
                    mesh: quad.clone(),
                    texture: assets.bike.top_handle.clone(),
                    transform: mat4::translate(vec3(0.0, 0.0, 1.4)),
                    billboard: false,
                    is_self: false,
                },
                ModelPart {
                    draw_mode: DrawMode::TriangleFan,
                    mesh: quad.clone(),
                    texture: assets.bike.side.clone(),
                    transform: mat4::translate(vec3(0.0, 0.0, 1.0))
                        * mat4::rotate_x(Angle::from_degrees(90.0)),
                    billboard: false,
                    is_self: false,
                },
                ModelPart {
                    draw_mode: DrawMode::TriangleFan,
                    mesh: quad.clone(),
                    texture: assets.bike.wheel.clone(),
                    transform: mat4::identity(),
                    billboard: false,
                    is_self: false,
                },
                ModelPart {
                    draw_mode: DrawMode::TriangleFan,
                    mesh: quad.clone(),
                    texture: assets.bike.wheel.clone(),
                    transform: mat4::identity(),
                    billboard: false,
                    is_self: false,
                },
            ],
            transform: mat4::identity(),
        },
        VehicleWheels {
            wheels: vec![
                Wheel {
                    model_part: 3,
                    transform: mat4::translate(vec3(0.5, 0.0, 0.6))
                        * mat4::rotate_x(Angle::from_degrees(90.0))
                        * mat4::translate(vec3(0.0, 0.0, 0.01))
                        * mat4::scale_uniform(0.5),
                },
                Wheel {
                    model_part: 4,
                    transform: mat4::translate(vec3(-0.5, 0.0, 0.6))
                        * mat4::rotate_x(Angle::from_degrees(90.0))
                        * mat4::translate(vec3(0.0, 0.0, 0.01))
                        * mat4::scale_uniform(0.5),
                },
            ],
            rotation: Angle::ZERO,
        },
    )
}

fn bike_unicycle(
    quad: &Rc<ugli::VertexBuffer<Vertex>>,
    assets: &Assets,
) -> (Object, VehicleWheels) {
    (
        Object {
            replace_color: None,
            parts: vec![
                ModelPart {
                    draw_mode: DrawMode::TriangleFan,
                    mesh: quad.clone(),
                    texture: assets.bike.unicycle_top.clone(),
                    transform: mat4::translate(vec3(0.0, 0.0, 1.1)),
                    billboard: false,
                    is_self: false,
                },
                ModelPart {
                    draw_mode: DrawMode::TriangleFan,
                    mesh: quad.clone(),
                    texture: assets.bike.unicycle_seat.clone(),
                    transform: mat4::translate(vec3(-0.2, 0.0, 1.5)),
                    billboard: false,
                    is_self: false,
                },
                ModelPart {
                    draw_mode: DrawMode::TriangleFan,
                    mesh: quad.clone(),
                    texture: assets.bike.unicycle_side.clone(),
                    transform: mat4::translate(vec3(0.0, 0.0, 1.0))
                        * mat4::rotate_x(Angle::from_degrees(90.0)),
                    billboard: false,
                    is_self: false,
                },
                ModelPart {
                    draw_mode: DrawMode::TriangleFan,
                    mesh: quad.clone(),
                    texture: assets.bike.wheel.clone(),
                    transform: mat4::identity(),
                    billboard: false,
                    is_self: false,
                },
            ],
            transform: mat4::identity(),
        },
        VehicleWheels {
            wheels: vec![Wheel {
                model_part: 3,
                transform: mat4::translate(vec3(-0.05, 0.0, 0.6))
                    * mat4::rotate_x(Angle::from_degrees(90.0))
                    * mat4::translate(vec3(0.0, 0.0, 0.01))
                    * mat4::scale_uniform(0.5),
            }],
            rotation: Angle::ZERO,
        },
    )
}
