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
                metallic: 0.2, // Added slight metallic for better light reflection
                perceptual_roughness: 0.6, // Reduced roughness for better light interaction
                ..default()
            }),
            red: materials.add(StandardMaterial {
                base_color: Color::srgb(0.945, 0.153, 0.153),
                metallic: 0.1,                                 // Added slight metallic
                perceptual_roughness: 0.5, // Smoother surface for better light reflection
                emissive: Color::srgb(0.2, 0.02, 0.02).into(), // Slight red glow
                ..default()
            }),
            blue: materials.add(StandardMaterial {
                base_color: Color::srgb(0.153, 0.431, 0.945),
                emissive: Color::srgb(0.05, 0.15, 0.35).into(),
                metallic: 0.1,             // Added slight metallic
                perceptual_roughness: 0.4, // Smoother for better light reflection
                ..default()
            }),
            black: materials.add(StandardMaterial {
                base_color: Color::srgb(0.1, 0.1, 0.1),
                metallic: 0.9,
                perceptual_roughness: 0.3, // Smoother for better reflections
                ..default()
            }),
            yellow: materials.add(StandardMaterial {
                base_color: Color::srgba(1.0, 0.85, 0.3, 0.75), // Rich golden color
                emissive: Color::srgb(0.9, 0.7, 0.15).into(),   // Moderate glow
                alpha_mode: AlphaMode::Premultiplied,           // Premultiplied alpha
                metallic: 0.3,                                  // Added metallic for golden sheen
                perceptual_roughness: 0.2, // Smooth surface for better light interaction

                ..default()
            }),
        }
    }
}
