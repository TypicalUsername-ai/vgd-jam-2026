use bevy::prelude::*;

pub(crate) fn draw_character_select(mut commands: Commands) {
    commands.spawn(make_select_menu());
}
pub(crate) fn clear_character_select() {
    todo!()
}

fn make_select_menu() -> impl Bundle {
    (
        Node {
            width: percent(100),
            height: percent(100),
            display: Display::Grid,
            grid_template_rows: vec![GridTrack::fr(6.0), GridTrack::fr(5.0)],
            grid_template_columns: vec![RepeatedGridTrack::fr(6, 1.0)],
            ..default()
        },
        children![
            make_roster_slots(),
            make_game_objectives(),
            make_available_characters(),
        ],
    )
}

fn make_roster_slots() -> impl Bundle {
    (
        Node {
            grid_column: GridPlacement::span(4),
            grid_row: GridPlacement::span(1),
            ..default()
        },
        children![Text::new("roster"),],
        BackgroundColor(Color::WHITE),
    )
}

fn make_game_objectives() -> impl Bundle {
    (
        Node {
            grid_column: GridPlacement::span(2),
            grid_row: GridPlacement::span(1),
            ..default()
        },
        children![Text::new("roster"),],
        BackgroundColor(Color::BLACK),
    )
}

fn make_available_characters() -> impl Bundle {
    (
        Node {
            grid_column: GridPlacement::span(6),
            grid_row: GridPlacement::span(1),
            ..default()
        },
        children![Text::new("roster"),],
        BackgroundColor(Color::srgb_u8(200, 0, 0)),
    )
}
