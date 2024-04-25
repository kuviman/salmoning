use crate::{
    interop::*,
    model::{Leaderboard, Level, Vehicle, VehicleProperties},
    ui::{Race, CUSTOMIZATIONS},
};
use geng::prelude::{
    batbox::prelude::*,
    itertools::Itertools,
    rand::distributions::{Alphanumeric, DistString},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Save {
    bike: usize,
    hat: Option<usize>,
    money: i64,
    name: String,
    #[serde(default)]
    unlocked_bikes: HashSet<usize>,
    #[serde(default)]
    unlocked_hats: HashSet<usize>,
}

struct Client {
    token: String,
    save: Save,
    quest_cost: i64,
    vehicle: Vehicle,
    quest_lock_timer: Timer,
    race_timer: Option<Timer>, // only used by the leader
    ready_count: usize,        // only used by the leader
    race_winner: bool,         // for computing payouts
    finished: usize,
    participants: usize,
    timer_time: f64,
    can_do_quests: bool,
    delivery: Option<usize>,
    leader: Option<Id>,
    pending_race: Option<Race>,
    // current checkpoint (if racing)
    active_race: Option<usize>,
    race_start_timer: Option<Timer>,
    sender: Box<dyn geng::net::Sender<ServerMessage>>,
    vehicle_properties: Option<VehicleProperties>,
}

#[derive(Deserialize)]
struct Config {
    race_wager: i64,
    team_timer: f64,
    leaderboard_places: usize,
    seed: u64,
    race_finish_timer: f64,
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
            // let leader = client.leader.unwrap_or(id);
            let row = rows.entry(id).or_default();
            row.0 += 1;
            row.1 += client.save.money;
        }
        let mut rows: Vec<_> = rows
            .into_iter()
            .map(|(id, row)| {
                let leader_name = &self.clients[&id].save.name;
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
        let client = state.clients.remove(&self.id).unwrap();
        std::fs::create_dir_all("save").unwrap();
        std::fs::write(
            format!("save/{}.json", client.token),
            serde_json::to_string_pretty(&client.save).unwrap(),
        )
        .unwrap();
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
                client.sender.send(ServerMessage::UnsetTeam(follower));
            }
        }

        let leaderboard = state.update_leaderboard();
        for client in state.clients.values_mut() {
            client
                .sender
                .send(ServerMessage::Leaderboard(leaderboard.clone()));
        }
    }
}

