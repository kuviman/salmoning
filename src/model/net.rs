use crate::interop::{ClientMessage, Id, ServerMessage};

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
}

#[derive(Component)]
struct Global {
    net_to_entity: HashMap<Id, EntityId>,
}

#[derive(Component)]
struct NetId(Id);

#[allow(clippy::type_complexity)]
fn update_bikes(
    receiver: Receiver<ServerMessage>,
    mut global: Single<&mut Global>,
    player: TrySingle<(&Bike, With<&Player>)>,
    mut sender: Sender<(ClientMessage, Spawn, Despawn, Insert<Bike>, Insert<NetId>)>,
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
                sender.insert(entity, NetId(*id));
                entity
            };
            sender.insert(entity, bike.clone());
        }
        _ => {}
    }
}
