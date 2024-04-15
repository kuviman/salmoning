use crate::{
    interop::*,
    model::{Leaderboard, Level, Vehicle, VehicleProperties},
};
use geng::prelude::batbox::prelude::*;

struct Client {
    bike: usize,
    hat: Option<usize>,
    quest_cost: i64,
    money: i64,
    name: String,
    vehicle: Vehicle,
    quest_lock_timer: Timer,
    timer_time: f64,
    can_do_quests: bool,
    delivery: Option<usize>,
    leader: Option<Id>,
    sender: Box<dyn geng::net::Sender<ServerMessage>>,
    vehicle_properties: Option<VehicleProperties>,
}

#[derive(Deserialize)]
struct Config {
    team_timer: f64,
    leaderboard_places: usize,
    seed: u64,
    quest_lock_timer: f64,
    quests_count: usize,
    quest_max_speed: f32,
    quest_activation_radius: f32,
    quest_money_per_distance: f32,
}

struct State {
    timer: Timer,
    next_id: Id,
    config: Config,
    level: Level,
    active_quests: HashSet<usize>,
    clients: HashMap<Id, Client>,
}

impl State {
    fn update_leaderboard(&self) -> Leaderboard {
        let mut rows = HashMap::<Id, (usize, i64)>::new();
        for (&id, client) in &self.clients {
            let leader = client.leader.unwrap_or(id);
            let row = rows.entry(leader).or_default();
            row.0 += 1;
            row.1 += client.money;
        }
        let mut rows: Vec<_> = rows
            .into_iter()
            .map(|(id, row)| {
                let leader_name = &self.clients[&id].name;
                let people = row.0;
                let money = row.1;
                let name = if people <= 1 {
                    leader_name.to_owned()
                } else {
                    format!("{leader_name} +{}", people - 1)
                };
                (name, money)
            })
            .collect();
        rows.sort_by_key(|(_, money)| -money);
        rows.truncate(self.config.leaderboard_places);
        Leaderboard { rows }
    }
    const TICKS_PER_SECOND: f32 = 10.0;
    fn new() -> Self {
        let config: Config =
            futures::executor::block_on(file::load_detect(run_dir().join("server.toml"))).unwrap();
        Self {
            timer: Timer::new(),
            active_quests: HashSet::new(),
            next_id: 0,
            config,
            level: futures::executor::block_on(Level::load(
                run_dir().join("assets").join("level.json"),
            ))
            .unwrap(),
            clients: HashMap::new(),
        }
    }
    fn tick(&mut self) {
        while self.active_quests.len() < self.config.quests_count
            && self.active_quests.len() < self.level.waypoints.len()
        {
            let new = thread_rng().gen_range(0..self.level.waypoints.len());
            if self.active_quests.insert(new) {
                for (_client_id, client) in &mut self.clients {
                    client.sender.send(ServerMessage::NewQuest(new));
                }
            }
        }
    }
}

pub struct App {
    state: Arc<Mutex<State>>,
    #[allow(dead_code)]
    background_thread: std::thread::JoinHandle<()>,
}

impl App {
    pub fn new() -> Self {
        let state = Arc::new(Mutex::new(State::new()));
        Self {
            state: state.clone(),
            background_thread: std::thread::spawn(move || loop {
                state.lock().unwrap().tick();
                std::thread::sleep(std::time::Duration::from_secs_f32(
                    1.0 / State::TICKS_PER_SECOND,
                ));
            }),
        }
    }
}

pub struct ClientConnection {
    id: Id,
    state: Arc<Mutex<State>>,
}

impl Drop for ClientConnection {
    fn drop(&mut self) {
        let mut state = self.state.lock().unwrap();
        state.clients.remove(&self.id);
        let mut followers = Vec::new();
        for (id, other) in &mut state.clients {
            if other.leader == Some(self.id) {
                other.leader = None;
                followers.push(*id);
            }
            other.sender.send(ServerMessage::Disconnect(self.id));
        }
        for follower in followers {
            for client in state.clients.values_mut() {
                client
                    .sender
                    .send(ServerMessage::SetTeam(follower, follower));
            }
        }
    }
}

