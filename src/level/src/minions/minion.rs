//use super::Action;
use bevy::prelude::*;

#[derive(Debug, Component, Clone)]
pub struct Minion {
    pub kind: MinionKind,
    pub health: f32,
    pub speed: f32,
    pub distance_traveled: f32,
    pub target_index: usize,
    // to animation state sprite_range: (usize, usize),
    // same ^^ sprite_index: usize,
}

impl Minion {}

#[derive(Debug, serde::Deserialize, Hash, PartialEq, Eq, Clone, Copy)]
pub(crate) enum MinionKind {
    Chicken,
    Llama,
    Pig,
    Sheep,
    Cow,
}
