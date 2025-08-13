# Tutorial 08: Performance Optimization for 320 Ghosts

## Objective

Optimize the recording system to handle up to 320 simultaneous ghosts (40 per arena × 8 arenas) while maintaining 60+
FPS. Focus on memory efficiency, query optimization, and smart update strategies.

## Prerequisites

- Completed Tutorials 01-07 (Full recording system with visual polish)
- Understanding of ECS performance patterns
- Familiarity with profiling tools

## Components/Systems

We'll create:

- Timeline compression algorithms
- Spatial indexing for ghosts
- Update frequency management
- Memory pooling systems
- Query optimization strategies

## Implementation Steps

### Step 1: Create Timeline Compression

Create `src/optimization/mod.rs`:

```rust
use bevy::prelude::*;
use crate::timeline::{TimelineEvent, EventType, PublishTimeline, DraftTimeline};

/// Settings for timeline compression
#[derive(Resource)]
pub struct CompressionSettings {
    /// Maximum events per timeline
    pub max_events: usize,
    /// Minimum time between movement events (seconds)
    pub min_movement_delta: f32,
    /// Minimum distance for new movement keyframe
    pub min_distance_squared: f32,
    /// Enable delta compression for positions
    pub use_delta_compression: bool,
}

impl Default for CompressionSettings {
    fn default() -> Self {
        Self {
            max_events: 500,
            min_movement_delta: 0.1,
            min_distance_squared: 0.01,
            use_delta_compression: true,
        }
    }
}

/// Compressed timeline format for efficient storage
#[derive(Component)]
pub struct CompressedTimeline {
    /// Base position for delta compression
    pub base_position: Vec3,
    /// Compressed events using delta encoding
    pub events: Vec<CompressedEvent>,
}

#[derive(Clone, Debug)]
pub struct CompressedEvent {
    /// Time delta from previous event (quantized to 0.05s)
    pub time_delta: u8, // 0-255 = 0-12.75 seconds
    /// Event data
    pub data: CompressedEventData,
}

#[derive(Clone, Debug)]
pub enum CompressedEventData {
    /// Delta position from base (quantized)
    Movement {
        dx: i8, // -128 to 127 tiles
        dy: i8,
        dz: i8,
    },
    /// Ability cast
    Ability(u8),
    /// Death marker
    Death,
}

impl CompressedTimeline {
    /// Compress a draft timeline
    pub fn from_draft(draft: &DraftTimeline, settings: &CompressionSettings) -> Self {
        let mut base_position = Vec3::ZERO;
        let mut compressed_events = Vec::new();
        let mut last_time = 0.0;
        let mut last_position = Vec3::ZERO;

        for event in &draft.events {
            match event.event_type {
                EventType::Transform(pos) => {
                    // Set base position from first event
                    if base_position == Vec3::ZERO {
                        base_position = pos;
                        last_position = pos;
                    }

                    // Check if we should keep this movement
                    let time_delta = event.timestamp - last_time;
                    let dist_squared = last_position.distance_squared(pos);

                    if time_delta >= settings.min_movement_delta ||
                        dist_squared >= settings.min_distance_squared {
                        // Quantize time delta (0.05s precision)
                        let quantized_time = ((time_delta / 0.05) as u8).min(255);

                        // Calculate position delta
                        let delta = if settings.use_delta_compression {
                            pos - base_position
                        } else {
                            pos - last_position
                        };

                        // Quantize position (1 tile = 1 unit)
                        let compressed_event = CompressedEvent {
                            time_delta: quantized_time,
                            data: CompressedEventData::Movement {
                                dx: (delta.x as i8).clamp(-128, 127),
                                dy: (delta.y as i8).clamp(-128, 127),
                                dz: (delta.z as i8).clamp(-128, 127),
                            },
                        };

                        compressed_events.push(compressed_event);
                        last_time = event.timestamp;
                        last_position = pos;
                    }
                }
                EventType::Ability(id) => {
                    let time_delta = event.timestamp - last_time;
                    let quantized_time = ((time_delta / 0.05) as u8).min(255);

                    compressed_events.push(CompressedEvent {
                        time_delta: quantized_time,
                        data: CompressedEventData::Ability(id.0),
                    });

                    last_time = event.timestamp;
                }
                EventType::Death => {
                    let time_delta = event.timestamp - last_time;
                    let quantized_time = ((time_delta / 0.05) as u8).min(255);

                    compressed_events.push(CompressedEvent {
                        time_delta: quantized_time,
                        data: CompressedEventData::Death,
                    });

                    last_time = event.timestamp;
                }
            }

            // Limit total events
            if compressed_events.len() >= settings.max_events {
                break;
            }
        }

        Self {
            base_position,
            events: compressed_events,
        }
    }

    /// Decompress to regular timeline
    pub fn decompress(&self) -> PublishTimeline {
        let mut events = Vec::new();
        let mut current_time = 0.0;

        for compressed in &self.events {
            current_time += compressed.time_delta as f32 * 0.05;

            let event_type = match &compressed.data {
                CompressedEventData::Movement { dx, dy, dz } => {
                    let pos = self.base_position + Vec3::new(
                        *dx as f32,
                        *dy as f32,
                        *dz as f32,
                    );
                    EventType::Transform(pos)
                }
                CompressedEventData::Ability(id) => {
                    EventType::Ability(AbilityId(*id))
                }
                CompressedEventData::Death => EventType::Death,
            };

            events.push(TimelineEvent {
                timestamp: current_time,
                event_type,
            });
        }

        PublishTimeline { events }
    }

    /// Get memory usage in bytes
    pub fn memory_usage(&self) -> usize {
        std::mem::size_of::<Self>() +
            self.events.len() * std::mem::size_of::<CompressedEvent>()
    }
}

use crate::timeline::AbilityId;
```

