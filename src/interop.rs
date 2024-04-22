use self::{
    model::{Leaderboard, Vehicle, VehicleProperties},
    ui::Race,
};

use super::*;

pub type Id = i64;

#[derive(evenio::event::Event, Debug, Serialize, Deserialize, Clone)]
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
    SetTeam(Id, Id),
    YourId(Id),
    CanDoQuests(Id, bool),
    SetBikeType(Id, usize),
    SetHatType(Id, Option<usize>),
    YourToken(String),
    YourUnlockedBikes(HashSet<usize>),
    YourUnlockedHats(HashSet<usize>),
    SetPendingRace(Race),
    StartRace(bool),
    RaceProgress(usize),
    RaceStatistic(Id, f32, usize, usize),
    RaceFinished,
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
    JoinTeam(Id),
    LeaveTeam,
    SetBikeType(usize),
    SetHatType(Option<usize>),
    Login(String),
    UnlockBike(usize),
    UnlockHat(usize),
    LoadRace(Race),
}

pub type ClientConnection = geng::net::client::Connection<ServerMessage, ClientMessage>;
