use bevy::prelude::*;


#[derive(Component)]
pub enum RouteSegment {
    Straight,
    Down,
    Left90Turn,
    Right90Turn,
    Left45Turn,
    Right45Turn,
    Left45,
    Right45,
}

pub fn spawn_route(mut commands: Commands, segment: RouteSegment) {
    commands.spawn((
        Text2d::new("===\n   \n==="),
        TextFont {
            font_size: 12.0,
            font: default(),
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_translation(Vec3::from_slice(&[-30.0, 0.0, 0.0])),
        segment
    ));
}
