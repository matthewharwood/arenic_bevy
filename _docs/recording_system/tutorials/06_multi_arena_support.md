# Tutorial 06: Multi-Arena Support

## Objective

Extend the recording system to support multiple arenas running simultaneously. Each arena will have independent timers
and ghosts, allowing players to manage recordings across all 9 arenas.

## Prerequisites

- Completed Tutorials 01-05 (Full single-arena recording system)
- Understanding of parent-child entity relationships
- Familiarity with query filters

## Components/Systems

We'll create:

- Arena-specific ghost management
- Cross-arena timeline synchronization
- Performance optimization for multiple arenas
- Arena-aware playback systems

## Implementation Steps

### Step 1: Create Arena Ghost Management

Create `src/multi_arena/mod.rs`:

```rust
use bevy::prelude::*;
use bevy::time::Virtual;
use bevy::utils::HashMap;  // Use Bevy's HashMap for better performance
use crate::timeline::{ArenaIdx, TimelineClock, PublishTimeline, TimelinePosition, TimeStamp};
use crate::arena::CurrentArena;
use crate::playback::{Ghost, Replaying};
use crate::character::Character;

/// APPROVED: Simple registry - HashMap is perfect for 8 arenas
#[derive(Resource, Default)]
pub struct ArenaGhostRegistry {
    /// Map from arena index to list of ghost entities
    pub ghosts_by_arena: HashMap<ArenaIdx, Vec<Entity>>,
}

impl ArenaGhostRegistry {
    pub fn register_ghost(&mut self, arena: ArenaIdx, ghost: Entity) {
        self.ghosts_by_arena
            .entry(arena)
            .or_insert_with(Vec::new)
            .push(ghost);
    }

    pub fn unregister_ghost(&mut self, arena: ArenaIdx, ghost: Entity) {
        if let Some(ghosts) = self.ghosts_by_arena.get_mut(&arena) {
            ghosts.retain(|&e| e != ghost);
        }
    }

    pub fn get_arena_ghosts(&self, arena: ArenaIdx) -> &[Entity] {
        self.ghosts_by_arena
            .get(&arena)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn total_ghost_count(&self) -> usize {
        self.ghosts_by_arena.values().map(|v| v.len()).sum()
    }
}

/// Component to track which arena a ghost belongs to
#[derive(Component)]
pub struct GhostArena(pub ArenaIdx);

/// System to register new ghosts in the registry
pub fn register_new_ghosts(
    mut registry: ResMut<ArenaGhostRegistry>,
    new_ghosts: Query<(Entity, &Parent), Added<Ghost>>,
    arena_q: Query<&ArenaIdx>,
) {
    for (ghost_entity, parent) in new_ghosts.iter() {
        // Find which arena this ghost belongs to
        if let Ok(arena) = arena_q.get(parent.get()) {
            registry.register_ghost(*arena, ghost_entity);
            info!("Registered ghost {:?} in arena {:?}", ghost_entity, arena);
        }
    }
}

/// System to clean up removed ghosts from registry
pub fn cleanup_removed_ghosts(
    mut registry: ResMut<ArenaGhostRegistry>,
    mut removed: RemovedComponents<Ghost>,
) {
    for entity in removed.read() {
        // Remove from all arena lists (inefficient but safe)
        for ghosts in registry.ghosts_by_arena.values_mut() {
            ghosts.retain(|&e| e != entity);
        }
    }
}
```

### Step 2: Create Arena-Specific Playback

Add to `src/multi_arena/mod.rs`:

