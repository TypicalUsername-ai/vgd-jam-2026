use bevy::prelude::*;
use std::path::PathBuf;
mod map_config;
mod path;
use crate::{
    level_map::map_config::LevelMapConfig,
    level_map::path::walk_mobs,
};
use state::{GlobalState, LevelState};

pub(crate) use path::PathWalker;

pub(crate) struct MapPlugin {
    pub(crate) maps_directory: PathBuf,
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GlobalState::ActiveLevel), load_level);
        app.add_systems(Update, walk_mobs.run_if(in_state(GlobalState::ActiveLevel)));
    }
}

fn load_level(mut commands: Commands, current_level: Res<State<LevelState>>) {
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
