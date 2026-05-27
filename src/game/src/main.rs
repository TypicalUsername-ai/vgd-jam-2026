use bevy::prelude::*;
#[cfg(feature = "inspector")]
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use level::LevelPlugin;
use level_selector::LevelSelectPlugin;
use main_menu::MainMenuPlugin;
use state::{ConfigFileLocation, GameStatePlugin};
use std::path::{Path, PathBuf};

mod window;

fn main() {
    //let config = std::path::Path::new("../assets/saves-config.ron");
    let config_location: PathBuf = Path::new("../assets").into();

    let mut app = App::new();

    app.insert_resource(ConfigFileLocation(config_location.clone()));
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
    .add_plugins(LevelSelectPlugin::from(
        config_location.join("saves-config.ron").as_path(),
    ))
    .add_plugins(LevelPlugin::new(config_location.clone()));

    #[cfg(feature = "inspector")]
    app.add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new());

    app.run();
}
