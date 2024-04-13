use super::*;

pub fn init(world: &mut World) {
    world.add_handler(bike_movement);
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
