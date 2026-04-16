use super::turrets::{Turret, TurretAnimation};
use bevy::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug)]
pub(crate) struct TurretConfig {
    building: Turret,
    sprite: Handle<Image>,
    animations: Handle<TextureAtlasLayout>,
    atlas_rows: HashMap<TurretAnimation, usize>,
}

#[derive(Debug, Resource)]
pub(crate) struct TurretConfigs(HashMap<String, TurretConfig>);

impl TurretConfigs {
    pub(crate) fn init(config_paths: Vec<PathBuf>, asset_server: &AssetServer) -> Self {
        todo!()
    }
}
