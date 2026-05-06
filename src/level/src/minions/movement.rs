use crate::level_map::LevelMapConfig;
use crate::minions::Minion;
use bevy::prelude::*;

/// moves all of the spawned minions
/// notice that the movement process is decoupled from the animation process
pub(crate) fn move_minions(
    mut query: Query<(&mut Transform, &mut Minion), With<Minion>>,
    level_config: Res<LevelMapConfig>,
) {
    for (mut transform, mut minion) in query.iter_mut() {
        let start = level_config.path_points[minion.target_index - 1];
        let end = level_config
            .path_points
            .get(minion.target_index)
            .unwrap_or(&start)
            .to_owned();
        let distance = start.distance(end);
        if minion.target_index >= level_config.path_points.len() {
            continue;
        }
        if minion.distance_traveled + minion.speed >= distance {
            minion.target_index += 1;
            minion.distance_traveled = 0.;
            transform.translation = end;
        } else {
            minion.distance_traveled += minion.speed;
            transform.translation = start.move_towards(end, minion.distance_traveled);
        }
    }
}
