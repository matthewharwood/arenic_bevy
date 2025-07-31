use bevy::prelude::*;
use bevy::window::WindowResolution;

// Module declarations
mod arena;
mod battleground;
mod boss;
mod camera;

mod character;
mod config;
mod game_state;
mod pseudo_states;
mod recording;
mod relationships;
mod tile;
mod trait_utils;
mod ui;
mod utils;

use crate::camera::CameraPlugin;
use crate::config::display::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::game_state::GameStatePlugin;
use crate::ui::UiPlugin;
use recording::RecordingPlugin;

const GAME_NAME: &str = "Arenic";
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: GAME_NAME.to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GameStatePlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(RecordingPlugin)
        .add_plugins(UiPlugin)
        .run();
}