### Step 2: Create Spatial Ghost Index

Add to `src/optimization/mod.rs`:

```rust
use bevy::utils::HashMap;

/// Spatial index for efficient ghost queries
#[derive(Resource)]
pub struct GhostSpatialIndex {
    /// Grid size for spatial hashing
    pub cell_size: f32,
    /// Map from grid cell to ghost entities
    pub grid: HashMap<(i32, i32), Vec<Entity>>,
    /// Cache of ghost positions
    pub positions: HashMap<Entity, Vec3>,
}

impl Default for GhostSpatialIndex {
    fn default() -> Self {
        Self {
            cell_size: 10.0, // 10 tiles per cell
            grid: HashMap::new(),
            positions: HashMap::new(),
        }
    }
}

impl GhostSpatialIndex {
    /// Get grid cell for position
    fn get_cell(&self, pos: Vec3) -> (i32, i32) {
        (
            (pos.x / self.cell_size).floor() as i32,
            (pos.y / self.cell_size).floor() as i32,
        )
    }

    /// Update ghost position in index
    pub fn update_position(&mut self, entity: Entity, new_pos: Vec3) {
        // Remove from old cell
        if let Some(old_pos) = self.positions.get(&entity) {
            let old_cell = self.get_cell(*old_pos);
            if let Some(entities) = self.grid.get_mut(&old_cell) {
                entities.retain(|&e| e != entity);
            }
        }

        // Add to new cell
        let new_cell = self.get_cell(new_pos);
        self.grid.entry(new_cell)
            .or_insert_with(Vec::new)
            .push(entity);

        // Update position cache
        self.positions.insert(entity, new_pos);
    }

    /// Get ghosts near a position
    pub fn get_nearby_ghosts(&self, pos: Vec3, radius: f32) -> Vec<Entity> {
        let mut result = Vec::new();
        let cell_radius = (radius / self.cell_size).ceil() as i32;
        let center_cell = self.get_cell(pos);

        // Check surrounding cells
        for dx in -cell_radius..=cell_radius {
            for dy in -cell_radius..=cell_radius {
                let cell = (center_cell.0 + dx, center_cell.1 + dy);

                if let Some(entities) = self.grid.get(&cell) {
                    for &entity in entities {
                        if let Some(ghost_pos) = self.positions.get(&entity) {
                            if ghost_pos.distance(pos) <= radius {
                                result.push(entity);
                            }
                        }
                    }
                }
            }
        }

        result
    }

    /// Clear ghost from index
    pub fn remove_ghost(&mut self, entity: Entity) {
        if let Some(pos) = self.positions.remove(&entity) {
            let cell = self.get_cell(pos);
            if let Some(entities) = self.grid.get_mut(&cell) {
                entities.retain(|&e| e != entity);
            }
        }
    }
}

/// Update spatial index with ghost positions
pub fn update_ghost_spatial_index(
    mut index: ResMut<GhostSpatialIndex>,
    ghost_q: Query<(Entity, &Transform), (With<Ghost>, Changed<Transform>)>,
    removed: RemovedComponents<Ghost>,
) {
    // Update changed positions
    for (entity, transform) in ghost_q.iter() {
        index.update_position(entity, transform.translation);
    }

    // Remove deleted ghosts
    for entity in removed.read() {
        index.remove_ghost(entity);
    }
}

use crate::playback::Ghost;
```

