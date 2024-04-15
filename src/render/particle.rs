use geng::prelude::batbox::rng;
use parry2d::na::{Quaternion, SimdValue};

use super::*;

#[derive(Component)]
pub struct Particle {
    pos: vec3<f32>,
    timer: Timer,
    velocity: vec3<f32>,
    lifetime: std::time::Duration,
}

impl Particle {
    fn new(pos: vec3<f32>, velocity: vec3<f32>, lifetime: std::time::Duration) -> Self {
        Self {
            pos,
            timer: Timer::new(),
            velocity,
            lifetime,
        }
    }
}

#[derive(Component)]
pub struct Emitter {
    timer: Timer,
    particle_kind: i32,
    pub pos: vec3<f32>,
    pub spawnrate: time::Duration,
    pub particle_speed: f32,
    pub particle_diection: vec3<f32>,
    pub particle_direction_rng: vec3<f32>,
    pub particle_lifetime: std::time::Duration,
    pub particle_size: f32,
    pub emitting: bool,
}

impl Emitter {
    pub fn new(
        pos: vec3<f32>,
        spawnrate: time::Duration,
        particle_kind: i32,
        particle_size: f32,
        particle_speed: f32,
        particle_lifetime: std::time::Duration,
        particle_diection: vec3<f32>,
        particle_direction_rng: vec3<f32>,
    ) -> Self {
        Self {
            timer: Timer::new(),
            pos,
            spawnrate,
            particle_kind,
            particle_speed,
            particle_lifetime,
            particle_diection: particle_diection.normalize(),
            particle_direction_rng,
            particle_size,
            emitting: true,
        }
    }
}

fn randomize_axis(mut axis: f32, rng: f32) -> f32 {
    if rng > 0. {
        axis += rng::thread_rng().gen_range(-rng..rng);
    }
    axis
}

pub fn emit_particles(
    _receiver: Receiver<Update>,
    global: Single<&super::Global>,
    mut emitters: Fetcher<&mut Emitter>,
    mut sender: Sender<(Spawn, Insert<Particle>, Insert<Object>)>,
) {
    for emitter in emitters.iter_mut() {
        if !emitter.emitting {
            continue;
        }
        if emitter.timer.elapsed().as_secs_f64() > emitter.spawnrate.as_secs_f64() {
            emitter.timer.reset();
            let entity = sender.spawn();

            // let velocity = emitter.
            let velocity = vec3(
                randomize_axis(
                    emitter.particle_diection.x,
                    emitter.particle_direction_rng.x,
                ),
                randomize_axis(
                    emitter.particle_diection.y,
                    emitter.particle_direction_rng.y,
                ),
                randomize_axis(
                    emitter.particle_diection.z,
                    emitter.particle_direction_rng.z,
                ),
            );

            let particle = Particle::new(
                emitter.pos,
                velocity * emitter.particle_speed,
                emitter.particle_lifetime,
            );

            let texture = &global.assets.particles[emitter.particle_kind as usize];
            sender.insert(
                entity,
                Object {
                    parts: vec![ModelPart {
                        is_self: false,
                        mesh: global.quad.clone(),
                        draw_mode: ugli::DrawMode::TriangleFan,
                        texture: texture.clone(),
                        transform: mat4::scale(vec3::splat(emitter.particle_size))
                            * mat4::rotate_x(Angle::from_degrees(90.0)),
                        billboard: true,
                    }],
                    transform: mat4::translate(particle.pos),
                    replace_color: None,
                },
            );
            sender.insert(entity, particle);
        }
    }
}

pub fn update_particles(
    receiver: Receiver<Update>,
    mut particles: Fetcher<(&mut Particle, &mut Object, EntityId)>,
    mut sender: Sender<(Despawn, Insert<Particle>, Insert<Object>)>,
) {
    for (particle, obj, id) in particles.iter_mut() {
        particle.pos += particle.velocity * receiver.event.delta_time.as_secs_f64() as f32;
        obj.transform = mat4::translate(particle.pos);

        if particle.timer.elapsed().as_secs_f64() > particle.lifetime.as_secs_f64() {
            sender.despawn(id);
        }
    }
}
