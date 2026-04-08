use bevy::prelude::*;
use state::GlobalState;
mod buttons;
mod ui;

#[derive(Component, Debug)]
#[require(Button)]
enum MenuButtons {
    Levels,
    Options,
    Quit,
}

#[derive(Debug, Component)]
struct MainMenuComponent {}

pub struct MainMenuPlugin {}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GlobalState::StartMenu), ui::draw_main_menu);
        app.add_systems(OnExit(GlobalState::StartMenu), ui::clear_main_menu);
        app.add_systems(
            Update,
            buttons::react_buttons.run_if(in_state(GlobalState::StartMenu)),
        );
    }
}
