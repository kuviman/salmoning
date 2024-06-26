use crate::{
    controls::{JoinTeam, SendInvite, TeamLeader},
    interop::{ClientMessage, EmoteType, Id, ServerMessage},
    render::{BikeJump, RaceStatistic, SetBikeType, SetHatType, Wheelie},
    sound::RingBell,
    ui::{OutboundUiMessage, Race},
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
    world.add_handler(leaderboard);
    world.add_handler(invite);
    world.add_handler(invitations);
    world.add_handler(join_team);
    world.add_handler(names);
    world.add_handler(team_leaders);
    world.add_handler(can_do_quests);
    world.add_handler(bike_type);
    world.add_handler(hat_type);
    world.add_handler(token);
    world.add_handler(race_statistics);
    world.add_handler(race_advertisements);
}

#[derive(Component)]
pub struct CanDoQuests;

fn token(receiver: Receiver<ServerMessage>) {
    if let ServerMessage::YourToken(token) = receiver.event {
        preferences::save("token", token);
    }
}

#[derive(Component)]
pub struct AvailableRace {
    pub race: Race,
}

fn race_advertisements(
    receiver: Receiver<ServerMessage>,
    global: Single<&Global>,
    mut sender: Sender<(Insert<AvailableRace>, Remove<AvailableRace>)>,
) {
    if let ServerMessage::AvailableRace(id, race) = receiver.event {
        let owner = global.net_to_entity[id];
        sender.insert(owner, AvailableRace { race: race.clone() });
    }
    if let ServerMessage::UnavailableRace(id) = receiver.event {
        let owner = global.net_to_entity[id];
        sender.remove::<AvailableRace>(owner);
    }
}

fn race_statistics(
    receiver: Receiver<ServerMessage>,
    global: Single<&Global>,
    names: Fetcher<&Name>,
    player: Single<(EntityId, With<&LocalPlayer>)>,
    mut sender: Sender<(RaceStatistic, OutboundUiMessage)>,
) {
    if let ServerMessage::UnsetTeam(id, silent) = receiver.event {
        if !*silent && player.0 .0 == global.net_to_entity[id] {
            sender.send(OutboundUiMessage::PhoneAlert {
                msg: "You are no longer in a race crew.".to_string(),
            });
        }
    }
    if let ServerMessage::RaceStatistic(id, duration, place, total) = receiver.event {
        let who = names
            .get(global.net_to_entity[id])
            .map_or("<salmoner>".to_string(), |x| x.0.clone());
        sender.send(RaceStatistic {
            who,
            duration: *duration,
            place: *place,
            total: *total,
        });
    }
}

fn bike_type(
    receiver: Receiver<ServerMessage>,
    global: Single<&Global>,
    mut sender: Sender<SetBikeType>,
) {
    if let ServerMessage::SetBikeType(id, typ) = receiver.event {
        sender.send(SetBikeType {
            bike_id: global.net_to_entity[id],
            bike_type: *typ,
        });
    }
}

fn hat_type(
    receiver: Receiver<ServerMessage>,
    global: Single<&Global>,
    mut sender: Sender<SetHatType>,
) {
    if let ServerMessage::SetHatType(id, typ) = receiver.event {
        sender.send(SetHatType {
            bike_id: global.net_to_entity[id],
            hat_type: *typ,
        });
    }
}

fn can_do_quests(
    receiver: Receiver<ServerMessage>,
    global: Single<&Global>,
    mut sender: Sender<(Insert<CanDoQuests>, Remove<CanDoQuests>)>,
) {
    if let ServerMessage::CanDoQuests(id, can) = receiver.event {
        if let Some(&entity) = global.net_to_entity.get(id) {
            if *can {
                sender.insert(entity, CanDoQuests);
            } else {
                sender.remove::<CanDoQuests>(entity);
            }
        }
    }
}

#[derive(Component)]
pub struct Name(pub String);

fn join_team(
    receiver: Receiver<JoinTeam>,
    net: Fetcher<&NetId>,
    mut sender: Sender<ClientMessage>,
) {
    let id = net.get(receiver.event.0).unwrap().0;
    sender.send(ClientMessage::JoinTeam(id));
}

fn team_leaders(
    receiver: Receiver<ServerMessage>,
    global: Single<&Global>,
    mut sender: Sender<(Insert<TeamLeader>, Remove<TeamLeader>)>,
) {
    if let ServerMessage::UnsetTeam(id, _) = receiver.event {
        let Some(&id) = global.net_to_entity.get(id) else {
            return;
        };
        sender.remove::<TeamLeader>(id);
    };
    if let ServerMessage::SetTeam(id, leader_id) = receiver.event {
        let Some(&id) = global.net_to_entity.get(id) else {
            return;
        };
        let Some(&leader_id) = global.net_to_entity.get(leader_id) else {
            return;
        };
        if id == leader_id {
            sender.insert(leader_id, TeamLeader(leader_id));
        } else {
            sender.insert(id, TeamLeader(leader_id));
            sender.insert(leader_id, TeamLeader(leader_id));
        }
    };
}