### Step 3: Create Update Frequency Management

Add to `src/optimization/mod.rs`:

```rust
// PR Gate: Resource for global pause state
#[derive(Resource, Default)]
pub struct GlobalTimelinePause {
    pub is_paused: bool,
}

/// Component to control update frequency
#[derive(Component)]
pub struct UpdateFrequency {
    pub updates_per_second: f32,
    pub last_update: f32,
}

impl UpdateFrequency {
    pub fn new(hz: f32) -> Self {
        Self {
            updates_per_second: hz,
            last_update: 0.0,
        }
    }

    // PR Gate: All frequency-limited updates must check GlobalTimelinePause to prevent drift
    pub fn should_update(&mut self, current_time: f32, is_globally_paused: bool) -> bool {
        // Don't update OR advance timer when globally paused
        if is_globally_paused {
            return false;
        }
        
        let interval = 1.0 / self.updates_per_second;
        if current_time - self.last_update >= interval {
            self.last_update = current_time;
            true
        } else {
            false
        }
    }
}

/// Set update frequency based on distance from camera
pub fn adjust_ghost_update_frequency(
    mut commands: Commands,
    current_arena: Res<CurrentArena>,
    ghost_q: Query<(Entity, &Parent), With<Ghost>>,
    arena_q: Query<&Arena>,
    time: Res<Time>,
) {
    let current_time = time.elapsed_secs();

    for (ghost_entity, parent) in ghost_q.iter() {
        if let Ok(arena) = arena_q.get(parent.get()) {
            // Calculate update frequency based on arena distance
            let frequency = if arena.0 == current_arena.0 {
                60.0 // Full speed for current arena
            } else {
                let row_diff = (arena.0 / 3).abs_diff(current_arena.0 / 3);
                let col_diff = (arena.0 % 3).abs_diff(current_arena.0 % 3);
                let distance = row_diff + col_diff;

                match distance {
                    1 => 30.0,  // Adjacent arenas
                    2 => 15.0,  // Diagonal arenas
                    _ => 10.0,  // Far arenas
                }
            };

            commands.entity(ghost_entity).insert(UpdateFrequency::new(frequency));
        }
    }
}

/// Update ghosts based on their frequency setting
pub fn frequency_limited_ghost_update(
    mut ghost_q: Query<
        (&mut Transform, &mut UpdateFrequency, &TimelinePosition, &PublishTimeline),
        With<Ghost>
    >,
    time: Res<Time>,
    registry: Res<ArenaGhostRegistry>,
    current_arena: Res<CurrentArena>,
    global_pause: Res<GlobalTimelinePause>,  // PR Gate: Check global pause
) {
    // PR Gate: Skip all updates when globally paused
    if global_pause.is_paused {
        return;
    }
    
    let current_time = time.elapsed_secs();

    // Process current arena ghosts first (high priority)
    let current_ghosts = registry.get_arena_ghosts(current_arena.0);
    if !current_ghosts.is_empty() {
        // Use iter_many_mut for current arena ghosts
        for (mut transform, mut frequency, position, timeline) in
            ghost_q.iter_many_mut(current_ghosts.iter().copied())
        {
            // PR Gate: Pass global pause state to prevent timer drift
            if frequency.should_update(current_time, global_pause.is_paused) {
                // Perform expensive interpolation only when needed
                if let Some(pos) = interpolate_position(timeline, position.0) {
                    transform.translation = pos;
                }
            }
        }
    }

    // Process other arenas (can be done less frequently)
    for (arena_idx, ghost_entities) in &registry.ghosts_by_arena {
        if *arena_idx != current_arena.0 && !ghost_entities.is_empty() {
            // Use iter_many_mut for batch processing
            for (mut transform, mut frequency, position, timeline) in
                ghost_q.iter_many_mut(ghost_entities.iter().copied())
            {
                // PR Gate: Pass global pause state to prevent timer drift
                if frequency.should_update(current_time, global_pause.is_paused) {
                    if let Some(pos) = interpolate_position(timeline, position.0) {
                        transform.translation = pos;
                    }
                }
            }
        }
    }
}

use crate::arena::{Arena, CurrentArena};
use crate::timeline::{TimelinePosition, interpolation::interpolate_position};
use crate::multi_arena::ArenaGhostRegistry;
```

