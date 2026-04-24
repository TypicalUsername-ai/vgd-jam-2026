use bevy::prelude::*;

use super::spawner_config::SpawnerBuildingConfig;
use crate::{
    animation::Action,
    level_map::LevelMapConfig,
    minions::{MinionConfigs, MinionKind},
};

/// uses building timers to spawn minions throughout the level
pub(crate) fn spawn_minions(
    mut commands: Commands,
    time: Res<Time>,
    level_config: Res<LevelMapConfig>,
    minion_configs: Res<MinionConfigs>,
    mut query: Query<(Entity, &mut SpawnerBuilding, &Transform)>,
) {
    for (entity, mut spawner, transform) in query.iter_mut() {
        // progress time for each spawner
        spawner.spawn_timer.tick(time.delta());
        if spawner.spawn_timer.just_finished() && spawner.spawned_minion != MinionKind::None {
            warn!("should spawn {:?}!", entity);
            minion_configs
                .get(&spawner.spawned_minion)
                .unwrap_or_else(|| panic!("minion {:?} is not configured", spawner.spawned_minion))
                .spawn(&mut commands, level_config.path_points[0].clone());
        }
    }
}

#[derive(Debug, Component, Clone)]
pub(crate) struct SpawnerBuilding {
    pub spawner_kind: SpawnerKind,
    pub spawn_timer: Timer,
    pub spawned_minion: MinionKind,
}

impl SpawnerBuilding {
    pub fn init(position: Vec3, config: &SpawnerBuildingConfig) -> impl Bundle {
        (
            config.building.clone(),
            Sprite::from_atlas_image(
                config.sprite.clone(),
                TextureAtlas {
                    layout: config.animations.clone(),
                    index: config
                        .atlas_rows
                        .get(&Action::Idle)
                        .unwrap_or_else(|| {
                            panic!("Spawner {:?} has no idle action configured", config)
                        })
                        .row
                        .to_owned(),
                },
            ),
            Transform::from_translation(position),
            Pickable::default(),
        )
    }
}

#[derive(Debug, Default, Clone, Copy, serde::Deserialize, PartialEq, Eq, Hash)]
pub(crate) enum SpawnerKind {
    #[default]
    None,
}