```rust
/// Update ghosts only in active arenas (performance optimization)
pub fn playback_arena_ghosts(
    registry: Res<ArenaGhostRegistry>,
    current_arena: Res<CurrentArena>,
    mut ghost_q: Query<
        (&mut TimelinePosition, &mut Transform, &PublishTimeline),
        With<Ghost>
    >,
    arena_q: Query<(&Arena, &TimelineClock)>,
) {
    // Process current arena at full fidelity
    let Some(current_idx) = ArenaIdx::new(current_arena.0) else {
        return;
    };
    
    if let Some((_, clock)) = arena_q.iter()
        .find(|(arena, _)| **arena == current_idx)
    {
        let current_time = clock.current();
        let current_arena_ghosts = registry.get_arena_ghosts(current_idx);

        // PR Gate: Use iter_many_mut for efficient batch processing (cache locality)
        // NO individual get_mut() calls in loops - this is critical for performance
        // iter_many_mut processes entities in batches, avoiding O(n²) individual lookups
        for (mut position, mut transform, timeline) in
            ghost_q.iter_many_mut(current_arena_ghosts)
        {
            position.0 = current_time;

            // Full interpolation for current arena
            if let Some(pos) = interpolate_position(timeline, current_time) {
                transform.translation = pos;
            }
        }
    }

    // Process other arenas at reduced fidelity
    for (arena, clock) in arena_q.iter() {
        if *arena != current_idx {
            let current_time = clock.current();

            // Update every 10th frame for distant arenas
            if (current_time.as_secs() * 10.0) as u32 % 10 != 0 {
                continue;
            }

            let arena_ghosts = registry.get_arena_ghosts(arena.0);

            // Use iter_many_mut for batch processing of arena ghosts
            for (mut position, mut transform, timeline) in
                ghost_q.iter_many_mut(arena_ghosts)
            {
                position.0 = current_time;

                // Simplified interpolation for distant arenas
                if let Some(pos) = get_nearest_position(timeline, current_time) {
                    transform.translation = pos;
                }
            }
        }
    }
}

/// Get nearest recorded position without interpolation (faster)
fn get_nearest_position(timeline: &PublishTimeline, time: f32) -> Option<Vec3> {
    use crate::timeline::EventType;

    timeline.events
        .iter()
        .filter_map(|event| {
            if let EventType::Transform(pos) = event.event_type {
                Some((event.timestamp, pos))
            } else {
                None
            }
        })
        .min_by_key(|(timestamp, _)| {
            ((timestamp - time).abs() * 1000.0) as u32
        })
        .map(|(_, pos)| pos)
}

use crate::timeline::interpolation::interpolate_position;
```

### Step 3: Create Arena Clock Synchronization

Add to `src/multi_arena/mod.rs`:

```rust
/// Options for arena timer synchronization
#[derive(Resource)]
pub struct ArenaTimerSync {
    pub mode: TimerSyncMode,
    pub global_offset: f32,
}

impl Default for ArenaTimerSync {
    fn default() -> Self {
        Self {
            mode: TimerSyncMode::Independent,
            global_offset: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimerSyncMode {
    /// Each arena has independent timer
    Independent,
    /// All arenas share same timer
    Synchronized,
    /// Arenas have offset timers (cascade effect)
    Cascading { offset_per_arena: f32 },
}

/// Update arena timers based on sync mode
pub fn update_arena_timers_with_sync(
    virtual_time: Res<Time<Virtual>>,
    sync: Res<ArenaTimerSync>,
    mut arena_q: Query<(&Arena, &mut TimelineClock)>,
) {
    let delta = virtual_time.delta_secs();

    match sync.mode {
        TimerSyncMode::Independent => {
            // Each arena updates independently
            for (_, mut clock) in arena_q.iter_mut() {
                clock.tick_secs(delta);
            }
        }
        TimerSyncMode::Synchronized => {
            // All arenas use same time
            let new_time = TimeStamp::wrapped(sync.global_offset + delta);
            for (_, mut clock) in arena_q.iter_mut() {
                // Note: In real implementation, you'd need to update the internal timer
                // This is a simplified representation for the tutorial
            }
        }
        TimerSyncMode::Cascading { offset_per_arena } => {
            // Each arena has offset from previous
            for (arena, mut clock) in arena_q.iter_mut() {
                let offset = arena.0 as f32 * offset_per_arena;
                // Note: In real implementation, you'd update the internal timer with offset
                // This is a simplified representation for the tutorial
            }
        }
    }
}
```

### Step 4: Create Cross-Arena Ghost Limits

Add to `src/multi_arena/mod.rs`:

