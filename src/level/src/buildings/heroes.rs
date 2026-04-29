use crate::level_map::HeroSlot;
use bevy::prelude::*;

use super::hero_config::HeroConfig;
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
    mut query: Query<(&HeroSlot)>,
) {
    for slot in query.iter() {
        if let Some(hero) = &slot.hero {
            // progress time for each spawner
            minion_configs
                .get(&hero.spawned_minion)
                .unwrap_or_else(|| panic!("minion {:?} is not configured", hero.spawned_minion))
                .spawn(&mut commands, level_config.path_points[0].clone());
        }
    }
}

#[derive(Debug, Component, Clone)]
pub(crate) struct ActiveHero {
    pub spawner_kind: HeroKind,
    pub spawn_timer: Timer,
    pub spawned_minion: MinionKind,
}

impl ActiveHero {
    pub fn init(position: Vec3, config: &HeroConfig) -> impl Bundle {
        (
            config.hero.clone(),
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

#[derive(Debug, Clone, Copy, serde::Deserialize, PartialEq, Eq, Hash)]
pub(crate) enum HeroKind {
    Chicken,
}
