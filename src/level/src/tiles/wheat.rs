use crate::{
    level_map::LevelMapConfig,
    tiles::buildable_tile::{BuildableTile, clear_context_menu, spawn_context_menu},
};
use bevy::{camera::visibility::RenderLayers, picking::hover::Hovered, prelude::*};
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
        let location = point.position.with_z(1.0);
        commands
            .spawn((
                //RenderLayers::layer(0),
                Sprite::from_atlas_image(
                    wheat_texture.clone(),
                    TextureAtlas {
                        layout: wheat_layout.clone(),
                        index: 17,
                    },
                ),
                Transform::from_translation(location).with_scale(Vec3 {
                    x: 3.,
                    y: 3.,
                    z: 1.,
                }),
                BuildableTile::default(),
                Pickable::default(),
                BackgroundColor(Color::WHITE),
            ))
            .observe(spawn_context_menu)
            //.observe(clear_context_menu)
            .observe(change_color_on_hover)
            .observe(change_color_on_out);
    }
}

fn change_color_on_hover(hover: On<Pointer<Over>>, mut sprites: Query<&mut Sprite>) {
    let mut sprite = sprites.get_mut(hover.entity).unwrap();

    sprite.color = crate::basic::YELLOW.into();
}

fn change_color_on_out(hover: On<Pointer<Out>>, mut sprites: Query<&mut Sprite>) {
    info!(
        "leaving sprite {:?}",
        hover.hit.position.unwrap_or_default()
    );
    let mut sprite = sprites.get_mut(hover.entity).unwrap();

    sprite.color = Color::WHITE;
}
