use bevy::prelude::*;

#[derive(Debug, Component)]
pub(crate) struct AnimationState {
    pub animation_timer: Timer,
    current_frame: usize,
    color: Color,
    pub action: Action,
    // needs current atlas position etc
}

impl AnimationState {
    pub fn next_frame(&mut self, sprite: &mut Sprite) {
        todo!();
    }
}

#[derive(Debug, Default, serde::Deserialize, Hash, PartialEq, Eq)]
pub enum Action {
    #[default]
    Idle,
    WalkUp,
    WalkDown,
    WalkLeft,
    WalkRight,
}

pub(crate) fn animate_all(mut query: Query<(&mut AnimationState, &mut Sprite)>, time: Res<Time>) {
    for (mut anim, mut sprite) in query.iter_mut() {
        anim.animation_timer.tick(time.delta());
        if anim.animation_timer.just_finished() {
            anim.next_frame(&mut sprite);
        }
    }
}

#[derive(Debug)]
pub struct ActionLocation {
    pub row: usize,
    pub len: usize,
}

impl ActionLocation {
    #[must_use]
    pub fn new(row: usize, len: usize) -> Self {
        Self { row, len }
    }
}
