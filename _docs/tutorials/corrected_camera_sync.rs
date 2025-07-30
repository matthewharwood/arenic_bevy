use bevy::prelude::*;
use crate::arena::CurrentArena;

/// Synchronizes lighting manager with camera zoom changes
/// Properly integrates with the existing camera system from camera.rs
pub fn sync_lighting_with_camera_zoom(
    camera_query: Query<&Projection, (With<Camera>, Changed<Projection>)>,
    mut lighting_manager: ResMut<LightingManager>,
    current_arena: Query<&CurrentArena>,
) {
    for projection in camera_query.iter() {
        if let Projection::Orthographic(ortho) = projection {
            let old_scale = lighting_manager.camera_zoom_scale;
            lighting_manager.camera_zoom_scale = ortho.scale;
            
            // Update focused arena based on zoom state and current arena
            if let Ok(arena) = current_arena.get_single() {
                if (ortho.scale - 1.0).abs() < 0.1 {
                    // Single arena mode - focus on current arena
                    lighting_manager.focused_arena = Some(ArenaIndex(arena.0));
                } else {
                    // Overview mode - no specific focus
                    lighting_manager.focused_arena = None;
                }
            }
            
            // Log zoom state changes for debugging - use precise comparison
            let old_overview = (old_scale - 3.0).abs() < 0.1;
            let new_overview = (ortho.scale - 3.0).abs() < 0.1;
            
            if old_overview != new_overview {
                let mode = if new_overview { 
                    "overview (all 9 arenas)" 
                } else { 
                    "focused (single arena)" 
                };
                info!("Camera zoom changed to {} mode (scale: {:.1})", mode, ortho.scale);
                
                // Force immediate performance re-evaluation when zoom changes
                lighting_manager.frame_time_history.clear();
            }
        }
    }
}

/// System to handle emergency lighting modes
/// Integrates with the existing camera and arena systems
pub fn apply_emergency_lighting(
    mut lighting_manager: ResMut<LightingManager>,
    boss_telegraphs: Query<&BossTelegraph>,
    critical_health_characters: Query<&CharacterHealth>,
    current_arena: Query<&CurrentArena>,
    camera: Query<&Projection, With<Camera>>,
) {
    let mut should_enable_emergency = false;
    
    // Check for multiple simultaneous boss attacks
    let active_telegraphs = boss_telegraphs.iter()
        .filter(|t| !matches!(t.current_phase, TelegraphPhase::Idle))
        .count();
    
    if active_telegraphs >= 2 {
        should_enable_emergency = true;
    }
    
    // Check for multiple critical health characters in view
    let critical_count = critical_health_characters.iter()
        .filter(|health| health.is_critical())
        .count();
    
    // In overview mode, be more aggressive about emergency lighting
    if let Ok(projection) = camera.get_single() {
        if let Projection::Orthographic(ortho) = projection {
            let is_overview = (ortho.scale - 3.0).abs() < 0.1;
            
            if is_overview && (critical_count >= 3 || active_telegraphs >= 1) {
                should_enable_emergency = true;
            } else if !is_overview && critical_count >= 2 {
                should_enable_emergency = true;
            }
        }
    }
    
    // Update emergency mode
    if should_enable_emergency != lighting_manager.emergency_mode {
        lighting_manager.emergency_mode = should_enable_emergency;
        if should_enable_emergency {
            info!("Emergency lighting mode activated: {} boss attacks, {} critical health characters", 
                  active_telegraphs, critical_count);
        } else {
            info!("Emergency lighting mode deactivated");
        }
    }
}

use crate::recording::ArenaIndex;

/// Boss telegraph definitions that work with the binary zoom system
#[derive(Component)]
pub struct BossTelegraph {
    pub attack_type: AttackType,
    pub current_phase: TelegraphPhase,
    pub phase_timer: Timer,
    pub light_entities: Vec<Entity>,
    pub audio_sync_offset: f32, // Keeps visuals synchronized with audio cues
}

#[derive(Debug, Clone)]
pub enum AttackType {
    SingleTarget { 
        target_pos: Vec2,
        damage_type: DamageType,
    },
    AoeCircle { 
        center: Vec2, 
        radius: f32,
        damage_type: DamageType,
    },
    AoeLine { 
        start: Vec2, 
        end: Vec2, 
        width: f32,
        damage_type: DamageType,
    },
    Environmental { 
        affected_tiles: Vec<Vec2>,
        damage_type: DamageType,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum TelegraphPhase {
    Buildup(f32),   // 0.0 to 1.0 progress - subtle environmental shift
    Warning(f32),   // 0.0 to 1.0 progress - clear geometric telegraph
    Danger(f32),    // 0.0 to 1.0 progress - urgent pulsing
    Execution(f32), // 0.0 to 1.0 progress - bright flash + aftermath
    Idle,           // No active telegraph
}

#[derive(Debug, Clone, Copy)]
pub enum DamageType {
    Physical,   // Orange-red colors
    Magical,    // Blue-purple colors  
    Fire,       // Bright red-orange
    Ice,        // Cyan-blue
    Poison,     // Toxic green
    Death,      // Dark magenta/purple
}

impl DamageType {
    pub fn telegraph_color(&self) -> Color {
        match self {
            DamageType::Physical => Color::srgb(1.0, 0.4, 0.2),  // Orange-red
            DamageType::Magical =>  Color::srgb(0.4, 0.2, 1.0),  // Blue-purple
            DamageType::Fire =>     Color::srgb(1.0, 0.2, 0.0),  // Bright red
            DamageType::Ice =>      Color::srgb(0.2, 0.8, 1.0),  // Cyan
            DamageType::Poison =>   Color::srgb(0.4, 1.0, 0.2),  // Toxic green
            DamageType::Death =>    Color::srgb(0.8, 0.0, 0.8),  // Dark magenta
        }
    }
}

impl AttackType {
    /// Get the damage type for lighting calculations
    pub fn damage_type(&self) -> DamageType {
        match self {
            AttackType::SingleTarget { damage_type, .. } => *damage_type,
            AttackType::AoeCircle { damage_type, .. } => *damage_type,
            AttackType::AoeLine { damage_type, .. } => *damage_type,
            AttackType::Environmental { damage_type, .. } => *damage_type,
        }
    }
}