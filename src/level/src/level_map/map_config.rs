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
        // all distance we need to compute
        let mut remaining = distance_covered + speed;
        let mut current_point = 1;
        while remaining >= f32::EPSILON && current_point < self.path_points.len() {
            let seg_start = self.path_points[current_point - 1];
            let seg_end = self.path_points[current_point];
            let distance = seg_start.distance(seg_end);
            match remaining
                .partial_cmp(&distance)
                .expect("No value should be a NaN")
            {
                std::cmp::Ordering::Less => return seg_start.move_towards(seg_end, remaining),
                std::cmp::Ordering::Equal => return seg_end,
                std::cmp::Ordering::Greater => {
                    info!("skipping one segment!");
                    current_point += 1;
                    remaining -= distance;
                }
            }
        }
        panic!("Movement logic failed");
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
