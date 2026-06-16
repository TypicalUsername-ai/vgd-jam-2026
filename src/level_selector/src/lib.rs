use bevy::{asset::io::embedded::GetAssetServer, prelude::*};
use state::GlobalState;
mod level_config;
mod level_select_config;
mod ui;

pub use level_config::{LevelConfiguration, SaveGameState};
pub use level_select_config::SaveSelectConfig;

use crate::{level_select_config::SaveSelectConfigHandle, ui::react_buttons};
use bevy_common_assets::ron::RonAssetPlugin;

pub struct LevelSelectPlugin {
    //config: SaveSelectConfig,
}

impl Plugin for LevelSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            //RonAssetPlugin::<LevelConfiguration>::new(&["level.ron"]),
            RonAssetPlugin::<SaveSelectConfig>::new(&["save.ron"]),
        ));
        app.insert_resource(SaveSelectConfigHandle(
            app.get_asset_server()
                .load::<SaveSelectConfig>("saves-config.save.ron"),
        ));
        //app.insert_resource(self.config.clone());
        app.add_systems(
            OnEnter(GlobalState::LevelSelect),
            (load_configs, ui::draw_level_select).chain(),
        );
        app.add_systems(OnExit(GlobalState::LevelSelect), ui::clear_level_select);
        app.add_systems(
            Update,
            react_buttons.run_if(in_state(GlobalState::LevelSelect)),
        );
    }
}

fn load_configs(
    mut commands: Commands,
    mut save_assets: ResMut<Assets<SaveSelectConfig>>,
    config_handle: Res<SaveSelectConfigHandle>,
) {
    let data = save_assets
        .remove(config_handle.0.id())
        .expect("Saves config should be loaded");
    commands.insert_resource(data);
}

/*
impl From<&Path> for LevelSelectPlugin {
    fn from(value: &Path) -> Self {
        Self {
            config: SaveSelectConfig::from(value),
        }
    }
}
*/
