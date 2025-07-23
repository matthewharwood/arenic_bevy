#![feature(gen_blocks)]
#![feature(yield_expr)]
#![feature(associated_type_defaults)]

use bevy::prelude::*;
use bevy::window::WindowResolution;

// Module declarations
mod animation;
mod bundles;
mod components;
mod config;
mod const_camera;
mod const_grid;
mod generators;
mod input;
mod plugins;
mod ui;
mod utils;

// Re-exports for convenience
use config::display::{WINDOW_HEIGHT, WINDOW_WIDTH};
use plugins::{ArenaPlugin, CameraPlugin, CharacterPlugin, InitializationPlugin, InputPlugin};
use ui::UIPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Arenic".to_string(),
                    resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                    ..default()
                }),
                ..default()
            }),
            InitializationPlugin,
            InputPlugin,
            ArenaPlugin,
            CameraPlugin,
            CharacterPlugin,
            UIPlugin,
        ))
        .run();
}
