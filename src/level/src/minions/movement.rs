use crate::level_map::LevelMapConfig;
use crate::minions::Minion;
use bevy::prelude::*;

/// moves all of the spawned minions
/// notice that the movement process is decoupled from the animation process
pub(crate) fn move_minions(
    mut query: Query<(&mut Transform, &Minion), With<Minion>>,
    level_config: Res<LevelMapConfig>,
) {
    for (mut transform, minion) in query.iter_mut() {
        let next_translation = level_config.compute_next(minion.speed, minion.distance_traveled);

        transform.translation = next_translation;
    }
}
