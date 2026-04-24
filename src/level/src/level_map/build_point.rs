use crate::buildings::SpawnerBuilding;
use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};
use serde::Deserialize;

use crate::level_map::map_config::LevelMapConfig;

/// a single point which can hold a single spawner
#[derive(Debug, Deserialize)]
pub(crate) struct BuildPoint {
    /// the center position of the point
    /// CAUTION! even though its 2D be mindful of the Z index location
    pub position: Vec3,
}

#[derive(Component, Debug)]
pub(crate) struct BuildingSpot {}

/// spawns marker entities for which locations will be able to have spawners built on
pub(crate) fn setup_build_points(mut commands: Commands, level_config: Res<LevelMapConfig>) {
    for spawn_point in level_config.spawner_points.iter() {
        commands
            .spawn(SpawnerBuilding::init(spawn_point.position))
            .observe(build_spot_menu);
    }
}

#[derive(Debug, Component)]
struct BuildContextMenu {}

//type BuildMenuQuery<'a> = (Entity, &'a mut BuildingSpot);
fn build_spot_menu(
    event: On<Pointer<Click>>,
    window_query: Single<&Window>,
    mut commands: Commands,
    query: Query<(Entity, &Transform, &SpawnerBuilding)>,
    ctx_query: Query<Entity, With<BuildContextMenu>>,
) {
    if let Some(ctx) = ctx_query.iter().next() {
        commands.entity(ctx).despawn();
    }
    let (_, transform, spawner) = query
        .iter()
        .find(|&qi| qi.0 == event.entity)
        .expect("This entity triggered the event");
    let cursor_position = window_query.cursor_position().expect("cursor on screen");
    let possible_upgrades = vec![];
    let bundle = (
        BuildContextMenu {},
        Node {
            min_width: px(20.0),
            min_height: px(20.0),
            padding: UiRect::all(px(20.0)),
            position_type: PositionType::Absolute,
            top: px(cursor_position.y + 40.0),
            left: px(cursor_position.x + 40.0),
            ..default()
        },
        BackgroundColor(Color::srgb_u8(0, 0, 100)),
        Visibility::Visible,
    );
    let mut e_cmds = commands
        .spawn(bundle)
        .with_children(|parent_cmds| upgrade_buttons(parent_cmds, possible_upgrades));
    warn!("spawned bundle");
    //spawn_building_menu(children.get(0), &mut commands)
}

fn upgrade_buttons(
    parent_cmds: &mut RelatedSpawnerCommands<ChildOf>,
    choices: Vec<SpawnerBuilding>,
) {
    if choices.is_empty() {
        parent_cmds.spawn(Text::new("No available towers / upgrades"));
    } else {
        for entry in choices {
            parent_cmds.spawn(Text::new(format!("{:?}", entry.spawner_kind)));
        }
    }
}
