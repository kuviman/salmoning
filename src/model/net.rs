use crate::{
    interop::{ClientMessage, EmoteType, Id, ServerMessage},
    render::{BikeJump, Wheelie},
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
    world.insert(global, Leaderboard { rows: vec![] });
    world.add_handler(update_bikes);
    world.add_handler(interpolation);
    world.add_handler(quests);
    world.add_handler(
        |receiver: Receiver<Insert<VehicleProperties>, (&Vehicle, With<&LocalPlayer>)>,
         mut sender: Sender<ClientMessage>| {
            sender.send(ClientMessage::UpdateBike(receiver.query.0.clone()));
            sender.send(ClientMessage::UpdateVehicleProperties(
                receiver.event.component.clone(),
            ));
        },
    );
    world.add_handler(emotes);
    world.add_handler(cars);
    world.add_handler(money);
    world.add_handler(leaders);
}

fn leaders(
    receiver: Receiver<ServerMessage>,
    global: TrySingle<(EntityId, With<&Global>)>,
    mut sender: Sender<Insert<Leaderboard>>,
) {
    if let ServerMessage::Leaderboard(data) = receiver.event {
        if let Ok((singleton, _)) = global.0 {
            sender.insert(singleton, data.clone());
        }
    }
}

fn money(
    receiver: Receiver<ServerMessage>,
    player: TrySingle<(EntityId, With<&LocalPlayer>)>,
    mut sender: Sender<Insert<Money>>,
) {
    if let ServerMessage::SetMoney(money) = receiver.event {
        if let Ok((player, _)) = player.0 {
            sender.insert(player, Money(*money));
        }
    }
}

fn cars(receiver: Receiver<ServerMessage>, config: Single<&Config>, cars: Fetcher<&mut CarPath>) {
    if let ServerMessage::Time(time) = *receiver.event {
        for car in cars {
            car.current_pos = time * config.car_speed;
        }
    }
}

fn emotes(
    receiver: Receiver<ServerMessage>,
    global: Single<&Global>,
    mut sender: Sender<(Insert<BikeJump>, Insert<Wheelie>)>,
) {
    if let ServerMessage::Emote(id, emote) = receiver.event {
        if let Some(&entity) = global.net_to_entity.get(id) {
            match emote {
                EmoteType::Jump => {
                    sender.insert(entity, BikeJump::default());
                }
                EmoteType::Wheelie(front) => {
                    sender.insert(entity, Wheelie::new(*front));
                }
            }
        }
    }
}

fn quests(
    receiver: Receiver<ServerMessage>,
    mut quests: Single<&mut Quests>,
    mut sender: Sender<QuestEvent>,
) {
    match *receiver.event {
        ServerMessage::NewQuest(index) => {
            quests.active.insert(index);
        }
        ServerMessage::RemoveQuest(index) => {
            quests.active.remove(&index);
        }
        ServerMessage::SetDelivery(index) => {
            quests.deliver = index;
            sender.send(if index.is_some() {
                QuestEvent::Start
            } else {
                QuestEvent::Complete
            });
        }
        _ => {}
    }
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
    player: TrySingle<(&Vehicle, With<&LocalPlayer>)>,
    mut sender: Sender<(
        ClientMessage,
        Spawn,
        Despawn,
        RingBell,
        Insert<Vehicle>,
        Insert<NetId>,
        Insert<Interpolation>,
        Insert<VehicleProperties>,
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
        ServerMessage::UpdateVehicleProperties(id, props) => {
            let entity = global.net_to_entity[&id];
            sender.insert(entity, props.clone());
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
