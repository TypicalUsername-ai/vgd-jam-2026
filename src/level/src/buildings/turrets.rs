use crate::AnimationState;
use crate::buildings::TurretKind;
use crate::minions::Minion;
use bevy::prelude::*;

type MinionQuery<'a> = (Entity, &'a Minion, &'a Transform);
/// responsible for andling target acquisition and firing of the spawned turrets
pub(crate) fn fire_turrets(
    mut commands: Commands,
    mut turret_query: Query<(&mut Turret, &Transform)>,
    mut minions_query: Query<(Entity, &mut Minion, &Transform)>,
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
                let (entity, mut minion, minion_transform) = minions_query
                    .get_mut(target)
                    .expect("The minion fired on exists");
                minion.health -= turret.damage;
                info!("{:?} shot at {:?} ({})", turret, minion.kind, minion.health);
                if minion.health <= f32::EPSILON {
                    info!("Minion {}, has died", entity);
                    commands.entity(entity).despawn();
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
