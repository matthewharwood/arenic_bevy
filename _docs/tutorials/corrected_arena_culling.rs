use bevy::prelude::*;
use crate::recording::ArenaIndex;
use crate::arena::{Arena, CurrentArena};

/// Corrected arena lighting component that properly integrates with existing systems
#[derive(Component)]
pub struct ArenaLighting {
    pub theme: ArenaTheme,
    pub functional_priority: bool, // True = gameplay over atmosphere
    pub ambient_lights: Vec<Entity>,
    pub emergency_override: bool,
    /// Zoom-specific intensity multipliers for binary system
    pub single_arena_intensity: f32,  // Multiplier when this arena is focused (scale 1.0)
    pub overview_intensity: f32,      // Multiplier when all 9 arenas visible (scale 3.0)
}

/// Arena theme enum that maps to the existing arena indices from the codebase
#[derive(Debug, Clone, Copy)]
pub enum ArenaTheme {
    GuildHouse, // Index 0
    Pawnshop,   // Index 1
    Crucible,   // Index 2
    Sanctum,    // Index 3
    Bastion,    // Index 4
    Labyrinth,  // Index 5
    Mountain,   // Index 6
    Casino,     // Index 7
    Gala,       // Index 8
}

impl ArenaTheme {
    /// Returns the base color that defines this arena's personality
    pub fn ambient_color(&self) -> Color {
        match self {
            ArenaTheme::GuildHouse => Color::srgb(0.5, 0.7, 0.5),   // Balanced green
            ArenaTheme::Pawnshop =>   Color::srgb(0.7, 0.7, 0.6),   // Cluttered gray
            ArenaTheme::Crucible =>   Color::srgb(0.8, 0.4, 0.2),   // Industrial orange-red
            ArenaTheme::Sanctum =>    Color::srgb(1.0, 0.95, 0.8),  // Divine gold
            ArenaTheme::Bastion =>    Color::srgb(0.4, 0.6, 0.8),   // Cool military blue
            ArenaTheme::Labyrinth =>  Color::srgb(0.6, 0.4, 0.8),   // Mysterious purple
            ArenaTheme::Mountain =>   Color::srgb(0.6, 0.5, 0.4),   // Earth brown
            ArenaTheme::Casino =>     Color::srgb(1.0, 0.8, 0.4),   // Rich gold
            ArenaTheme::Gala =>       Color::srgb(0.9, 0.9, 0.95),  // Elegant white
        }
    }
    
    /// Returns appropriate intensity multipliers for binary zoom system
    pub fn zoom_intensity_profile(&self) -> (f32, f32) {
        match self {
            // High-contrast themes work well at both zoom levels
            ArenaTheme::Crucible | ArenaTheme::Bastion => (1.0, 0.8),
            
            // Subtle themes need intensity boost when zoomed out for visibility
            ArenaTheme::Gala | ArenaTheme::Sanctum => (0.9, 0.9),
            
            // Chaotic themes benefit from reduction when all arenas visible
            ArenaTheme::Casino | ArenaTheme::Pawnshop => (1.0, 0.6),
            
            // Standard profile for other themes
            _ => (1.0, 0.7),
        }
    }
    
    /// Convert from the arena index to theme type (matches existing codebase indices)
    pub fn from_arena_index(index: u8) -> Option<Self> {
        match index {
            0 => Some(ArenaTheme::GuildHouse),
            1 => Some(ArenaTheme::Pawnshop),
            2 => Some(ArenaTheme::Crucible),
            3 => Some(ArenaTheme::Sanctum),
            4 => Some(ArenaTheme::Bastion),
            5 => Some(ArenaTheme::Labyrinth),
            6 => Some(ArenaTheme::Mountain),
            7 => Some(ArenaTheme::Casino),
            8 => Some(ArenaTheme::Gala),
            _ => None,
        }
    }
}

/// System that automatically sets up lighting when arenas are created
/// Corrected to work with the existing Arena component and ArenaIndex
pub fn setup_arena_lighting(
    mut commands: Commands,
    // Find arenas that were just created but don't have lighting yet
    new_arenas: Query<(Entity, &ArenaIndex), (Added<Arena>, Without<ArenaLighting>)>,
) {
    for (arena_entity, arena_index) in new_arenas.iter() {
        // Get the theme for this arena based on its index
        let theme = ArenaTheme::from_arena_index(arena_index.get_index())
            .unwrap_or(ArenaTheme::GuildHouse);

        // Get binary zoom intensity profile for this theme
        let (single_intensity, overview_intensity) = theme.zoom_intensity_profile();

        // Create the main ambient light for this arena
        let ambient_light = commands.spawn((
            PointLight {
                intensity: 500.0,
                range: 200.0,
                color: theme.ambient_color(),
                shadows_enabled: false, // 2D doesn't need shadows
                ..default()
            },
            Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)),
        )).id();

        // Create theme-specific accent lighting
        let accent_lights = create_theme_accent_lights(&mut commands, &theme);
        
        // Combine ambient + accent lights
        let mut all_lights = vec![ambient_light];
        all_lights.extend(accent_lights);

        // Attach the lighting component to the arena
        commands.entity(arena_entity).insert(ArenaLighting {
            theme,
            functional_priority: false, // Start with atmosphere, switch during combat
            ambient_lights: all_lights,
            emergency_override: false,
            single_arena_intensity: single_intensity,
            overview_intensity: overview_intensity,
        });
    }
}

