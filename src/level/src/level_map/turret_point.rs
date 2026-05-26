use crate::animation::Action;
use bevy::prelude::*;
use serde::Deserialize;
use state::LevelState;

use super::LevelMapConfig;
use crate::{
    animation::AnimationState,
    turrets::{TurretConfigs, TurretKind},
};

/// a single point which can holds a specified turret
#[derive(Debug, Deserialize)]
pub(crate) struct TurretPoint {
    /// the center position of the point
    pub position: Vec3,
    pub kind: TurretKind,
}

/// spawns all of the turrets at the locations specified in [LevelMapConfig]
pub(crate) fn setup_turrets(
    mut commands: Commands,
    level_config: Res<LevelMapConfig>,
    turret_configs: Res<TurretConfigs>,
) {
    for entry in level_config.tower_points.iter() {
        info!("Spawning >> {:?}", entry);
        let conf = turret_configs
            .get(&entry.kind)
            .expect("All turret configs should be loaded!");
        commands.spawn((
            conf.building.clone(),
            DespawnOnExit(LevelState::Active),
            Transform::from_translation(entry.position),
            Sprite::from_atlas_image(
                conf.sprite.clone(),
                TextureAtlas {
                    layout: conf.animations.clone(),
                    index: 0,
                },
            ),
            AnimationState::new(
                0.25,
                Action::Idle,
                conf.atlas_rows
                    .get(&Action::Idle)
                    .expect("action should be configured")
                    .clone(),
            ),
        ));
    }
}
