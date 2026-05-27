use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct LevelConfiguration {
    pub(crate) name: String,
    pub(crate) id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Resource)]
pub struct SaveGameState {
    pub(crate) save_name: String,
    pub current_level_id: String,
}
