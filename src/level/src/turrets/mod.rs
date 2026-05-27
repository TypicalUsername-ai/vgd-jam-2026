mod turret_config;

pub(crate) use turret_config::{
    TurretConfig, TurretConfigHandles, TurretConfigKeys, TurretConfigs, TurretKind, setup_turrets,
};
//pub(crate) use turrets::{Turret, fire_turrets};

use bevy::prelude::*;

use crate::{level_map::TracksHpFor, minions::Minion};

type MinionQuery<'a> = (Entity, &'a Minion, &'a Transform);
/// responsible for andling target acquisition and firing of the spawned turrets
pub(crate) fn fire_turrets(
    mut commands: Commands,
    mut turret_query: Query<(&mut Turret, &Transform)>,
    mut minions_query: Query<(Entity, &mut Minion, &Transform)>,
    mut tracker_query: Query<(&TracksHpFor, &mut ImageNode)>,
    time: Res<Time>,
) {
    // iter over turrets
    for (mut turret, turret_trs) in turret_query.iter_mut() {
        let turret_pos = turret_trs.translation.xy();
        // progress shot timer
        turret.shot_timer.tick(time.delta());
        // check if turret timer finished
        if turret.shot_timer.just_finished() {
            if let Some(target) = turret.firing_on {
                //turret.shot_timer.reset();
                let (entity, mut minion, _minion_transform) = minions_query
                    .get_mut(target)
                    .expect("The minion fired on exists");
                info!("{:?}", tracker_query);
                let (_tracked, mut tracker_sprite) = tracker_query
                    .iter_mut()
                    .find(|(t, _s)| t.collection().contains(&target))
                    .expect("every entity health should be tracked");
                let atlas = tracker_sprite
                    .texture_atlas
                    .as_mut()
                    .expect("hp bar should have atlas");
                let new_index = ((1. - minion.health / minion.max_health) * 10.).round() as usize;
                atlas.index = new_index;
                //warn!("{}/{} -> {}", minion.health, minion.max_health, new_index);
                minion.health -= turret.damage;
                info!("{:?} shot at {:?} ({})", turret, minion.kind, minion.health);
                if minion.health <= f32::EPSILON {
                    info!("Minion {}, has died", entity);
                    commands.entity(entity).despawn();
                    atlas.index = 10;
                    turret.firing_on = None;
                }
                //(turret.shoot_function)(&mut commands, *entity);
            } else {
                //  calculate furthest target for current turret as new firing target
                turret.firing_on = minions_query
                    .iter_mut()
                    .sort_by::<MinionQuery>(|i1, i2| {
                        i1.1.distance_traveled.total_cmp(&i2.1.distance_traveled)
                    })
                    .find_map(|emt| {
                        if emt.2.translation.xy().distance(turret_pos) <= turret.range {
                            Some(emt.0)
                        } else {
                            None
                        }
                    })
            }
        }
    }
}

#[derive(Debug, Component, Clone)]
pub(crate) struct Turret {
    pub kind: TurretKind,
    pub shot_timer: Timer,
    pub damage: f32,
    pub range: f32,
    pub firing_on: Option<Entity>,
    pub shoot_function: fn(cmds: &mut Commands, target: Entity),
}
