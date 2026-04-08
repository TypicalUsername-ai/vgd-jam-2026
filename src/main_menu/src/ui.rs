use crate::{MainMenuComponent, MenuButtons};
use bevy::prelude::*;

pub(crate) fn draw_main_menu(mut commands: Commands) {
    info!("spawning main menu text");
    commands.spawn((
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        Camera2d,
        Pickable::IGNORE,
        IsDefaultUiCamera,
    ));
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
        children![menu_title("Game main menu"), menu_options()],
        MainMenuComponent {},
    ));
}

fn menu_options() -> impl Bundle {
    (
        Node {
            width: percent(100),
            height: percent(60),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceEvenly,
            ..default()
        },
        // BackgroundColor(tailwind::BLUE_300.into()),
        children![
            menu_button("Levels", MenuButtons::Levels),
            menu_button("Options", MenuButtons::Options),
            menu_button("Quit", MenuButtons::Quit)
        ],
    )
}

fn menu_title(title: impl Into<String>) -> impl Bundle {
    (
        Node {
            width: percent(100),
            height: percent(40),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceEvenly,
            justify_items: JustifyItems::Center,
            ..default()
        },
        children![(
            Text::new(title),
            TextFont {
                font_size: 44.0,
                font: default(),
                ..default()
            },
            TextColor(Color::WHITE),
        )],
    )
}

pub(crate) fn clear_main_menu(
    mut commands: Commands,
    query: Single<Entity, With<MainMenuComponent>>,
) {
    commands.entity(*query).despawn();
}

fn menu_button(text: impl Into<String>, component: impl Component) -> impl Bundle {
    (
        Node {
            border: UiRect::all(px(5.0)),
            min_height: px(40),
            padding: UiRect::all(px(10)),
            border_radius: BorderRadius::all(px(10)),
            ..default()
        },
        children![(
            Text::new(text),
            TextFont {
                font_size: 22.0,
                font: default(),
                ..default()
            },
            TextColor(Color::BLACK),
        )],
        BackgroundColor(Color::WHITE),
        component,
    )
}
