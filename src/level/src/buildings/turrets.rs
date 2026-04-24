use crate::AnimationState;
use crate::minions::Minion;
use bevy::prelude::*;

type MinionQuery<'a> = (Entity, &'a Minion, &'a Transform);
/// responsible for andling target acquisition and firing of the spawned turrets
pub(crate) fn fire_turrets(
    mut commands: Commands,
    mut turret_query: Query<(&Turret, &Transform, &mut AnimationState)>,
    mut minions_query: Query<(Entity, &mut Minion, &Transform)>,
) {
    // iter over turrets
    for (turret, turret_trs, mut turret_anim) in turret_query.iter_mut() {
        let turret_pos = turret_trs.translation.xy();
        //  calculate furthest target for each turret
        if let Some((entity, mut minion, _transform)) = minions_query
            .iter_mut()
            .sort_by::<MinionQuery>(|i1, i2| {
                i1.1.distance_traveled.total_cmp(&i2.1.distance_traveled)
            })
            .find(|emt| emt.2.translation.xy().distance(turret_pos) <= turret.range)
        {
            minion.health -= turret.damage;
            if minion.health <= f32::EPSILON {
                info!("Minion {}, has died", entity);
                commands.entity(entity).despawn();
            }
            //(turret.shoot_function)(&mut commands, *entity);
        }
    }
}

#[derive(Debug, Component)]
pub(crate) struct Turret {
    pub shot_timer: Timer,
    pub damage: f32,
    pub range: f32,
    pub shoot_function: fn(cmds: &mut Commands, target: Entity),
}