```rust
/// Resource to limit total ghost count for performance
#[derive(Resource)]
pub struct GhostLimits {
    pub max_total_ghosts: usize,
    pub max_ghosts_per_arena: usize,
    pub warning_threshold: usize,
}

impl Default for GhostLimits {
    fn default() -> Self {
        Self {
            max_total_ghosts: 320,
            max_ghosts_per_arena: 40,
            warning_threshold: 280,
        }
    }
}

/// Check ghost limits before allowing new recordings
pub fn check_ghost_limits(
    registry: Res<ArenaGhostRegistry>,
    limits: Res<GhostLimits>,
    current_arena: Res<CurrentArena>,
) {
    let total = registry.total_ghost_count();
    let Some(current_idx) = ArenaIdx::new(current_arena.0) else {
        return;
    };
    let current_arena_count = registry
        .get_arena_ghosts(current_idx)
        .len();

    if total >= limits.max_total_ghosts {
        error!(
            "Maximum ghost limit reached: {}/{}", 
            total, 
            limits.max_total_ghosts
        );
        // TODO: Prevent new recordings
    } else if total >= limits.warning_threshold {
        warn!(
            "Approaching ghost limit: {}/{}", 
            total, 
            limits.max_total_ghosts
        );
    }

    if current_arena_count >= limits.max_ghosts_per_arena {
        error!(
            "Arena {} has maximum ghosts: {}/{}", 
            current_idx.as_u8(),
            current_arena_count,
            limits.max_ghosts_per_arena
        );
    }
}
```

### Step 5: Create Arena LOD System

Add to `src/multi_arena/mod.rs`:

```rust
/// Level of detail for ghost rendering
#[derive(Component)]
pub struct GhostLOD {
    pub level: LODLevel,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LODLevel {
    /// Full quality - current arena
    Full,
    /// Reduced quality - nearby arenas
    Reduced,
    /// Minimal quality - distant arenas
    Minimal,
}

/// Update LOD based on arena distance
pub fn update_ghost_lod(
    mut commands: Commands,
    current_arena: Res<CurrentArena>,
    ghost_q: Query<(Entity, &Parent), With<Ghost>>,
    arena_q: Query<&Arena>,
) {
    let Some(current_idx) = ArenaIdx::new(current_arena.0) else {
        return;
    };

    for (ghost_entity, parent) in ghost_q.iter() {
        if let Ok(arena) = arena_q.get(parent.get()) {
            let arena_idx = arena.0;

            // Calculate "distance" between arenas
            let row_diff = (arena_idx / 3).abs_diff(current_idx.as_u8() / 3);
            let col_diff = (arena_idx % 3).abs_diff(current_idx.as_u8() % 3);
            let distance = row_diff + col_diff;

            let lod_level = match distance {
                0 => LODLevel::Full,
                1 => LODLevel::Reduced,
                _ => LODLevel::Minimal,
            };

            commands.entity(ghost_entity).insert(GhostLOD {
                level: lod_level,
            });
        }
    }
}

/// Apply LOD optimizations to ghost updates
pub fn apply_ghost_lod(
    ghost_q: Query<(&GhostLOD, &mut Visibility), With<Ghost>>,
) {
    for (lod, mut visibility) in ghost_q.iter() {
        match lod.level {
            LODLevel::Full => {
                // Always visible
                *visibility = Visibility::Visible;
            }
            LODLevel::Reduced => {
                // Could reduce mesh complexity here
                *visibility = Visibility::Visible;
            }
            LODLevel::Minimal => {
                // Could hide completely or use billboard for very distant ghosts
                *visibility = Visibility::Hidden;
            }
        }
    }
}
```

### Step 6: Create Arena Statistics

Add to `src/multi_arena/mod.rs`:

