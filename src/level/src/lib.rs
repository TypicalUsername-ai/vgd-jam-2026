use bevy::{asset::io::embedded::GetAssetServer, prelude::*};
use bevy_common_assets::ron::RonAssetPlugin;
use state::{GlobalState, LevelState, PauseState};
mod ui_assets;

mod animation;
mod heroes;
mod level_map;
mod minions;
mod turrets;
use animation::Action;

use crate::{
    heroes::{HeroConfigHandles, HeroConfigKeys},
    level_map::{LevelConfigHandles, LevelMapConfig},
    minions::{MinionConfigHandles, MinionConfigKeys},
    turrets::{TurretConfigHandles, TurretConfigKeys},
};

pub struct LevelPlugin {}

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            //RonAssetPlugin::<LevelConfiguration>::new(&["level.ron"]),
            RonAssetPlugin::<HeroConfigKeys>::new(&["hero.ron"]),
            RonAssetPlugin::<MinionConfigKeys>::new(&["minion.ron"]),
            RonAssetPlugin::<TurretConfigKeys>::new(&["turret.ron"]),
            RonAssetPlugin::<LevelMapConfig>::new(&["level.ron"]),
        ));
        app.insert_resource(ui_assets::UiAssets::init(app.get_asset_server()));
        let animals = ["chicken", "cow", "llama", "pig", "sheep"];
        let turrets = ["basic"];
        let levels = ["tutorial", "demo_1"];
        app.insert_resource(HeroConfigHandles::load_heroes(
            &animals,
            app.get_asset_server(),
        ));
        app.insert_resource(MinionConfigHandles::load_minions(
            &animals,
            app.get_asset_server(),
        ));
        app.insert_resource(TurretConfigHandles::load_turrets(
            &turrets,
            app.get_asset_server(),
        ));
        app.insert_resource(LevelConfigHandles::load_levels(
            &levels,
            app.get_asset_server(),
        ));
        app.add_systems(
            OnEnter(LevelState::Pre),
            (
                level_map::setup_levels.run_if(not(resource_exists::<level_map::LevelConfigs>)),
                level_map::load_level,
                level_map::setup_background,
                level_map::setup_path,
                level_map::display_messages,
                heroes::setup_heroes.run_if(not(resource_exists::<heroes::HeroConfigs>)),
                turrets::setup_turrets.run_if(not(resource_exists::<turrets::TurretConfigs>)),
                minions::setup_minions.run_if(not(resource_exists::<minions::MinionConfigs>)),
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
        app.add_systems(OnEnter(LevelState::Won), level_map::draw_win_screen);
        app.add_systems(OnEnter(LevelState::Lost), level_map::draw_loss_screen);
    }
}