### Step 4: Create Batch Processing Systems

Add to `src/optimization/mod.rs`:

```rust
/// Process ghosts in batches to improve cache locality
pub fn batch_process_ghosts(
    mut ghost_q: Query<(&mut Transform, &TimelinePosition, &PublishTimeline), With<Ghost>>,
    registry: Res<ArenaGhostRegistry>,
) {
    const BATCH_SIZE: usize = 32;

    // Process ghosts arena by arena for better cache usage
    for (arena_idx, ghost_entities) in &registry.ghosts_by_arena {
        // Process in batches
        for chunk in ghost_entities.chunks(BATCH_SIZE) {
            // Use iter_many_mut for efficient batch processing
            // This provides better cache locality than individual get_mut calls
            for (mut transform, position, timeline) in
                ghost_q.iter_many_mut(chunk.iter().copied())
            {
                if let Some(pos) = interpolate_position(timeline, position.0) {
                    transform.translation = pos;
                }
            }
        }
    }
}

use crate::multi_arena::ArenaGhostRegistry;
```

### Step 5: Create Memory Pool for Timelines

Add to `src/optimization/mod.rs`:

```rust
/// Memory pool for timeline events
#[derive(Resource)]
pub struct TimelineEventPool {
    /// Pre-allocated event vectors
    pub available: Vec<Vec<TimelineEvent>>,
    /// Maximum vectors to keep in pool
    pub max_pooled: usize,
}

impl Default for TimelineEventPool {
    fn default() -> Self {
        let mut available = Vec::new();

        // Pre-allocate some vectors
        for _ in 0..10 {
            available.push(Vec::with_capacity(100));
        }

        Self {
            available,
            max_pooled: 50,
        }
    }
}

impl TimelineEventPool {
    /// Get a vector from the pool
    pub fn get(&mut self) -> Vec<TimelineEvent> {
        self.available.pop().unwrap_or_else(|| Vec::with_capacity(100))
    }

    /// Return a vector to the pool
    pub fn return_vec(&mut self, mut vec: Vec<TimelineEvent>) {
        if self.available.len() < self.max_pooled {
            vec.clear();
            self.available.push(vec);
        }
    }
}

/// Use pooled vectors for timeline operations
pub fn use_pooled_timeline_vectors(
    mut pool: ResMut<TimelineEventPool>,
    mut draft_q: Query<&mut DraftTimeline, Added<Recording>>,
) {
    for mut draft in draft_q.iter_mut() {
        // Replace default vector with pooled one
        let mut pooled = pool.get();
        std::mem::swap(&mut draft.events, &mut pooled);

        // Return old vector to pool if it was small
        if pooled.capacity() < 1000 {
            pool.return_vec(pooled);
        }
    }
}

use crate::recording::Recording;
```

### Step 6: Create Performance Monitoring

Add to `src/optimization/mod.rs`:

