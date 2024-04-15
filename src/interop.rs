use self::model::{Leaderboard, Vehicle, VehicleProperties};

use super::*;

pub type Id = i64;

#[derive(evenio::event::Event, Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    Disconnect(Id),
    Ping,
    YourName(String),
    Name(Id, String),
    UpdateBike(Id, Vehicle),
    Rng(u64),
    RingBell(Id),
    NewQuest(usize),
    RemoveQuest(usize),
    SetDelivery(Option<usize>),
    UpdateVehicleProperties(Id, VehicleProperties),
    Emote(Id, EmoteType),
    Time(f32),
    SetMoney(i64),
    Leaderboard(Leaderboard),
    Invitation(Id),
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EmoteType {
    Jump,
    Wheelie(bool),
}

#[derive(Debug, evenio::event::Event, Serialize, Deserialize)]
pub enum ClientMessage {
    Pong,
    SetName(String),
    UpdateBike(Vehicle),
    RingBell,
    UpdateVehicleProperties(VehicleProperties),
    Emote(EmoteType),
    Invite(Id),
}

pub type ClientConnection = geng::net::client::Connection<ServerMessage, ClientMessage>;
