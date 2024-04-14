use super::*;

pub fn init(world: &mut World) {
    world.add_handler(bike_movement);
    world.add_handler(bike_collisions);
}

fn bike_movement(
    receiver: Receiver<Update>,
    roads: Single<&RoadGraph>,
    bikes: Fetcher<(&VehicleController, &VehicleProperties, &mut Vehicle)>,
) {
    let delta_time = receiver.event.delta_time.as_secs_f64() as f32;
    for (controller, props, bike) in bikes {
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
        bike.rotation_speed = (bike.rotation_speed
            + (props.max_rotation_speed * controller.rotate * bike.speed.signum()
                - bike.rotation_speed)
                .clamp_abs(props.rotation_accel * delta_time))
        .clamp_abs(props.max_rotation_speed);
        bike.rotation = (bike.rotation + bike.rotation_speed * delta_time).normalized_pi();
        bike.pos += vec2(1.0, 0.0).rotate(bike.rotation) * bike.speed * delta_time;
    }
}

fn bike_collisions(
    receiver: Receiver<Update>,
    bikes: Fetcher<&mut Vehicle>,
    buildings: Fetcher<&Building>,
    trees: Fetcher<&Tree>,
) {
    for bike in bikes {
        let bike_shape = parry2d::shape::Ball::new(0.8);
        let bike_iso =
            parry2d::math::Isometry::new(parry2d::na::Vector2::new(bike.pos.x, bike.pos.y), 0.0);

        for building in buildings.iter() {
            let aabb = Aabb2::ZERO.extend_symmetric(building.half_size);
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
                parry2d::na::Vector2::new(building.pos.x, building.pos.y),
                building.rotation.as_radians(),
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
                let normal = contact.normal1.into_inner();
                let point = contact.point1;
                let point = vec2(point.x, point.y);
                let normal = vec2(normal.x, normal.y);
                let penetration = -contact.dist;

                bike.pos -= normal * penetration;
            }
        }

        for tree in trees.iter() {
            let tree_shape = parry2d::shape::Ball::new(0.8);
            let tree_iso = parry2d::math::Isometry::new(
                parry2d::na::Vector2::new(tree.pos.x, tree.pos.y),
                tree.rotation.as_radians(),
            );

            let prediction = 0.0;
            if let Some(contact) =
                parry2d::query::contact(&bike_iso, &bike_shape, &tree_iso, &tree_shape, prediction)
                    .unwrap()
            {
                let normal = contact.normal1.into_inner();
                let point = contact.point1;
                let point = vec2(point.x, point.y);
                let normal = vec2(normal.x, normal.y);
                let penetration = -contact.dist;

                bike.pos -= normal * penetration;
            }
        }
    }
}
