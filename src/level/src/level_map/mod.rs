mod controls;
mod hero_modifiers;
mod hero_slots;
mod map_background;
mod map_config;
mod messages;
mod path;
mod turret_point;
use bevy::prelude::*;
use level_selector::SaveGameState;

pub(crate) use controls::{
    draw_loss_screen, draw_win_screen, ingame_pause, setup_controls, spawn_heroes,
};
pub(crate) use hero_slots::setup_hero_slots;
pub(crate) use hero_slots::{HeroSlot, HpTracker, TracksHpFor, roll_upgrades};
pub(crate) use map_background::setup_background;
pub(crate) use map_config::{LevelConfigHandles, LevelConfigs, LevelMapConfig, setup_levels};
pub(crate) use messages::display_messages;
pub(crate) use path::setup_path;
use turret_point::TurretPoint;
pub(crate) use turret_point::setup_turrets;

/// Loads the level based on current [LevelState] resource
/// requires [Commands] for inserting a [LevelMapConfig] resource
pub(crate) fn load_level(
    mut commands: Commands,
    save: Res<SaveGameState>,
    configs: Res<LevelConfigs>,
) {
    let current_level = configs
        .get(&save.current_level_id)
        .expect("all levels should be loaded");
    commands.insert_resource(current_level.to_owned());
}
