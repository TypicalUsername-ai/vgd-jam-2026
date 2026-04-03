use crate::GlobalState;
use bevy::prelude::*;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

#[derive(SubStates, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[source(GlobalState = GlobalState::ActiveLevel)]
pub enum LevelState {
    #[default]
    Pre,
    Active {
        id: String,
        map_config_path: PathBuf,
    },
    Post,
}

impl LevelState {
    pub fn load(id: String, config_folder: PathBuf) -> Self {
        let mut map_config_path = config_folder.join(id.clone());
        map_config_path.add_extension("ron");
        Self::Active {
            id,
            map_config_path, //       map_config: LevelMapConfig::from(format!("../assets/maps/{}.ron", value)),
        }
    }
}
