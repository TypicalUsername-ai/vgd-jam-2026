use bevy::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct TowerPoint {
    pub position: Vec3,
}
