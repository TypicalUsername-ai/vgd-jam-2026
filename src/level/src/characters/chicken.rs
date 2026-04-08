use crate::characters::utils::{AnimationState, AnimationTime};
use crate::level_map::PathWalker;
use tracing::warn;

use super::utils::Facing;
use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;

// width of walking is 128 x 128 px
// 4 x 4 tiles so tile size is : 32 x 32 px
const WALK_TILE_SIZE: u32 = 32;
const WALK_FRAMES: usize = 4;

#[derive(Debug, Component)]
pub struct Chicken {}

pub fn spawn_chicken(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let walk_texture = asset_server.load("sprites/chicken_walk.png");
    let walk_layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(WALK_TILE_SIZE),
        WALK_FRAMES as u32,
        WALK_FRAMES as u32,
        None,
        None,
    ));

    let facing_init = Facing::Left;
    let start_idx = facing_init.into();

    commands.spawn((
        RenderLayers::layer(0),
        Sprite::from_atlas_image(
            walk_texture,
            TextureAtlas {
                layout: walk_layout,
                index: start_idx,
            },
        ),
        Transform::from_translation(Vec3::ZERO.with_z(5.0)),
        Chicken {},
        PathWalker::default(),
        AnimationState {
            facing: facing_init,
            moving: true,
            was_moving: false,
        },
        AnimationTime(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
    warn!("spawned chicken")
}

pub fn animate_chicken(
    time: Res<Time>,
    query: Query<(&mut AnimationState, &mut AnimationTime, &mut Sprite), With<Chicken>>,
) {
    for (mut anim, mut timer, mut sprite) in query {
        let atlas = match sprite.texture_atlas.as_mut() {
            Some(a) => a,
            None => return,
        };
        let target_row: usize = anim.facing.into();
        let mut current_col = atlas.index % WALK_FRAMES;
        let mut current_row = atlas.index / WALK_FRAMES;

        if current_row != target_row {
            atlas.index = anim.facing.into();
            current_col = 0;
            current_row = target_row;
            timer.reset();
        }

        let just_started = anim.moving && !anim.was_moving;
        let just_stopped = !anim.moving && anim.was_moving;

        if anim.moving {
            if just_started {
                // On tap or movement start, immediately advance one frame for visible feedback
                let row_start: usize = WALK_FRAMES * anim.facing as usize;
                let next_col = (current_col + 1) % WALK_FRAMES;
                atlas.index = row_start + next_col;
                // Restart the timer so the next advance uses a full interval
                timer.reset();
            } else {
                // Continuous movement: advance based on timer cadence
                timer.tick(time.delta());
                if timer.just_finished() {
                    let row_start: usize = WALK_FRAMES * anim.facing as usize;
                    let next_col = (current_col + 1) % WALK_FRAMES;
                    atlas.index = row_start + next_col;
                }
            }
        } else if just_stopped {
            // Not moving: keep current frame to avoid snap. Reset timer on transition to idle.
            timer.reset();
        }

        // Update previous movement state
        anim.was_moving = anim.moving;
    }
}
