use bevy::prelude::*;
use crate::character::Character;

/// Selection highlight component that works with the existing Character struct
#[derive(Component)]
pub struct SelectionHighlight {
    pub light_entity: Entity,
    pub pulse_timer: Timer,
    pub base_intensity: f32,
    pub emergency_boost: f32, // Extra brightness when health is critical
}

#[derive(Component)]
pub struct Selected(pub bool);

/// Health component that we'll add to characters for lighting purposes
/// Since the existing Character struct has private health field, we need this
#[derive(Component)]
pub struct CharacterHealth {
    pub current: u32,
    pub maximum: u32,
}

impl CharacterHealth {
    pub fn new(current: u32, maximum: u32) -> Self {
        Self { current, maximum }
    }
    
    pub fn health_ratio(&self) -> f32 {
        self.current as f32 / self.maximum as f32
    }
    
    pub fn is_critical(&self) -> bool {
        self.health_ratio() < 0.25
    }
}

/// Creates selection highlighting for characters that don't have it yet
/// Works with the existing Character component from the codebase
pub fn add_selection_highlighting(
    mut commands: Commands,
    // Find characters without selection highlighting
    characters_needing_lights: Query<Entity, (With<Character>, Without<SelectionHighlight>)>,
) {
    for character_entity in characters_needing_lights.iter() {
        // Create a bright white light that will pulse when selected
        let light_entity = commands.spawn((
            PointLight {
                intensity: 0.0, // Start dim, will be controlled by selection state
                range: 35.0,    // Slightly larger than character sprite
                color: Color::WHITE,
                shadows_enabled: false,
                ..default()
            },
            // Position slightly above character for proper layering
            Transform::from_translation(Vec3::new(0.0, 0.0, 8.0)),
        )).id();

        // Attach the selection highlight to the character
        commands.entity(character_entity).insert((
            SelectionHighlight {
                light_entity,
                pulse_timer: Timer::from_seconds(1.2, TimerMode::Repeating),
                base_intensity: 400.0,
                emergency_boost: 0.0,
            },
            Selected(false),
        ));
        
        // Add health component if character doesn't have one
        // This allows the lighting system to respond to health changes
        if !commands.get_entity(character_entity).unwrap().contains::<CharacterHealth>() {
            commands.entity(character_entity).insert(CharacterHealth::new(100, 100));
        }
    }
}

/// Updates selection highlighting with corrected health integration and binary zoom awareness
pub fn update_character_selection_lighting(
    mut highlights: Query<(&mut SelectionHighlight, &Selected, Option<&CharacterHealth>)>,
    mut lights: Query<&mut PointLight>,
    time: Res<Time>,
    lighting_manager: Res<LightingManager>,
) {
    for (mut highlight, selected, health_opt) in highlights.iter_mut() {
        // Update the pulse timer
        highlight.pulse_timer.tick(time.delta());
        
        // Calculate emergency boost for low health characters
        if let Some(health) = health_opt {
            if health.is_critical() {
                highlight.emergency_boost = (0.25 - health.health_ratio()) * 4.0; // 0.0 to 1.0 boost
            } else {
                highlight.emergency_boost = 0.0;
            }
        } else {
            highlight.emergency_boost = 0.0;
        }
        
        // Binary zoom intensity adjustment - use precise comparison
        let zoom_multiplier = if (lighting_manager.camera_zoom_scale - 3.0).abs() < 0.1 {
            1.2 // Boost intensity when all 9 arenas visible to maintain visibility
        } else {
            1.0 // Normal intensity when focused on single arena
        };
        
        // Update the actual light
        if let Ok(mut light) = lights.get_mut(highlight.light_entity) {
            if selected.0 {
                // Selected: Pulsing bright white with emergency boost and zoom adjustment
                let pulse_factor = (highlight.pulse_timer.elapsed_secs() * 3.0).sin() * 0.3 + 0.7;
                let base_intensity = highlight.base_intensity * (1.0 + highlight.emergency_boost);
                light.intensity = base_intensity * pulse_factor * zoom_multiplier;
                light.color = Color::WHITE;
            } else if highlight.emergency_boost > 0.0 {
                // Not selected but low health: Pulsing red emergency light
                let pulse_factor = (highlight.pulse_timer.elapsed_secs() * 5.0).sin().abs();
                light.intensity = 300.0 * pulse_factor * highlight.emergency_boost * zoom_multiplier;
                light.color = Color::srgb(1.0, 0.2, 0.2); // Emergency red
            } else {
                // Normal state: Subtle ambient glow for character identity
                light.intensity = 100.0 * zoom_multiplier;
                light.color = Color::srgb(0.8, 0.8, 0.9); // Subtle blue-white
            }
        }
    }
}

/// Ghost lighting system for recording overlaps - corrected to work with existing ArenaIndex
#[derive(Component)]
pub struct GhostLighting {
    pub recording_age: u32,        // How many cycles old this recording is
    pub base_light: Entity,        // The character's base lighting
    pub depth_factor: f32,         // 0.0 = oldest, 1.0 = current recording
    pub emergency_override: bool,  // Forces visibility regardless of age
}

/// Updates ghost lighting intensity based on recording age and health status
/// Properly integrates with binary zoom system for optimal visibility
pub fn update_ghost_lighting_depth(
    mut ghost_lighting: Query<(&mut GhostLighting, Option<&CharacterHealth>, &Selected)>,
    mut lights: Query<&mut PointLight>,
    time: Res<Time>,
    lighting_manager: Res<LightingManager>,
) {
    // Binary zoom adjustment for ghost visibility - use precise comparison
    let overview_boost = if (lighting_manager.camera_zoom_scale - 3.0).abs() < 0.1 { 
        1.4 
    } else { 
        1.0 
    };
    
    for (mut ghost, health_opt, selected) in ghost_lighting.iter_mut() {
        // Calculate depth factor (newer recordings are brighter)
        ghost.depth_factor = (1.0 - (ghost.recording_age as f32 * 0.08)).max(0.15);
        
        // Health emergency overrides aging
        if let Some(health) = health_opt {
            if health.is_critical() {
                ghost.emergency_override = true;
                ghost.depth_factor = ghost.depth_factor.max(0.8); // Force high visibility
            } else {
                ghost.emergency_override = false;
            }
        }
        
        // Selection overrides everything
        if selected.0 {
            ghost.depth_factor = 1.0;
        }
        
        // Apply to the actual light
        if let Ok(mut light) = lights.get_mut(ghost.base_light) {
            // Base intensity adjusted by depth and binary zoom
            let base_intensity = 300.0 * ghost.depth_factor * overview_boost;
            
            // Add pulsing for emergency health
            if ghost.emergency_override {
                let pulse = (time.elapsed_seconds() * 4.0).sin().abs();
                light.intensity = base_intensity * (0.7 + 0.3 * pulse);
                light.color = Color::srgb(1.0, 0.3, 0.3); // Emergency red tint
            } else {
                light.intensity = base_intensity;
                // Older ghosts get cooler, more desaturated colors
                let warmth = ghost.depth_factor;
                light.color = Color::srgb(
                    0.8 + 0.2 * warmth,     // Red component
                    0.8 + 0.1 * warmth,     // Green component  
                    0.9,                    // Blue stays high (cooler for older)
                );
            }
        }
    }
}