use bevy::prelude::*;

pub(crate) fn make_bundle() -> impl Bundle {
    (
        Node {
            grid_column: GridPlacement::span(4),
            grid_row: GridPlacement::span(1),
            display: Display::Flex,
            flex_direction: FlexDirection::RowReverse,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceEvenly,
            ..default()
        },
        children![
            make_character_slot(1),
            make_character_slot(2),
            make_character_slot(3),
            make_character_slot(4)
        ],
        BackgroundColor(Color::WHITE),
    )
}

#[derive(Debug, Component)]
struct CharacterSelectionSlot {
    slot_id: u8,
}

fn make_character_slot(slot: u8) -> impl Bundle {
    (
        Node {
            width: percent(20),
            height: percent(20),
            padding: UiRect::all(px(100)),
            ..default()
        },
        Text::new(format!("Character {}", slot)),
        CharacterSelectionSlot { slot_id: slot },
        BackgroundColor(Color::srgb_u8(0, 30, 0)),
    )
}
