use crate::{
    assets::{Assets, Sounds},
    controls::{self, GengEvent},
    editor,
    interop::{ClientConnection, ClientMessage, ServerMessage},
    model::{self, *},
    race_editor, render, sound, ui,
};

use evenio::prelude::*;
use geng::prelude::*;

#[allow(dead_code)]
pub struct Game {
    connection: ClientConnection,
    sends: std::sync::mpsc::Receiver<ClientMessage>,
    geng: Geng,
    world: World,
    minimap_texture: ugli::Texture,
    minimap_buffer: ugli::Renderbuffer<ugli::DepthComponent>,
}

impl Game {
    pub async fn new(
        geng: &Geng,
        mut connection: ClientConnection,
        assets: &Rc<Assets>,
        editor: bool,
    ) -> Self {
        let ServerMessage::Rng(seed) = connection.next().await.unwrap().unwrap() else {
            unreachable!()
        };
        let (sender, sends) = std::sync::mpsc::channel();

        let level = Level::load(run_dir().join("assets").join("level.json"));
        let level = level.await.unwrap_or_else(|err| {
            log::error!("Failed to load level: {:?}", err);
            log::warn!("Using default level");
            Level::default()
        });
        let startup = Startup { level };

        Self {
            sends,
            connection,
            geng: geng.clone(),
            world: {
                let mut world = World::new();
                let rng = world.spawn();
                let mut gen = StdRng::seed_from_u64(seed);
                model::init(&mut world).await;
                render::init(&mut world, geng, assets, &mut gen, editor, &startup).await;
                model::post_init(&mut world).await;
                world.insert(rng, model::RngStuff { seed, gen });
                race_editor::init(&mut world, geng).await;
                if editor {
                    editor::init(&mut world, geng, startup.level.clone()).await;
                }
                controls::init(&mut world, geng).await;
                sound::init(&mut world, geng, &assets.clone()).await;
                #[cfg(target_arch = "wasm32")]
                {
                    ui::init(&mut world, geng).await;
                }
                world.add_handler(move |receiver: ReceiverMut<ClientMessage>| {
                    let _ = sender.send(EventMut::take(receiver.event));
                });
                world.send(startup);
                if let Some(token) = preferences::load("token") {
                    world.send(ClientMessage::Login(token));
                }
                world
            },
            minimap_texture: ugli::Texture::new_with(geng.ugli(), vec2(256, 256), |_| Rgba::GREEN),
            minimap_buffer: ugli::Renderbuffer::new(geng.ugli(), vec2(256, 256)),
        }
    }
}

impl geng::State for Game {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
        {
            let framebuffer = &mut *framebuffer;
            self.world.send(crate::render::Draw {
                framebuffer: unsafe {
                    // SAFETY: this is safe
                    std::mem::transmute(framebuffer)
                },
            });
        }

        let mut minimap_buffer = ugli::Framebuffer::new(
            self.geng.ugli(),
            ugli::ColorAttachment::Texture(&mut self.minimap_texture),
            ugli::DepthAttachment::Renderbuffer(&mut self.minimap_buffer),
        );
        ugli::clear(
            &mut minimap_buffer,
            Some(Rgba::try_from("#6abe30").unwrap()),
            Some(1.0),
            None,
        );
        self.world.send(crate::render::MinimapDraw {
            framebuffer: unsafe {
                // SAFETY: this is safe
                std::mem::transmute(&mut minimap_buffer)
            },
        });

        let minimap_size = framebuffer.size().y as f32 * 0.25;
        let target = Aabb2::point(framebuffer.size().map(|x| x as f32) - vec2::splat(10.0))
            .extend_left(minimap_size)
            .extend_down(minimap_size);
        self.geng.draw2d().quad(
            framebuffer,
            &geng::PixelPerfectCamera,
            target.extend_uniform(5.0),
            Rgba::BLACK,
        );
        self.geng.draw2d().textured_quad(
            framebuffer,
            &geng::PixelPerfectCamera,
            target,
            &self.minimap_texture,
            Rgba::WHITE,
        );
    }

    fn handle_event(&mut self, event: geng::Event) {
        self.world.send(GengEvent(event));
    }

    fn update(&mut self, delta_time: f64) {
        for message in self.connection.new_messages() {
            let message = message.unwrap();
            self.world.send(message);
        }
        for message in ui::new_messages() {
            self.world.send(message);
        }
        while let Ok(message) = self.sends.try_recv() {
            self.connection.send(message);
        }
        let delta_time = time::Duration::from_secs_f64(delta_time);
        self.world.send(crate::model::Update { delta_time });
    }
}
