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

pub enum MinionKind {
    Chicken,
}

pub enum MinionState {}
