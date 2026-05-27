pub use bevy::prelude::*;
use std::path::PathBuf;

#[derive(Debug, Resource, Deref)]
pub struct ConfigFileLocation(pub PathBuf);
