use crate::level_map::LevelMapConfig;
use bevy::{camera::visibility::RenderLayers, prelude::*};
// width of walking is 96 x 192 px
// 4 x 4 tiles so tile size is : 32 x 32 px
const TILE_SIZE: u32 = 32;
const ROWS: u32 = 6;
const COLUMNS: u32 = 3;

pub(crate) fn spawn_soil_path(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    level_config: Res<LevelMapConfig>,
) {
    let soil_texture = asset_server.load("sprites/plowed_soil.png");
    let soil_layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(TILE_SIZE),
        COLUMNS,
        ROWS,
        None,
        None,
    ));

    for window in level_config.path_points.windows(2) {
        let start = window[0];
        let stop = window[1];
        let width = start.distance(stop.clone());
        let mut cursor = start.clone();
        while cursor.distance(stop.clone()) > f32::EPSILON {
            commands.spawn((
                RenderLayers::layer(0),
                Sprite::from_atlas_image(
                    soil_texture.clone(),
                    TextureAtlas {
                        layout: soil_layout.clone(),
                        index: 17,
                    },
                ),
                Transform::from_translation(cursor.clone().with_z(0.0)),
            ));
            cursor = cursor.move_towards(stop.clone(), TILE_SIZE as f32);
        }
    }
}
