use bevy::prelude::*;
use std::sync::Arc;

#[derive(Debug, Component)]
pub(crate) struct AnimationState {
    pub animation_timer: Timer,
    current_frame: usize,
    color: Color,
    pub action: Action,
    // needs current atlas position etc
}

#[derive(Debug)]
pub enum Action {}
