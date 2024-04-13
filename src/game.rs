use crate::{
    assets::Assets,
    controls::{self, GengEvent},
    interop::{ClientConnection, ClientMessage},
    model::{self, *},
    render, sound,
};

use evenio::prelude::*;
use geng::prelude::*;

#[allow(dead_code)]
pub struct Game {
    connection: ClientConnection,
    sends: std::sync::mpsc::Receiver<ClientMessage>,
    geng: Geng,
    world: World,
}

impl Game {
    pub async fn new(geng: &Geng, connection: ClientConnection, assets: &Rc<Assets>) -> Self {
        let (sender, sends) = std::sync::mpsc::channel();
        Self {
            sends,
            connection,
            geng: geng.clone(),
            world: {
                let mut world = World::new();
                model::init(&mut world);
                render::init(&mut world, geng, assets).await;
                controls::init(&mut world, geng).await;
                sound::init(&mut world, geng, assets);
                world.add_handler(move |receiver: ReceiverMut<ClientMessage>| {
                    let _ = sender.send(EventMut::take(receiver.event));
                });
                world.send(Startup);
                world
            },
        }
    }
}

impl geng::State for Game {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
        self.world.send(crate::render::Draw {
            framebuffer: unsafe {
                // SAFETY: this is safe
                std::mem::transmute(framebuffer)
            },
        });
    }

    fn handle_event(&mut self, event: geng::Event) {
        self.world.send(GengEvent(event));
    }

    fn update(&mut self, delta_time: f64) {
        for message in self.connection.new_messages() {
            let message = message.unwrap();
            self.world.send(message);
        }
        while let Ok(message) = self.sends.try_recv() {
            self.connection.send(message);
        }
        let delta_time = time::Duration::from_secs_f64(delta_time);
        self.world.send(crate::model::Update { delta_time });
    }
}
