use super::spawner::{SpawnerAnimationState, SpawnerBuilding};
use bevy::prelude::*;

/// registers all of the available spawner building types
pub(crate) fn register_spawners() {
    todo!()
}

/// animates all of the currently built spawner buildings
pub(crate) fn animate_spawners(
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &SpawnerBuilding,
        &mut Sprite,
        &mut SpawnerAnimationState,
    )>,
) {
    for (entity, spawner, mut sprite, mut spawner_anim) in query.iter_mut() {
        // tick every animation timer
        spawner_anim.animation_timer.tick(time.delta());
        if spawner_anim.animation_timer.is_finished() {
            let mut atlas = sprite
                .texture_atlas
                .as_mut()
                .expect("Tower should have a texture atlas!");

            // progress to next frame in atlas layout
            let (color, sprite_idx) = spawner_anim.next_frame();
            atlas.index = sprite_idx;
            drop(atlas);
            sprite.color = color;
        }
    }
}

/// uses building timers to spawn minions throughout the level
pub(crate) fn spawn_minions(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut SpawnerBuilding, &Transform)>,
) {
    for (entity, mut spawner, transform) in query.iter_mut() {
        // progress time for each spawner
        spawner.spawn_timer.tick(time.delta());
        if spawner.spawn_timer.is_finished() {
            warn!("should spawn {:?}!", entity)
        }
    }
}
