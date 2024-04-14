use crate::{
    interop::*,
    model::{Leaderboard, Level, Vehicle, VehicleProperties},
};
use geng::prelude::batbox::prelude::*;

struct Client {
    quest_cost: i64,
    money: i64,
    name: String,
    vehicle: Vehicle,
    quest_lock_timer: Timer,
    delivery: Option<usize>,
    sender: Box<dyn geng::net::Sender<ServerMessage>>,
    vehicle_properties: Option<VehicleProperties>,
}

#[derive(Deserialize)]
struct Config {
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
        let mut rows: Vec<_> = self
            .clients
            .values()
            .map(|client| (client.name.clone(), client.money))
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
        for other in state.clients.values_mut() {
            other.sender.send(ServerMessage::Disconnect(self.id));
        }
    }
}

impl geng::net::Receiver<ClientMessage> for ClientConnection {
    fn handle(&mut self, message: ClientMessage) {
        let mut state = self.state.lock().unwrap();
        let state: &mut State = state.deref_mut();
        match message {
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

                if data.speed < state.config.quest_max_speed
                    && state.clients[&self.id]
                        .quest_lock_timer
                        .elapsed()
                        .as_secs_f64()
                        > state.config.quest_lock_timer
                {
                    if let Some(delivery) = state.clients[&self.id].delivery {
                        if (state.level.waypoints[delivery].pos - data.pos).len()
                            < state.config.quest_activation_radius
                        {
                            let client = state.clients.get_mut(&self.id).unwrap();
                            client.delivery = None;
                            client.money += client.quest_cost;
                            client.sender.send(ServerMessage::SetMoney(client.money));
                            client.quest_lock_timer = Timer::new();
                            client.sender.send(ServerMessage::SetDelivery(None));

                            let leaderboard = state.update_leaderboard();
                            for client in state.clients.values_mut() {
                                client
                                    .sender
                                    .send(ServerMessage::Leaderboard(leaderboard.clone()));
                            }
                        }
                    } else if let Some(&quest) = state.active_quests.iter().find(|&&quest| {
                        let point = &state.level.waypoints[quest];
                        (point.pos - data.pos).len() < state.config.quest_activation_radius
                    }) {
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
                        client
                            .sender
                            .send(ServerMessage::SetDelivery(Some(deliver_to)));
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
        for (&id, client) in &state.clients {
            sender.send(ServerMessage::Name(id, client.name.clone()));
            sender.send(ServerMessage::UpdateBike(id, client.vehicle.clone()));
            if let Some(props) = &client.vehicle_properties {
                sender.send(ServerMessage::UpdateVehicleProperties(id, props.clone()));
            }
        }
        let id = state.next_id;
        state.clients.insert(
            id,
            Client {
                quest_cost: 0,
                money: 0,
                vehicle: Vehicle::default(),
                quest_lock_timer: Timer::new(),
                delivery: None,
                name: "<salmoner>".to_owned(),
                sender,
                vehicle_properties: None,
            },
        );
        state.next_id += 1;
        ClientConnection {
            id,
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
