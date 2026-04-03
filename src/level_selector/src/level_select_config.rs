use crate::LevelConfiguration;
use bevy::prelude::*;
use ron;
use serde::Deserialize;
use std::{fs::File, io::Read, path::Path};

#[derive(Deserialize, Resource, Clone, Deref)]
pub struct LevelSelectConfig {
    pub(crate) levels: Vec<LevelConfiguration>,
}

impl From<&Path> for LevelSelectConfig {
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
