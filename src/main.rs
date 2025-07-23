use bevy::prelude::*;
use bevy::window::WindowResolution;

// Module declarations
mod arena;
mod bundles;
mod camera;
mod character;
mod components;
mod config;
mod movement;
mod recording;
mod ui;
mod utils;

// Re-exports for convenience
use arena::ArenaPlugin;
use camera::CameraPlugin;
use character::CharacterPlugin;
use config::display::*;
use movement::MovementPlugin;
use recording::RecordingPlugin;
use ui::UiPlugin;

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
        .add_plugins(ArenaPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(CharacterPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(RecordingPlugin)
        .add_plugins(UiPlugin)
        .run();
}
