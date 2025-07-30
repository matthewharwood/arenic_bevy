use bevy::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

// Import all the corrected systems
use crate::lighting::{
    LightingManager, PerformanceMonitor,
    setup_arena_lighting, manage_binary_arena_lighting,
    add_selection_highlighting, update_character_selection_lighting,
    update_ghost_lighting_depth, sync_lighting_with_camera_zoom,
    apply_emergency_lighting, monitor_lighting_performance,
    animate_boss_telegraphs,
};

/// Complete lighting plugin that properly integrates with the existing Arenic codebase
pub struct ArenicLightingPlugin;

impl Plugin for ArenicLightingPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize core resources
            .init_resource::<LightingManager>()
            .init_resource::<PerformanceMonitor>()
            
            // Ensure frame time diagnostics are available for performance monitoring
            .add_plugins(FrameTimeDiagnosticsPlugin)
            
            // Core lighting systems that run every frame
            .add_systems(
                Update,
                (
                    // Performance monitoring (highest priority) 
                    monitor_lighting_performance,
                    
                    // Camera synchronization (must run before other lighting systems)
                    sync_lighting_with_camera_zoom,
                    
                    // Arena management with binary zoom awareness
                    setup_arena_lighting,
                    manage_binary_arena_lighting,
                    
                    // Character lighting with zoom-aware intensity
                    add_selection_highlighting,
                    update_character_selection_lighting,
                    update_ghost_lighting_depth,
                    
                    // Boss telegraphs with binary zoom boost
                    animate_boss_telegraphs,
                    
                    // Emergency systems (runs last to override other lighting)
                    apply_emergency_lighting,
                ).chain() // Run in order to avoid frame lag and ensure proper dependencies
            );
    }
}

/// Integration instructions for main.rs
/// 
/// To integrate this lighting system with the existing Arenic codebase:
/// 
/// 1. Add to main.rs imports:
/// ```rust
/// use crate::lighting::ArenicLightingPlugin;
/// ```
/// 
/// 2. Add to the plugin chain in main():
/// ```rust
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins.set(WindowPlugin {
///             primary_window: Some(Window {
///                 title: GAME_NAME.to_string(),
///                 resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
///                 ..default()
///             }),
///             ..default()
///         }))
///         .add_plugins(GameStatePlugin)
///         .add_plugins(CameraPlugin)
///         .add_plugins(RecordingPlugin)
///         .add_plugins(ArenicLightingPlugin) // Add this line
///         .add_plugins(UiPlugin)
///         .run();
/// }
/// ```
/// 
/// 3. Create a new file src/lighting.rs with all the corrected implementations
/// 
/// 4. Add to src/main.rs module declarations:
/// ```rust
/// mod lighting;
/// ```

/// Example usage for spawning a character with lighting support
pub fn spawn_character_with_lighting(
    commands: &mut Commands,
    asset_server: &AssetServer,
    position: Vec3,
    name: String,
) -> Entity {
    commands.spawn((
        // Existing character components
        Character::new(name, 100, 1),
        Transform::from_translation(position),
        Sprite {
            image: asset_server.load("character_sprite.png"),
            ..default()
        },
        
        // Lighting support components
        CharacterHealth::new(100, 100),
        // SelectionHighlight will be added automatically by add_selection_highlighting system
    )).id()
}

/// Example usage for spawning an arena with lighting
pub fn spawn_arena_with_lighting(
    commands: &mut Commands,
    arena_index: u8,
    position: Vec3,
) -> Entity {
    use crate::recording::ArenaIndex;
    use crate::arena::Arena;
    
    let arena_index_component = ArenaIndex::new(arena_index)
        .unwrap_or_else(|_| ArenaIndex(0));
    
    commands.spawn((
        // Existing arena components
        Arena,
        arena_index_component,
        Transform::from_translation(position),
        
        // ArenaLighting will be added automatically by setup_arena_lighting system
    )).id()
}

/// Example boss telegraph creation
pub fn create_boss_telegraph(
    commands: &mut Commands,
    attack_type: AttackType,
    position: Vec3,
) -> Entity {
    // Create the main telegraph light
    let telegraph_light = commands.spawn((
        PointLight {
            intensity: 0.0, // Will be controlled by animation system
            range: 100.0,
            color: attack_type.damage_type().telegraph_color(),
            shadows_enabled: false,
            ..default()
        },
        Transform::from_translation(position),
    )).id();
    
    // Create the telegraph component
    commands.spawn(BossTelegraph {
        attack_type,
        current_phase: TelegraphPhase::Buildup(0.0),
        phase_timer: Timer::from_seconds(2.0, TimerMode::Once),
        light_entities: vec![telegraph_light],
        audio_sync_offset: 0.0,
    }).id()
}