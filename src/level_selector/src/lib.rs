use bevy::prelude::*;
use state::GlobalState;
use std::path::Path;
mod level_config;
mod level_select_config;
mod ui;

pub use level_config::LevelConfiguration;
pub use level_select_config::LevelSelectConfig;

use crate::ui::react_buttons;

pub struct LevelSelectPlugin {
    config: LevelSelectConfig,
}

impl Plugin for LevelSelectPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.config.clone());
        app.add_systems(OnEnter(GlobalState::LevelSelect), ui::draw_level_select);
        app.add_systems(OnExit(GlobalState::LevelSelect), ui::clear_level_select);
        app.add_systems(
            Update,
            react_buttons.run_if(in_state(GlobalState::LevelSelect)),
        );
    }
}

impl From<&Path> for LevelSelectPlugin {
    fn from(value: &Path) -> Self {
        Self {
            config: LevelSelectConfig::from(value),
        }
    }
}
