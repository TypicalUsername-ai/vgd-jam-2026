use bevy::prelude::*;
use level::level_map::MapPlugin;
use level_selector::LevelSelectPlugin;
use main_menu::MainMenuPlugin;
use state::GameStatePlugin;
use tracing::info;

mod window;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(window::default_fulscreen_plugin()))
        .add_plugins(MapPlugin {})
        .add_plugins(GameStatePlugin {})
        .add_plugins(MainMenuPlugin {})
        .add_plugins(LevelSelectPlugin {})
        .run();
}
