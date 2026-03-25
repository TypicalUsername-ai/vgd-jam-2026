use bevy::prelude::*;
use state::GlobalState;
mod ui;

pub struct LevelSelectPlugin {}

impl Plugin for LevelSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GlobalState::LevelSelect), ui::draw_level_select);
    }
}
