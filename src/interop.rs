use self::model::Vehicle;

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
}

#[derive(Debug, evenio::event::Event, Serialize, Deserialize)]
pub enum ClientMessage {
    Pong,
    SetName(String),
    UpdateBike(Vehicle),
    RingBell,
}

pub type ClientConnection = geng::net::client::Connection<ServerMessage, ClientMessage>;
