use crate::buildings::HeroKind;

use super::TurretPoint;
use bevy::prelude::*;
use serde::Deserialize;
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

/// used to configure the initial map state and resource locations for a given level
/// handles necessary components for each level:
#[derive(Debug, Deserialize, Resource)]
pub(crate) struct LevelMapConfig {
    /// set of points which connected make the path minions have to walk
    pub path_points: Vec<Vec3>,
    /// hold locations of all [TowerPoint]s
    pub tower_points: Vec<TurretPoint>,
    /// all [HeroKind] available for a given level
    pub available_heroes: Vec<HeroKind>,
    /// background color for level
    pub bg_color: Color,
    /// background sprite for level backdrop
    pub bg_image: Option<PathBuf>,
}

impl LevelMapConfig {
    pub fn compute_next(&self, speed: f32, distance_covered: f32) -> Vec3 {
        let mut remaining = distance_covered;
        self.path_points
            .iter()
            .fold(self.path_points[0], |pos, item| {
                let dist_to_next = item.distance(pos);
                if dist_to_next > remaining + dist_to_next {
                    pos.move_towards(item.to_owned(), remaining + dist_to_next)
                } else {
                    remaining -= dist_to_next;
                    item.to_owned()
                }
            })
        // find segment
        // move to +1
    }
}

/// Load configs from .ron files
impl From<&Path> for LevelMapConfig {
    fn from(value: &Path) -> Self {
        let config = File::open(value);
        match config {
            Ok(mut config_file) => {
                let mut buf = String::new();
                config_file
                    .read_to_string(&mut buf)
                    .expect("Unexpected IO error");
                ron::from_str(&buf).unwrap_or_else(|e| {
                    panic!("error parsing options file {}! {}", value.display(), e)
                })
            }
            Err(_err) => {
                panic!("error reading file!! {}", value.display())
            }
        }
    }
}
