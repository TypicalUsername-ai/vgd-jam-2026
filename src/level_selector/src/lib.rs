use bevy::prelude::*;
use state::GlobalState;
mod ui;

pub struct MainMenuPlugin {}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GlobalState::LevelSelect), ui::draw_level_select);
    }
}
