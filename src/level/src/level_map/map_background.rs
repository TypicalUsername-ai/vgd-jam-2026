use super::LevelMapConfig;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

/// sets up flat non-interactive background
pub(crate) fn setup_background(
    mut commands: Commands,
    level_config: Res<LevelMapConfig>,
    main_window: Single<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let sprite = if let Some(image) = level_config.bg_image.as_ref() {
        let handle = asset_server.load(image);
        Sprite::from_image(handle)
    } else {
        Sprite::from_color(
            level_config.bg_color,
            Vec2 {
                x: main_window.width(),
                y: main_window.height(),
            },
        )
    };
    commands.spawn(sprite);
}
