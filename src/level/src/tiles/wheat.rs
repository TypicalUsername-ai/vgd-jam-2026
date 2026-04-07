use crate::level_map::LevelMapConfig;
use bevy::{camera::visibility::RenderLayers, prelude::*};
// width of walking is 96 x 192 px
// 4 x 4 tiles so tile size is : 32 x 32 px
const TILE_SIZE: u32 = 32;
const ROWS: u32 = 6;
const COLUMNS: u32 = 3;

pub(crate) fn spawn_build_locations(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    level_config: Res<LevelMapConfig>,
) {
    let wheat_texture = asset_server.load("sprites/youngwheat.png");
    let wheat_layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(TILE_SIZE),
        COLUMNS,
        ROWS,
        None,
        None,
    ));

    for point in level_config.spawner_points.iter() {
        commands.spawn((
            RenderLayers::layer(0),
            Sprite::from_atlas_image(
                wheat_texture.clone(),
                TextureAtlas {
                    layout: wheat_layout.clone(),
                    index: 17,
                },
            ),
            Transform::from_translation(point.position.with_z(1.0))
                .with_scale(Vec3::splat(3.0).with_z(0.0)),
        ));
    }
}
