use super::BuildPoint;
use super::TurretPoint;
use bevy::prelude::*;
use serde::Deserialize;
use std::{fs::File, io::Read, path::Path};

/// used to configure the initial map state and resource locations for a given level
/// handles necessary components for each level:
#[derive(Debug, Deserialize, Resource)]
pub(crate) struct LevelMapConfig {
    /// set of points which connected make the path minions have to walk
    pub path_points: Vec<Vec3>,
    /// holds all [BuildPoint] locations
    pub spawner_points: Vec<BuildPoint>,
    /// hold locations of all [TowerPoint]s
    pub tower_points: Vec<TurretPoint>,
}

impl From<&Path> for LevelMapConfig {
    #[must_use = "Load configs from .ron files!"]
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
