use bevy::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct SpawnerPoint {
    pub position: Vec3,
}
