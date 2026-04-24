use super::turrets::Turret;
use crate::Action;
use crate::animation::ActionLocation;
use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Debug)]
pub(crate) struct TurretConfig {
    building: Turret,
    sprite: Handle<Image>,
    animations: Handle<TextureAtlasLayout>,
    atlas_rows: HashMap<Action, ActionLocation>,
}

#[derive(Debug, Resource)]
pub(crate) struct TurretConfigs(HashMap<String, TurretConfig>);

impl TurretConfigs {
    pub(crate) fn init(config_paths: &Vec<PathBuf>, asset_server: &AssetServer) -> Self {
        let hmap = config_paths
            .iter()
            .map(|p| {
                let sck = TurretConfigKeys::from(p);
                (sck.name.clone(), TurretConfig::build(sck, asset_server))
            })
            .collect();
        TurretConfigs(hmap)
    }
}

impl TurretConfig {
    fn build(value: TurretConfigKeys, asset_server: &AssetServer) -> Self {
        let rows = value.animations.len();
        let cols = value.animations.iter().map(|a| a.1).max().unwrap_or(0);
        let building = Turret {
            shot_timer: Timer::from_seconds(value.shot_time, TimerMode::Repeating),
            damage: value.damage,
            range: value.range,
            shoot_function: |_, _| todo!(),
        };
        let atlas_layout =
            TextureAtlasLayout::from_grid(value.tile_size, cols as u32, rows as u32, None, None);
        TurretConfig {
            building,
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

#[derive(Debug, Deserialize)]
struct TurretConfigKeys {
    name: String,
    shot_time: f32,
    damage: f32,
    range: f32,
    animations: Vec<(Action, usize)>,
    sprite_path: PathBuf,
    tile_size: UVec2,
    //projectile_config_path: PathBuf,
}

impl From<&PathBuf> for TurretConfigKeys {
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
