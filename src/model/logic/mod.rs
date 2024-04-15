use crate::render::particle;
use crate::render::BikeJump;
use crate::sound::BonkEvent;

use super::*;

pub fn init(world: &mut World) {
    world.add_handler(bike_movement);
    world.add_handler(bike_collisions);
    world.add_handler(create_particle_emitters);
    world.add_handler(update_particle_emitter_position);
    world.add_handler(cars);
}

fn cars(
    receiver: Receiver<Update>,
    config: Single<&Config>,
    cars: Fetcher<(&mut Vehicle, &mut CarPath)>,
) {
    let delta_time = receiver.event.delta_time.as_secs_f64() as f32;
    for (vehicle, path) in cars {
        path.current_pos = (path.current_pos + delta_time * config.car_speed)
            .rem_euclid(path.nodes.last().unwrap().0);
        (vehicle.pos, vehicle.rotation) = path.get();
    }
}

// so tthe way the cars is working is that the cars go and cargo run go run cars but not with go its rust
impl CarPath {
    pub fn get(&self) -> (vec2<f32>, Angle) {
        let index = match self
            .nodes
            .binary_search_by_key(&r32(self.current_pos), |(dist, _)| r32(*dist))
        {
            Ok(index) => index,
            Err(index) => index - 1,
        };
        let from = &self.nodes[index];
        let to = self.nodes.get(index + 1).unwrap();
        let pos = from.1 + (to.1 - from.1) * (self.current_pos - from.0) / (to.0 - from.0);
        let angle = (to.1 - from.1).arg();
        (pos, angle)
    }
}

// TODO: add missing unit tests
fn bike_movement(
    receiver: Receiver<Update>,
    roads: Single<&RoadGraph>,
    bikes: Fetcher<(
        &VehicleController,
        &VehicleProperties,
        &mut Vehicle,
        Has<&BikeJump>,
    )>,
) {
    let delta_time = receiver.event.delta_time.as_secs_f64() as f32;
    for (controller, props, bike, jumping) in bikes {
        if !jumping.get() {
            if controller.accelerate == 0.0 {
                bike.speed -= bike.speed.signum()
                    * (props.auto_deceleration * delta_time).clamp_abs(bike.speed.abs());
            } else {
                let offroad = !roads
                    .connections
                    .iter()
                    .map(|edge| {
                        let a = roads.roads[edge[0]].position;
                        let b = roads.roads[edge[1]].position;
                        let p = bike.pos;
                        if vec2::dot(a - b, p - b) < 0.0 {
                            return (b - p).len();
                        }
                        if vec2::dot(b - a, p - a) < 0.0 {
                            return (a - p).len();
                        }
                        vec2::skew((a - b).normalize_or_zero(), p - a).abs()
                    })
                    .any(|distance| distance < 3.0);
                let max_speed = if offroad {
                    props.max_offroad_speed
                } else {
                    props.max_speed
                };
                let target_speed = if controller.accelerate > 0.0 {
                    max_speed
                } else {
                    -props.max_backward_speed
                };
                let acceleration = if bike.speed > max_speed {
                    props.brake_deceleration
                } else if controller.accelerate != 0.0 {
                    props.acceleration
                } else {
                    props.auto_deceleration
                };
                bike.speed += (target_speed - bike.speed).clamp_abs(acceleration * delta_time);
            }
            if controller.brakes {
                bike.speed = bike.speed
                    - (bike.speed.signum() * props.brake_deceleration * delta_time)
                        .clamp_abs(bike.speed);
            }
        }
        bike.rotation_speed = (bike.rotation_speed
            + (props.max_rotation_speed * controller.rotate - bike.rotation_speed)
                .clamp_abs(props.rotation_accel * delta_time))
        .clamp_abs(props.max_rotation_speed);
        bike.rotation = (bike.rotation + bike.rotation_speed * delta_time).normalized_pi();
        bike.pos += vec2(1.0, 0.0).rotate(bike.rotation) * bike.speed * delta_time;
    }
}

