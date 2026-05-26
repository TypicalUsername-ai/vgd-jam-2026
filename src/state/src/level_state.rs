use crate::GlobalState;
use bevy::prelude::*;
use std::path::PathBuf;

#[derive(SubStates, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[source(GlobalState = GlobalState::ActiveLevel)]
pub enum LevelState {
    #[default]
    Pre,
    Active,
    Lost,
    Won,
}

//impl bevy::state::state::FreelyMutableState for LevelState {}
