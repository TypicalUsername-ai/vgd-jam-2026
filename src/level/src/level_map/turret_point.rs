use bevy::prelude::*;
use serde::Deserialize;

use super::LevelMapConfig;

/// a single point which can holds a specified turret
#[derive(Debug, Deserialize)]
pub(crate) struct TurretPoint {
    /// the center position of the point
    pub position: Vec3,
}

/// spawns all of the turrets at the locations specified in [LevelMapConfig]
pub(crate) fn setup_turrets(mut commands: Commands, level_config: Res<LevelMapConfig>) {
    todo!()
}
