use geng::prelude::*;

#[derive(Clone, Deref)]
pub struct Texture(Rc<ugli::Texture>);

impl Texture {
    pub fn ugli(&self) -> &ugli::Texture {
        &self.0
    }
}

impl geng::asset::Load for Texture {
    type Options = ();
    fn load(
        manager: &geng::asset::Manager,
        path: &std::path::Path,
        _options: &Self::Options,
    ) -> geng::asset::Future<Self> {
        let texture = manager.load_with(
            path,
            &geng::asset::TextureOptions {
                filter: ugli::Filter::Nearest,
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
pub struct Assets {
    pub bike: Bike,
    pub shaders: Shaders,
}

impl Assets {
    pub async fn load(manager: &geng::asset::Manager) -> anyhow::Result<Self> {
        geng::asset::Load::load(manager, &run_dir().join("assets"), &())
            .await
            .context("failed to load assets")
    }
}
