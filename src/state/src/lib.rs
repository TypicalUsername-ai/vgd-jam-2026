mod global_state;
use bevy::prelude::*;
pub use global_state::{CurrentLevelState, GlobalState};

pub struct GameStatePlugin {}

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GlobalState>();
        app.init_state::<CurrentLevelState>();
    }
}