impl geng::net::Receiver<ClientMessage> for ClientConnection {
    fn handle(&mut self, message: ClientMessage) {
        let mut state = self.state.lock().unwrap();
        let state: &mut State = state.deref_mut();
        match message {
            ClientMessage::SetBikeType(typ) => {
                state.clients.get_mut(&self.id).unwrap().bike = typ;
                for (&client_id, client) in &mut state.clients {
                    if self.id != client_id {
                        client.sender.send(ServerMessage::SetBikeType(self.id, typ));
                    }
                }
            }
            ClientMessage::SetHatType(typ) => {
                state.clients.get_mut(&self.id).unwrap().hat = typ;
                for (&client_id, client) in &mut state.clients {
                    if self.id != client_id {
                        client.sender.send(ServerMessage::SetHatType(self.id, typ));
                    }
                }
            }
            ClientMessage::JoinTeam(leader_id) => {
                state.clients.get_mut(&self.id).unwrap().leader = Some(leader_id);
                for (&client_id, client) in &mut state.clients {
                    client
                        .sender
                        .send(ServerMessage::SetTeam(self.id, leader_id));
                }
            }
            ClientMessage::Invite(id) => {
                if let Some(client) = state.clients.get_mut(&id) {
                    client.sender.send(ServerMessage::Invitation(self.id));
                }
            }
            ClientMessage::Emote(typ) => {
                for (&client_id, client) in &mut state.clients {
                    if self.id != client_id {
                        client.sender.send(ServerMessage::Emote(self.id, typ));
                    }
                }
            }
            ClientMessage::UpdateVehicleProperties(data) => {
                state.clients.get_mut(&self.id).unwrap().vehicle_properties = Some(data.clone());
                for (&client_id, client) in &mut state.clients {
                    if self.id != client_id {
                        client.sender.send(ServerMessage::UpdateVehicleProperties(
                            self.id,
                            data.clone(),
                        ));
                    }
                }
            }
            ClientMessage::UpdateBike(data) => {
                for (&client_id, client) in &mut state.clients {
                    if self.id != client_id {
                        client
                            .sender
                            .send(ServerMessage::UpdateBike(self.id, data.clone()));
                    }
                }

                let leader = state.clients[&self.id].leader.unwrap_or(self.id);

                if let Some(delivery) = state.clients[&self.id].delivery {
                    if (state.level.waypoints[delivery].pos - data.pos).len()
                        < state.config.quest_activation_radius
                    {
                        state.clients.get_mut(&self.id).unwrap().delivery = None;
                        state.clients.get_mut(&leader).unwrap().quest_lock_timer = Timer::new();
                        if dbg!(state.clients.iter().any(|(id, client)| {
                            client.leader.unwrap_or(*id) == leader && client.delivery.is_some()
                        })) {
                            state.clients.get_mut(&leader).unwrap().timer_time =
                                state.config.team_timer;
                        } else {
                            state.clients.get_mut(&leader).unwrap().timer_time =
                                state.config.quest_lock_timer;
                        }
                        let client = state.clients.get_mut(&self.id).unwrap();
                        client.money += client.quest_cost;
                        client.sender.send(ServerMessage::SetMoney(client.money));
                        client.sender.send(ServerMessage::SetDelivery(None));

                        let leaderboard = state.update_leaderboard();
                        for client in state.clients.values_mut() {
                            client
                                .sender
                                .send(ServerMessage::Leaderboard(leaderboard.clone()));
                        }
                    }
                }

                if data.speed < state.config.quest_max_speed
                    && state.clients[&leader]
                        .quest_lock_timer
                        .elapsed()
                        .as_secs_f64()
                        > state.clients[&leader].timer_time
                {
                    let leader_client = state.clients.get_mut(&leader).unwrap();
                    #[allow(clippy::collapsible_if)]
                    if !leader_client.can_do_quests {
                        if leader_client.timer_time == state.config.team_timer
                            || !state.clients.iter().any(|(id, client)| {
                                client.leader.unwrap_or(*id) == leader && client.delivery.is_some()
                            })
                        {
                            state.clients.get_mut(&leader).unwrap().can_do_quests = true;
                            for (client_id, client) in &mut state.clients {
                                client.sender.send(ServerMessage::CanDoQuests(leader, true));
                                if client.leader.unwrap_or(*client_id) == leader {
                                    client.delivery = None;
                                    client.sender.send(ServerMessage::SetDelivery(None));
                                }
                            }
                        }
                    }
                    if state.clients.iter().any(|(id, client)| {
                        client.leader.unwrap_or(*id) == leader && client.delivery.is_some()
                    }) {
                        // waiting for the team
                    } else if let Some(&quest) = state.active_quests.iter().find(|&&quest| {
                        let point = &state.level.waypoints[quest];
                        (point.pos - data.pos).len() < state.config.quest_activation_radius
                    }) {
                        let leader_client = state.clients.get_mut(&leader).unwrap();
                        leader_client.can_do_quests = false;
                        leader_client.timer_time = state.config.quest_lock_timer;
                        for client in state.clients.values_mut() {
                            client
                                .sender
                                .send(ServerMessage::CanDoQuests(leader, false));
                        }
                        state.active_quests.remove(&quest);
                        for (&_client_id, client) in &mut state.clients {
                            client.sender.send(ServerMessage::RemoveQuest(quest));
                        }
                        let deliver_to = loop {
                            let to = thread_rng().gen_range(0..state.level.waypoints.len());
                            if to != quest {
                                break to;
                            }
                        };
                        let client = state.clients.get_mut(&self.id).unwrap();
                        client.quest_cost = ((state.level.waypoints[quest].pos
                            - state.level.waypoints[deliver_to].pos)
                            .len()
                            * state.config.quest_money_per_distance)
                            .ceil() as i64;
                        client.delivery = Some(deliver_to);

                        for (&other_id, other) in &mut state.clients {
                            if other.leader.unwrap_or(other_id) == leader {
                                other.delivery = Some(deliver_to);
                                other
                                    .sender
                                    .send(ServerMessage::SetDelivery(Some(deliver_to)));
                            }
                        }
                    }
                }
            }
            ClientMessage::Pong => {
                let client = state
                    .clients
                    .get_mut(&self.id)
                    .expect("Sender not found for client");
                client.sender.send(ServerMessage::Time(
                    state.timer.elapsed().as_secs_f64() as f32
                ));
                client.sender.send(ServerMessage::Ping);
            }
            ClientMessage::SetName(name) => {
                let name = name.chars().filter(|c| c.is_ascii_alphabetic()).take(15);
                let name: String = rustrict::CensorIter::censor(name).collect();

                state.clients.get_mut(&self.id).unwrap().name = name.clone();
                for (&client_id, client) in &mut state.clients {
                    if self.id == client_id {
                        client.sender.send(ServerMessage::YourName(name.clone()));
                    } else {
                        client
                            .sender
                            .send(ServerMessage::Name(self.id, name.clone()));
                    }
                }
            }
            ClientMessage::RingBell => {
                for (&client_id, client) in &mut state.clients {
                    if self.id != client_id {
                        client.sender.send(ServerMessage::RingBell(self.id));
                    }
                }
            }
        }
    }
}

