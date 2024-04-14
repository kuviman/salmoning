use super::*;

pub fn init(world: &mut World) {
    world.add_handler(bike_movement);
    world.add_handler(bike_jump);
    world.add_handler(bike_collisions);
}

fn bike_movement(
    receiver: Receiver<Update>,
    bikes: Fetcher<(&VehicleController, &VehicleProperties, &mut Vehicle)>,
) {
    let delta_time = receiver.event.delta_time.as_secs_f64() as f32;
    for (controller, props, bike) in bikes {
        bike.speed = (bike.speed + controller.accelerate * props.acceleration * delta_time)
            .clamp(-props.max_backward_speed, props.max_speed);
        if controller.brakes {
            bike.speed = bike.speed
                - (bike.speed.signum() * props.brake_deceleration * delta_time)
                    .clamp_abs(bike.speed);
        }
        bike.rotation_speed = (bike.rotation_speed
            + (props.max_rotation_speed * controller.rotate - bike.rotation_speed)
                .clamp_abs(props.rotation_accel * delta_time))
        .clamp_abs(props.max_rotation_speed);
        bike.rotation = (bike.rotation + bike.rotation_speed * delta_time).normalized_pi();
        bike.pos += vec2(1.0, 0.0).rotate(bike.rotation) * bike.speed * delta_time;
    }
}

fn bike_jump(receiver: Receiver<Update>, bikes: Fetcher<&mut Vehicle>) {
    let delta_time = receiver.event.delta_time.as_secs_f64() as f32;
    for bike in bikes {
        if let Some(jump) = &mut bike.jump {
            *jump += delta_time * 3.0;
            if *jump > 1.0 {
                bike.jump = None;
            }
        }
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
