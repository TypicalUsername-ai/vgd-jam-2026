use crate::MenuButtons;
use bevy::prelude::*;
use state::GlobalState;
use tracing::info;

pub(crate) fn react_buttons(
    query: Query<(&MenuButtons, &Interaction), Changed<Interaction>>,
    mut next_global: ResMut<NextState<GlobalState>>,
) {
    for (button, interaction) in query {
        match *button {
            MenuButtons::Levels => handle_levels(interaction, &mut next_global),
            MenuButtons::Options => handle_options(interaction),
            MenuButtons::Quit => handle_quit(interaction),
        }
    }
}

fn handle_levels(action: &Interaction, next_global: &mut ResMut<NextState<GlobalState>>) {
    match action {
        Interaction::Pressed => {
            warn!("Moving to level select");
            next_global.set(GlobalState::LevelSelect);
        }
        Interaction::Hovered => (),
        Interaction::None => (),
    }
}
fn handle_quit(action: &Interaction) {
    match action {
        Interaction::Pressed => todo!(),
        Interaction::Hovered => (),
        Interaction::None => (),
    }
}
fn handle_options(action: &Interaction) {
    match action {
        Interaction::Pressed => todo!(),
        Interaction::Hovered => (),
        Interaction::None => (),
    }
}
