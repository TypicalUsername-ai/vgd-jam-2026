use super::{available_characters, game_objectives, roster_slots};
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
            roster_slots::make_bundle(),
            game_objectives::make_bundle(),
            available_characters::make_bundle(),
        ],
    )
}
