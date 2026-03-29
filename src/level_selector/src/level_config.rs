use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LevelConfiguration {
    name: String,
}
