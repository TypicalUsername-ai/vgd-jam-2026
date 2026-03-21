use bevy::prelude::*;
use level::level_map::MapPlugin;
use state::{GameStatePlugin, GlobalState};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MapPlugin {})
        .add_plugins(GameStatePlugin {})
        .run();
}
