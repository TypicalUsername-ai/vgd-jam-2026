use bevy::prelude::*;
use level::level_map::MapPlugin;
use state::{GameStatePlugin, GlobalState};
use tracing::info;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MapPlugin {})
        .add_plugins(GameStatePlugin {})
        .add_plugins(MainMenuPlugin {})
        .run();
}

struct MainMenuPlugin {}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GlobalState::StartMenu), draw_main_menu);
    }
}

fn draw_main_menu(mut commands: Commands) {
    info!("spawning main menu text");
    commands.spawn((Camera2d, IsDefaultUiCamera));
    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            display: Display::Flex,
            justify_content: JustifyContent::SpaceAround,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            justify_items: JustifyItems::Center,
            ..default()
        },
        children![Text::new("Main Menu"), Text::new("Level selection")],
    ));
    /*
    commands.spawn((
        Text::new("Main Menu"),
        TextLayout::new_with_justify(Justify::Center),
        TextFont {
            font_size: 24.0,
            font: default(),
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    */
}
