use geng::prelude::*;

use crate::render::obj::Obj;

#[derive(Clone, Deref)]
pub struct Texture(pub Rc<ugli::Texture>);

impl Texture {
    pub fn ugli(&self) -> &ugli::Texture {
        &self.0
    }
}

#[derive(Default, Clone)]
pub struct TextureOptions {
    pub wrap: bool,
}

impl geng::asset::Load for Texture {
    type Options = TextureOptions;
    fn load(
        manager: &geng::asset::Manager,
        path: &std::path::Path,
        options: &Self::Options,
    ) -> geng::asset::Future<Self> {
        let texture = manager.load_with(
            path,
            &geng::asset::TextureOptions {
                filter: ugli::Filter::Nearest,
                wrap_mode: match options.wrap {
                    true => ugli::WrapMode::Repeat,
                    false => ugli::WrapMode::Clamp,
                },
                ..default()
            },
        );
        async move { Ok(Self(texture.await?)) }.boxed_local()
    }
    const DEFAULT_EXT: Option<&'static str> = Some("png");
}

#[derive(geng::asset::Load)]
pub struct Bike {
    pub top: Texture,
    pub top_handle: Texture,
    pub side: Texture,
    pub wheel: Texture,
    pub unicycle_seat: Texture,
    pub unicycle_side: Texture,
    pub unicycle_top: Texture,
}

#[derive(geng::asset::Load)]
pub struct Shaders {
    pub main_instancing: ugli::Program,
    pub main_no_instancing: ugli::Program,
    pub waypoint: ugli::Program,
    pub minimap: ugli::Program,
    pub billboard: ugli::Program,
    pub sky: ugli::Program,
}

#[derive(geng::asset::Load)]
pub struct Models {
    pub salmon: Obj,
    #[load(listed_in = "_list.json")]
    pub hats: Vec<Obj>,
}

#[derive(geng::asset::Load)]
pub struct Road {
    #[load(options(wrap = "true"))]
    pub asphalt: Texture,
    #[load(options(wrap = "true"))]
    pub border: Texture,
    #[load(options(wrap = "true"))]
    pub road: Texture,
}

#[derive(geng::asset::Load)]
pub struct BuildingType {
    #[load(listed_in = "list.json")]
    pub tops: Vec<Texture>,
    #[load(listed_in = "list.json")]
    pub sides: Vec<Texture>,
}

#[derive(geng::asset::Load)]
pub struct SmallBuildingType {
    pub side_a: Texture,
    pub side_b: Texture,
    pub top: Texture,
}

#[derive(geng::asset::Load)]
pub struct Car {
    pub bottomfront: Texture,
    pub bottomside: Texture,
    pub bottomtop: Texture,
    pub topside: Texture,
    pub toptop: Texture,
}

#[derive(geng::asset::Load)]
pub struct GarageType {
    pub awning: Texture,
    pub top: Texture,
    pub back: Texture,
    pub door: Texture,
    pub front: Texture,
    pub side_a: Texture,
    pub side_b: Texture,
    pub sign: Texture,
}

#[derive(geng::asset::Load)]
pub struct Assets {
    #[load(options(wrap = "true"))]
    pub ground: Texture,
    pub models: Models,
    pub bike: Bike,
    pub garage: GarageType,
    pub shaders: Shaders,
    pub salmon: Texture,
    pub salmon2: Texture,
    pub salmonfin: Texture,
    pub billboard_legs: Texture,
    pub billboard_top: Texture,
    pub road: Road,
    pub car: Car,
    #[load(listed_in = "list.json")]
    pub buildings: Vec<BuildingType>,
    #[load(listed_in = "list.json")]
    pub small_items: Vec<SmallBuildingType>,
    #[load(listed_in = "list.json")]
    pub flora: Vec<Texture>,
    #[load(listed_in = "list.json")]
    pub particles: Vec<Texture>,
    pub sounds: Sounds,
}

#[derive(geng::asset::Load)]
pub struct Sounds {
    #[load(ext = "mp3", options(looped = "true"))]
    pub music: geng::Sound,
    #[load(ext = "mp3", options(looped = "true"))]
    pub salmon_radio: geng::Sound,
    #[load(ext = "mp3")]
    pub bell: geng::Sound,
    #[load(ext = "mp3")]
    pub quest_start: geng::Sound,
    #[load(ext = "mp3")]
    pub quest_complete: geng::Sound,
    #[load(ext = "mp3", options(looped = "true"))]
    pub pedaling: geng::Sound,
    #[load(ext = "mp3")]
    pub brake: geng::Sound,
    #[load(ext = "mp3")]
    pub honk: geng::Sound,
    #[load(ext = "mp3")]
    pub bonk: geng::Sound,
}

#[derive(geng::asset::Load)]
pub struct Particles {}

impl Assets {
    pub async fn load(manager: &geng::asset::Manager) -> anyhow::Result<Self> {
        geng::asset::Load::load(manager, &run_dir().join("assets"), &())
            .await
            .context("failed to load assets")
    }
}
