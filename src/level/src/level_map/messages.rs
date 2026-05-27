use bevy::prelude::*;
use state::LevelState;

use crate::level_map::LevelMapConfig;

pub(crate) fn display_messages(mut commands: Commands, level_config: Res<LevelMapConfig>) {
    commands
        .spawn((
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
            DespawnOnExit(LevelState::Pre),
        ))
        .with_children(|pane| {
            for msg in level_config.messages.iter() {
                pane.spawn(Text::new(msg));
            }
        })
        .observe(move_conversation);
}

fn move_conversation(
    _event: On<Pointer<Click>>,
    mut next_level_state: ResMut<NextState<LevelState>>,
) {
    next_level_state.set(LevelState::Setup);
}
