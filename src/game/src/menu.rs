use bevy::{color::palettes::tailwind, prelude::*};
use state::GlobalState;

pub(crate) struct MainMenuPlugin {}

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
        children![menu_title("Game main menu"), menu_options_bundle()],
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
            make_button("Levels", Button),
            make_button("Options", Button),
            make_button("Quit", Button)
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
