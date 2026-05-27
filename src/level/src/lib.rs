use bevy::{asset::io::embedded::GetAssetServer, prelude::*};
use state::{GlobalState, LevelState, PauseState};
use std::path::PathBuf;
mod ui_assets;

mod animation;
mod heroes;
mod level_map;
mod minions;
mod turrets;
use animation::Action;

use crate::level_map::LevelMapConfig;

pub struct LevelPlugin {
    hero_configs: Vec<PathBuf>,
    turret_configs: Vec<PathBuf>,
    minion_configs: Vec<PathBuf>,
}

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ui_assets::UiAssets::init(app.get_asset_server()));
        app.insert_resource(heroes::HeroConfigs::init(
            &self.hero_configs,
            app.get_asset_server(),
        ));
        app.insert_resource(turrets::TurretConfigs::init(
            &self.turret_configs,
            app.get_asset_server(),
        ));
        app.insert_resource(minions::MinionConfigs::init(
            &self.minion_configs,
            app.get_asset_server(),
        ));
        app.add_systems(
            OnEnter(LevelState::Pre),
            (
                level_map::load_level,
                level_map::setup_background,
                level_map::setup_path,
                level_map::display_messages,
            )
                .run_if(in_state(GlobalState::ActiveLevel))
                .chain(),
        );
        app.add_systems(
            OnEnter(LevelState::Setup),
            (
                level_map::setup_turrets,
                level_map::setup_hero_slots,
                level_map::roll_upgrades,
                level_map::setup_controls,
            )
                .chain(),
        );
        app.add_systems(
            OnEnter(PauseState::Paused),
            level_map::ingame_pause.run_if(in_state(GlobalState::ActiveLevel)),
        );
        app.add_systems(
            OnEnter(LevelState::Active),
            (level_map::spawn_heroes).chain(),
        );
        app.add_systems(
            Update,
            (
                turrets::fire_turrets,
                minions::move_minions,
                animation::animate_all,
            )
                .chain()
                .run_if(in_state(LevelState::Active).and(in_state(PauseState::Running))),
        );
        app.add_systems(OnEnter(LevelState::Won), (level_map::draw_win_screen));
        app.add_systems(OnEnter(LevelState::Lost), (level_map::draw_loss_screen));
    }
}

impl LevelPlugin {
    #[must_use]
    pub fn new(
        configs_path: PathBuf,
        //hero_configs_path: PathBuf,
        //turret_configs_path: PathBuf,
        //minion_configs_path: PathBuf,
    ) -> Self {
        Self {
            hero_configs: read_configs_dir(configs_path.join("heroes")),
            turret_configs: read_configs_dir(configs_path.join("turrets")),
            minion_configs: read_configs_dir(configs_path.join("minions")),
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
