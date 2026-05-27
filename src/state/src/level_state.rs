use crate::GlobalState;
use bevy::prelude::*;

#[derive(SubStates, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[source(GlobalState = GlobalState::ActiveLevel)]
pub enum LevelState {
    #[default]
    Pre,
    Setup,
    Active,
    Lost,
    Won,
}

//impl bevy::state::state::FreelyMutableState for LevelState {}
