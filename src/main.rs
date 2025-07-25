use bevy::prelude::*;
use bevy::window::WindowResolution;

// Module declarations
mod arena;
mod battleground;

mod recording;

mod boss;
mod character;
mod config;
mod relationships;

use crate::config::display::{WINDOW_HEIGHT, WINDOW_WIDTH};
use recording::RecordingPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Arenic".to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RecordingPlugin)
        .run();
}
