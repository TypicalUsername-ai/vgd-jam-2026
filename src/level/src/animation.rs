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

#[derive(Debug, serde::Deserialize, Hash, PartialEq, Eq)]
pub enum Action {}

pub(crate) fn animate_all(mut query: Query<(&mut AnimationState, &mut Sprite)>, time: Res<Time>) {
    for (mut anim, mut sprite) in query.iter_mut() {
        anim.animation_timer.tick(time.delta());
        if anim.animation_timer.just_finished() {
            anim.next_frame(&mut sprite);
        }
    }
}
