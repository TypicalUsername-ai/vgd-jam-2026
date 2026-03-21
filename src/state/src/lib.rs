mod global_state;
use bevy::prelude::*;
pub use global_state::GlobalState;

pub struct GameStatePlugin {}

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GlobalState>();
    }
}
