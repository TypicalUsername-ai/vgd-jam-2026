mod global_state;
mod level_state;
mod pause_state;
use bevy::prelude::*;
pub use global_state::GlobalState;
pub use level_state::LevelState;
pub use pause_state::PauseState;

pub struct GameStatePlugin {}

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GlobalState>();
        app.init_state::<LevelState>();
        app.init_state::<PauseState>();
    }
}
