use bevy::prelude::*;

#[derive(States, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum GlobalState {
    #[default]
    StartMenu,
    LevelSelect,
    ActiveLevel,
}

#[derive(States, Debug, Default, Clone, PartialEq, Eq, Hash, Deref, DerefMut)]
pub struct CurrentLevelState(Option<String>);

impl From<Option<String>> for CurrentLevelState {
    fn from(value: Option<String>) -> Self {
        Self(value)
    }
}
