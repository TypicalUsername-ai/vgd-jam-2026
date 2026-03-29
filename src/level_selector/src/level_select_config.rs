use crate::LevelConfiguration;
use ron;
use serde::Deserialize;
use std::{fs::File, io::Read, path::Path};

#[derive(Deserialize)]
pub struct LevelSelectConfig {
    levels: Vec<LevelConfiguration>,
}

impl From<&Path> for LevelSelectConfig {
    fn from(value: &Path) -> Self {
        let config = File::open(value);
        match config {
            Ok(mut config_file) => {
                let mut buf = String::new();
                let _ = config_file.read_to_string(&mut buf);
                let levels_config =
                    ron::from_str(&buf).expect("error parsing options file!! {value}");
                return levels_config;
            }
            Err(_err) => {
                panic!("error reading file!! {}", value.display())
            }
        }
    }
}
