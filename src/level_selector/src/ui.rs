use bevy::prelude::*;
use state::GlobalState;
use tracing::warn;

use crate::LevelSelectConfig;

#[derive(Debug, Component)]
pub(crate) struct LevelSelectMenu {}

#[derive(Debug, Component)]
#[require(Button)]
pub(crate) struct LevelSelectButton {
    pub(crate) level_id: String,
}

pub(crate) fn react_buttons(
    query: Query<(&LevelSelectButton, &Interaction), Changed<Interaction>>,
    mut next_global: ResMut<NextState<GlobalState>>,
) {
    for (button, interaction) in query {
        match *interaction {
            Interaction::Pressed => {
                warn!("Pressed a button {:?}", button);
                next_global.set(GlobalState::ActiveLevel);
            }
            Interaction::Hovered => (),
            Interaction::None => (),
        }
    }
}

pub(crate) fn draw_level_select(mut commands: Commands, levels_config: Res<LevelSelectConfig>) {
    info!("spawning main menu text");
    commands
        .spawn((
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
            LevelSelectMenu {},
        ))
        .with_children(|parent| {
            parent.spawn(level_select_title("Select level"));
            parent
                .spawn(
                    Node {
                        width: percent(100),
                        height: percent(80),
                        display: Display::Grid,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceEvenly,
                        ..default()
                    },
                    // BackgroundColor(tailwind::BLUE_300.into()),
                )
                .with_children(|level_pane| {
                    for configuration in levels_config.levels.iter() {
                        level_pane.spawn(make_button(
                            configuration.name.clone(),
                            LevelSelectButton {
                                level_id: configuration.id.clone(),
                            },
                        ));
                    }
                });
        });
}

pub(crate) fn clear_level_select(
    mut commands: Commands,
    query: Single<Entity, With<LevelSelectMenu>>,
) {
    commands.entity(*query).despawn();
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
