use std::ops::Deref;

use crate::{
    buildings::{ActiveHero, HeroConfig, HeroConfigs, HeroKind},
    ui_assets::UiAssets,
};
use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};
use serde::Deserialize;

use super::map_config::LevelMapConfig;

#[derive(Debug, Component)]
#[relationship(relationship_target = AvailableMenus)]
pub struct MenuFor(Entity);

#[derive(Debug, Component)]
#[relationship_target(relationship = MenuFor)]
pub struct AvailableMenus(Vec<Entity>);

/// a single point which can hold a single [ActiveHero]
#[derive(Debug, Component, Default)]
pub(crate) struct HeroSlot {
    pub hero: Option<ActiveHero>,
}

pub(crate) fn setup_hero_slots(
    mut commands: Commands,
    //level_config: Res<LevelMapConfig>,
    spawner_configs: Res<HeroConfigs>,
    ui_assets: Res<UiAssets>,
) {
    commands
        .spawn((
            Node {
                display: Display::Grid,
                width: px(250.),
                height: percent(100.),
                row_gap: px(10.),
                padding: UiRect::axes(percent(0.), percent(1.)),
                grid_template_columns: vec![GridTrack::percent(100.)],
                grid_template_rows: vec![RepeatedGridTrack::percent(4, 24.)],
                ..default()
            },
            //ImageNode::new(texture),
        ))
        .with_children(|cs| {
            for i in 0..4 {
                cs.spawn((
                    HeroSlot::default(),
                    ImageNode::solid_color(Color::BLACK.with_alpha(0.)),
                ))
                .observe(build_spot_menu)
                .with_child(ImageNode::new(ui_assets.portrait_bg.clone()));
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
    hero_configs: Res<HeroConfigs>,
    ctx_query: Query<Entity, With<BuildContextMenu>>,
) {
    if let Some(ctx) = ctx_query.iter().next() {
        commands.entity(ctx).despawn_children().despawn();
    }
    warn!("{:?}", event.entity);
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
            position_type: PositionType::Relative,
            left: percent(2.),
            display: Display::Flex,
            row_gap: px(10.),
            flex_direction: FlexDirection::Column,
            align_content: AlignContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb_u8(0, 20, 100)),
        MenuFor(slot_entity),
        Pickable::IGNORE,
    );
    let mut e_cmds = commands.spawn(bundle);
    e_cmds.set_parent_in_place(slot_entity);
    if let Some(hero) = &slot.hero {
        //panic!()
    } else {
        let available_configs: Vec<&HeroConfig> = hero_configs
            .iter()
            .filter_map(|(k, v)| {
                if level_config.available_heroes.contains(k) {
                    Some(v)
                } else {
                    None
                }
            })
            .collect();
        e_cmds.with_children(|parent_cmds| build_buttons(parent_cmds, &available_configs));
    }

    warn!("spawned bundle");
    //spawn_building_menu(children.get(0), &mut commands)
}

#[derive(Debug, Component)]
struct SelectChoice {
    hero: ActiveHero,
    portrait: Handle<Image>,
}

fn build_buttons(parent_cmds: &mut RelatedSpawnerCommands<ChildOf>, choices: &[&HeroConfig]) {
    if choices.is_empty() {
        parent_cmds.spawn(Text::new("No available towers / upgrades"));
    } else {
        for entry in choices.iter() {
            parent_cmds
                .spawn((
                    SelectChoice {
                        hero: entry.hero.clone(),
                        portrait: entry.sprite.clone(),
                    },
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        height: percent(50.),
                        aspect_ratio: Some(1.),
                        //min_width: px(200.),
                        flex_grow: 1.,
                        ..default()
                    },
                    ImageNode::new(entry.sprite.clone()).with_mode(NodeImageMode::Stretch),
                    BackgroundColor(Color::BLACK),
                    children![Text::new(format!("{:?}", entry.hero.spawner_kind)),],
                ))
                .observe(replace_slot);
        }
    }
}

fn replace_slot(
    event: On<Pointer<Click>>,
    mut commands: Commands,
    mut slot_query: Query<(&mut HeroSlot, &mut ImageNode), With<AvailableMenus>>,
    options: Query<(Entity, &SelectChoice)>,
) {
    for (mut slot, mut portrait) in slot_query.iter_mut() {
        if let Some((_e, new_hero)) = options.iter().find(|(e, sc)| event.entity == *e) {
            portrait.image = new_hero.portrait.clone();
            portrait.color = Color::WHITE.with_alpha(100.);
            slot.hero = Some(new_hero.hero.clone());
        }
        info!("click event {} >> {:?}", event.entity, slot.hero)
    }
}
