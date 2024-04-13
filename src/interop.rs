use self::model::Bike;

use super::*;

pub type Id = i64;

#[derive(evenio::event::Event, Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    Disconnect(Id),
    Ping,
    YourName(String),
    Name(Id, String),
    UpdateBike(Id, Bike),
}

#[derive(Debug, evenio::event::Event, Serialize, Deserialize)]
pub enum ClientMessage {
    Pong,
    SetName(String),
    UpdateBike(Bike),
}

pub type ClientConnection = geng::net::client::Connection<ServerMessage, ClientMessage>;
