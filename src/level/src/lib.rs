use crate::level_map::LevelMapConfig;
use bevy::color::palettes::basic;
use bevy::{camera::visibility::RenderLayers, prelude::*};
use state::GlobalState;

mod characters;
mod level_map;
mod tiles;

pub struct CharacterSelectPlugin {}

#[derive(Debug, Component)]
pub struct LevelCamera {}

const DEFAULT_LAYERS: [usize; 6] = [0, 1, 2, 3, 4, 5];

pub(crate) fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        RenderLayers::from_layers(&DEFAULT_LAYERS),
        LevelCamera {},
    ));
}

impl Plugin for CharacterSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(level_map::MapPlugin {
            maps_directory: "../assets".into(),
        });
        app.add_systems(
            OnEnter(GlobalState::ActiveLevel),
            (
                //setup_camera.before(characters::chicken::spawn_chicken), //character_select::ui::draw_character_select.before(characters::chicken::spawn_chicken),
                characters::chicken::spawn_chicken,
            ),
        );
        app.add_systems(Update, (characters::chicken::animate_chicken).chain());
        //app.add_systems(OnExit(GlobalState::ActiveLevel), ());
    }
}

pub struct LevelPlugin {}
