// materials.rs
use bevy::prelude::*;

#[derive(Resource)]
pub struct Materials {
    pub blue: Handle<StandardMaterial>,
    pub gray: Handle<StandardMaterial>,
    pub red: Handle<StandardMaterial>,
    pub black: Handle<StandardMaterial>,
    pub yellow: Handle<StandardMaterial>,
}

impl Materials {
    pub fn new(materials: &mut ResMut<Assets<StandardMaterial>>) -> Self {
        Self {
            gray: materials.add(StandardMaterial {
                base_color: Color::srgb(0.91, 0.91, 0.91),
                metallic: 0.0,
                perceptual_roughness: 1.0,
                ..default()
            }),
            red: materials.add(StandardMaterial {
                base_color: Color::srgb(0.945, 0.153, 0.153),
                metallic: 0.0,
                perceptual_roughness: 1.0,
                ..default()
            }),
            blue: materials.add(StandardMaterial {
                base_color: Color::srgb(0.153, 0.431, 0.945),
                emissive: Color::srgb(0.05, 0.15, 0.35).into(),
                metallic: 0.0,
                perceptual_roughness: 0.8,
                ..default()
            }),
            black: materials.add(StandardMaterial {
                base_color: Color::srgb(0.1, 0.1, 0.1),
                metallic: 0.0,
                perceptual_roughness: 1.0,
                ..default()
            }),
            yellow: materials.add(StandardMaterial {
                base_color: Color::srgba(0.9647059, 0.7372549, 0.18431373, 0.3),
                ..default()
            }),
        }
    }
}
