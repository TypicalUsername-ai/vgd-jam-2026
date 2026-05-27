use super::minion::MinionKind;
use crate::animation::{Action, ActionLocation, AnimationState};
use crate::heroes::UpgradeChoice;
use crate::level_map::HpTracker;
use crate::minions::Minion;
use bevy::prelude::*;
use state::LevelState;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Debug)]
pub(crate) struct MinionConfig {
    pub minion: Minion,
    pub sprite: Handle<Image>,
    pub animations: Handle<TextureAtlasLayout>,
    pub atlas_rows: HashMap<Action, ActionLocation>,
}

impl MinionConfig {
    pub fn spawn(
        &self,
        commands: &mut Commands,
        upgrades: &[UpgradeChoice],
        hp_tracker_id: Entity,
        spawn_location: Vec3,
    ) {
        let mut minion = self.minion.clone();
        for upgrade in upgrades {
            upgrade.apply(&mut minion);
        }
        commands.spawn((
            DespawnOnExit(LevelState::Active),
            Sprite::from_atlas_image(
                self.sprite.clone(),
                TextureAtlas {
                    layout: self.animations.clone(),
                    index: 0,
                },
            ),
            minion,
            HpTracker(hp_tracker_id),
            Transform::from_translation(spawn_location),
            AnimationState::new(
                0.25,
                Action::WalkLeft,
                self.atlas_rows
                    .get(&Action::WalkLeft)
                    .expect("All actions are configured")
                    .clone(),
            ),
        ));
    }

    fn build(value: MinionConfigKeys, asset_server: &AssetServer) -> Self {
        let rows = value.animations.len();
        let cols = value.animations.iter().map(|a| a.1).max().unwrap_or(0);
        let atlas_layout =
            TextureAtlasLayout::from_grid(value.tile_size, cols as u32, rows as u32, None, None);
        Self {
            minion: Minion {
                kind: value.kind,
                max_health: value.health,
                health: value.health,
                speed: value.speed,
                distance_traveled: 0.,
                target_index: 1,
            },
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

pub(crate) fn setup_minions(
    mut commands: Commands,
    config_handles: Res<MinionConfigHandles>,
    mut configs: ResMut<Assets<MinionConfigKeys>>,
    asset_server: Res<AssetServer>,
) {
    let hmap = config_handles
        .iter()
        .map(|h| {
            let sck = configs
                .remove(h)
                .expect("all minion configs should be loaded");
            (sck.kind, MinionConfig::build(sck, &asset_server))
        })
        .collect();
    commands.insert_resource(MinionConfigs(hmap));
}

#[derive(Debug, Resource, Deref)]
pub(crate) struct MinionConfigs(HashMap<MinionKind, MinionConfig>);

#[derive(Debug, Resource, Deref)]
pub(crate) struct MinionConfigHandles(pub Vec<Handle<MinionConfigKeys>>);

impl MinionConfigHandles {
    pub(crate) fn load_minions(configs: &[&str], asset_server: &AssetServer) -> Self {
        let handles = configs
            .iter()
            .map(|c| format!("minions/{}.minion.ron", c))
            .map(|p| asset_server.load::<MinionConfigKeys>(p))
            .collect::<Vec<_>>();
        Self(handles)
    }
}

#[derive(Debug, serde::Deserialize, Asset, TypePath)]
#[serde(rename = "MinionConfig")]
pub(crate) struct MinionConfigKeys {
    kind: MinionKind,
    health: f32,
    speed: f32,
    animations: Vec<(Action, usize)>,
    sprite_path: PathBuf,
    tile_size: UVec2,
}
