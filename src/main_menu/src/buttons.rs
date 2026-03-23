use crate::MenuButtons;
use bevy::{prelude::*, ui::Pressed};
use tracing::info;

pub(crate) fn react_buttons(query: Query<(&MenuButtons, &Interaction), Changed<Interaction>>) {
    for (button, interaction) in query {
        match *interaction {
            Interaction::Pressed => info!("{:?} pressed", button),
            Interaction::Hovered => info!("Hover"),
            Interaction::None => info!("None"),
        }
    }
}
