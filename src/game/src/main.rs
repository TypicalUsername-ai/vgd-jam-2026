use bevy::prelude::*;
use level::level_map::MapPlugin;
use level_selector::LevelSelectPlugin;
use main_menu::MainMenuPlugin;
use state::GameStatePlugin;
use tracing::info;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MapPlugin {})
        .add_plugins(GameStatePlugin {})
        .add_plugins(MainMenuPlugin {})
        .add_plugins(LevelSelectPlugin {})
        .run();
}
