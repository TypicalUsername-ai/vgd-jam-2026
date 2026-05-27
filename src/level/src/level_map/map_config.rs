use crate::heroes::HeroKind;

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
    /// upgrade points available to spend
    pub upgrade_points: u8,
    /// next level to load
    pub next_level_id: Option<String>, // TODO
    /// background color for level
    pub bg_color: Color,
    /// background sprite for level backdrop
    pub bg_image: Option<String>,
    /// conversations and tips to be displayed before a level
    pub messages: Vec<String>,
    /// available slots for heroes
    pub hero_slots: u8,
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
