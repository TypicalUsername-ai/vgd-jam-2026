mod build_point;
mod map_background;
mod map_config;
mod turret_point;
use bevy::prelude::*;
use state::{GlobalState, LevelState};

use build_point::BuildPoint;
pub(crate) use build_point::setup_build_points;
pub(crate) use map_background::setup_background;
use map_config::LevelMapConfig;
use turret_point::TurretPoint;
pub(crate) use turret_point::setup_turrets;

/// Loads the level based on current [LevelState] resource
/// requires [Commands] for inserting a [LevelMapConfig] resource
pub(crate) fn load_level(mut commands: Commands, current_level: Res<State<LevelState>>) {
    match &**current_level {
        LevelState::Active {
            id: _,
            map_config_path,
        } => {
            let map_conf = map_config::LevelMapConfig::from(map_config_path.as_path());
            commands.insert_resource(map_conf);
        }
        _ => todo!(),
    };
}
