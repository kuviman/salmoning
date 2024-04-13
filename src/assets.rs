use geng::prelude::*;

#[derive(Clone, Deref)]
pub struct Texture(Rc<ugli::Texture>);

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
}

#[derive(geng::asset::Load)]
pub struct Shaders {
    pub main: ugli::Program,
}

#[derive(geng::asset::Load)]
pub struct Road {
    #[load(options(wrap = "true"))]
    pub straight: Texture,
}

#[derive(geng::asset::Load)]
pub struct Assets {
    #[load(options(wrap = "true"))]
    pub ground: Texture,
    pub bike: Bike,
    pub shaders: Shaders,
    pub salmon: Texture,
    pub road: Road,
}

impl Assets {
    pub async fn load(manager: &geng::asset::Manager) -> anyhow::Result<Self> {
        geng::asset::Load::load(manager, &run_dir().join("assets"), &())
            .await
            .context("failed to load assets")
    }
}