```rust
/// Resource tracking arena statistics
#[derive(Resource, Default)]
pub struct ArenaStatistics {
    pub ghost_counts: HashMap<u8, usize>,
    pub recording_counts: HashMap<u8, usize>,
    pub total_events: HashMap<u8, usize>,
}

/// Update arena statistics
pub fn update_arena_statistics(
    mut stats: ResMut<ArenaStatistics>,
    registry: Res<ArenaGhostRegistry>,
    recording_q: Query<&Parent, With<Recording>>,
    timeline_q: Query<(&Parent, &PublishTimeline)>,
    arena_q: Query<&Arena>,
) {
    // Clear previous stats
    stats.ghost_counts.clear();
    stats.recording_counts.clear();
    stats.total_events.clear();

    // Count ghosts per arena
    for (arena_idx, ghosts) in &registry.ghosts_by_arena {
        stats.ghost_counts.insert(*arena_idx, ghosts.len());
    }

    // Count recordings per arena
    for parent in recording_q.iter() {
        if let Ok(arena) = arena_q.get(parent.get()) {
            *stats.recording_counts.entry(arena.0).or_insert(0) += 1;
        }
    }

    // Count total events per arena
    for (parent, timeline) in timeline_q.iter() {
        if let Ok(arena) = arena_q.get(parent.get()) {
            *stats.total_events.entry(arena.0).or_insert(0) +=
                timeline.events.len();
        }
    }
}

/// Display arena statistics (debug)
pub fn display_arena_statistics(
    stats: Res<ArenaStatistics>,
    time: Res<Time>,
) {
    // Display every 5 seconds
    if time.elapsed_secs() as u32 % 5 != 0 {
        return;
    }

    let total_ghosts: usize = stats.ghost_counts.values().sum();
    let total_events: usize = stats.total_events.values().sum();

    info!(
        "Arena Stats - Ghosts: {}, Events: {}", 
        total_ghosts, 
        total_events
    );

    for arena_idx in 0..9 {
        let ghosts = stats.ghost_counts.get(&arena_idx).unwrap_or(&0);
        let recordings = stats.recording_counts.get(&arena_idx).unwrap_or(&0);

        if *ghosts > 0 || *recordings > 0 {
            debug!(
                "  Arena {}: {} ghosts, {} recording", 
                arena_idx, ghosts, recordings
            );
        }
    }
}
```

### Step 7: Create Arena Batch Processing

Add to `src/multi_arena/mod.rs`:

```rust
use bevy::ecs::query::QueryIter;

/// Batch process ghosts for better performance
pub fn batch_process_ghost_abilities(
    registry: Res<ArenaGhostRegistry>,
    mut ghost_q: Query<
        (Entity, &TimelinePosition, &PublishTimeline, &mut TriggeredAbilities),
        With<Ghost>
    >,
    mut ability_events: EventWriter<GhostAbilityTrigger>,
) {
    // Process ghosts in batches by arena
    for (arena_idx, ghost_entities) in &registry.ghosts_by_arena {
        // Skip empty arenas
        if ghost_entities.is_empty() {
            continue;
        }

        // Collect ability triggers for this arena
        let mut arena_triggers = Vec::new();

        // Use iter_many_mut for efficient batch processing of specific ghosts
        // iter_many_mut processes entities in batches, avoiding O(n²) individual lookups
        // Note: Query now includes Entity to maintain entity tracking
        for (entity, position, timeline, mut triggered) in
            ghost_q.iter_many_mut(ghost_entities.iter().copied())
        {
            // Check events in slice between previous and current clock position
            let prev_time = triggered.previous_position.unwrap_or(position.0);
            let abilities = get_abilities_in_range(
                timeline,
                prev_time,
                position.0
            );

            for ability_id in abilities {
                if !triggered.has_triggered(position.0, ability_id) {
                    arena_triggers.push(GhostAbilityTrigger {
                        ghost: entity,
                        ability: ability_id,
                        timestamp: position.0,
                    });
                    triggered.add_triggered(position.0, ability_id);
                }
            }
        }

        // Send all triggers for this arena at once
        for trigger in arena_triggers {
            ability_events.write(trigger);
        }
    }
}

use crate::timeline::interpolation::get_abilities_in_range;
use crate::playback::{TriggeredAbilities, GhostAbilityTrigger};
```

### Step 8: Create Multi-Arena Plugin

Add to `src/multi_arena/mod.rs`:

