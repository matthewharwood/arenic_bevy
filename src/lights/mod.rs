use bevy::prelude::*;
use crate::arena::{ARENA_HEIGHT_HALF, ARENA_WIDTH_HALF};
use crate::arena_camera::CAMERA_CENTER;

pub fn spawn_lights(mut commands: Commands) {
    commands.spawn(DirectionalLight {
        illuminance: 10000.0,
        color: Color::WHITE,
        shadows_enabled: true,
        ..default()
    });

    commands.spawn((
        SpotLight {
            intensity: 10000000.0, // lumens
            color: Color::srgb(1.0, 0.0, 0.0),
            shadows_enabled: true,
            inner_angle: 0.6,
            outer_angle: 0.6,
            ..default()
        },
        Transform::from_xyz(ARENA_WIDTH_HALF, ARENA_HEIGHT_HALF, 9.0)
            .looking_at(CAMERA_CENTER, Vec3::Y),
    ));
}