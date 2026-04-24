use bevy::prelude::*;

/// uses building timers to spawn minions throughout the level
pub(crate) fn spawn_minions(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut SpawnerBuilding, &Transform)>,
) {
    for (entity, mut spawner, transform) in query.iter_mut() {
        // progress time for each spawner
        spawner.spawn_timer.tick(time.delta());
        if spawner.spawn_timer.just_finished() {
            warn!("should spawn {:?}!", entity);
            (spawner.spawn_function)(&mut commands, transform.translation);
        }
    }
}

#[derive(Debug, Component)]
pub(crate) struct SpawnerBuilding {
    pub spawner_kind: SpawnerKind,
    pub spawn_timer: Timer,
    pub spawn_function: fn(cmds: &mut Commands, pos: Vec3),
}

impl SpawnerBuilding {
    pub fn init(position: Vec3) -> impl Bundle {
        (
            Self {
                spawner_kind: SpawnerKind::default(),
                spawn_timer: Timer::from_seconds(0.0, TimerMode::Once),
                spawn_function: |_, _| {},
            },
            Sprite::from_color(Color::srgb_u8(255, 0, 0), Vec2::splat(100.0)),
            Transform::from_translation(position),
            Pickable::default(),
        )
    }
}

#[derive(Debug, Default, Clone, Copy, serde::Deserialize, PartialEq, Eq, Hash)]
pub(crate) enum SpawnerKind {
    #[default]
    None,
}
