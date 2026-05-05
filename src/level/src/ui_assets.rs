use bevy::prelude::*;

#[derive(Debug, Resource)]
pub(crate) struct UiAssets {
    pub portrait_bg: Handle<Image>,
}

impl UiAssets {
    pub fn init(asset_server: &AssetServer) -> Self {
        Self {
            portrait_bg: asset_server.load("ui/hero_portrait.png"),
        }
    }
}
