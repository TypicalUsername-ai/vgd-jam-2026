use bevy::{color::palettes::tailwind, prelude::*};
use state::GlobalState;

use crate::LevelSelectPlugin;

#[derive(Debug, Component)]
pub(crate) struct LevelSelectMenu {}

#[derive(Debug, Component)]
#[require(Button)]
pub(crate) struct LevelSelectButton {}

pub(crate) fn draw_level_select(mut commands: Commands) {
    info!("spawning main menu text");
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
        children![
            level_select_title("Game main menu"),
            available_levels_bundle()
        ],
    ));
}

pub(crate) fn clear_level_select(
    mut commands: Commands,
    query: Single<Entity, With<LevelSelectMenu>>,
) {
    commands.entity(*query).despawn();
}

fn available_levels_bundle() -> impl Bundle {
    (
        Node {
            width: percent(100),
            height: percent(80),
            display: Display::Grid,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceEvenly,
            ..default()
        },
        // BackgroundColor(tailwind::BLUE_300.into()),
        children![make_button("Demo level", LevelSelectButton {})],
    )
}

fn level_select_title(title: impl Into<String>) -> impl Bundle {
    (
        Node {
            width: percent(100),
            height: percent(20),
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
