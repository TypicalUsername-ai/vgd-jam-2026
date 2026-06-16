use crate::minions::MinionKind;
use bevy::prelude::*;

#[derive(Debug, Component, Clone)]
pub(crate) struct ActiveHero {
    pub spawner_kind: HeroKind,
    pub spawned_minion: MinionKind,
    pub applied_upgrades: Vec<super::UpgradeChoice>,
}

#[derive(Debug, Clone, Copy, serde::Deserialize, PartialEq, Eq, Hash)]
pub(crate) enum HeroKind {
    Chicken,
    Llama,
    Pig,
    Sheep,
    Cow,
}
