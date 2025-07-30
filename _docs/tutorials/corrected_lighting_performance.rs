use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use std::collections::VecDeque;
use crate::recording::ArenaIndex;

/// Performance monitoring that properly integrates with Bevy 0.16's diagnostic system
#[derive(Resource)]
pub struct PerformanceMonitor {
    /// Rolling average of frame times in milliseconds
    pub frame_time_samples: VecDeque<f32>,
    /// How many lights are currently active
    pub active_light_count: u32,
    /// Target frame time in milliseconds (16.67ms = 60fps)
    pub target_frame_time: f32,
    /// When we last adjusted performance level
    pub last_adjustment: f32,
    /// Minimum time between performance adjustments (prevents oscillation)
    pub adjustment_cooldown: f32,
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self {
            frame_time_samples: VecDeque::with_capacity(120), // 2 seconds at 60fps
            active_light_count: 0,
            target_frame_time: 16.67, // 60fps
            last_adjustment: 0.0,
            adjustment_cooldown: 2.0, // 2 second cooldown
        }
    }
}

/// Corrected performance monitoring system that properly integrates with Bevy 0.16
pub fn monitor_lighting_performance(
    mut performance_monitor: ResMut<PerformanceMonitor>,
    mut lighting_manager: ResMut<LightingManager>,
    diagnostics: Res<DiagnosticsStore>, // Correct Bevy 0.16 resource
    time: Res<Time>,
    active_lights: Query<&PointLight>,
) {
    // Update light count
    performance_monitor.active_light_count = active_lights.iter().count() as u32;
    
    // Sample current frame time using proper Bevy 0.16 API
    if let Some(frame_time_diag) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME) {
        if let Some(current_frame_time) = frame_time_diag.average() {
            performance_monitor.frame_time_samples.push_back(current_frame_time as f32);
            
            // Keep only recent samples
            if performance_monitor.frame_time_samples.len() > 120 {
                performance_monitor.frame_time_samples.pop_front();
            }
        }
    }
    
    // Check if it's time to adjust performance
    let current_time = time.elapsed_seconds();
    if current_time - performance_monitor.last_adjustment > performance_monitor.adjustment_cooldown {
        if should_adjust_performance(&performance_monitor, &lighting_manager) {
            adjust_performance_level(&mut lighting_manager, &performance_monitor);
            performance_monitor.last_adjustment = current_time;
        }
    }
}

/// The central lighting manager with corrected binary zoom logic
#[derive(Resource)]
pub struct LightingManager {
    /// Current performance level - automatically adjusts based on zoom state and frame rate
    pub performance_level: PerformanceLevel,
    /// Which arena gets full lighting attention - uses existing ArenaIndex type
    pub focused_arena: Option<ArenaIndex>,
    /// Emergency override - forces maximum visibility for critical situations
    pub emergency_mode: bool,
    /// Current camera zoom scale (1.0 = single arena, 3.0 = all arenas)
    pub camera_zoom_scale: f32,
    /// Performance tracking for auto-optimization
    pub frame_time_history: VecDeque<f32>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PerformanceLevel {
    /// Maximum quality - all effects enabled
    Ultra,
    /// Balanced quality - reduced atmospheric lighting when showing all 9 arenas
    High,
    /// Minimal lighting - emergency health + selection + boss telegraphs only
    Low,
    /// Absolute minimum - only critical survival information
    Emergency,
}

impl Default for LightingManager {
    fn default() -> Self {
        Self {
            performance_level: PerformanceLevel::Ultra,
            focused_arena: None,
            emergency_mode: false,
            camera_zoom_scale: 1.0, // Default to zoomed-in view
            frame_time_history: VecDeque::with_capacity(60), // 1 second of history
        }
    }
}

/// Determines if performance adjustment is needed, with corrected binary zoom logic
fn should_adjust_performance(monitor: &PerformanceMonitor, lighting_manager: &LightingManager) -> bool {
    if monitor.frame_time_samples.len() < 60 {
        return false; // Need enough samples for reliable measurement
    }
    
    let avg_frame_time: f32 = monitor.frame_time_samples.iter().sum::<f32>() 
        / monitor.frame_time_samples.len() as f32;
    
    // Binary zoom system: different performance expectations based on exact scale values
    let target_multiplier = if (lighting_manager.camera_zoom_scale - 3.0).abs() < 0.1 {
        1.4 // More lenient target when showing all 9 arenas (scale 3.0)
    } else {
        1.0 // Strict target when showing single arena (scale 1.0)
    };
    
    let adjusted_target = monitor.target_frame_time * target_multiplier;
    
    // Adjust if consistently above or below target with hysteresis
    avg_frame_time > adjusted_target * 1.15 || // 15% over adjusted target
    avg_frame_time < adjusted_target * 0.85    // 15% under adjusted target (can upgrade)
}

/// Adjusts performance level with proper binary zoom state detection
fn adjust_performance_level(
    lighting_manager: &mut LightingManager,
    monitor: &PerformanceMonitor,
) {
    let avg_frame_time: f32 = monitor.frame_time_samples.iter().sum::<f32>() 
        / monitor.frame_time_samples.len() as f32;
    
    // Binary zoom consideration - use precise float comparison
    let is_overview_mode = (lighting_manager.camera_zoom_scale - 3.0).abs() < 0.1;
    let target_multiplier = if is_overview_mode { 1.4 } else { 1.0 };
    let adjusted_target = monitor.target_frame_time * target_multiplier;
    
    if avg_frame_time > adjusted_target * 1.15 {
        // Performance is poor, reduce quality
        lighting_manager.performance_level = match lighting_manager.performance_level {
            PerformanceLevel::Ultra => PerformanceLevel::High,
            PerformanceLevel::High => PerformanceLevel::Low,
            PerformanceLevel::Low => PerformanceLevel::Emergency,
            PerformanceLevel::Emergency => PerformanceLevel::Emergency, // Can't go lower
        };
        info!("Lighting performance reduced to {:?} (frame time: {:.2}ms, overview mode: {})", 
              lighting_manager.performance_level, avg_frame_time, is_overview_mode);
    } else if avg_frame_time < adjusted_target * 0.85 {
        // Performance is good, can increase quality
        lighting_manager.performance_level = match lighting_manager.performance_level {
            PerformanceLevel::Emergency => PerformanceLevel::Low,
            PerformanceLevel::Low => PerformanceLevel::High,
            PerformanceLevel::High => PerformanceLevel::Ultra,
            PerformanceLevel::Ultra => PerformanceLevel::Ultra, // Already at max
        };
        info!("Lighting performance increased to {:?} (frame time: {:.2}ms, overview mode: {})", 
              lighting_manager.performance_level, avg_frame_time, is_overview_mode);
    }
}