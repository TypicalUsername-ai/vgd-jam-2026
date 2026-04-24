use crate::LevelConfiguration;
use bevy::prelude::*;
use serde::Deserialize;
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

#[derive(Deserialize, Resource, Clone)]
pub struct LevelSelectConfig {
    pub(crate) map_config_folder: PathBuf,
    pub(crate) levels: Vec<LevelConfiguration>,
}

impl From<&Path> for LevelSelectConfig {
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
            Err(_err) => {
                panic!("error reading file!! {}", value.display())
            }
        }
    }
}
