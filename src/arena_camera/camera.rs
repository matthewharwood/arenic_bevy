use crate::arena_camera::CAMERA_CENTER;
use bevy::prelude::*;

const ZOOM: (f32, f32) = (24.0, 72.0);
/// Setup camera to center on a specific arena
pub fn setup_camera(commands: &mut Commands) {
    // Camera positioned 3x further back for 3x zoom out effect
    // Original Z: 24.0, New Z: 72.0

    commands.spawn((
        Camera3d::default(),
        Camera {
            hdr: true,
            ..default()
        },
        Projection::Perspective(PerspectiveProjection {
            fov: std::f32::consts::FRAC_PI_8,
            aspect_ratio: 16.0 / 9.0,
            near: 0.05,
            far: 150.0, // Increased far plane to accommodate further camera distance
        }),
        Transform::from_xyz(8.125, 3.5, ZOOM.0).looking_at(CAMERA_CENTER, Vec3::Y),
    ));
}

pub fn toggle_camera_zoom(
    keycode: Res<ButtonInput<KeyCode>>,
    camera_query: Single<&mut Transform, With<Camera>>,
) {
    if keycode.just_pressed(KeyCode::KeyP) {
        let mut camera = camera_query.into_inner();
        if (camera.translation.z == ZOOM.0) {
            camera.translation.z = ZOOM.1;
        } else {
            camera.translation.z = ZOOM.0;
        }
    }
    if keycode.just_pressed(KeyCode::BracketRight) {}
}

pub fn move_camera_left(
    keycode: Res<ButtonInput<KeyCode>>,
    camera_query: Single<&mut Transform, With<Camera>>,
) {
    if keycode.just_pressed(KeyCode::BracketLeft) {
        let mut camera = camera_query.into_inner();
        // set arenaid
    }
}

pub fn move_camera_right(
    keycode: Res<ButtonInput<KeyCode>>,
    camera_query: Single<&mut Transform, With<Camera>>,
) {
    if keycode.just_pressed(KeyCode::BracketRight) {
        let mut camera = camera_query.into_inner();
    }
}
