use super::building::Building;
use bevy::{ecs::observer::ObservedBy, prelude::*};

#[derive(Debug, Component, Default)]
pub(crate) struct BuildableTile {
    pub structure: Option<Building>,
}

#[derive(Debug, Component)]
#[relationship(relationship_target = InTowerContext)]
pub(crate) struct RelatedToTower(pub Entity);

#[derive(Debug, Component)]
#[relationship_target(relationship = RelatedToTower, linked_spawn)]
pub(crate) struct InTowerContext(Vec<Entity>);

pub(crate) fn spawn_context_menu(
    pick_event: On<Pointer<Click>>,
    mut commands: Commands,
    window_query: Single<&Window>,
    //query: Query<(Entity, &mut BuildableTile, &ObservedBy)>,
    open_menu: Query<(Entity, &RelatedToTower), With<RelatedToTower>>,
) {
    let position = window_query
        .cursor_position()
        .expect("cursor position is available");
    // spawn by default
    // gets entity and children of the main build tile
    let tile_entity = pick_event.entity;
    match open_menu.iter().next() {
        Some((menu, options)) => {
            info!("{} is handling: {}, {:?}", tile_entity, menu, options);
            commands.entity(menu).despawn();
            // check if active menu is already a child of our entity
            if options.0 == tile_entity {
                info!("early return!!");
                return;
            }
        }
        None => {
            info!("no existing spawn context");
        }
    }
    commands
        .spawn(build_context_ui(position, tile_entity))
        .with_children(|parent| {
            parent.spawn(make_tower_button()).observe(build_tower);
        });
}

fn build_tower(
    trigger: On<Pointer<Click>>,
    mut commands: Commands,
    mut query: Query<(&mut Sprite, &mut BuildableTile, &InTowerContext)>,
) {
    info!("trigger");
    for (mut sprite, mut tile, tower_ctx) in query.iter() {
        info!("{:?}", tower_ctx);
    }

    //sprite.color = crate::basic::YELLOW.into();
    //tile.structure = None;
}

fn build_context_ui(position: Vec2, related: Entity) -> impl Bundle {
    (
        Node {
            border: UiRect::all(px(5.0)),
            min_height: px(200),
            padding: UiRect::all(px(10)),
            border_radius: BorderRadius::all(px(10)),
            position_type: PositionType::Absolute,
            top: px(position.y),
            left: px(position.x),
            ..default()
        },
        children![(
            Text::new("Build some"),
            TextFont {
                font_size: 22.0,
                font: default(),
                ..default()
            },
            TextColor(Color::BLACK),
        )],
        RelatedToTower(related),
        BackgroundColor(Color::WHITE),
    )
}

pub(crate) fn clear_context_menu(
    pick_event: On<Pointer<Click>>,
    mut commands: Commands,
    mut query: Query<Entity, With<RelatedToTower>>,
) {
    for (entity) in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn make_tower_button() -> impl Bundle {
    (
        Node {
            min_height: px(30),
            ..default()
        },
        children![
            Text::new("Chicken"),
            TextFont {
                font_size: 22.0,
                font: default(),
                ..default()
            },
            TextColor(Color::BLACK),
        ],
        BackgroundColor(crate::basic::GRAY.into()),
    )
}
