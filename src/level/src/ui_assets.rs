use bevy::prelude::*;

#[derive(Debug, Resource)]
pub(crate) struct UiAssets {
    pub portrait_bg: Handle<Image>,
    pub hp_bar: Handle<Image>,
    pub hp_bar_layout: Handle<TextureAtlasLayout>,
}

impl UiAssets {
    pub fn init(asset_server: &AssetServer) -> Self {
        Self {
            portrait_bg: asset_server.load("ui/hero_portrait.png"),
            hp_bar: asset_server.load("ui/hp_bar.png"),
            hp_bar_layout: asset_server.add(TextureAtlasLayout::from_grid(
                UVec2 { x: 64, y: 256 },
                11,
                1,
                None,
                None,
            )),
        }
    }
}
