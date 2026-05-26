use super::map_config::LevelMapConfig;
use crate::{
    heroes::{ActiveHero, HeroConfig, HeroConfigs, Upgrade, UpgradeKind},
    ui_assets::UiAssets,
};
use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};
/// a single point which can hold a single [ActiveHero]
#[derive(Debug, Component)]
pub(crate) struct HeroSlot {
    pub hero: Option<ActiveHero>,
    pub tracker_id: Entity,
}

impl HeroSlot {
    #[must_use]
    pub(crate) fn new(hp_tracker: Entity) -> Self {
        Self {
            tracker_id: hp_tracker,
            hero: None,
        }
    }
}

pub(crate) fn setup_hero_slots(mut commands: Commands, ui_assets: Res<UiAssets>) {
    commands
        .spawn((
            Node {
                display: Display::Grid,
                width: px(300.),
                height: percent(100.),
                row_gap: px(10.),
                padding: UiRect::axes(percent(0.), percent(1.)),
                grid_template_columns: vec![GridTrack::percent(20.), GridTrack::percent(80.)],
                grid_template_rows: vec![RepeatedGridTrack::percent(4, 24.)],
                ..default()
            },
            //ImageNode::new(texture),
        ))
        .with_children(|cs| {
            for _i in 0..4 {
                let bar_id = cs
                    .spawn((
                        ImageNode::from_atlas_image(
                            ui_assets.hp_bar.clone(),
                            TextureAtlas {
                                layout: ui_assets.hp_bar_layout.clone(),
                                index: 0,
                            },
                        ),
                        TracksHpFor(vec![]),
                    ))
                    .id();
                build_portrait(cs, &ui_assets.portrait_bg, bar_id);
            }
        });
}

fn build_portrait(
    spawner: &mut RelatedSpawnerCommands<ChildOf>,
    background: &Handle<Image>,
    tracker_id: Entity,
) {
    let hero_id = spawner
        .spawn((
            Node {
                width: percent(100.),
                ..default()
            },
            HeroSlot::new(tracker_id),
            ImageNode::solid_color(Color::BLACK.with_alpha(0.)),
        ))
        .observe(super::hero_modifiers::build_spot_menu)
        .with_child(ImageNode::new(background.clone()))
        .id();
    spawner.commands().spawn_batch([
        Upgrade::new(hero_id, UpgradeKind::Speed, 1),
        Upgrade::new(hero_id, UpgradeKind::Health, 1),
        Upgrade::new(hero_id, UpgradeKind::Speed, 2),
        Upgrade::new(hero_id, UpgradeKind::Health, 2),
    ]);
}

#[derive(Debug, Component)]
#[relationship(relationship_target = TracksHpFor)]
pub(crate) struct HpTracker(pub Entity);

#[derive(Debug, Component)]
#[relationship_target(relationship = HpTracker)]
pub(crate) struct TracksHpFor(Vec<Entity>);