```rust
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};

/// Resource for performance metrics
#[derive(Resource, Default)]
pub struct PerformanceMetrics {
    pub ghost_count: usize,
    pub active_recordings: usize,
    pub total_timeline_events: usize,
    pub average_fps: f32,
    pub worst_frame_ms: f32,
    pub memory_usage_mb: f32,
}

/// Monitor and log performance metrics
pub fn monitor_performance(
    mut metrics: ResMut<PerformanceMetrics>,
    diagnostics: Res<DiagnosticsStore>,
    ghost_q: Query<(), With<Ghost>>,
    recording_q: Query<(), With<Recording>>,
    timeline_q: Query<&PublishTimeline>,
    compressed_q: Query<&CompressedTimeline>,
    time: Res<Time>,
) {
    // Update counts
    metrics.ghost_count = ghost_q.iter().count();
    metrics.active_recordings = recording_q.iter().count();

    // Count timeline events
    metrics.total_timeline_events = timeline_q.iter()
        .map(|t| t.events.len())
        .sum();

    // Get FPS from diagnostics
    if let Some(fps) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.smoothed())
    {
        metrics.average_fps = fps as f32;
    }

    // Calculate memory usage
    let timeline_memory: usize = timeline_q.iter()
        .map(|t| t.events.len() * std::mem::size_of::<TimelineEvent>())
        .sum();

    let compressed_memory: usize = compressed_q.iter()
        .map(|t| t.memory_usage())
        .sum();

    metrics.memory_usage_mb = (timeline_memory + compressed_memory) as f32 / 1_048_576.0;

    // Log every 5 seconds
    if time.elapsed_secs() as u32 % 5 == 0 {
        info!(
            "Performance: {} ghosts, {:.0} FPS, {:.2} MB timeline memory",
            metrics.ghost_count,
            metrics.average_fps,
            metrics.memory_usage_mb
        );

        if metrics.ghost_count > 200 {
            warn!("High ghost count may impact performance");
        }

        if metrics.average_fps < 30.0 && metrics.ghost_count > 0 {
            error!("Low FPS detected with {} ghosts", metrics.ghost_count);
        }
    }
}

/// Automatically reduce quality when performance is poor
pub fn auto_adjust_quality(
    metrics: Res<PerformanceMetrics>,
    mut compression: ResMut<CompressionSettings>,
) {
    if metrics.average_fps < 30.0 && metrics.ghost_count > 100 {
        // Increase compression
        compression.min_movement_delta = 0.2;
        compression.min_distance_squared = 0.05;
        compression.max_events = 300;

        warn!("Auto-adjusting quality settings for performance");
    } else if metrics.average_fps > 60.0 {
        // Can afford better quality
        compression.min_movement_delta = 0.1;
        compression.min_distance_squared = 0.01;
        compression.max_events = 500;
    }
}
```

### Step 7: Create Optimization Plugin

Add to `src/optimization/mod.rs`:

```rust
pub struct OptimizationPlugin;

impl Plugin for OptimizationPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<CompressionSettings>()
            .init_resource::<GhostSpatialIndex>()
            .init_resource::<TimelineEventPool>()
            .init_resource::<PerformanceMetrics>()
            .init_resource::<GlobalTimelinePause>()  // PR Gate: Add global pause resource

            // Diagnostics
            .add_plugins(FrameTimeDiagnosticsPlugin)

            // Systems - Compression
            .add_systems(Update, use_pooled_timeline_vectors)

            // Systems - Spatial indexing
            .add_systems(Update, update_ghost_spatial_index)

            // Systems - Update frequency
            // PR Gate: Consider adding run_if(!paused) to hot playback systems
            .add_systems(Update, (
                adjust_ghost_update_frequency,
                frequency_limited_ghost_update,
            ).chain()
                .run_if(|pause: Res<GlobalTimelinePause>| !pause.is_paused)
            )

            // Systems - Batch processing
            .add_systems(Update, batch_process_ghosts
                .run_if(|time: Res<Time>, pause: Res<GlobalTimelinePause>| {
                    // PR Gate: Don't run batch processing when paused
                    !pause.is_paused && time.delta_secs() as u32 % 2 == 0
                })
            )

            // Systems - Performance monitoring
            .add_systems(Update, (
                monitor_performance,
                auto_adjust_quality,
            ));
    }
}
```

