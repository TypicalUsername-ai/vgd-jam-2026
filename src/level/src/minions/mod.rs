mod animation;
mod damage;
mod minion;
mod movement;

pub(crate) use animation::animate_minions;
pub(crate) use damage::handle_damage;
pub(crate) use minion::Minion;
pub(crate) use movement::move_minions;
