use bevy::{
    prelude::*,
    window::{ExitCondition::OnPrimaryClosed, WindowResolution},
};

pub(crate) fn default_fulscreen_plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            // present_mode: (),
            mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Current),
            resolution: WindowResolution::new(1920, 1080),
            title: "Game".into(),
            //name: (),
            // composite_alpha_mode: (),
            // resize_constraints: (),
            resizable: false,
            // enabled_buttons: (),
            decorations: false,
            focused: true,
            ..default()
        }),
        exit_condition: OnPrimaryClosed,
        close_when_requested: true,
        primary_cursor_options: None,
    }
}
