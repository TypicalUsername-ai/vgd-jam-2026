use crate::{LevelConfiguration, level_config::SaveGameState};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use state::ConfigFileLocation;
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

#[derive(Deserialize, Serialize, Asset, Clone, TypePath, Resource, Deref)]
pub struct SaveSelectConfig {
    pub(crate) saves: Vec<SaveGameState>, // TODO move to mage saves
}

#[derive(Debug, Resource)]
pub(crate) struct SaveSelectConfigHandle(pub Handle<SaveSelectConfig>);

/*
impl From<&Path> for SaveSelectConfig {
    fn from(value: &Path) -> Self {
        let config = File::open(value);
        match config {
            Ok(mut config_file) => {
                let mut buf = String::new();
                let _ = config_file.read_to_string(&mut buf);
                ron::from_str(&buf).unwrap_or_else(|e| {
                    panic!("error parsing options file {:?} with error {}", value, e)
                })
            }
            Err(err) => {
                panic!("error reading file!! {}\n{}", value.display(), err)
            }
        }
    }
}
*/

/*
pub fn sync_to_save_file(config_location: Res<ConfigFileLocation>, save_config: Res<LoadedSaves>) {
    let data = ron::to_string(save_config).expect("save config should be serializeable");
    std::fs::write(
        config_location.join("saves-config").with_extension("ron"),
        data,
    )
    .expect("writing to file should work");
}
*/
