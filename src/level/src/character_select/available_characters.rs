use bevy::prelude::*;

pub(crate) fn make_bundle() -> impl Bundle {
    (
        Node {
            grid_column: GridPlacement::span(6),
            grid_row: GridPlacement::span(1),
            display: Display::Grid,
            grid_template_columns: RepeatedGridTrack::fr(4, 1.0),
            ..default()
        },
        children![Text::new("roster"),],
        BackgroundColor(Color::srgb_u8(200, 0, 0)),
    )
}
