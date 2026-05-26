use std::{collections::HashSet, default};

use crate::minions::{Minion, MinionConfig};

use super::ActiveHero;
use bevy::prelude::*;
use rand;

#[derive(Debug, Clone, Copy, Component, PartialEq, Hash)]
pub struct UpgradeChoice {
    pub kind: UpgradeKind,
    pub bonus: u16,
    pub level: u8,
}

impl std::cmp::Eq for UpgradeChoice {}

impl UpgradeChoice {
    pub fn roll(kind: UpgradeKind, level: u8) -> Self {
        let modifier_range = match level {
            1 => 10..=30,
            2 => 30..=50,
            3 => 50..=70,
            _ => unimplemented!(),
        };
        Self {
            kind,
            bonus: rand::random_range(modifier_range),
            level,
        }
    }

    pub fn apply(&self, minion: &mut Minion) {
        let mult = 1. + 0.1 * self.bonus as f32;
        match self.kind {
            UpgradeKind::Speed => minion.speed *= mult,
            UpgradeKind::Health => {
                minion.health *= mult;
                minion.max_health = minion.health;
            }
        }
    }
}

#[derive(Debug, Resource, Deref, DerefMut)]
pub(crate) struct AvailableUpgrades(pub HashSet<UpgradeChoice>);

#[derive(Debug, Clone, Component)]
#[relationship(relationship_target = AppliedUpgrades)]
pub(crate) struct HeroUpgrade {
    #[relationship]
    pub applied_to: Entity,
    pub kind: UpgradeKind,
    pub value_modifier: f32,
}

#[derive(Debug, Component)]
#[relationship_target(relationship = HeroUpgrade)]
pub(crate) struct AppliedUpgrades(Vec<Entity>);

impl HeroUpgrade {
    #[must_use]
    pub fn new(hero: Entity, upgrade: UpgradeChoice) -> Self {
        Self {
            applied_to: hero,
            kind: upgrade.kind,
            value_modifier: 0.1 * upgrade.bonus as f32 + 1.,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub(crate) enum UpgradeKind {
    #[default]
    Health,
    Speed,
}

#[derive(Debug, Resource, Deref, DerefMut)]
pub(crate) struct UpgradePoints(pub u8);
