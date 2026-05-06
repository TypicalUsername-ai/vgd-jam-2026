use super::LevelMapConfig;
use bevy::prelude::*;

pub(crate) fn setup_path(
    mut commands: Commands,
    level_config: Res<LevelMapConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let mesh_points: Vec<Vec2> = level_config.path_points.iter().map(|p| p.xy()).collect();

    let mesh_handle = meshes.add(Polyline2d::new(mesh_points));
    let mat_handle = materials.add(ColorMaterial {
        //texture: Some(asset_server.load("sprites/build_plot.png")),
        color: Color::BLACK,
        ..default()
    });
    commands.spawn((
        Mesh2d(mesh_handle),
        MeshMaterial2d(mat_handle),
        Transform::from_translation(Vec3::ZERO.with_z(10.)),
    ));
}