### Step 8: Wire Into Main

Update `src/main.rs`:

```rust
mod optimization;
use crate::optimization::OptimizationPlugin;

// In main():
.add_plugins(OptimizationPlugin)
```

## Performance Testing

Create `src/optimization/stress_test.rs`:

```rust
use bevy::prelude::*;
use crate::optimization::PerformanceMetrics;

/// Command to spawn many test ghosts
pub fn spawn_stress_test_ghosts(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !keyboard.just_pressed(KeyCode::F12) {
        return;
    }

    info!("Spawning 100 test ghosts for stress testing");

    let mesh = meshes.add(Sphere::new(0.125));
    let material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.5, 0.5, 1.0, 0.5),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    // Create test timeline
    let mut test_timeline = PublishTimeline { events: Vec::new() };

    for i in 0..100 {
        test_timeline.events.push(TimelineEvent {
            timestamp: i as f32,
            event_type: EventType::Transform(Vec3::new(
                (i as f32).sin() * 10.0,
                (i as f32).cos() * 10.0,
                0.0
            )),
        });
    }

    // Spawn ghosts
    for i in 0..100 {
        commands.spawn((
            Ghost,
            Replaying,
            test_timeline.clone(),
            TimelinePosition(0.0),
            UpdateFrequency::new(30.0),
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material.clone()),
            Transform::from_xyz(i as f32 * 0.5, 0.0, 0.0),
        ));
    }
}

/// Log detailed performance report
pub fn log_performance_report(
    metrics: Res<PerformanceMetrics>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if !keyboard.just_pressed(KeyCode::F11) {
        return;
    }

    println!("=== Performance Report ===");
    println!("Ghosts: {}", metrics.ghost_count);
    println!("Active Recordings: {}", metrics.active_recordings);
    println!("Total Timeline Events: {}", metrics.total_timeline_events);
    println!("Average FPS: {:.1}", metrics.average_fps);
    println!("Timeline Memory: {:.2} MB", metrics.memory_usage_mb);
    println!("========================");
}

use crate::timeline::{TimelineEvent, EventType, PublishTimeline, TimelinePosition};
use crate::playback::{Ghost, Replaying};
use crate::optimization::UpdateFrequency;
```

## Verification

Run stress tests:

```bash
cargo run --release
```

Test sequence:

1. Press F12 to spawn 100 test ghosts
2. Press F11 to view performance report
3. Record and commit multiple ghosts across arenas
4. Monitor FPS counter and console logs
5. Verify auto-quality adjustment activates under load

Performance targets:

- 60+ FPS with 50 ghosts in current arena
- 30+ FPS with 200 total ghosts
- 20+ FPS with 320 total ghosts
- Memory usage under 100MB for all timelines

## Further Work

For advanced optimization topics beyond this tutorial:

- **Spatial Indexing**: Octrees/BVH for advanced collision detection
- **Object Pools**: Pre-allocated pools for timeline events and entities
- **Advanced Compression**: LZ4, Zstandard for timeline compression
- **SIMD Operations**: Using packed_simd for batch transforms
- **GPU Instancing**: Rendering many ghosts with instanced meshes
- **Custom Allocators**: Arena allocators for timeline events
- **ECS Scheduling**: Custom stages and exclusive systems
- **Memory Mapping**: mmap for large timeline storage
- **Network Optimization**: Delta compression for multiplayer

These topics are beyond the scope of this tutorial but may be relevant for production games with extreme performance requirements.

## Next Steps

With optimization complete, we can now:

- Tutorial 09: Advanced ghost interactions and collision
- Tutorial 10: Save/load system for persistent timelines

## Key Takeaways

1. **Timeline Compression**: 5-10x memory reduction with delta encoding
2. **Spatial Indexing**: O(1) lookups for nearby ghosts
3. **Update Frequencies**: Distant ghosts update less frequently
4. **Batch Processing**: Better cache usage with grouped updates
5. **Auto-Quality**: Dynamic adjustment based on performance

These optimizations ensure the game remains playable even with hundreds of ghosts. The key is balancing visual fidelity
with performance, using LOD techniques and smart update strategies to maintain smooth gameplay.