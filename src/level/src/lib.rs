use bevy::{asset::io::embedded::GetAssetServer, camera::visibility::RenderLayers, prelude::*};
use state::GlobalState;
use std::path::PathBuf;

mod animation;
mod buildings;
mod level_map;
mod minions;
use animation::{Action, AnimationState};
use level_map::LevelMapConfig;

pub struct LevelPlugin {
    spawner_configs: Vec<PathBuf>,
    turret_configs: Vec<PathBuf>,
    minion_configs: Vec<PathBuf>,
}

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

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(buildings::SpawnerConfigs::init(
            &self.spawner_configs,
            app.get_asset_server(),
        ));
        app.insert_resource(buildings::TurretConfigs::init(
            &self.turret_configs,
            app.get_asset_server(),
        ));
        app.insert_resource(minions::MinionConfigs::init(
            &self.minion_configs,
            app.get_asset_server(),
        ));
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
                buildings::fire_turrets,
                buildings::spawn_minions,
                //minions::handle_damage, // or move handle damage to turret firing action
                minions::move_minions,
                animation::animate_all,
            )
                .chain()
                .run_if(in_state(GlobalState::ActiveLevel).and(resource_exists::<LevelMapConfig>)),
        );
    }
}

impl LevelPlugin {
    #[must_use]
    pub fn new(
        spawner_configs_path: PathBuf,
        turret_configs_path: PathBuf,
        minion_configs_path: PathBuf,
    ) -> Self {
        Self {
            spawner_configs: read_configs_dir(spawner_configs_path),
            turret_configs: read_configs_dir(turret_configs_path),
            minion_configs: read_configs_dir(minion_configs_path),
        }
    }
}

fn read_configs_dir(dir: PathBuf) -> Vec<PathBuf> {
    dir.read_dir()
        .expect("configs path is a directory")
        .filter_map(|e| {
            // dont need to check for exists as we enumerate a directory
            if let Ok(f) = e
                && f.path().extension().is_some_and(|e| e == "ron")
            {
                Some(f.path())
            } else {
                // Dir entry is invalid
                None
            }
        })
        .collect()
}
