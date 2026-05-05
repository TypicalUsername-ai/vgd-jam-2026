use crate::buildings::{Turret, TurretConfigs};
use bevy::prelude::*;

#[derive(Debug, Component)]
pub(crate) struct AnimationState {
    pub animation_timer: Timer,
    current_frame: std::iter::Cycle<ActionLocation>,
    color: Color,
    pub action: Action,
    // needs current atlas position etc
}

impl AnimationState {
    pub fn next_frame(&mut self, sprite: &mut Sprite) {
        sprite
            .texture_atlas
            .as_mut()
            .expect("all animated sprites should have texture atlas!")
            .index = self
            .current_frame
            .next()
            .expect("Animation iterator is cyclical");
    }

    pub fn new(timer_secs: f32, action: Action, loc: ActionLocation) -> Self {
        Self {
            animation_timer: Timer::from_seconds(timer_secs, TimerMode::Repeating),
            current_frame: loc.cycle(),
            color: Color::WHITE,
            action,
        }
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

pub type ActionLocation = std::ops::Range<usize>;
