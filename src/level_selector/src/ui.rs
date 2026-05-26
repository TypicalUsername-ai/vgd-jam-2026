use bevy::prelude::*;
use chrono::{DateTime, Utc};
use state::{GlobalState, LevelState};
use tracing::warn;

use crate::{SaveSelectConfig, level_config::SaveGameState};

#[derive(Debug, Component)]
pub(crate) struct LevelSelectMenu {}

#[derive(Debug, Component)]
#[require(Button)]
pub(crate) struct SaveSelectButton {
    pub(crate) save: Option<SaveGameState>,
}

pub(crate) fn react_buttons(
    mut commands: Commands,
    query: Query<(&SaveSelectButton, &Interaction), Changed<Interaction>>,
    mut lvl_config: ResMut<SaveSelectConfig>,
    mut next_global: ResMut<NextState<GlobalState>>,
    mut next_level: ResMut<NextState<LevelState>>,
) {
    for (button, interaction) in query {
        match *interaction {
            Interaction::Pressed => {
                warn!("Pressed a button {:?}", button);
                let save = match button.save.as_ref() {
                    Some(s) => s,
                    None => {
                        let new_save = new_save_file();
                        lvl_config.saves.push(new_save.clone());
                        lvl_config.saves.last().expect("just added one")
                    }
                };
                commands.insert_resource(save.to_owned());
                next_level.set(LevelState::Pre);
                next_global.set(GlobalState::ActiveLevel);
            }
            Interaction::Hovered => (),
            Interaction::None => (),
        }
    }
}

pub(crate) fn draw_level_select(mut commands: Commands, saves_config: Res<SaveSelectConfig>) {
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
            parent.spawn(level_select_title("Load game save"));
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
                    level_pane.spawn(make_button(
                        "New game save",
                        SaveSelectButton { save: None },
                    ));
                    for configuration in saves_config.saves.iter() {
                        level_pane.spawn(make_button(
                            format!(
                                "save {} (level: {})",
                                configuration.save_name.clone(),
                                configuration.current_level_id.clone()
                            ),
                            SaveSelectButton {
                                save: Some(configuration.clone()),
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

fn new_save_file() -> SaveGameState {
    let ts = Utc::now();
    SaveGameState {
        save_name: format!("Save {}", ts),
        current_level_id: "tutorial".to_owned(),
    }
}