impl geng::net::Receiver<ClientMessage> for ClientConnection {
    fn handle(&mut self, message: ClientMessage) {
        let mut state = self.state.lock().unwrap();
        let state: &mut State = state.deref_mut();
        match message {
            ClientMessage::Login(token) => {
                state.clients.get_mut(&self.id).unwrap().token = token.clone();
                state
                    .clients
                    .get_mut(&self.id)
                    .unwrap()
                    .sender
                    .send(ServerMessage::YourToken(token.clone()));
                if let Ok(save) = futures::executor::block_on(file::load_json::<Save>(format!(
                    "save/{token}.json"
                ))) {
                    state.clients.get_mut(&self.id).unwrap().save = save.clone();
                    for (&client_id, client) in &mut state.clients {
                        client
                            .sender
                            .send(ServerMessage::SetBikeType(self.id, save.bike));
                        client
                            .sender
                            .send(ServerMessage::SetHatType(self.id, save.hat));
                        client
                            .sender
                            .send(ServerMessage::Name(self.id, save.name.clone()));
                        if client_id == self.id {
                            // messages only to the person logging in
                            client.sender.send(ServerMessage::YourUnlockedBikes(
                                save.unlocked_bikes.clone(),
                            ));
                            client
                                .sender
                                .send(ServerMessage::YourUnlockedHats(save.unlocked_hats.clone()));
                            client.sender.send(ServerMessage::SetMoney(save.money));
                        }
                    }
                }
            }
            ClientMessage::SetBikeType(typ) => {
                state.clients.get_mut(&self.id).unwrap().save.bike = typ;
                for (&client_id, client) in &mut state.clients {
                    if self.id != client_id {
                        client.sender.send(ServerMessage::SetBikeType(self.id, typ));
                    }
                }
            }
            ClientMessage::SetHatType(typ) => {
                state.clients.get_mut(&self.id).unwrap().save.hat = typ;
                for (&client_id, client) in &mut state.clients {
                    if self.id != client_id {
                        client.sender.send(ServerMessage::SetHatType(self.id, typ));
                    }
                }
            }
            ClientMessage::LeaveTeam => {
                let self_client = state.clients.get_mut(&self.id).unwrap();
                self_client.can_do_quests = true;
                self_client.pending_race = None;
                self_client.active_race = None;
                self_client
                    .sender
                    .send(ServerMessage::CanDoQuests(self.id, true));
                for (_, client) in &mut state.clients {
                    client.sender.send(ServerMessage::UnsetTeam(self.id));
                }
                let mut followers = Vec::new();
                for (id, other) in &mut state.clients {
                    if other.leader == Some(self.id) {
                        other.leader = None;
                        followers.push(*id);
                    }
                }
                for follower in followers {
                    state.clients.get_mut(&follower).unwrap().pending_race = None;
                    state.clients.get_mut(&follower).unwrap().active_race = None;
                    for client in state.clients.values_mut() {
                        client.sender.send(ServerMessage::UnsetTeam(follower));
                        client
                            .sender
                            .send(ServerMessage::CanDoQuests(follower, true));
                    }
                }
            }
            ClientMessage::JoinTeam(leader_id) => {
                let leader_client = state.clients.get_mut(&leader_id).unwrap();
                leader_client.can_do_quests = false;
                leader_client.delivery = None;
                let race = leader_client.pending_race.clone();

                let self_client = state.clients.get_mut(&self.id).unwrap();
                self_client.delivery = None;
                self_client.can_do_quests = false;
                self_client.leader = Some(leader_id);
                self_client.pending_race = race.clone();
                if let Some(race) = race {
                    self_client.sender.send(ServerMessage::SetPendingRace(race));
                }
                state.clients.get_mut(&leader_id).unwrap().leader = Some(leader_id);
                for (&client_id, client) in &mut state.clients {
                    client
                        .sender
                        .send(ServerMessage::SetTeam(self.id, leader_id));
                    client
                        .sender
                        .send(ServerMessage::CanDoQuests(self.id, false));
                    client.sender.send(ServerMessage::SetDelivery(None));
                    client
                        .sender
                        .send(ServerMessage::CanDoQuests(leader_id, false));
                    client.sender.send(ServerMessage::SetDelivery(None));
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

                state.clients.get_mut(&self.id).unwrap().vehicle = data.clone();

                let has_leader = state.clients[&self.id].leader.is_some();
                let leader = state.clients[&self.id].leader.unwrap_or(self.id);

                if has_leader {
                    // Let's check if the race timer has expired
                    {
                        let leader_client = &state.clients[&leader];
                        if let Some(timer) = &leader_client.race_timer {
                            if timer.elapsed().as_secs_f64() > state.config.race_finish_timer {
                                // yep, race is over, pack it up
                                println!("Race timed out");
                                let mut followers = Vec::new();
                                for (id, other) in &mut state.clients {
                                    if other.leader == Some(leader) {
                                        followers.push(*id);
                                    }
                                }
                                for follower in followers {
                                    let client = state.clients.get_mut(&follower).unwrap();
                                    client.race_timer = None;
                                    client.pending_race = None;
                                    client.active_race = None;
                                    client.race_start_timer = None;
                                    client.sender.send(ServerMessage::RaceFinished);
                                }
                            }
                        }
                    }
                    {
                        // ok we are going to do some race updates now
                        let mut new_race_timer: Option<Option<Timer>> = None;
                        let mut new_finished: usize = state.clients[&leader].finished;
                        let mut follower_messages: Vec<ServerMessage> = Vec::new();
                        let mut winner = false;
                        let mut race_finished = false;
                        let mut self_increment = 0;
                        let participants = state.clients[&leader].participants;

                        let mut followers = Vec::new();
                        for (id, other) in &mut state.clients {
                            if other.leader == Some(leader) {
                                followers.push(*id);
                            }
                        }
                        let Some(pending) = state.clients[&leader].pending_race.clone() else {
                            return;
                        };
                        if self.id == leader {
                            // update ready counts
                            let old_ready_count = state.clients[&leader].ready_count;
                            let mut new_ready_count = 0;
                            for &follower in &followers {
                                if (state.clients[&follower].vehicle.pos
                                    - *pending.track.get(0).unwrap())
                                .len()
                                    < 4.0
                                {
                                    new_ready_count += 1;
                                }
                            }
                            if old_ready_count != new_ready_count {
                                state.clients.get_mut(&leader).unwrap().ready_count =
                                    new_ready_count;
                                state.clients.get_mut(&leader).unwrap().sender.send(
                                    ServerMessage::UpdateReadyCount(
                                        new_ready_count,
                                        followers.len(),
                                    ),
                                );
                            }
                            // Determine the race rankings

                            let rankings: Vec<i64> = followers
                                .iter()
                                .map(|follower| {
                                    let idx = state.clients[follower].active_race.unwrap_or(0);
                                    let dist = (state.clients[follower].vehicle.pos
                                        - pending.track[idx.clamp_max(pending.track.len() - 1)])
                                    .len_sqr();
                                    return (follower, (100000 - idx, r32(dist)));
                                })
                                .sorted_by_key(|k| k.1)
                                .map(|a| *a.0)
                                .collect();
                            follower_messages.push(ServerMessage::UpdateRacePlaces(rankings));
                        }
                        let duration = state.clients[&leader]
                            .race_start_timer
                            .as_ref()
                            .map_or(0.0, |x| x.elapsed().as_secs_f64());
                        let Some(active) = state.clients[&self.id].active_race else {
                            return;
                        };
                        if let Some(waypoint) = &pending.track.get(active) {
                            if (**waypoint - state.clients[&self.id].vehicle.pos).len() < 4.0 {
                                self_increment = 1;
                                if active + 1 == pending.track.len() {
                                    if state.clients[&leader].race_timer.is_none() {
                                        new_race_timer = Some(Some(Timer::new()));
                                        winner = true;
                                    }
                                    self_increment =
                                        1 + state.clients[&leader].participants - new_finished;
                                    // congrats we finished the race
                                    new_finished += 1;
                                    follower_messages.push(ServerMessage::RaceStatistic(
                                        self.id,
                                        duration as f32,
                                        state.clients[&leader].finished,
                                        state.clients[&leader].participants,
                                    ));
                                    if new_finished == state.clients[&leader].participants {
                                        // we can early end the race
                                        println!("early finish!");
                                        race_finished = true;
                                        follower_messages.push(ServerMessage::RaceFinished);
                                    }
                                }
                            }
                        }
                        if let Some(new_race_timer) = new_race_timer {
                            state.clients.get_mut(&leader).unwrap().race_timer = new_race_timer;
                        }
                        state.clients.get_mut(&leader).unwrap().finished = new_finished;
                        if winner {
                            state.clients.get_mut(&self.id).unwrap().race_winner = true;
                        }
                        if self_increment > 0 {
                            state.clients.get_mut(&self.id).unwrap().active_race =
                                Some(active + self_increment);
                            state
                                .clients
                                .get_mut(&self.id)
                                .unwrap()
                                .sender
                                .send(ServerMessage::RaceProgress(active + self_increment));
                        }
                        for follower in followers {
                            if race_finished {
                                let won = state.clients[&follower].race_winner;
                                let prize = if won {
                                    (participants as i64 - 1) * state.config.race_wager
                                } else {
                                    -state.config.race_wager
                                };
                                let new_money =
                                    (state.clients[&follower].save.money + prize).clamp_min(0);
                                state.clients.get_mut(&follower).unwrap().save.money = new_money;
                                state
                                    .clients
                                    .get_mut(&follower)
                                    .unwrap()
                                    .sender
                                    .send(ServerMessage::SetMoney(new_money));

                                state.clients.get_mut(&follower).unwrap().race_timer = None;
                                state.clients.get_mut(&follower).unwrap().race_timer = None;
                                state.clients.get_mut(&follower).unwrap().pending_race = None;
                                state.clients.get_mut(&follower).unwrap().active_race = None;
                                state.clients.get_mut(&follower).unwrap().race_start_timer = None;
                                state.clients.get_mut(&follower).unwrap().race_winner = false;
                            }
                            for message in follower_messages.iter() {
                                state
                                    .clients
                                    .get_mut(&follower)
                                    .unwrap()
                                    .sender
                                    .send(message.clone());
                            }
                        }
                    }

                    // here be dragons - we skip the rest of this function. it is the legacy
                    // code for team quests, inter-twined with the code for solo quests.
                    // it is very confusing so i am leaving it here
                    // so i don't hurt myself in confusion
                    return;
                }
                if let Some(delivery) = state.clients[&self.id].delivery {
                    if (state.level.waypoints[delivery].pos - data.pos).len()
                        < state.config.quest_activation_radius
                    {
                        state.clients.get_mut(&self.id).unwrap().delivery = None;
                        state.clients.get_mut(&leader).unwrap().quest_lock_timer = Timer::new();
                        if state.clients.iter().any(|(id, client)| {
                            client.leader.unwrap_or(*id) == leader && client.delivery.is_some()
                        }) {
                            state.clients.get_mut(&leader).unwrap().timer_time =
                                state.config.team_timer;
                        } else {
                            state.clients.get_mut(&leader).unwrap().timer_time =
                                state.config.quest_lock_timer;
                        }
                        let client = state.clients.get_mut(&self.id).unwrap();
                        client.save.money += client.quest_cost;
                        client
                            .sender
                            .send(ServerMessage::SetMoney(client.save.money));
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
                                    if client.delivery.take().is_some() {
                                        client.sender.send(ServerMessage::SetDelivery(None));
                                    }
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

                state.clients.get_mut(&self.id).unwrap().save.name = name.clone();
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
                let Some(leader) = state.clients[&self.id].leader else {
                    return;
                };
                // Only leaders may start a race
                if leader != self.id {
                    return;
                }

                let leader_client = state.clients.get_mut(&leader).unwrap();
                let Some(race) = &leader_client.pending_race else {
                    return;
                };

                // i need to make sure leader in circle
                let start = *race.track.get(0).unwrap();
                if (start - leader_client.vehicle.pos).len() >= 4.0 {
                    return;
                }
                // tell everyone the race is starting and if they are included or not
                leader_client.race_timer = None;
                leader_client.finished = 0;
                leader_client.race_start_timer = Some(Timer::new());
                leader_client.sender.send(ServerMessage::StartRace(true));
                let mut participants = 0;
                leader_client.active_race = Some(1);
                let mut followers = Vec::new();
                for (id, other) in &mut state.clients {
                    if other.leader == Some(leader) {
                        followers.push(*id);
                    }
                }
                for follower in followers {
                    let follower_client = state.clients.get_mut(&follower).unwrap();
                    let dist = (start - follower_client.vehicle.pos).len();
                    if dist < 4.0 {
                        participants += 1;
                        follower_client.race_start_timer = Some(Timer::new());
                        follower_client.active_race = Some(1);
                    } else {
                        follower_client.race_start_timer = None;
                        follower_client.active_race = None;
                    }
                    follower_client
                        .sender
                        .send(ServerMessage::StartRace(dist < 4.0));
                }
                state.clients.get_mut(&leader).unwrap().participants = participants;
            }
            ClientMessage::UnlockBike(i) => {
                // Sanity check

                if i < 20 {
                    let client = state
                        .clients
                        .get_mut(&self.id)
                        .expect("Sender not found for client");
                    if let Some(unlock) = CUSTOMIZATIONS.bike_names.get(i) {
                        if client.save.money >= unlock.cost
                            && !client.save.unlocked_bikes.contains(&i)
                        {
                            client.save.money -= unlock.cost;
                            client
                                .sender
                                .send(ServerMessage::SetMoney(client.save.money));
                            client.save.unlocked_bikes.insert(i);
                        }
                    }
                }
            }
            ClientMessage::UnlockHat(i) => {
                // Sanity check
                if i < 20 {
                    let client = state
                        .clients
                        .get_mut(&self.id)
                        .expect("Sender not found for client");
                    if let Some(unlock) = CUSTOMIZATIONS.hat_names.get(i) {
                        if !client.save.unlocked_hats.contains(&i) {
                            if client.save.money < unlock.as_ref().map_or(0, |x| x.cost) {
                                return;
                            }
                            client.save.unlocked_hats.insert(i);
                            client.save.money -= unlock.as_ref().map_or(0, |x| x.cost);
                            client
                                .sender
                                .send(ServerMessage::SetMoney(client.save.money));
                        }
                    }
                }
            }
            ClientMessage::LoadRace(race) => {
                if state.clients[&self.id].leader.is_none() {
                    // become our own leader
                    let client = state.clients.get_mut(&self.id).unwrap();
                    client.leader = Some(self.id);
                    client.delivery = None;
                    client.can_do_quests = false;
                    client.sender.send(ServerMessage::SetDelivery(None));
                    client.sender.send(ServerMessage::SetTeam(self.id, self.id));
                    client
                        .sender
                        .send(ServerMessage::CanDoQuests(self.id, false));
                }

                // tell everyone else
                for (_, client) in &mut state.clients {
                    client.sender.send(ServerMessage::SetTeam(self.id, self.id));
                }

                let Some(leader) = state.clients[&self.id].leader else {
                    return;
                };
                // Only leaders may load a race
                if leader != self.id {
                    return;
                }
                if race.track.len() < 2 {
                    println!("invalid track");
                    return;
                }
                state.clients.get_mut(&leader).unwrap().pending_race = Some(race.clone());
                let mut followers = Vec::new();
                for (id, other) in &mut state.clients {
                    if other.leader == Some(leader) {
                        followers.push(*id);
                    }
                }
                state.clients.get_mut(&leader).unwrap().ready_count = 0;
                state
                    .clients
                    .get_mut(&leader)
                    .unwrap()
                    .sender
                    .send(ServerMessage::UpdateReadyCount(0, followers.len()));
                for follower in followers {
                    let client = state.clients.get_mut(&follower).unwrap();
                    client.pending_race = Some(race.clone());
                    client
                        .sender
                        .send(ServerMessage::SetPendingRace(race.clone()));
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
        let token = Alphanumeric.sample_string(&mut thread_rng(), 16);
        sender.send(ServerMessage::YourToken(token.clone()));
        for &quest in &state.active_quests {
            sender.send(ServerMessage::NewQuest(quest));
        }

        for (&other_id, other_client) in &mut state.clients {
            sender.send(ServerMessage::UpdateBike(
                other_id,
                other_client.vehicle.clone(),
            ));
            sender.send(ServerMessage::Name(
                other_id,
                other_client.save.name.clone(),
            ));
            sender.send(ServerMessage::SetBikeType(other_id, other_client.save.bike));
            sender.send(ServerMessage::SetHatType(other_id, other_client.save.hat));
            if let Some(props) = &other_client.vehicle_properties {
                sender.send(ServerMessage::UpdateVehicleProperties(
                    other_id,
                    props.clone(),
                ));
            }
        }

        for (&other_id, other_client) in &state.clients {
            if let Some(leader) = other_client.leader {
                sender.send(ServerMessage::SetTeam(other_id, leader));
            }
        }

        let my_id = state.next_id;
        sender.send(ServerMessage::YourId(my_id));
        for (&other_id, other) in &state.clients {
            sender.send(ServerMessage::CanDoQuests(other_id, other.can_do_quests));
        }
        let mut client = Client {
            token,
            save: Save {
                bike: 0,
                hat: None,
                money: 0,
                name: "<salmoner>".to_owned(),
                unlocked_bikes: HashSet::new(),
                unlocked_hats: HashSet::new(),
            },
            can_do_quests: false,
            timer_time: state.config.quest_lock_timer,
            race_timer: None,
            race_start_timer: None,
            race_winner: false,
            ready_count: 0,
            quest_cost: 0,
            leader: None,
            participants: 0,
            finished: 0,
            pending_race: None,
            active_race: None,
            vehicle: Vehicle::default(),
            quest_lock_timer: Timer::new(),
            delivery: None,
            sender,
            vehicle_properties: None,
        };

        client
            .sender
            .send(ServerMessage::YourName(client.save.name.clone()));

        for (&other_id, other_client) in &mut state.clients {
            other_client
                .sender
                .send(ServerMessage::UpdateBike(my_id, client.vehicle.clone()));
            other_client
                .sender
                .send(ServerMessage::Name(my_id, client.save.name.clone()));
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