impl geng::net::server::App for App {
    type Client = ClientConnection;
    type ServerMessage = ServerMessage;
    type ClientMessage = ClientMessage;
    fn connect(
        &mut self,
        mut sender: Box<dyn geng::net::Sender<Self::ServerMessage>>,
    ) -> ClientConnection {
        let mut state = self.state.lock().unwrap();
        sender.send(ServerMessage::Rng(state.config.seed));
        sender.send(ServerMessage::Ping);
        sender.send(ServerMessage::SetMoney(0));
        for &quest in &state.active_quests {
            sender.send(ServerMessage::NewQuest(quest));
        }

        for (&other_id, other_client) in &mut state.clients {
            sender.send(ServerMessage::UpdateBike(
                other_id,
                other_client.vehicle.clone(),
            ));
            sender.send(ServerMessage::Name(other_id, other_client.name.clone()));
            sender.send(ServerMessage::SetBikeType(other_id, other_client.bike));
            sender.send(ServerMessage::SetHatType(other_id, other_client.hat));
            if let Some(props) = &other_client.vehicle_properties {
                sender.send(ServerMessage::UpdateVehicleProperties(
                    other_id,
                    props.clone(),
                ));
            }
        }

        let my_id = state.next_id;
        sender.send(ServerMessage::YourId(my_id));
        for (&other_id, other) in &state.clients {
            sender.send(ServerMessage::CanDoQuests(other_id, other.can_do_quests));
        }
        let mut client = Client {
            bike: 0,
            hat: None,
            can_do_quests: false,
            timer_time: state.config.quest_lock_timer,
            quest_cost: 0,
            money: 0,
            leader: None,
            vehicle: Vehicle::default(),
            quest_lock_timer: Timer::new(),
            delivery: None,
            name: "<salmoner>".to_owned(),
            sender,
            vehicle_properties: None,
        };

        client
            .sender
            .send(ServerMessage::YourName(client.name.clone()));

        for (&other_id, other_client) in &mut state.clients {
            other_client
                .sender
                .send(ServerMessage::UpdateBike(my_id, client.vehicle.clone()));
            other_client
                .sender
                .send(ServerMessage::Name(my_id, client.name.clone()));
        }
        state.clients.insert(my_id, client);
        state.next_id += 1;
        ClientConnection {
            id: my_id,
            state: self.state.clone(),
        }
    }
}

#[test]
fn test_brainoid() {
    assert_eq!(
        "brainoid",
        rustrict::CensorIter::censor("brainoid".chars()).collect::<String>()
    );
}
