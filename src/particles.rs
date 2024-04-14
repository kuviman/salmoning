use evenio::prelude::*;
use geng::prelude::*;

use crate::model::{Update, Vehicle};

#[derive(Component)]
struct Particle {}

fn bike_trail(_receiver: Receiver<Update>, bikes: Fetcher<&Vehicle>, sender: Sender<Spawn>) {
    for bike in bikes.iter() {}
}
