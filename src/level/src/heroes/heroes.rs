use crate::animation::AnimationState;
use bevy::prelude::*;

use super::hero_config::HeroConfig;
use crate::{animation::Action, minions::MinionKind};

#[derive(Debug, Component, Clone)]
pub(crate) struct ActiveHero {
    pub spawner_kind: HeroKind,
    pub spawn_timer: Timer,
    pub spawned_minion: MinionKind,
    pub applied_upgrades: Vec<super::UpgradeChoice>,
}

impl ActiveHero {
    pub fn init(position: Vec3, config: &HeroConfig) -> impl Bundle {
        (
            config.hero.clone(),
            Sprite::from_atlas_image(
                config.sprite.clone(),
                TextureAtlas {
                    layout: config.animations.clone(),
                    index: 0,
                },
            ),
            AnimationState::new(
                0.25,
                Action::Idle,
                config
                    .atlas_rows
                    .get(&Action::Idle)
                    .expect("Animations are configured")
                    .clone(),
            ),
            Transform::from_translation(position),
            Pickable::default(),
        )
    }
}

#[derive(Debug, Clone, Copy, serde::Deserialize, PartialEq, Eq, Hash)]
pub(crate) enum HeroKind {
    Chicken,
    Llama,
    Pig,
    Sheep,
    Cow,
}