fn bike_collisions(
    _receiver: Receiver<Update>,
    config: Single<&Config>,
    bikes: Fetcher<(&mut Vehicle, Not<With<&Car>>)>,
    buildings: Fetcher<&Building>,
    shop: Single<&Shop>,
    cars: Fetcher<(&Vehicle, With<&Car>)>,
    trees: Fetcher<&Tree>,
    mut sender: Sender<BonkEvent>,
) {
    for (bike, _) in bikes {
        let bike_shape = parry2d::shape::Ball::new(0.8);
        let bike_iso =
            parry2d::math::Isometry::new(parry2d::na::Vector2::new(bike.pos.x, bike.pos.y), 0.0);

        let mut contacts = Vec::new();

        for (pos, half_size, rotation) in itertools::chain![
            buildings
                .iter()
                .map(|building| (building.pos, building.half_size, building.rotation)),
            // cars.iter()
            //     .map(|(vehicle, _)| (vehicle.pos, config.car_half_size, vehicle.rotation)),
        ] {
            let aabb = Aabb2::ZERO.extend_symmetric(half_size);
            let points = aabb.corners().map(|p| {
                let vec2(x, y) = p;
                parry2d::math::Point::new(x, y)
            });

            let building_shape: Box<dyn parry2d::shape::Shape> =
                match parry2d::shape::ConvexPolygon::from_convex_hull(&points) {
                    Some(poly) => Box::new(poly),
                    None => Box::new(parry2d::shape::Ball::new(0.0)),
                };
            let building_iso = parry2d::math::Isometry::new(
                parry2d::na::Vector2::new(pos.x, pos.y),
                rotation.as_radians(),
            );

            let prediction = 0.0;
            if let Some(contact) = parry2d::query::contact(
                &bike_iso,
                &bike_shape,
                &building_iso,
                &*building_shape,
                prediction,
            )
            .unwrap()
            {
                contacts.push(contact);
            }
        }

        for (pos, radius) in itertools::chain![
            trees.iter().map(|tree| (tree.pos, 0.8)),
            cars.iter()
                .map(|(vehicle, _)| (vehicle.pos, config.car_radius)),
        ] {
            let tree_shape = parry2d::shape::Ball::new(radius);
            let tree_iso =
                parry2d::math::Isometry::new(parry2d::na::Vector2::new(pos.x, pos.y), 0.0);

            let prediction = 0.0;
            if let Some(contact) =
                parry2d::query::contact(&bike_iso, &bike_shape, &tree_iso, &tree_shape, prediction)
                    .unwrap()
            {
                contacts.push(contact);
            }
        }

        {
            let shop = *shop;
            let half_size = vec2(3.0, 6.0);

            let width = 0.2;
            let aabb = Aabb2::ZERO.extend_symmetric(half_size);
            let edges = vec![
                Aabb2::point(aabb.bottom_left())
                    .extend_up(width)
                    .extend_right(aabb.width()),
                Aabb2::point(aabb.bottom_right())
                    .extend_up(aabb.height())
                    .extend_left(width),
                Aabb2::point(aabb.top_left())
                    .extend_down(width)
                    .extend_right(aabb.width()),
            ];

            for edge in edges {
                let points = edge.corners().map(|p| {
                    let vec2(x, y) = p;
                    parry2d::math::Point::new(x, y)
                });

                let shop_shape: Box<dyn parry2d::shape::Shape> =
                    match parry2d::shape::ConvexPolygon::from_convex_hull(&points) {
                        Some(poly) => Box::new(poly),
                        None => Box::new(parry2d::shape::Ball::new(0.0)),
                    };
                let shop_iso = parry2d::math::Isometry::new(
                    parry2d::na::Vector2::new(shop.pos.x, shop.pos.y),
                    shop.rotation.as_radians(),
                );

                let prediction = 0.0;
                if let Some(contact) = parry2d::query::contact(
                    &bike_iso,
                    &bike_shape,
                    &shop_iso,
                    &*shop_shape,
                    prediction,
                )
                .unwrap()
                {
                    contacts.push(contact);
                }
            }
        }

        for contact in contacts {
            let normal = contact.normal1.into_inner();
            let normal = vec2(normal.x, normal.y);
            let penetration = -contact.dist;

            bike.pos -= normal * penetration;
            let mut vel = vec2(bike.speed, 0.0).rotate(bike.rotation);
            let vel_into_wall = vec2::dot(normal, vel) - config.wall_speed_hack;
            if vel_into_wall > 0.0 {
                vel -= normal * vel_into_wall;
                bike.speed = vel.len();
                sender.send(BonkEvent {
                    velocity: vel_into_wall,
                });
            }
        }
    }
}

pub fn create_particle_emitters(
    receiver: Receiver<Insert<Vehicle>, ()>,
    mut sender: Sender<(Spawn, Insert<particle::Emitter>)>,
) {
    sender.insert(
        receiver.event.entity,
        particle::Emitter::new(
            receiver.event.component.pos.extend(0.0),
            time::Duration::from_secs_f64(0.2),
            0,
            0.3,
            2.0,
            std::time::Duration::from_secs_f64(1.0),
            vec3(0.0, 0.0, 1.0),
            vec3(1.0, 1.0, 0.0),
        ),
    )
}

pub fn update_particle_emitter_position(
    _receiver: Receiver<Update>,
    mut emitters: Fetcher<(&mut particle::Emitter, &Vehicle)>,
) {
    for (emitter, vehicle) in emitters.iter_mut() {
        emitter.pos = vehicle.pos.extend(0.0);
        emitter.emitting = vehicle.speed > 5.0;
    }
}
