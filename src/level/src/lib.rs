use bevy::prelude::*;
use state::GlobalState;

// pub mod level_map;
mod character_select;

pub struct CharacterSelectPlugin {}

impl Plugin for CharacterSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GlobalState::CharacterSelect),
            character_select::ui::draw_character_select,
        );
        app.add_systems(
            OnExit(GlobalState::CharacterSelect),
            character_select::ui::clear_character_select,
        );
    }
}

pub struct LevelPlugin {}
