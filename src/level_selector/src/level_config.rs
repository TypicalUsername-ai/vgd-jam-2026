use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct LevelConfiguration {
    pub(crate) name: String,
    pub(crate) id: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SaveGameState {
    pub(crate) save_name: String,
    pub(crate) current_level_id: String,
}
