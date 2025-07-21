use bevy::prelude::*;
use bevy::window::WindowResolution;

pub const TILE_SIZE: f32 = 19.0;
pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;
pub const HALF_WINDOW_WIDTH: f32 = WINDOW_WIDTH / 2.0;
pub const HALF_WINDOW_HEIGHT: f32 = WINDOW_HEIGHT / 2.0;
pub const HALF_TILE_SIZE: f32 = TILE_SIZE / 2.0;
pub const GRID_WIDTH: usize = 65;
pub const GRID_HEIGHT: usize = 31;

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
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands
        .spawn(Transform::from_xyz(
            -HALF_WINDOW_WIDTH + HALF_TILE_SIZE,
            HALF_WINDOW_HEIGHT - HALF_TILE_SIZE,
            0.0,
        ))
        .with_children(|parent| {
            parent.spawn(Sprite {
                image: asset_server.load("default_grid_tile.png"),
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..default()
            });
        });
}
