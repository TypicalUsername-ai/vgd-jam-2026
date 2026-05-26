use super::{HeroSlot, LevelMapConfig};
use crate::heroes::{ActiveHero, HeroConfig, HeroConfigs, Upgrade};
use crate::ui_assets::UiAssets;
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;

#[derive(Debug, Component)]
pub(crate) struct BuildContextMenu {}

#[derive(Debug, Component)]
#[relationship(relationship_target = AvailableMenus)]
pub struct MenuFor(Entity);

#[derive(Debug, Component)]
#[relationship_target(relationship = MenuFor)]
pub struct AvailableMenus(Vec<Entity>);

//type BuildMenuQuery<'a> = (Entity, &'a mut BuildingSpot);
pub(crate) fn build_spot_menu(
    event: On<Pointer<Click>>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut HeroSlot)>,
    level_config: Res<LevelMapConfig>,
    hero_configs: Res<HeroConfigs>,
    ctx_query: Query<Entity, With<BuildContextMenu>>,
    upgrades_query: Query<&Upgrade>,
    ui_assets: Res<UiAssets>,
) {
    if let Some(ctx) = ctx_query.iter().next() {
        commands.entity(ctx).despawn_children().despawn();
    }
    let (slot_entity, mut slot) = query
        .iter_mut()
        .find(|qi| qi.0 == event.entity)
        .expect("This entity triggered the event");
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
        ImageNode::new(ui_assets.portrait_bg.clone()),
        MenuFor(slot_entity),
        Pickable::IGNORE,
    );
    let mut e_cmds = commands.spawn(bundle);
    e_cmds.set_parent_in_place(slot_entity);
    if let Some(hero) = &slot.hero {
        e_cmds.with_children(|parent_cmds| {
            build_upgrades(
                parent_cmds,
                &upgrades_query
                    .iter()
                    .filter(|u| u.hero == slot_entity)
                    .collect::<Vec<&Upgrade>>(),
                hero,
            )
        });
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

#[derive(Debug, Component)]
struct UpgradeChoice {
    upgrade: Upgrade,
}

fn build_upgrades(
    parent_cmds: &mut RelatedSpawnerCommands<ChildOf>,
    choices: &[&Upgrade],
    hero: &ActiveHero,
) {
    if choices.is_empty() {
        parent_cmds.spawn(Text::new("No available towers / upgrades"));
    } else {
        for entry in choices.into_iter() {
            parent_cmds
                .spawn((
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
                    children![
                        (
                            Text::new(format!(
                                "{:?} lv. {} (+{:.1}%)",
                                entry.kind,
                                entry.level,
                                (entry.value_modifier - 1.) * 100.
                            )),
                            TextFont {
                                font_size: 10.,
                                ..default()
                            },
                        ),
                        TextColor(Color::BLACK),
                    ],
                ))
                .observe(add_upgrade);
        }
    }
}

fn add_upgrade(
    event: On<Pointer<Click>>,
    mut commands: Commands,
    mut hero_query: Query<&mut HeroSlot, With<AvailableMenus>>,
    upgrades_query: Query<(Entity, &UpgradeChoice)>,
) {
    let (upgrade_id, chosen_upgrade) = upgrades_query
        .iter()
        .find(|(e, _u)| e == &event.entity)
        .expect("should trigger on clicked upgrade");
    let hero = hero_query
        .iter_mut()
        .next()
        .expect("single hero selection should work");
    //chosen_upgrade.applied = true;
    warn!("UPGRADE!! {:?}, {:?}", hero, chosen_upgrade);
}
