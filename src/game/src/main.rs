use bevy::prelude::*;
#[cfg(feature = "inspector")]
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use level::LevelPlugin;
use level_selector::LevelSelectPlugin;
use main_menu::MainMenuPlugin;
use state::GameStatePlugin;
use std::path::{Path, PathBuf};

mod window;

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(window::default_fulscreen_plugin())
            .set(AssetPlugin {
                #[cfg(debug_assertions)]
                file_path: "../../assets".into(),
                ..default()
            }),
    )
    .add_plugins(GameStatePlugin {})
    .add_plugins(MainMenuPlugin {})
    .add_plugins(LevelSelectPlugin {})
    .add_plugins(LevelPlugin {});

    #[cfg(feature = "inspector")]
    app.add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new());

    app.run();
}
