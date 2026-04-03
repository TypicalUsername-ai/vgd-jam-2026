use bevy::prelude::*;

#[derive(States, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum GlobalState {
    #[default]
    StartMenu,
    LevelSelect,
    ActiveLevel,
}

#[derive(SubStates, Debug, Default, Clone, PartialEq, Eq, Hash)]
#[source(GlobalState = GlobalState::ActiveLevel)]
pub struct LevelState(Option<String>);

impl From<Option<String>> for LevelState {
    fn from(value: Option<String>) -> Self {
        Self(value)
    }
}
