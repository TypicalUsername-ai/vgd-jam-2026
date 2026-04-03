use bevy::prelude::*;

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq)]
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

const FACE_XY_VALUES: [[Facing; 2]; 2] =
    [[Facing::Left, Facing::Right], [Facing::Down, Facing::Up]];

impl From<Vec3> for Facing {
    fn from(value: Vec3) -> Self {
        let mpos = value.abs().max_position();
        // 0 for false, 1 for true
        let ineg = value.to_array()[mpos].is_sign_positive() as usize;
        // info!("{} | mpos {}, ineg {}", value.normalize(), mpos, ineg);
        FACE_XY_VALUES[mpos][ineg]
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