/// Creates additional lights that give each arena its unique personality
fn create_theme_accent_lights(commands: &mut Commands, theme: &ArenaTheme) -> Vec<Entity> {
    match theme {
        ArenaTheme::Casino => {
            // Glittering accent lights for luxury feel
            let mut lights = Vec::new();
            for i in 0..6 {
                let angle = (i as f32 / 6.0) * std::f32::consts::TAU;
                let radius = 150.0;
                let pos = Vec2::new(radius * angle.cos(), radius * angle.sin());
                
                let light = commands.spawn((
                    PointLight {
                        intensity: 200.0,
                        range: 50.0,
                        color: Color::srgb(1.0, 0.9, 0.6), // Warm gold
                        shadows_enabled: false,
                        ..default()
                    },
                    Transform::from_translation(Vec3::new(pos.x, pos.y, 3.0)),
                )).id();
                lights.push(light);
            }
            lights
        },
        ArenaTheme::Crucible => {
            // Industrial heat sources
            vec![
                commands.spawn((
                    PointLight {
                        intensity: 800.0,
                        range: 100.0,
                        color: Color::srgb(1.0, 0.3, 0.1), // Hot orange-red
                        shadows_enabled: false,
                        ..default()
                    },
                    Transform::from_translation(Vec3::new(-100.0, 0.0, 4.0)),
                )).id(),
                commands.spawn((
                    PointLight {
                        intensity: 800.0,
                        range: 100.0,
                        color: Color::srgb(1.0, 0.3, 0.1),
                        shadows_enabled: false,
                        ..default()
                    },
                    Transform::from_translation(Vec3::new(100.0, 0.0, 4.0)),
                )).id(),
            ]
        },
        // For this example, we'll implement Casino and Crucible as examples
        // Other themes follow similar patterns
        _ => Vec::new(),
    }
}

/// Corrected binary arena lighting management that integrates with existing camera system
pub fn manage_binary_arena_lighting(
    mut arena_lighting: Query<(&mut ArenaLighting, &ArenaIndex)>,
    mut lights: Query<&mut PointLight>,
    camera: Query<&Projection, With<Camera>>,
    current_arena: Query<&CurrentArena>,
    lighting_manager: Res<LightingManager>,
) {
    if let Ok(projection) = camera.get_single() {
        if let Projection::Orthographic(ortho) = projection {
            let is_overview_mode = (ortho.scale - 3.0).abs() < 0.1; // Scale 3.0 = overview
            let focused_arena_index = current_arena.get_single().map(|a| a.0).unwrap_or(0);
            
            for (mut lighting, arena_index) in arena_lighting.iter_mut() {
                let is_focused_arena = arena_index.get_index() == focused_arena_index;
                
                // Binary decision: should this arena have lighting?
                let should_have_lighting = match (lighting_manager.performance_level, is_overview_mode) {
                    // Ultra: Always light everything
                    (PerformanceLevel::Ultra, _) => true,
                    
                    // High performance strategies differ by zoom state
                    (PerformanceLevel::High, false) => {
                        // Single arena mode: Only light the focused arena
                        is_focused_arena
                    },
                    (PerformanceLevel::High, true) => {
                        // Overview mode: Light all arenas but with reduced intensity
                        true
                    },
                    
                    // Low performance strategies
                    (PerformanceLevel::Low, false) => {
                        // Single arena mode: Only the focused arena gets minimal lighting
                        is_focused_arena
                    },
                    (PerformanceLevel::Low, true) => {
                        // Overview mode: No atmospheric lighting, only functional
                        false
                    },
                    
                    // Emergency: Minimal lighting regardless of zoom
                    (PerformanceLevel::Emergency, _) => false,
                };
                
                // Determine intensity multiplier for active lighting
                let intensity_multiplier = if should_have_lighting {
                    match (lighting_manager.performance_level, is_overview_mode) {
                        // Ultra: Use arena's preferred intensity profile
                        (PerformanceLevel::Ultra, false) => lighting.single_arena_intensity,
                        (PerformanceLevel::Ultra, true) => lighting.overview_intensity,
                        
                        // High: Balanced approach
                        (PerformanceLevel::High, false) => lighting.single_arena_intensity,
                        (PerformanceLevel::High, true) => lighting.overview_intensity * 0.8,
                        
                        // Low: Minimal but functional
                        (PerformanceLevel::Low, false) => 0.5,
                        (PerformanceLevel::Low, true) => 0.2,
                        
                        (PerformanceLevel::Emergency, _) => 0.0,
                    }
                } else {
                    0.0
                };
                
                // Apply lighting changes to all lights in this arena
                for &light_entity in &lighting.ambient_lights {
                    if let Ok(mut light) = lights.get_mut(light_entity) {
                        if intensity_multiplier > 0.0 {
                            // Calculate base intensity for this light type based on range
                            let base_intensity = match light.range.round() as u32 {
                                200 => 500.0, // Main ambient light
                                100 => 800.0, // Theme accent lights (industrial)
                                50 => 200.0,  // Theme accent lights (decorative)
                                _ => 300.0,   // Default fallback
                            };
                            
                            light.intensity = base_intensity * intensity_multiplier;
                        } else {
                            light.intensity = 0.0; // Turn off completely
                        }
                    }
                }
                
                // Update arena state
                lighting.functional_priority = should_have_lighting;
            }
        }
    }
}