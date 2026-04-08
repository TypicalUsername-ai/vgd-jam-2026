use super::SpawnerPoint;
use super::TowerPoint;
use bevy::prelude::*;
use serde::Deserialize;
use std::{fs::File, io::Read, path::Path};

#[derive(Debug, Deserialize, Resource)]
pub(crate) struct LevelMapConfig {
    pub path_points: Vec<Vec3>,
    pub spawner_points: Vec<SpawnerPoint>,
    pub tower_points: Vec<TowerPoint>,
}

impl From<&Path> for LevelMapConfig {
    fn from(value: &Path) -> Self {
        let config = File::open(value);
        match config {
            Ok(mut config_file) => {
                let mut buf = String::new();
                let _ = config_file.read_to_string(&mut buf);
                ron::from_str(&buf).expect("error parsing options file!! {value}")
            }
            Err(_err) => {
                panic!("error reading file!! {}", value.display())
            }
        }
    }
}
