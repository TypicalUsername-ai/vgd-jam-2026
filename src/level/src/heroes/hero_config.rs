use super::active_hero::{ActiveHero, HeroKind};
use crate::Action;
use crate::animation::ActionLocation;
use crate::minions::MinionKind;
use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Debug)]
pub(crate) struct HeroConfig {
    pub hero: ActiveHero,
    pub sprite: Handle<Image>,
    pub animations: Handle<TextureAtlasLayout>,
    pub atlas_rows: HashMap<Action, ActionLocation>,
}

#[derive(Debug, Resource, Deref)]
pub(crate) struct HeroConfigs(HashMap<HeroKind, HeroConfig>);

//impl HeroConfigs {
pub(crate) fn setup_heroes(
    mut commands: Commands,
    config_handles: Res<HeroConfigHandles>,
    mut configs: ResMut<Assets<HeroConfigKeys>>,
    asset_server: Res<AssetServer>,
) {
    let hmap = config_handles
        .iter()
        .map(|h| {
            let sck = configs
                .remove(h)
                .expect("all hero configs should be loaded");
            (sck.kind, HeroConfig::build(sck, &asset_server))
        })
        .collect();
    commands.insert_resource(HeroConfigs(hmap));
}
//}

impl HeroConfig {
    fn build(value: HeroConfigKeys, asset_server: &AssetServer) -> Self {
        let rows = value.animations.len();
        let cols = value.animations.iter().map(|a| a.1).max().unwrap_or(0);
        let hero = ActiveHero {
            spawner_kind: value.kind,
            spawned_minion: value.spawned_minion,
            applied_upgrades: vec![],
        };
        let atlas_layout =
            TextureAtlasLayout::from_grid(value.tile_size, cols as u32, rows as u32, None, None);
        Self {
            hero,
            sprite: asset_server.load(value.sprite_path),
            animations: asset_server.add(atlas_layout),
            atlas_rows: value
                .animations
                .into_iter()
                .enumerate()
                .map(|(idx, (action, len))| (action, (idx * rows)..(idx * rows + len)))
                .collect(),
        }
    }
}

#[derive(Debug, Deserialize, Asset, TypePath)]
#[serde(rename = "HeroConfig")]
pub(crate) struct HeroConfigKeys {
    kind: HeroKind,
    animations: Vec<(Action, usize)>,
    sprite_path: PathBuf,
    tile_size: UVec2,
    spawned_minion: MinionKind,
}
#[derive(Debug, Resource, Deref)]
pub(crate) struct HeroConfigHandles(pub Vec<Handle<HeroConfigKeys>>);

impl HeroConfigHandles {
    pub fn load_heroes(configs: &[&str], asset_server: &AssetServer) -> Self {
        let handles = configs
            .iter()
            .map(|c| format!("heroes/{}.hero.ron", c))
            .map(|p| asset_server.load::<HeroConfigKeys>(p))
            .collect::<Vec<_>>();
        Self(handles)
    }
}