fn names(
    receiver: Receiver<ServerMessage>,
    global: Single<&Global>,
    player: Single<(EntityId, With<&LocalPlayer>)>,
    mut sender: Sender<Insert<Name>>,
) {
    match receiver.event {
        ServerMessage::Name(id, name) => {
            let entity = global.net_to_entity[&id]; // doesnt panic because after udpatebike, yea
            sender.insert(entity, Name(name.clone()));
        }
        ServerMessage::YourName(name) => {
            sender.insert(player.0 .0, Name(name.clone()));
        }
        _ => {}
    }
}

#[derive(Component)]
pub struct Invitation {
    pub entity_id: EntityId,
}

fn invite(receiver: Receiver<SendInvite>, mut sender: Sender<ClientMessage>) {
    sender.send(ClientMessage::Invite(receiver.event.0.net_id));
}

fn invitations(
    receiver: Receiver<ServerMessage>,
    global: Single<&Global>,
    player: Single<(EntityId, With<&LocalPlayer>)>,
    mut sender: Sender<Insert<Invitation>>,
) {
    if let ServerMessage::Invitation(id) = receiver.event {
        let entity_id = global.net_to_entity[&id];
        sender.insert(player.0 .0, Invitation { entity_id });
    }
}

fn leaderboard(
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

#[derive(Component, Debug)]
pub struct NetId(pub Id);

#[derive(Component, Debug)]
pub struct RacePlace {
    pub place: usize,
    pub racers: usize,
}

fn interpolation(receiver: Receiver<Update>, bikes: Fetcher<(&mut Vehicle, &mut Interpolation)>) {
    let delta_time = receiver.event.delta_time.as_secs_f64() as f32;
    const SPEED: f32 = 10.0;
    let k = (SPEED * delta_time).min(1.0);
    for (bike, interpolation) in bikes {
        let target = &mut interpolation.0;
        target.pos += vec2(target.speed, 0.0).rotate(target.rotation) * delta_time;
        bike.pos += (target.pos - bike.pos) * k;
        bike.rotation += (target.rotation - bike.rotation).normalized_pi() * k;
        bike.speed = target.speed;
    }
}

#[allow(clippy::type_complexity)]
fn update_bikes(
    receiver: Receiver<ServerMessage>,
    mut global: Single<&mut Global>,
    player: TrySingle<(EntityId, &Vehicle, With<&LocalPlayer>)>,
    fish: Fetcher<(EntityId, &Fish)>,
    race_places: Fetcher<&RacePlace>,
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
        Insert<Fish>,
        Insert<RacePlace>,
    )>,
) {
    match receiver.event {
        ServerMessage::YourId(id) => {
            if let Ok((player, ..)) = player.0 {
                global.net_to_entity.insert(*id, player);
                // sender.insert(player, NetId(id));
            }
        }
        ServerMessage::Disconnect(id) => {
            if let Some(&entity) = global.net_to_entity.get(id) {
                if let Some((fish, _)) = fish.iter().find(|(_, fish)| fish.bike == entity) {
                    sender.despawn(fish);
                }
                sender.despawn(entity);
            }
        }
        ServerMessage::Ping => {
            sender.send(ClientMessage::Pong);
            if let Ok((_, player, _)) = player.0 {
                sender.send(ClientMessage::UpdateBike(player.clone()));
            }
        }
        ServerMessage::UpdateVehicleProperties(id, props) => {
            let entity = global.net_to_entity[&id];
            sender.insert(entity, props.clone());
        }
        ServerMessage::UpdateRacePlaces(places) => {
            for (i, place) in places.iter().enumerate() {
                if let Some(&entity) = global.net_to_entity.get(place) {
                    let place = race_places.get(entity).map_or(100000, |x| x.place);
                    if place != i {
                        sender.insert(
                            entity,
                            RacePlace {
                                place: i,
                                racers: places.len(),
                            },
                        );
                    }
                }
            }
        }
        ServerMessage::UpdateBike(id, bike) => {
            let entity = if let Some(&entity) = global.net_to_entity.get(id) {
                entity
            } else {
                let entity = sender.spawn();
                global.net_to_entity.insert(*id, entity);
                sender.insert(
                    entity,
                    Bike {
                        hat_type: None,
                        bike_type: 0,
                    },
                );
                sender.insert(entity, NetId(*id));
                sender.insert(entity, bike.clone());

                let fish = sender.spawn();
                sender.insert(
                    fish,
                    Fish {
                        bike: entity,
                        local: false,
                    },
                );

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
