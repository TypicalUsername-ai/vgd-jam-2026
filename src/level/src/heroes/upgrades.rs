use crate::minions::{Minion, MinionConfig};

use super::ActiveHero;
use bevy::prelude::*;
use rand;

#[derive(Debug, Clone)]
pub(crate) struct HeroUpgrade {
    pub kind: UpgradeKind,
    pub value_modifier: f32,
    pub level: u8,
}

impl HeroUpgrade {
    #[must_use]
    pub fn new(kind: UpgradeKind, level: u8) -> Self {
        let modifier_range = match level {
            1 => 1.1..=1.3,
            2 => 1.3..=1.5,
            3 => 1.5..=1.7,
            _ => unimplemented!(),
        };
        Self {
            kind,
            value_modifier: rand::random_range(modifier_range),
            level,
        }
    }

    pub fn apply(&self, minion: &mut Minion) {
        match self.kind {
            UpgradeKind::Speed => minion.speed *= self.value_modifier,
            UpgradeKind::Health => {
                minion.health *= self.value_modifier;
                minion.max_health = minion.health;
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum UpgradeKind {
    Speed,
    Health,
}