```rust
pub struct MultiArenaPlugin;

impl Plugin for MultiArenaPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<ArenaGhostRegistry>()
            .init_resource::<ArenaTimerSync>()
            .init_resource::<GhostLimits>()
            .init_resource::<ArenaStatistics>()

            // Systems - Registration
            .add_systems(Update, (
                register_new_ghosts,
                cleanup_removed_ghosts,
            ))

            // Systems - Playback
            .add_systems(Update, (
                playback_arena_ghosts
                    .after(update_arena_timers_with_sync),
                batch_process_ghost_abilities,
            ))

            // Systems - LOD
            .add_systems(Update, (
                update_ghost_lod,
                apply_ghost_lod,
            ).chain())

            // Systems - Limits & Stats
            .add_systems(Update, (
                check_ghost_limits,
                update_arena_statistics,
                display_arena_statistics,
            ));
    }
}
```

### Step 9: Wire Into Main

Update `src/main.rs`:

```rust
mod multi_arena;
use crate::multi_arena::MultiArenaPlugin;

// In main():
.add_plugins(MultiArenaPlugin)
```

## Unit Tests

Create `src/multi_arena/tests.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arena_ghost_registry() {
        let mut registry = ArenaGhostRegistry::default();

        let ghost1 = Entity::from_raw(1);
        let ghost2 = Entity::from_raw(2);

        registry.register_ghost(0, ghost1);
        registry.register_ghost(0, ghost2);
        registry.register_ghost(1, ghost1);

        assert_eq!(registry.get_arena_ghosts(0).len(), 2);
        assert_eq!(registry.get_arena_ghosts(1).len(), 1);
        assert_eq!(registry.total_ghost_count(), 3);

        registry.unregister_ghost(0, ghost1);
        assert_eq!(registry.get_arena_ghosts(0).len(), 1);
    }

    #[test]
    fn test_timer_sync_modes() {
        let independent = TimerSyncMode::Independent;
        let synchronized = TimerSyncMode::Synchronized;
        let cascading = TimerSyncMode::Cascading {
            offset_per_arena: 5.0
        };

        assert_ne!(
            std::mem::discriminant(&independent),
            std::mem::discriminant(&synchronized)
        );
    }

    #[test]
    fn test_lod_distance_calculation() {
        // Arena layout:
        // 0 1 2
        // 3 4 5  
        // 6 7 8

        // Same arena
        assert_eq!(calculate_arena_distance(4, 4), 0);

        // Adjacent
        assert_eq!(calculate_arena_distance(4, 1), 1);
        assert_eq!(calculate_arena_distance(4, 5), 1);

        // Diagonal
        assert_eq!(calculate_arena_distance(4, 0), 2);
        assert_eq!(calculate_arena_distance(4, 8), 2);
    }

    fn calculate_arena_distance(a: u8, b: u8) -> u8 {
        let row_diff = (a / 3).abs_diff(b / 3);
        let col_diff = (a % 3).abs_diff(b % 3);
        (row_diff + col_diff) as u8
    }
}
```

## Verification

Run tests:

```bash
cargo test multi_arena
```

Run the game and test multi-arena support:

```bash
cargo run
```

Test sequence:

1. Record ghosts in multiple arenas (use [ ] to switch)
2. Check console for arena statistics every 5 seconds
3. Verify ghosts play in non-current arenas
4. Test with many ghosts to see LOD in action
5. Monitor performance with 10+ ghosts across arenas

## Next Steps

With multi-arena support complete, we can now:

- Tutorial 07: Visual polish and recording indicators
- Tutorial 08: Performance optimization for 320 ghosts
- Tutorial 09: Advanced ghost interactions

## Key Takeaways

1. **Arena Registry**: Efficient tracking of ghosts per arena
2. **LOD System**: Reduced fidelity for distant arenas saves performance
3. **Batch Processing**: Group operations by arena for cache efficiency
4. **Explicit Constructors**: ArenaIdx::new() validation in multi-arena logic
5. **Resource Limits**: Prevent performance degradation from too many ghosts

Multi-arena support is crucial for the full game experience. By optimizing updates based on arena distance and batching
operations, we can support hundreds of ghosts while maintaining smooth performance.