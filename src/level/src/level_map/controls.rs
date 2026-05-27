use bevy::prelude::*;
use level_selector::SaveGameState;
use state::{LevelState, PauseState};

use crate::{
    level_map::{HeroSlot, LevelMapConfig},
    minions::MinionConfigs,
};

pub(crate) fn setup_controls(mut commands: Commands) {
    commands
        .spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                align_content: AlignContent::Center,
                width: percent(10.),
                height: px(75.),
                position_type: PositionType::Absolute,
                top: px(0.),
                left: percent(45.),
                padding: UiRect::axes(px(20.), px(30)),
                ..default()
            },
            BackgroundColor(Color::srgb_u8(155, 0, 120)),
            children![Text::new("Start / Reset level")],
        ))
        .observe(start_level);

    commands
        .spawn((
            Node {
                //right: percent(5.),
                align_self: AlignSelf::Start,
                justify_self: JustifySelf::End,
                top: px(0.),
                ..default()
            },
            BackgroundColor(Color::srgb_u8(120, 120, 0)),
            children![Text::new("Pause / Settings")],
        ))
        .observe(toggle_pause);
}

pub fn start_level(
    _event: On<Pointer<Click>>,
    level_state: Res<State<LevelState>>,
    mut next_level_state: ResMut<NextState<LevelState>>,
) {
    match **level_state {
        LevelState::Pre => next_level_state.set(LevelState::Setup),
        LevelState::Setup => next_level_state.set(LevelState::Active),
        LevelState::Active => next_level_state.set(LevelState::Pre),
        LevelState::Lost => todo!(),
        LevelState::Won => todo!(),
    }
}

pub(crate) fn spawn_heroes(
    mut commands: Commands,
    portraits: Query<&HeroSlot>,
    minion_configs: Res<MinionConfigs>,
    level_config: Res<LevelMapConfig>,
) {
    for slot in portraits.iter() {
        if let Some(hero) = &slot.hero {
            let avatar = minion_configs
                .get(&hero.spawned_minion)
                .unwrap_or_else(|| panic!("minion {:?} has no config", &hero.spawned_minion));
            avatar.spawn(
                &mut commands,
                &hero.applied_upgrades,
                slot.tracker_id,
                level_config.path_points[0],
            );
        }
    }
}

pub(crate) fn draw_win_screen(mut commands: Commands) {
    let mut base = commands.spawn((
        Node {
            width: percent(100.),
            height: percent(100.),
            display: Display::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::WHITE.with_alpha(90.)),
        DespawnOnExit(LevelState::Won),
    ));

    base.with_children(|pane| {
        pane.spawn(Text::new("YOU WON!!"));
        let mut center = pane.spawn((
            Node {
                width: percent(40.),
                height: percent(40.),
                display: Display::Flex,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceAround,
                align_content: AlignContent::SpaceBetween,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            TextColor(Color::BLACK),
            BackgroundColor(Color::srgb_u8(10, 60, 0)),
        ));
        center.with_children(|center_pane| {
            center_pane.spawn(Text::new("YOU WON!"));
            center_pane
                .spawn(Text::new("next level"))
                .observe(load_next_level);
        });
    });
}

fn load_next_level(
    _event: On<Pointer<Click>>,
    level_config: Res<LevelMapConfig>,
    mut save_game: ResMut<SaveGameState>,
    mut next_level_state: ResMut<NextState<LevelState>>,
) {
    let next_level = level_config
        .next_level_id
        .as_ref()
        .expect("There is a level");
    save_game.current_level_id = next_level.to_owned();
    next_level_state.set(LevelState::Pre);
}

pub(crate) fn draw_loss_screen(mut commands: Commands) {
    let mut base = commands.spawn((
        Node {
            width: percent(100.),
            height: percent(100.),
            display: Display::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        DespawnOnExit(LevelState::Lost),
        BackgroundColor(Color::WHITE.with_alpha(90.)),
    ));

    base.with_children(|pane| {
        pane.spawn(Text::new("YOU WON!!"));
        let mut center = pane.spawn((
            Node {
                width: percent(40.),
                height: percent(40.),
                display: Display::Flex,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceAround,
                align_content: AlignContent::SpaceBetween,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            TextColor(Color::BLACK),
            BackgroundColor(Color::srgb_u8(60, 10, 0)),
        ));
        center.with_children(|center_pane| {
            center_pane.spawn(Text::new("YOU LOST..."));
            center_pane
                .spawn(Text::new("retry level"))
                .observe(reload_level);
        });
    });
}

fn reload_level(_event: On<Pointer<Click>>, mut next_level_state: ResMut<NextState<LevelState>>) {
    next_level_state.set(LevelState::Setup);
}

pub(crate) fn ingame_pause(mut commands: Commands) {
    commands.spawn((
        Node {
            width: percent(60.),
            height: percent(60.),
            display: Display::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            ..default()
        },
        BackgroundColor(Color::srgb_u8(0, 40, 40)),
        DespawnOnExit(PauseState::Paused),
    ));
}

fn toggle_pause(
    _event: On<Pointer<Click>>,
    pause_state: Res<State<PauseState>>,
    mut next_pause_state: ResMut<NextState<PauseState>>,
) {
    let next_state = match **pause_state {
        PauseState::Paused => PauseState::Running,
        PauseState::Running => PauseState::Paused,
    };
    next_pause_state.set(next_state);
}
