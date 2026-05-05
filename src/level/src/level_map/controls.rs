use bevy::prelude::*;

use crate::{
    level_map::{HeroSlot, LevelMapConfig},
    minions::MinionConfigs,
};

pub(crate) fn setup_controls(
    mut commands: Commands,
    //level_config: Res<LevelMapConfig>,
    //spawner_configs: Res<HeroConfigs>,
) {
    commands
        .spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                align_content: AlignContent::Center,
                width: percent(10.),
                height: px(75.),
                position_type: PositionType::Absolute,
                top: px(0.),
                left: percent(45.),
                padding: UiRect::axes(px(20.), px(30)),
                ..default()
            },
            BackgroundColor(Color::srgb_u8(155, 0, 120)),
            children![Text::new("Start level")],
        ))
        .observe(start_level);

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

fn start_level(
    event: On<Pointer<Click>>,
    mut commands: Commands,
    portraits: Query<&HeroSlot>,
    minion_configs: Res<MinionConfigs>,
    level_config: Res<LevelMapConfig>,
) {
    for slot in portraits.iter() {
        if let Some(hero) = &slot.hero {
            let avatar = minion_configs
                .get(&hero.spawned_minion)
                .unwrap_or_else(|| panic!("minion {:?} has no config", &hero.spawned_minion));
            avatar.spawn(&mut commands, level_config.path_points[0]);
        }
    }
    info!("event!! {:?}", event);
}
