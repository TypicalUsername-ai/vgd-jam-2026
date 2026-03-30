use crate::LevelCamera;
use crate::characters::utils::{AnimationState, AnimationTime};
use tracing::warn;

use super::utils::Facing;
use bevy::camera::visibility::RenderLayers;
use bevy::render::render_graph::RenderLabel;
use bevy::{math::VectorSpace, prelude::*};

// width of walking is 128 x 128 px
// 4 x 4 tiles so tile size is : 32 x 32 px
const WALK_TILE_SIZE: u32 = 32;
const WALK_FRAMES: usize = 4;

#[derive(Debug, Component)]
struct Chicken {}

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
    let start_idx = facing_init.clone().into();

    commands.spawn((
        RenderLayers::layer(0),
        Sprite::from_atlas_image(
            walk_texture,
            TextureAtlas {
                layout: walk_layout,
                index: start_idx,
            },
        ),
        Transform::from_translation(Vec3::ZERO),
        Chicken {},
        AnimationState {
            facing: facing_init,
            moving: false,
            was_moving: false,
        },
        AnimationTime(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
    warn!("spawned chicken")
}
