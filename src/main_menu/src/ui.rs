use crate::{MainMenuComponent, MenuButtons};
use bevy::{color::palettes::tailwind, prelude::*};
use state::GlobalState;

pub(crate) fn draw_main_menu(mut commands: Commands) {
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
        children![menu_title("Game main menu"), menu_options_bundle()],
        MainMenuComponent {},
    ));
}

fn menu_options_bundle() -> impl Bundle {
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
            make_button("Levels", MenuButtons::Levels),
            make_button("Options", MenuButtons::Options),
            make_button("Quit", MenuButtons::Quit)
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

fn make_button(text: impl Into<String>, component: impl Component) -> impl Bundle {
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
