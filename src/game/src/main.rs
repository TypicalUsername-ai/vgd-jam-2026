use std::path::PathBuf;

use bevy::prelude::*;
#[cfg(feature = "inspector")]
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use level::LevelPlugin;
use level_selector::LevelSelectPlugin;
use main_menu::MainMenuPlugin;
use state::GameStatePlugin;

mod window;

fn main() {
    let config = std::path::Path::new("../assets/level-config.ron");

    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(window::default_fulscreen_plugin())
            .set(AssetPlugin {
                file_path: "../../assets".into(),
                ..default()
            }),
    )
    .add_plugins(GameStatePlugin {})
    .add_plugins(MainMenuPlugin {})
    .add_plugins(LevelSelectPlugin::from(config))
    .add_plugins(LevelPlugin::new(
        PathBuf::from("../assets/buildings"),
        PathBuf::from("../assets/turrets"),
        PathBuf::from("../assets/minions"),
    ));

    #[cfg(feature = "inspector")]
    app.add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new());

    app.run();
}
