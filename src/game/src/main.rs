use bevy::prelude::*;
use level::CharacterSelectPlugin;
use level_selector::LevelSelectPlugin;
use main_menu::MainMenuPlugin;
use state::GameStatePlugin;

mod window;

fn main() {
    let config = std::path::Path::new("../assets/level-config.ron");

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(window::default_fulscreen_plugin())
                .set(AssetPlugin {
                    file_path: "../assets".into(),
                    ..default()
                }),
        )
        .add_plugins(GameStatePlugin {})
        .add_plugins(MainMenuPlugin {})
        .add_plugins(LevelSelectPlugin::from(config))
        .add_plugins(CharacterSelectPlugin {})
        .run();
}
