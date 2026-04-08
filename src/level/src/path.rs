use bevy::input::mouse::MouseButton;
use bevy::prelude::*;
use tracing::{info, warn};

#[derive(Component)]
pub struct PathMarker;

pub fn place_marker(
    input: Res<ButtonInput<MouseButton>>,
    mut markers: Single<&mut Transform, With<PathMarker>>,
) {
    if input.pressed(MouseButton::Left) {
        warn!("LMB pressed");
    } else if input.pressed(MouseButton::Right) {
        warn!("RMB pressed");
    }
    info!("this trigger ran fully");
}
