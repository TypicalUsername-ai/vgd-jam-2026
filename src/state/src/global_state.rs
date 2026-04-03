use bevy::prelude::*;

#[derive(States, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum GlobalState {
    #[default]
    StartMenu,
    LevelSelect,
    ActiveLevel,
}
