use bevy::prelude::*;

#[derive(Debug, Component)]
pub(crate) struct SpawnerBuilding {
    pub spawn_timer: Timer,
}

#[derive(Debug, Component)]
pub(crate) struct SpawnerAnimationState {
    pub animation_timer: Timer,
    current_frame: usize,
    color: Color,
    pub action: SpawnerAnimation,
    // needs current atlas position etc
}

impl SpawnerAnimationState {
    pub fn next_frame(&mut self) -> (Color, usize) {
        match self.action {
            SpawnerAnimation::None => (),
            SpawnerAnimation::JustBuilt(start, end) => {
                self.current_frame = ((self.current_frame + 1) % end).max(start);
            }
            SpawnerAnimation::UpgradeReady => {
                if self.color == Color::WHITE {
                    self.color = Color::BLACK;
                } else {
                    self.color = Color::WHITE;
                }
            }
        }
        (self.color, self.current_frame)
    }
}

#[derive(Debug, Default)]
pub(crate) enum SpawnerAnimation {
    #[default]
    None,
    JustBuilt(usize, usize), // a sprite animation
    UpgradeReady,            // just blinking maybe ??
}
