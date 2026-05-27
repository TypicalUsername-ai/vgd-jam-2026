use std::collections::HashMap;

use crate::heroes::HeroKind;

use super::TurretPoint;
use bevy::prelude::*;
use serde::Deserialize;

/// used to configure the initial map state and resource locations for a given level
/// handles necessary components for each level:
#[derive(Debug, Deserialize, Resource, Asset, TypePath, Clone)]
pub(crate) struct LevelMapConfig {
    /// string id of the level
    pub id: String,
    /// set of points which connected make the path minions have to walk
    pub path_points: Vec<Vec3>,
    /// hold locations of all [TowerPoint]s
    pub tower_points: Vec<TurretPoint>,
    /// all [HeroKind] available for a given level
    pub available_heroes: Vec<HeroKind>,
    /// upgrade points available to spend
    pub upgrade_points: u8,
    /// next level to load
    pub next_level_id: Option<String>, // TODO
    /// background color for level
    pub bg_color: Color,
    /// background sprite for level backdrop
    pub bg_image: Option<String>,
    /// conversations and tips to be displayed before a level
    pub messages: Vec<String>,
    /// available slots for heroes
    pub hero_slots: u8,
}

#[derive(Debug, Resource, Deref)]
pub(crate) struct LevelConfigHandles(pub Vec<Handle<LevelMapConfig>>);

impl LevelConfigHandles {
    pub(crate) fn load_levels(configs: &[&str], asset_server: &AssetServer) -> Self {
        let handles = configs
            .iter()
            .map(|c| format!("maps/{}.level.ron", c))
            .map(|p| asset_server.load::<LevelMapConfig>(p))
            .collect::<Vec<_>>();
        Self(handles)
    }
}

#[derive(Debug, Resource, Deref)]
pub(crate) struct LevelConfigs(HashMap<String, LevelMapConfig>);

pub(crate) fn setup_levels(
    mut commands: Commands,
    config_handles: Res<LevelConfigHandles>,
    mut configs: ResMut<Assets<LevelMapConfig>>,
    asset_server: Res<AssetServer>,
) {
    let hmap = config_handles
        .iter()
        .map(|h| {
            let sck = configs
                .remove(h)
                .expect("all level configs should be loaded");
            (sck.id.clone(), sck)
        })
        .collect();
    commands.insert_resource(LevelConfigs(hmap));
}
