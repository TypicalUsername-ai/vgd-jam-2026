use bevy::prelude::*;

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
            let atlas = sprite
                .texture_atlas
                .as_mut()
                .expect("Tower should have a texture atlas!");

            // progress to next frame in atlas layout
            let (color, sprite_idx) = spawner_anim.next_frame();
            atlas.index = sprite_idx;
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
            warn!("should spawn {:?}!", entity);
            (spawner.spawn_function)(&mut commands, transform.translation);
        }
    }
}

#[derive(Debug, Component)]
pub(crate) struct SpawnerBuilding {
    pub spawn_timer: Timer,
    pub spawn_function: fn(cmds: &mut Commands, pos: Vec3),
}

#[derive(Debug, Component)]
pub(crate) struct SpawnerAnimationState {
    pub animation_timer: Timer,
    current_frame: usize,
    color: Color,
    pub action: SpawnerAnimation,
    // needs current atlas position etc
}

impl SpawnerAnimationState {
    pub fn next_frame(&mut self) -> (Color, usize) {
        match self.action {
            SpawnerAnimation::None => (),
            SpawnerAnimation::JustBuilt(start, end) => {
                self.current_frame = ((self.current_frame + 1) % end).max(start);
            }
            SpawnerAnimation::UpgradeReady => {
                if self.color == Color::WHITE {
                    self.color = Color::BLACK;
                } else {
                    self.color = Color::WHITE;
                }
            }
        }
        (self.color, self.current_frame)
    }
}

#[derive(Debug, Default)]
pub(crate) enum SpawnerAnimation {
    #[default]
    None,
    JustBuilt(usize, usize), // a sprite animation
    UpgradeReady,            // just blinking maybe ??
}
