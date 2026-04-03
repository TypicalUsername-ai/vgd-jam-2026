use super::LevelMapConfig;
use crate::characters::utils::{AnimationState, Facing};
use bevy::prelude::*;

#[derive(Debug, Component)]
pub(crate) struct PathWalker {
    pub timer: Timer,
    pub speed: f32,
    pub segment: usize,
}

impl std::default::Default for PathWalker {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.25, TimerMode::Repeating),
            speed: 3.0,
            segment: 0,
        }
    }
}

pub(crate) fn walk_mobs(
    _commands: Commands,
    time: Res<Time>,
    mobs_query: Query<(&mut PathWalker, &mut Transform, &mut AnimationState), With<PathWalker>>,
    map_config: Res<LevelMapConfig>,
) {
    for (mut walker, mut transform, mut state) in mobs_query {
        // advance time
        walker.timer.tick(time.delta());
        // check if timer ticked to progress
        if walker.timer.just_finished() {
            //info!("Walk timer firing!");
            // segment nr means to which point youre going
            if let Some(tgt) = map_config.path_points.get(walker.segment)
                && state.moving
            {
                let face = Facing::from(tgt - transform.translation);
                if face != state.facing {
                    state.facing = face;
                }
                // info!("Current position: {}", transform.translation);
                let overshoot = tgt.distance(transform.translation) - walker.speed;
                if overshoot >= 0.0 {
                    transform.translation = transform
                        .translation
                        .move_towards(*tgt, walker.speed);
                } else {
                    walker.segment += 1;
                    if walker.segment < map_config.path_points.len() {
                        let new_tgt = map_config
                            .path_points
                            .get(walker.segment)
                            .expect("Length bounds were just checked");
                        transform.translation = tgt.move_towards(*new_tgt, -overshoot);
                    } else {
                        state.moving = false;
                    }
                }
            }
        }
    }
}
