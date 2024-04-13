#[cfg(not(target_arch = "wasm32"))]
mod server;

mod assets;
mod controls;
mod editor;
mod game;
mod interop;
mod model;
mod render;
mod sound;

use geng::prelude::*;

#[derive(clap::Parser)]
struct Args {
    #[clap(long)]
    pub server: Option<String>,
    #[clap(long)]
    pub connect: Option<String>,
    #[clap(long)]
    pub editor: bool,
    #[clap(flatten)]
    pub geng: geng::CliArgs,
}

fn main() {
    logger::init();
    geng::setup_panic_handler();

    let mut args: Args = clap::Parser::parse();

    let mut geng_options = geng::ContextOptions::default();
    geng_options.window.title = "Salmoning".to_string();
    geng_options.with_cli(&args.geng);

    if args.connect.is_none() && args.server.is_none() {
        #[cfg(target_arch = "wasm32")]
        {
            args.connect = Some(
                option_env!("CONNECT")
                    .filter(|addr| !addr.is_empty())
                    .map(|addr| addr.to_owned())
                    .unwrap_or_else(|| {
                        let window = web_sys::window().unwrap();
                        let location = window.location();
                        let mut new_uri = String::new();
                        if location.protocol().unwrap() == "https" {
                            new_uri += "wss://";
                        } else {
                            new_uri += "ws://";
                        }
                        new_uri += &location.host().unwrap();
                        new_uri += &location.pathname().unwrap();
                        new_uri
                    }),
            );
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            args.server = Some("127.0.0.1:1155".to_owned());
            args.connect = Some("ws://127.0.0.1:1155".to_owned());
        }
    }

    if args.server.is_some() && args.connect.is_none() {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let server =
                geng::net::Server::new(server::App::new(), args.server.as_deref().unwrap());
            let server_handle = server.handle();
            ctrlc::set_handler(move || server_handle.shutdown()).unwrap();
            server.run();
        }
    } else {
        #[cfg(not(target_arch = "wasm32"))]
        let server = if let Some(addr) = &args.server {
            let server = geng::net::Server::new(server::App::new(), addr);
            let server_handle = server.handle();
            let server_thread = std::thread::spawn(move || {
                server.run();
            });
            Some((server_handle, server_thread))
        } else {
            None
        };

        Geng::run_with(&geng_options, move |geng| async move {
            let connection = geng::net::client::connect(&args.connect.unwrap())
                .await
                .unwrap();
            let manager = geng.asset_manager();
            let assets = assets::Assets::load(manager).await.unwrap();
            let state = game::Game::new(&geng, connection, &Rc::new(assets), args.editor).await;
            geng.run_state(state).await;
        });

        #[cfg(not(target_arch = "wasm32"))]
        if let Some((server_handle, server_thread)) = server {
            server_handle.shutdown();
            server_thread.join().unwrap();
        }
    }
}
