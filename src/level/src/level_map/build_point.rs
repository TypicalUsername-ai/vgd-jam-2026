use bevy::prelude::*;
use serde::Deserialize;

use crate::level_map::map_config::LevelMapConfig;

#[derive(Debug, Deserialize)]
pub(crate) struct BuildPoint {
    pub position: Vec3,
}

pub(crate) fn setup_build_points(mut commands: Commands, level_config: Res<LevelMapConfig>) {
    todo!();
}
