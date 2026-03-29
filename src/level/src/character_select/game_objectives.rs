use bevy::prelude::*;

pub(crate) fn make_bundle() -> impl Bundle {
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
