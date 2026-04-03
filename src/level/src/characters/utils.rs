use bevy::prelude::*;

#[derive(Debug, Component, Clone, Copy)]
pub(crate) enum Facing {
    Up,
    Left,
    Down,
    Right,
}

impl From<Facing> for usize {
    fn from(value: Facing) -> Self {
        match value {
            Facing::Up => 0,
            Facing::Left => 1,
            Facing::Down => 2,
            Facing::Right => 3,
        }
    }
}

#[derive(Debug, Component, Deref, DerefMut)]
pub(crate) struct AnimationTime(pub Timer);

#[derive(Debug, Component)]
pub(crate) struct AnimationState {
    pub facing: Facing,
    pub moving: bool,
    pub was_moving: bool,
}
