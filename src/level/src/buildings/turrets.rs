use crate::AnimationState;
use crate::minions::Minion;
use bevy::prelude::*;

/// animates all of the currently spawned turrets
pub(crate) fn animate_turrets(
    time: Res<Time>,
    mut query: Query<(&mut AnimationState, &mut Sprite), With<Turret>>,
) {
    for (mut anim_state, mut sprite) in query.iter_mut() {
        // tick animation timer
        anim_state.animation_timer.tick(time.delta());
        // check if we should progress animation
        if anim_state.animation_timer.just_finished() {
            // TODO // also probably valid to merge the animation states and use entity filters
            todo!()
        }
    }
}

type MinionQuery<'a> = (Entity, &'a Minion, &'a Transform);
/// responsible for andling target acquisition and firing of the spawned turrets
pub(crate) fn fire_turrets(
    mut commands: Commands,
    mut turret_query: Query<(&Turret, &Transform, &mut AnimationState)>,
    minions_query: Query<MinionQuery>,
) {
    // iter over turrets
    for (turret, turret_trs, mut turret_anim) in turret_query.iter_mut() {
        let turret_pos = turret_trs.translation.xy();
        //  calculate target for each turret
        if let Some((entity, minion, transform)) = minions_query
            .iter()
            .sort_by::<MinionQuery>(|i1, i2| {
                i1.1.distance_traveled.total_cmp(&i2.1.distance_traveled)
            })
            .filter(|emt| emt.2.translation.xy().distance(turret_pos) <= turret.range)
            .next()
        {
            (turret.shoot_function)(&mut commands, entity);
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

#[derive(Debug, Default, serde::Deserialize, Hash, PartialEq, Eq)]
pub(crate) enum TurretAnimation {
    #[default]
    Idle,
    Shooting,
}
