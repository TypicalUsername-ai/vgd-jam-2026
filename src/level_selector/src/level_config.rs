use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct LevelConfiguration {
    pub(crate) name: String,
    pub(crate) id: String,
}
