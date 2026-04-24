use super::spawners::{SpawnerBuilding, SpawnerKind};
use crate::Action;
use crate::animation::ActionLocation;
use crate::minions::MinionKind;
use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::ops::Range;
use std::path::PathBuf;

#[derive(Debug)]
pub(crate) struct SpawnerBuildingConfig {
    pub building: SpawnerBuilding,
    pub sprite: Handle<Image>,
    pub animations: Handle<TextureAtlasLayout>,
    pub atlas_rows: HashMap<Action, ActionLocation>,
}

#[derive(Debug, Resource, Deref)]
pub(crate) struct SpawnerConfigs(HashMap<SpawnerKind, SpawnerBuildingConfig>);

impl SpawnerConfigs {
    pub(crate) fn init(config_paths: &Vec<PathBuf>, asset_server: &AssetServer) -> Self {
        let hmap = config_paths
            .iter()
            .map(|p| {
                let sck = SpawnerConfigKeys::from(p);
                (sck.kind, SpawnerBuildingConfig::build(sck, asset_server))
            })
            .collect();
        SpawnerConfigs(hmap)
    }
}

impl SpawnerBuildingConfig {
    fn build(value: SpawnerConfigKeys, asset_server: &AssetServer) -> Self {
        let rows = value.animations.len();
        let cols = value.animations.iter().map(|a| a.1).max().unwrap_or(0);
        let building = SpawnerBuilding {
            spawner_kind: value.kind,
            spawn_timer: Timer::from_seconds(value.spawn_time, TimerMode::Repeating),
            spawned_minion: value.spawned_minion,
        };
        let atlas_layout =
            TextureAtlasLayout::from_grid(value.tile_size, cols as u32, rows as u32, None, None);
        SpawnerBuildingConfig {
            building,
            sprite: asset_server.load(value.sprite_path),
            animations: asset_server.add(atlas_layout),
            atlas_rows: value
                .animations
                .into_iter()
                .enumerate()
                .map(|(row, (action, len))| (action, ActionLocation::new(row, len)))
                .collect(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename = "SpawnerConfig")]
struct SpawnerConfigKeys {
    kind: SpawnerKind,
    spawn_time: f32,
    animations: Vec<(Action, usize)>,
    sprite_path: PathBuf,
    tile_size: UVec2,
    spawned_minion: MinionKind,
}

impl From<&PathBuf> for SpawnerConfigKeys {
    fn from(value: &PathBuf) -> Self {
        let config = File::open(value);
        match config {
            Ok(mut config_file) => {
                let mut buf = String::new();
                let _ = config_file.read_to_string(&mut buf);
                ron::from_str(&buf).expect("error parsing options file!! {value}")
            }
            Err(_err) => {
                panic!("error reading file!! {}", value.display())
            }
        }
    }
}
