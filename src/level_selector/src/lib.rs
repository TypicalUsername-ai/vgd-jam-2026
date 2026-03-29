use bevy::prelude::*;
use state::GlobalState;
use std::path::Path;
mod level_config;
mod level_select_config;
mod ui;

pub use level_config::LevelConfiguration;
pub use level_select_config::LevelSelectConfig;

pub struct LevelSelectPlugin {
    config: LevelSelectConfig,
}

impl Plugin for LevelSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GlobalState::LevelSelect), ui::draw_level_select);
    }
}

impl From<&Path> for LevelSelectPlugin {
    fn from(value: &Path) -> Self {
        Self {
            config: LevelSelectConfig::from(value),
        }
    }
}
