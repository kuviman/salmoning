use crate::interop::*;
use geng::prelude::batbox::prelude::*;

struct Client {
    name: String,
    sender: Box<dyn geng::net::Sender<ServerMessage>>,
}

#[derive(Deserialize)]
struct Config {
    seed: u64,
}

struct State {
    next_id: Id,
    config: Config,
    clients: HashMap<Id, Client>,
}

impl State {
    const TICKS_PER_SECOND: f32 = 10.0;
    fn new() -> Self {
        let config: Config =
            futures::executor::block_on(file::load_detect(run_dir().join("server.toml"))).unwrap();
        Self {
            next_id: 0,
            config,
            clients: HashMap::new(),
        }
    }
    fn tick(&mut self) {}
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
            ClientMessage::UpdateBike(data) => {
                for (&client_id, client) in &mut state.clients {
                    if self.id != client_id {
                        client
                            .sender
                            .send(ServerMessage::UpdateBike(self.id, data.clone()));
                    }
                }
            }
            ClientMessage::Pong => {
                state
                    .clients
                    .get_mut(&self.id)
                    .expect("Sender not found for client")
                    .sender
                    .send(ServerMessage::Ping);
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
        for (&id, client) in &state.clients {
            sender.send(ServerMessage::Name(id, client.name.clone()));
        }
        let id = state.next_id;
        state.clients.insert(
            id,
            Client {
                name: String::new(),
                sender,
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
