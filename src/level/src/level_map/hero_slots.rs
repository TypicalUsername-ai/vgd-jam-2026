use crate::buildings::{ActiveHero, HeroConfigs};
use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};
use serde::Deserialize;

use super::map_config::LevelMapConfig;
use crate::buildings::HeroKind;

/// a single point which can hold a single [ActiveHero]
#[derive(Debug, Component, Default)]
pub(crate) struct HeroSlot {
    pub hero: Option<ActiveHero>,
}

pub(crate) fn setup_hero_slots(
    mut commands: Commands,
    //level_config: Res<LevelMapConfig>,
    spawner_configs: Res<HeroConfigs>,
) {
    commands
        .spawn((
            Node {
                display: Display::Grid,
                width: px(250.),
                height: percent(100.),
                grid_template_columns: vec![GridTrack::px(250.)],
                grid_template_rows: vec![RepeatedGridTrack::percent(4, 25.)],
                ..default()
            },
            BackgroundColor(Color::srgb_u8(155, 0, 120)),
        ))
        .with_children(|cs| {
            for i in 0..4 {
                cs.spawn((
                    Node {
                        width: percent(100.),
                        height: percent(100.),
                        padding: UiRect::all(px(10.)),
                        ..default()
                    },
                    HeroSlot::default(),
                    children![ImageNode::solid_color(Color::srgb_u8(0, 0, 155))],
                ))
                .observe(build_spot_menu);
            }
        });
    /*
    for spawn_point in level_config.spawner_points.iter() {
        commands
            .spawn(SpawnerBuilding::init(
                spawn_point.position,
                spawner_configs
                    .get(&SpawnerKind::None)
                    .expect("Default config has to exist"),
            ))
            .observe(build_spot_menu);
    }
    */
}

#[derive(Debug, Component)]
struct BuildContextMenu {}

//type BuildMenuQuery<'a> = (Entity, &'a mut BuildingSpot);
fn build_spot_menu(
    event: On<Pointer<Click>>,
    window_query: Single<&Window>,
    mut commands: Commands,
    query: Query<(Entity, &HeroSlot)>,
    level_config: Res<LevelMapConfig>,
    ctx_query: Query<Entity, With<BuildContextMenu>>,
) {
    if let Some(ctx) = ctx_query.iter().next() {
        commands.entity(ctx).despawn();
    }
    let (slot_entity, slot) = query
        .iter()
        .find(|&qi| qi.0 == event.entity)
        .expect("This entity triggered the event");
    let cursor_position = window_query.cursor_position().expect("cursor on screen");
    let bundle = (
        BuildContextMenu {},
        Node {
            min_width: px(200.),
            min_height: px(100.0),
            padding: UiRect::all(px(20.0)),
            position_type: PositionType::Absolute,
            left: percent(105.),
            ..default()
        },
        BackgroundColor(Color::srgb_u8(0, 0, 100)),
        Visibility::Visible,
    );
    if let Some(hero) = &slot.hero {
        panic!()
    } else {
        let mut e_cmds = commands
            .entity(slot_entity)
            .with_child(bundle)
            .with_children(|parent_cmds| {
                build_buttons(parent_cmds, &level_config.available_heroes)
            });
    }

    warn!("spawned bundle");
    //spawn_building_menu(children.get(0), &mut commands)
}

fn build_buttons(parent_cmds: &mut RelatedSpawnerCommands<ChildOf>, choices: &[HeroKind]) {
    if choices.is_empty() {
        parent_cmds.spawn(Text::new("No available towers / upgrades"));
    } else {
        for entry in choices {
            parent_cmds.spawn(Text::new(format!("{:?}", entry)));
        }
    }
}
