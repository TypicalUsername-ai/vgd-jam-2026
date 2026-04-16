use bevy::{camera::visibility::RenderLayers, prelude::*};
use state::GlobalState;

mod buildings;
mod level_map;
mod minions;

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
        //app.init_resource::<MinionHandles>();
        app.insert_resource(buildings::SpawnerConfigs::init());
        app.insert_resource(buildings::TurretConfigs::init());
        app.add_systems(
            OnEnter(GlobalState::ActiveLevel),
            (
                level_map::load_level,
                level_map::setup_background,
                level_map::setup_turrets,
                level_map::setup_build_points,
            )
                .chain(),
        );
        app.add_systems(
            Update,
            (
                buildings::animate_turrets,
                buildings::animate_spawners,
                buildings::fire_turrets,
                buildings::spawn_minions,
                minions::handle_damage,
                minions::animate_minions,
                minions::move_minions,
            )
                .chain(),
        );
    }
}

pub struct LevelPlugin {}
