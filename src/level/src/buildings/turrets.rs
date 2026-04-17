use bevy::prelude::*;

/// animates all of the currently spawned turrets
pub(crate) fn animate_turrets() {
    todo!()
}

/// responsible for andling target acquisition and firing of the spawned turrets
pub(crate) fn fire_turrets() {
    todo!()
}

#[derive(Debug, Component)]
pub(crate) struct Turret {
    pub shot_timer: Timer,
    pub damage: f32,
    pub range: f32,
    pub shoot_function: fn(cmds: &mut Commands, target: Entity),
}

#[derive(Debug, Default, serde::Deserialize, Hash, PartialEq, Eq)]
pub(crate) enum TurretAnimation {
    #[default]
    Idle,
    Shooting,
}

#[derive(Debug, Component)]
pub(crate) struct TurretAnimationState {
    pub animation_timer: Timer,
    current_frame: usize,
    color: Color,
    pub action: TurretAnimation,
    // needs current atlas position etc
}
