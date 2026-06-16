use super::Turret;
use crate::Action;
use crate::animation::ActionLocation;
use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug)]
pub(crate) struct TurretConfig {
    pub building: Turret,
    pub sprite: Handle<Image>,
    pub animations: Handle<TextureAtlasLayout>,
    pub atlas_rows: HashMap<Action, ActionLocation>,
}

#[derive(Debug, Resource, Deref)]
pub(crate) struct TurretConfigs(HashMap<TurretKind, TurretConfig>);

pub(crate) fn setup_turrets(
    mut commands: Commands,
    config_handles: Res<TurretConfigHandles>,
    mut configs: ResMut<Assets<TurretConfigKeys>>,
    asset_server: Res<AssetServer>,
) {
    let hmap = config_handles
        .iter()
        .map(|h| {
            let sck = configs
                .remove(h)
                .expect("all turret configs should be loaded");
            (sck.kind, TurretConfig::build(sck, &asset_server))
        })
        .collect();
    commands.insert_resource(TurretConfigs(hmap));
}

impl TurretConfig {
    fn build(value: TurretConfigKeys, asset_server: &AssetServer) -> Self {
        let rows = value.animations.len();
        let cols = value.animations.iter().map(|a| a.1).max().unwrap_or(0);
        let building = Turret {
            kind: value.kind,
            shot_timer: Timer::from_seconds(value.shot_time, TimerMode::Repeating),
            damage: value.damage,
            range: value.range,
            firing_on: None,
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
                .map(|(idx, (action, len))| (action, (idx * rows)..(idx * rows + len)))
                .collect(),
        }
    }
}

#[derive(Debug, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TurretKind {
    Basic,
}

#[derive(Debug, Deserialize, Asset, TypePath)]
#[serde(rename = "TurretConfig")]
pub struct TurretConfigKeys {
    kind: TurretKind,
    shot_time: f32,
    damage: f32,
    range: f32,
    animations: Vec<(Action, usize)>,
    sprite_path: PathBuf,
    tile_size: UVec2,
    //projectile_config_path: PathBuf,
}

#[derive(Debug, Resource, Deref)]
pub struct TurretConfigHandles(pub Vec<Handle<TurretConfigKeys>>);

impl TurretConfigHandles {
    pub fn load_turrets(configs: &[&str], asset_server: &AssetServer) -> Self {
        let handles = configs
            .iter()
            .map(|c| format!("turrets/{}.turret.ron", c))
            .map(|p| asset_server.load::<TurretConfigKeys>(p))
            .collect::<Vec<_>>();
        Self(handles)
    }
}
