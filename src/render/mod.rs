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

#[derive(Component)]
pub struct Object {
    pub mesh: Rc<ugli::VertexBuffer<Vertex>>,
    pub texture: Texture,
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

#[derive(Component)]
pub struct Global {
    geng: Geng,
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
        ugli::draw(
            framebuffer,
            &global.assets.shaders.main,
            ugli::DrawMode::TriangleFan,
            &*object.mesh,
            (
                ugli::uniforms! {
                    u_texture: object.texture.ugli(),
                    u_model_matrix: object.transform,
                },
                camera.uniforms(framebuffer.size().map(|x| x as f32)),
            ),
            ugli::DrawParameters {
                depth_func: Some(ugli::DepthFunc::Less),
                ..default()
            },
        )
    }
}

pub fn init(world: &mut World, geng: &Geng, assets: &Rc<Assets>) {
    let quad = Rc::new(ugli::VertexBuffer::new_static(
        geng.ugli(),
        vec![
            Vertex {
                a_pos: vec3(-1.0, -1.0, 0.0),
                a_uv: vec2(0.0, 0.0),
            },
            Vertex {
                a_pos: vec3(1.0, -1.0, 0.0),
                a_uv: vec2(1.0, 0.0),
            },
            Vertex {
                a_pos: vec3(1.0, 1.0, 0.0),
                a_uv: vec2(1.0, 1.0),
            },
            Vertex {
                a_pos: vec3(-1.0, 1.0, 0.0),
                a_uv: vec2(0.0, 1.0),
            },
        ],
    ));

    let global = world.spawn();
    world.insert(
        global,
        Global {
            geng: geng.clone(),
            assets: assets.clone(),
            quad: quad.clone(),
        },
    );
    world.insert(
        global,
        Camera {
            attack_angle: Angle::from_degrees(60.0),
            rotation: Angle::from_degrees(20.0),
            position: vec3(0.0, 0.0, 0.0),
            distance: 50.0,
            fov: Angle::from_degrees(30.0),
        },
    );
    world.spawn();
    world.add_handler(clear);
    world.add_handler(draw_sprites);

    // test
    let top = world.spawn();
    world.insert(
        top,
        Object {
            mesh: quad.clone(),
            texture: assets.bike.top.clone(),
            transform: mat4::translate(vec3(0.0, 0.0, 1.1)),
        },
    );
    let top_handle = world.spawn();
    world.insert(
        top_handle,
        Object {
            mesh: quad.clone(),
            texture: assets.bike.top_handle.clone(),
            transform: mat4::translate(vec3(0.0, 0.0, 1.4)),
        },
    );
    let side = world.spawn();
    world.insert(
        side,
        Object {
            mesh: quad.clone(),
            texture: assets.bike.side.clone(),
            transform: mat4::translate(vec3(0.0, 0.0, 1.0))
                * mat4::rotate_x(Angle::from_degrees(90.0)),
        },
    );
}
