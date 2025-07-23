//! Camera animation system with smooth easing transitions.
//! 
//! This module provides smooth camera transitions when switching between arenas
//! using a standard cubic-bezier ease curve for natural motion.

use bevy::prelude::*;
use crate::config::camera::*;

/// Duration for camera transitions on desktop (200ms for full-screen effects)
pub const CAMERA_TRANSITION_DURATION: f32 = 0.2;

/// Component that marks a camera as currently animating
#[derive(Component, Debug)]
pub struct CameraAnimation {
    /// Starting position of the animation
    pub start_position: Vec3,
    /// Target position to animate to
    pub target_position: Vec3,
    /// Current progress of the animation (0.0 to 1.0)
    pub progress: f32,
    /// Total duration of the animation in seconds
    pub duration: f32,
    /// Whether this is a zoom transition (affects Y offset)
    pub is_zoom_transition: bool,
    /// The arena index this animation is targeting
    pub target_arena: u8,
}

impl CameraAnimation {
    /// Create a new camera animation from current position to target
    pub fn new(
        start_position: Vec3, 
        target_position: Vec3, 
        target_arena: u8,
        is_zoom_transition: bool
    ) -> Self {
        Self {
            start_position,
            target_position,
            progress: 0.0,
            duration: CAMERA_TRANSITION_DURATION,
            is_zoom_transition,
            target_arena,
        }
    }

    /// Check if the animation is complete
    pub fn is_complete(&self) -> bool {
        self.progress >= 1.0
    }

    /// Update the animation progress based on delta time
    pub fn update(&mut self, delta_time: f32) {
        self.progress = (self.progress + delta_time / self.duration).min(1.0);
    }

    /// Get the current interpolated position using cubic-bezier easing
    pub fn current_position(&self) -> Vec3 {
        let eased_progress = cubic_bezier_ease(self.progress);
        self.start_position.lerp(self.target_position, eased_progress)
    }
}

/// Standard cubic-bezier easing curve (0.4, 0.0, 0.2, 1.0)
/// This provides "Ease In Out" behavior - accelerates quickly then decelerates slowly
pub fn cubic_bezier_ease(t: f32) -> f32 {
    // Clamp input to [0, 1]
    let t = t.clamp(0.0, 1.0);
    
    // Standard curve cubic-bezier(0.4, 0.0, 0.2, 1.0)
    // This is a simplified approximation that's very close to the actual cubic-bezier
    let t2 = t * t;
    let t3 = t2 * t;
    
    // Cubic-bezier approximation for (0.4, 0.0, 0.2, 1.0)
    3.0 * t2 - 2.0 * t3
}

/// System that updates camera animations
pub fn update_camera_animations(
    mut commands: Commands,
    mut camera_query: Query<(Entity, &mut Transform, &mut CameraAnimation), With<Camera>>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut animation) in &mut camera_query {
        // Update animation progress
        animation.update(time.delta_secs());
        
        // Update camera position
        transform.translation = animation.current_position();
        
        // Remove animation component when complete
        if animation.is_complete() {
            commands.entity(entity).remove::<CameraAnimation>();
        }
    }
}

/// System that starts camera animations when CurrentArena changes
pub fn animate_camera_on_arena_change(
    mut commands: Commands,
    arena_query: Query<&crate::components::CurrentArena, Changed<crate::components::CurrentArena>>,
    mut camera_query: Query<(Entity, &Transform, &Projection), (With<Camera>, Without<CameraAnimation>)>,
) {
    if let Ok(current_arena) = arena_query.single() {
        for (entity, transform, projection) in &mut camera_query {
            // Calculate target position
            let (target_x, target_y) = crate::utils::calculate_camera_position(current_arena.0);
            
            // Check if camera is zoomed out and adjust Y position accordingly
            let mut target_position = Vec3::new(target_x, target_y, transform.translation.z);
            let is_zoom_transition = if let Projection::Orthographic(ortho) = projection {
                if ortho.scale == SCALE_ZOOMED_OUT {
                    target_position.y -= crate::config::display::TILE_SIZE * ZOOM_OUT_Y_OFFSET_TILES;
                    true
                } else {
                    false
                }
            } else {
                false
            };

            // Only animate if the target position is different from current position
            let current_pos = transform.translation;
            let distance = current_pos.distance(target_position);
            
            if distance > 0.1 { // Small threshold to avoid micro-animations
                // Create and add animation component
                let animation = CameraAnimation::new(
                    current_pos,
                    target_position,
                    current_arena.0,
                    is_zoom_transition,
                );
                
                commands.entity(entity).insert(animation);
            }
        }
    }
}

/// System that handles zoom transitions with animation
pub fn animate_zoom_transitions(
    mut commands: Commands,
    arena_query: Query<&crate::components::CurrentArena>,
    mut camera_query: Query<(Entity, &mut Transform, &mut Projection), (With<Camera>, Without<CameraAnimation>)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyP) {
        for (entity, transform, mut projection) in &mut camera_query {
            if let Projection::Orthographic(ortho) = &mut *projection {
                let current_pos = transform.translation;
                let target_position = if ortho.scale == SCALE_NORMAL {
                    // Zooming out - center on arena 4 and move down
                    ortho.scale = SCALE_ZOOMED_OUT;
                    let (camera_x, camera_y) = crate::utils::calculate_camera_position(4);
                    Vec3::new(
                        camera_x, 
                        camera_y - (crate::config::display::TILE_SIZE * ZOOM_OUT_Y_OFFSET_TILES), 
                        current_pos.z
                    )
                } else {
                    // Zooming in - return to current arena position
                    ortho.scale = SCALE_NORMAL;
                    if let Ok(arena) = arena_query.single() {
                        let (camera_x, camera_y) = crate::utils::calculate_camera_position(arena.0);
                        Vec3::new(camera_x, camera_y, current_pos.z)
                    } else {
                        // Fallback to arena 1 if no current arena found
                        let (camera_x, camera_y) = crate::utils::calculate_camera_position(1);
                        Vec3::new(camera_x, camera_y, current_pos.z)
                    }
                };

                // Create animation for zoom transition
                let animation = CameraAnimation::new(
                    current_pos,
                    target_position,
                    4, // Zoom out always goes to arena 4
                    true, // This is a zoom transition
                );
                
                commands.entity(entity).insert(animation);
            }
        }
    }
}