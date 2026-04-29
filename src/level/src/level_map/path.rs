use super::LevelMapConfig;
use bevy::{math::VectorSpace, prelude::*};

pub(crate) fn setup_path(
    mut commands: Commands,
    level_config: Res<LevelMapConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let delta = 10.;
    let delta_x = Vec3::default().with_x(delta);
    let delta_y = Vec3::default().with_x(delta);
    let mesh_points = level_config.path_points.iter().map(|p| p.xy());

    let mesh_handle = meshes.add(Polyline2d::new(mesh_points));
    let mat_handle = materials.add(Color::srgb_u8(0, 150, 0));
    commands.spawn((
        Mesh2d(mesh_handle),
        MeshMaterial2d(mat_handle),
        Transform::from_translation(Vec3::ZERO.with_z(10.)),
    ));
}
