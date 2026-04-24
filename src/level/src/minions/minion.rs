//use super::Action;
use bevy::prelude::*;

#[derive(Component)]
pub struct Minion {
    kind: MinionKind,
    state: MinionState,
    pub health: f32,
    pub speed: f32,
    pub distance_traveled: f32,
    // to animation state sprite_range: (usize, usize),
    // same ^^ sprite_index: usize,
}

impl Minion {}

#[derive(Debug, serde::Deserialize, Hash, PartialEq, Eq, Clone, Copy)]
pub(crate) enum MinionKind {
    None,
    Chicken,
}

pub enum MinionState {}
