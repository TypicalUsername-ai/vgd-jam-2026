use bevy::prelude::*;

#[derive(Debug, States, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PauseState {
    Paused,
    #[default]
    Running,
}
