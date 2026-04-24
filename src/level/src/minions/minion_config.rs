use super::minion::MinionKind;
use crate::animation::{Action, ActionLocation};
use bevy::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Debug)]
pub(crate) struct MinionConfig {
    pub kind: MinionKind,
    pub base_health: f32,
    pub base_speed: f32,
    pub sprite: Handle<Image>,
    pub animations: Handle<TextureAtlasLayout>,
    pub atlas_rows: HashMap<Action, ActionLocation>,
}

impl MinionConfig {
    pub fn spawn(&self, commands: &mut Commands, spawn_location: Vec3) {
        todo!()
    }

    fn build(value: MinionConfigKeys, asset_server: &AssetServer) -> Self {
        let rows = value.animations.len();
        let cols = value.animations.iter().map(|a| a.1).max().unwrap_or(0);
        let atlas_layout =
            TextureAtlasLayout::from_grid(value.tile_size, cols as u32, rows as u32, None, None);
        Self {
            kind: value.kind,
            base_health: value.health,
            base_speed: value.speed,
            sprite: asset_server.load(value.sprite_path),
            animations: asset_server.add(atlas_layout),
            atlas_rows: value
                .animations
                .into_iter()
                .enumerate()
                .map(|(idx, (action, len))| (action, ActionLocation::new(idx, len)))
                .collect(),
        }
    }
}

#[derive(Debug, Resource, Deref)]
pub(crate) struct MinionConfigs(HashMap<MinionKind, MinionConfig>);

impl MinionConfigs {
    pub(crate) fn init(config_paths: &Vec<PathBuf>, asset_server: &AssetServer) -> Self {
        let hmap = config_paths
            .iter()
            .map(|p| {
                let mck = MinionConfigKeys::from(p);
                (mck.kind, MinionConfig::build(mck, asset_server))
            })
            .collect();
        MinionConfigs(hmap)
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename = "MinionConfig")]
struct MinionConfigKeys {
    kind: MinionKind,
    health: f32,
    speed: f32,
    animations: Vec<(Action, usize)>,
    sprite_path: PathBuf,
    tile_size: UVec2,
}

impl From<&PathBuf> for MinionConfigKeys {
    fn from(value: &PathBuf) -> Self {
        let config = File::open(value);
        match config {
            Ok(mut config_file) => {
                let mut buf = String::new();
                let _ = config_file.read_to_string(&mut buf);
                ron::from_str(&buf).unwrap_or_else(|e| {
                    panic!("error parsing options file {}! {}", value.display(), e)
                })
            }
            Err(_err) => {
                panic!("error reading file!! {}", value.display())
            }
        }
    }
}
