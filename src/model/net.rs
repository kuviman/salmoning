use crate::{
    interop::{ClientMessage, Id, ServerMessage},
    sound::RingBell,
};

use super::*;

pub fn init(world: &mut World) {
    let global = world.spawn();
    world.insert(
        global,
        Global {
            net_to_entity: default(),
        },
    );
    world.add_handler(update_bikes);
    world.add_handler(interpolation);
}

#[derive(Component)]
struct Global {
    net_to_entity: HashMap<Id, EntityId>,
}

#[derive(Component)]
struct Interpolation(Vehicle);

#[derive(Component)]
struct NetId(Id);

fn interpolation(receiver: Receiver<Update>, bikes: Fetcher<(&mut Vehicle, &mut Interpolation)>) {
    let delta_time = receiver.event.delta_time.as_secs_f64() as f32;
    const SPEED: f32 = 10.0;
    let k = (SPEED * delta_time).min(1.0);
    for (bike, interpolation) in bikes {
        let target = &mut interpolation.0;
        target.pos += vec2(target.speed, 0.0).rotate(target.rotation) * delta_time;
        bike.pos += (target.pos - bike.pos) * k;
        bike.rotation += (target.rotation - bike.rotation).normalized_pi() * k;
    }
}

#[allow(clippy::type_complexity)]
fn update_bikes(
    receiver: Receiver<ServerMessage>,
    mut global: Single<&mut Global>,
    player: TrySingle<(&Vehicle, With<&Player>)>,
    mut sender: Sender<(
        ClientMessage,
        Spawn,
        Despawn,
        RingBell,
        Insert<Vehicle>,
        Insert<NetId>,
        Insert<Interpolation>,
        Insert<Bike>,
    )>,
) {
    match receiver.event {
        ServerMessage::Disconnect(id) => {
            if let Some(&entity) = global.net_to_entity.get(id) {
                sender.despawn(entity);
            }
        }
        ServerMessage::Ping => {
            sender.send(ClientMessage::Pong);
            if let Ok((player, _)) = player.0 {
                sender.send(ClientMessage::UpdateBike(player.clone()));
            }
        }
        ServerMessage::UpdateBike(id, bike) => {
            let entity = if let Some(&entity) = global.net_to_entity.get(id) {
                entity
            } else {
                let entity = sender.spawn();
                global.net_to_entity.insert(*id, entity);
                sender.insert(entity, Bike);
                sender.insert(entity, NetId(*id));
                sender.insert(entity, bike.clone());
                entity
            };
            sender.insert(entity, Interpolation(bike.clone()));
        }
        ServerMessage::RingBell(id) => {
            if let Some(&entity) = global.net_to_entity.get(id) {
                sender.send(RingBell { entity });
            };
        }
        _ => {}
    }
}
