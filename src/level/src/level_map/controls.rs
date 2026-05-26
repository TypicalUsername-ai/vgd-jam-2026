use bevy::prelude::*;
use state::LevelState;

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
            children![Text::new("Start level")],
        ))
        .observe(start_level);

    commands.spawn((
        Node {
            right: percent(5.),
            top: px(0.),
            ..default()
        },
        BackgroundColor(Color::srgb_u8(120, 120, 0)),
        children![Text::new("Pause / Settings")],
    ));
}

pub fn start_level(
    event: On<Pointer<Click>>,
    level_state: Res<State<LevelState>>,
    mut next_level_state: ResMut<NextState<LevelState>>,
) {
    match **level_state {
        LevelState::Pre => next_level_state.set(LevelState::Active),
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
    commands.spawn((
        Node {
            width: percent(80.),
            height: percent(80.),
            ..default()
        },
        BackgroundColor(Color::WHITE),
    ));
}
pub(crate) fn draw_loss_screen(mut commands: Commands) {
    commands.spawn((
        Node {
            width: percent(80.),
            height: percent(80.),
            ..default()
        },
        BackgroundColor(Color::WHITE),
    ));
}
