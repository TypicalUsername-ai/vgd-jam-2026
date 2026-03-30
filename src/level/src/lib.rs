use bevy::prelude::*;
use state::GlobalState;

// pub mod level_map;
mod character_select;
mod characters;

pub struct CharacterSelectPlugin {}

#[derive(Debug, Component)]
pub struct LevelCamera {}

pub(crate) fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, LevelCamera {}));
}

impl Plugin for CharacterSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GlobalState::CharacterSelect),
            (
                setup_camera.before(characters::chicken::spawn_chicken), //character_select::ui::draw_character_select.before(characters::chicken::spawn_chicken),
                characters::chicken::spawn_chicken,
            ),
        );
        app.add_systems(
            OnExit(GlobalState::CharacterSelect),
            character_select::ui::clear_character_select,
        );
    }
}

pub struct LevelPlugin {}
