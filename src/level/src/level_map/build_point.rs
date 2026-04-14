use bevy::prelude::*;
use serde::Deserialize;

use crate::level_map::map_config::LevelMapConfig;

/// a single point which can hold a single spawner
#[derive(Debug, Deserialize)]
pub(crate) struct BuildPoint {
    /// the center position of the point
    /// CAUTION! even though its 2D be mindful of the Z index location
    pub position: Vec3,
}

/// spawns marker entities for which locations will be able to have spawners built on
pub(crate) fn setup_build_points(mut commands: Commands, level_config: Res<LevelMapConfig>) {
    todo!();
}
