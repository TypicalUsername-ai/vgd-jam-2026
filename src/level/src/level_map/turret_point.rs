use bevy::prelude::*;
use serde::Deserialize;

use super::LevelMapConfig;

#[derive(Debug, Deserialize)]
pub(crate) struct TurretPoint {
    pub position: Vec3,
}

pub(crate) fn setup_turrets(mut commands: Commands, level_config: Res<LevelMapConfig>) {
    todo!()
}
