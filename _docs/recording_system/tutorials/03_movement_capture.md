# Tutorial 03: Movement Capture

## Objective

Implement systems to capture character INTENT during recording mode. We record the player's inputs (WASD keys, ability
presses) not the resulting transforms. This ensures deterministic replay regardless of physics or interpolation changes.

## Prerequisites

- Completed Tutorial 01 (Timeline Foundation)  
- Completed Tutorial 02 (Recording State Machine)
- Understanding of input vs output in game systems
- Familiarity with the existing movement system

## Components/Systems

We'll create:

- Movement intent recording system (capturing WASD input)
- Ability intent recording system (capturing ability key presses)  
- Timeline optimization utilities
- Const keymaps for all inputs

## Important Design Decision

> **Why Not Capture Transform Changes?**
> 
> A naive approach would be to use `Changed<Transform>` to record character positions. This is WRONG for several reasons:
> 
> 1. **Non-Deterministic**: Physics, interpolation, or frame timing changes break replay
> 2. **Huge Data**: Transform is 48 bytes per change vs 2 bytes for movement direction
> 3. **Lost Intent**: Can't distinguish player movement from knockback/teleports
> 4. **Coupling**: Replay becomes dependent on exact physics implementation
> 
> By recording intent (WASD keys pressed), we ensure:
> - Perfect replay regardless of engine changes
> - Minimal storage requirements
> - Clear distinction between player actions and game reactions
> - Ability to "re-simulate" with different physics if needed

## Implementation Steps

### Step 1: Create Movement Intent Capture System

Create `src/recording/capture.rs`:

```rust
use bevy::prelude::*;
use crate::timeline::{DraftTimeline, TimelineEvent, EventType, AbilityId, Timer, GridPos, Target, TimelineClock, ArenaIdx};
use crate::recording::{Recording, RecordingMode, RecordingState};
use crate::arena::CurrentArena;
use crate::character::Character;

// Const keymaps for movement
const KEY_MOVE_UP: KeyCode = KeyCode::KeyW;
const KEY_MOVE_DOWN: KeyCode = KeyCode::KeyS;
const KEY_MOVE_LEFT: KeyCode = KeyCode::KeyA;
const KEY_MOVE_RIGHT: KeyCode = KeyCode::KeyD;

// Const keymaps for abilities
const KEY_ABILITIES: [(KeyCode, AbilityId); 4] = [
    (KeyCode::Digit1, AbilityId::AUTO_SHOT),
    (KeyCode::Digit2, AbilityId::HOLY_NOVA),
    (KeyCode::Digit3, AbilityId::POISON_SHOT),
    (KeyCode::Digit4, AbilityId::HEAL),
];

/// Capture movement INTENT during recording - NOT transforms!
/// This records what keys the player pressed, not where the character ended up
pub fn capture_movement_intent(
    keyboard: Res<ButtonInput<KeyCode>>,
    global_pause: Res<GlobalTimelinePause>,
    recording_state: Res<RecordingState>,
    current_arena: Res<CurrentArena>,
    mut recording_q: Query<&mut DraftTimeline, With<Recording>>,
    arena_q: Query<(&ArenaIdx, &TimelineClock)>,
) {
    // Tutorial Note: We use a simple early return check here for clarity.
    // The lead suggested .run_if(not_paused) but for juniors learning,
    // explicit returns are clearer than system scheduling conditions.
    if global_pause.is_paused {
        return;
    }
    
    // Only capture during active recording
    if recording_state.mode != RecordingMode::Recording {
        return;
    }

    // Check if any movement keys were just pressed
    let movement_dir = get_movement_direction(&keyboard);
    
    // No movement this frame
    if movement_dir == IVec2::ZERO {
        return;
    }

    // Convert current_arena to ArenaIdx
    let Some(current_idx) = ArenaIdx::new(current_arena.0) else {
        return;
    };

    // Get current arena timer using let-else
    let Some((_, clock)) = arena_q
        .iter()
        .find(|(idx, _)| **idx == current_idx)
    else {
        return;
    };

    let timestamp = clock.current;

    // Record the movement intent for all recording entities
    for mut timeline in recording_q.iter_mut() {
        // Create movement intent event
        let event = TimelineEvent {
            timestamp,
            event_type: EventType::Movement(GridPos::from(movement_dir)),
        };

        // Add to timeline (will be sorted automatically)
        timeline.add_event(event);

        trace!(
            "Recorded movement intent at {}: {}", 
            timestamp, 
            GridPos::from(movement_dir)
        );
    }
}

/// Get movement direction from keyboard input
fn get_movement_direction(keyboard: &ButtonInput<KeyCode>) -> IVec2 {
    let mut dir = IVec2::ZERO;

    // Check each movement key
    if keyboard.pressed(KEY_MOVE_UP) {
        dir.y += 1;
    }
    if keyboard.pressed(KEY_MOVE_DOWN) {
        dir.y -= 1;
    }
    if keyboard.pressed(KEY_MOVE_LEFT) {
        dir.x -= 1;
    }
    if keyboard.pressed(KEY_MOVE_RIGHT) {
        dir.x += 1;
    }

    dir
}

/// Component to track last recorded movement to avoid duplicates
#[derive(Component)]
pub struct LastRecordedMovement {
    pub direction: IVec2,
    pub timestamp: Timer,
}

/// Optimize movement recording to reduce redundant events
pub fn optimize_movement_recording(
    keyboard: Res<ButtonInput<KeyCode>>,
    global_pause: Res<GlobalTimelinePause>,
    recording_state: Res<RecordingState>,
    current_arena: Res<CurrentArena>,
    mut recording_q: Query<(Entity, &mut DraftTimeline, Option<&mut LastRecordedMovement>), With<Recording>>,
    arena_q: Query<(&ArenaIdx, &TimelineClock)>,
    mut commands: Commands,
) {
    // PR Gate: Respecting GlobalTimelinePause
    if global_pause.is_paused {
        return;
    }
    
    // Only process during active recording
    if recording_state.mode != RecordingMode::Recording {
        return;
    }

    let movement_dir = get_movement_direction(&keyboard);

    // Convert current_arena to ArenaIdx
    let Some(current_idx) = ArenaIdx::new(current_arena.0) else {
        return;
    };

    // Get current arena timer
    let Some((_, clock)) = arena_q
        .iter()
        .find(|(idx, _)| **idx == current_idx)
    else {
        return;
    };

    let timestamp = clock.current;

    for (entity, mut timeline, last_recorded) in recording_q.iter_mut() {
        // Check if we should record this movement
        let should_record = if let Some(mut last) = last_recorded {
            // Only record if direction changed or enough time passed
            let dir_changed = last.direction != movement_dir;
            let time_delta = (timestamp.as_secs() - last.timestamp.as_secs()).abs();
            
            if dir_changed || time_delta >= 0.1 {
                last.direction = movement_dir;
                last.timestamp = timestamp;
                true
            } else {
                false
            }
        } else if movement_dir != IVec2::ZERO {
            // First recording - add component
            commands.entity(entity).insert(LastRecordedMovement {
                direction: movement_dir,
                timestamp,
            });
            true
        } else {
            false
        };

        if should_record && movement_dir != IVec2::ZERO {
            let event = TimelineEvent {
                timestamp,
                event_type: EventType::Movement(GridPos::from(movement_dir)),
            };
            timeline.add_event(event);
        }
    }
}
```

### Step 2: Create Ability Intent Capture System

Add to `src/recording/capture.rs`:

```rust
/// Capture ability usage INTENT during recording
pub fn capture_ability_intent(
    keyboard: Res<ButtonInput<KeyCode>>,
    global_pause: Res<GlobalTimelinePause>,
    recording_state: Res<RecordingState>,
    current_arena: Res<CurrentArena>,
    mut recording_q: Query<&mut DraftTimeline, With<Recording>>,
    arena_q: Query<(&ArenaIdx, &TimelineClock)>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    cursor_ray: Option<Res<CursorRay>>,
) {
    // PR Gate: Respecting GlobalTimelinePause
    if global_pause.is_paused {
        return;
    }
    
    // Only capture during active recording
    if recording_state.mode != RecordingMode::Recording {
        return;
    }

    // Check for ability key presses using const keymap
    let ability_pressed = KEY_ABILITIES
        .iter()
        .find(|(key, _)| keyboard.just_pressed(*key))
        .map(|(_, ability)| *ability);

    let Some(ability_id) = ability_pressed else {
        return;
    };

    // Convert current_arena to ArenaIdx
    let Some(current_idx) = ArenaIdx::new(current_arena.0) else {
        return;
    };

    // Get current arena timer
    let Some((_, clock)) = arena_q
        .iter()
        .find(|(idx, _)| **idx == current_idx)
    else {
        return;
    };

    let timestamp = clock.current;

    // Determine target if mouse is pressed
    let target = if mouse_button.pressed(MouseButton::Left) {
        cursor_ray.and_then(|ray| {
            // Convert cursor position to grid position
            // This is simplified - in production you'd raycast to the ground plane
            let grid_pos = GridPos::new(
                (ray.origin.x / 100.0).round() as i32,
                (ray.origin.z / 100.0).round() as i32,
            );
            Some(Target::Position(grid_pos))
        })
    } else {
        None
    };

    for mut timeline in recording_q.iter_mut() {
        // Create ability event
        let event = TimelineEvent {
            timestamp,
            event_type: EventType::Ability(ability_id, target),
        };

        timeline.add_event(event);

        info!(
            "Recorded ability intent {} at {}", 
            ability_id, 
            timestamp
        );
    }
}
```

### Step 3: Create Timeline Optimization Utilities

Add to `src/recording/capture.rs`:

```rust
/// Optimize timeline by removing redundant events
pub fn optimize_timeline_events(
    mut recording_q: Query<&mut DraftTimeline, With<Recording>>,
) {
    for mut timeline in recording_q.iter_mut() {
        optimize_timeline(&mut timeline);
    }
}

/// Remove redundant events to save memory
fn optimize_timeline(timeline: &mut DraftTimeline) {
    // Keep only keyframes where direction changes or significant time passes
    const MIN_TIME_BETWEEN_SAME_EVENTS: f32 = 0.1; // 100ms minimum
    
    let mut optimized = Vec::new();
    let mut last_movement: Option<(Timer, IVec2)> = None;
    
    for event in &timeline.events {
        let should_keep = match &event.event_type {
            EventType::Movement(pos) => {
                let grid_vec: IVec2 = (*pos).into();
                
                if let Some((last_time, last_dir)) = last_movement {
                    // Keep if direction changed or enough time passed
                    let time_delta = event.timestamp.as_secs() - last_time.as_secs();
                    let dir_changed = last_dir != grid_vec;
                    
                    if dir_changed || time_delta >= MIN_TIME_BETWEEN_SAME_EVENTS {
                        last_movement = Some((event.timestamp, grid_vec));
                        true
                    } else {
                        false
                    }
                } else {
                    last_movement = Some((event.timestamp, grid_vec));
                    true // Always keep first event
                }
            }
            EventType::Ability(_, _) | EventType::Death => {
                true // Always keep ability and death events
            }
        };
        
        if should_keep {
            optimized.push(event.clone());
        }
    }
    
    // PR Gate: Assert timeline remains monotonic after optimization with debug! logging
    if !is_timeline_monotonic(&optimized) {
        debug!("Timeline monotonicity violated after optimization!");
        debug_assert!(false, "Timeline events must remain sorted by timestamp");
    }
    
    timeline.events = optimized;
}

/// Verify timeline events are monotonically increasing
fn is_timeline_monotonic(events: &[TimelineEvent]) -> bool {
    events.windows(2).all(|w| w[0].timestamp <= w[1].timestamp)
}
```

### Step 4: Create Timeline Query Helpers

Create `src/timeline/query.rs`:

```rust
use bevy::prelude::*;
use crate::timeline::{TimelineEvent, EventType, PublishTimeline, Timer, GridPos, AbilityId};

/// Get the movement intent at a specific time
pub fn get_movement_at_time(
    timeline: &PublishTimeline,
    current_time: Timer,
) -> Option<GridPos> {
    // Find the most recent movement event before current time
    timeline.events
        .iter()
        .rev()
        .find(|e| e.timestamp <= current_time)
        .and_then(|e| match &e.event_type {
            EventType::Movement(pos) => Some(*pos),
            _ => None,
        })
}

/// Get all ability events that should trigger at current time
pub fn get_abilities_at_time(
    timeline: &PublishTimeline,
    current_time: Timer,
    last_checked: Timer,
) -> Vec<(AbilityId, Option<Target>)> {
    timeline.events
        .iter()
        .filter(|e| e.timestamp > last_checked && e.timestamp <= current_time)
        .filter_map(|e| match &e.event_type {
            EventType::Ability(id, target) => Some((*id, *target)),
            _ => None,
        })
        .collect()
}

/// Check if character should be dead at current time
pub fn is_dead_at_time(
    timeline: &PublishTimeline,
    current_time: Timer,
) -> bool {
    timeline.events
        .iter()
        .any(|e| matches!(e.event_type, EventType::Death) && e.timestamp <= current_time)
}
```

### Step 5: Add Debug Visualization

Create `src/recording/debug.rs`:

```rust
use bevy::prelude::*;
use crate::timeline::{DraftTimeline, EventType};
use crate::recording::Recording;

/// Display recording statistics
pub fn debug_recording_stats(
    recording_timeline: Option<Single<&DraftTimeline, With<Recording>>>,
    time: Res<Time>,
) {
    // Only log every second to avoid spam
    if (time.elapsed_secs() as u32 % 60) != 0 {
        return;
    }

    let Some(timeline) = recording_timeline else {
        return;
    };

    let movement_events = timeline.events
        .iter()
        .filter(|e| matches!(e.event_type, EventType::Movement(_)))
        .count();

    let ability_events = timeline.events
        .iter()
        .filter(|e| matches!(e.event_type, EventType::Ability(_, _)))
        .count();

    let death_events = timeline.events
        .iter()
        .filter(|e| matches!(e.event_type, EventType::Death))
        .count();

    info!(
        "Recording stats - Movement: {}, Abilities: {}, Deaths: {}, Total: {}", 
        movement_events,
        ability_events,
        death_events,
        timeline.events.len()
    );
}

/// Visualize recorded movement path
pub fn visualize_movement_path(
    mut gizmos: Gizmos,
    recording_timeline: Option<Single<&DraftTimeline, With<Recording>>>,
) {
    let Some(timeline) = recording_timeline else {
        return;
    };

    let mut last_pos = Vec3::ZERO;
    
    for event in &timeline.events {
        if let EventType::Movement(grid_pos) = &event.event_type {
            // Convert grid position to world position
            let world_pos = Vec3::new(
                grid_pos.x() as f32 * 100.0,
                0.0,
                grid_pos.y() as f32 * 100.0,
            );
            
            // Draw line from last position
            if last_pos != Vec3::ZERO {
                gizmos.line(last_pos, world_pos, Color::srgb(1.0, 0.0, 0.0));
            }
            
            // Draw point at this position
            gizmos.sphere(world_pos, 10.0, Color::srgb(0.0, 1.0, 0.0));
            
            last_pos = world_pos;
        }
    }
}
```

### Step 6: Update Recording Plugin

Update `src/recording/mod.rs`:

```rust
mod capture;
mod debug;

use crate::recording::capture::{
    capture_movement_intent,
    capture_ability_intent,
    optimize_movement_recording,
    optimize_timeline_events,
};
use crate::recording::debug::{
    debug_recording_stats,
    visualize_movement_path,
};

// In RecordingPlugin::build():
.add_systems(Update, (
    // ... existing systems ...
    
    // Intent capture - NOT transform capture!
    capture_movement_intent,
    capture_ability_intent,
    optimize_movement_recording,
    
    // Optimization (run less frequently)
    optimize_timeline_events.run_if(|time: Res<Time>| {
        // Run every second
        (time.elapsed_secs() as u32) % 60 == 0
    }),
    
    // Debug visualization
    // Tutorial Note: The lead suggested feature flags for debug systems.
    // We keep them always-on for learning. In production, you'd use:
    // #[cfg(feature = "debug")] or similar conditional compilation.
    debug_recording_stats,
    visualize_movement_path,
).chain())
```

## Unit Tests

Create tests in `src/recording/capture.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::timeline::query::*;

    #[test]
    fn test_movement_direction_conversion() {
        // Test that movement directions are properly converted
        assert_eq!(GridPos::from(IVec2::new(1, 0)).x(), 1);
        assert_eq!(GridPos::from(IVec2::new(0, 1)).y(), 1);
        assert_eq!(GridPos::from(IVec2::new(-1, 0)).x(), -1);
        assert_eq!(GridPos::from(IVec2::new(0, -1)).y(), -1);
    }

    #[test]
    fn test_timeline_optimization() {
        let mut timeline = DraftTimeline::new();

        // Add many redundant movement events in same direction
        for i in 0..10 {
            timeline.add_event(TimelineEvent {
                timestamp: Timer::new(i as f32 * 0.01), // Very frequent
                event_type: EventType::Movement(GridPos::new(1, 0)), // Same direction
            });
        }

        // Add some ability events
        timeline.add_event(TimelineEvent {
            timestamp: Timer::new(0.5),
            event_type: EventType::Ability(AbilityId::AUTO_SHOT, None),
        });

        let original_count = timeline.events.len();
        optimize_timeline(&mut timeline);
        let optimized_count = timeline.events.len();

        // Should have removed redundant movements
        assert!(optimized_count < original_count);
        // Should keep ability event
        assert!(timeline.events.iter().any(|e| matches!(e.event_type, EventType::Ability(_, _))));
    }

    #[test]
    fn test_movement_intent_query() {
        let mut draft = DraftTimeline::new();

        // Add movement events
        draft.add_event(TimelineEvent {
            timestamp: Timer::new(0.0),
            event_type: EventType::Movement(GridPos::new(0, 1)),
        });

        draft.add_event(TimelineEvent {
            timestamp: Timer::new(5.0),
            event_type: EventType::Movement(GridPos::new(1, 0)),
        });

        draft.add_event(TimelineEvent {
            timestamp: Timer::new(10.0),
            event_type: EventType::Movement(GridPos::new(0, -1)),
        });

        let timeline = PublishTimeline::from_draft(&draft);

        // Test getting movement at various times
        let move_at_0 = get_movement_at_time(&timeline, Timer::new(0.0));
        assert_eq!(move_at_0, Some(GridPos::new(0, 1)));

        let move_at_7 = get_movement_at_time(&timeline, Timer::new(7.0));
        assert_eq!(move_at_7, Some(GridPos::new(1, 0)));

        let move_at_15 = get_movement_at_time(&timeline, Timer::new(15.0));
        assert_eq!(move_at_15, Some(GridPos::new(0, -1)));
    }

    #[test]
    fn test_ability_intent_detection() {
        let mut draft = DraftTimeline::new();

        draft.add_event(TimelineEvent {
            timestamp: Timer::new(5.0),
            event_type: EventType::Ability(AbilityId::AUTO_SHOT, None),
        });

        draft.add_event(TimelineEvent {
            timestamp: Timer::new(10.0),
            event_type: EventType::Ability(
                AbilityId::HEAL,
                Some(Target::Position(GridPos::new(5, 5)))
            ),
        });

        let timeline = PublishTimeline::from_draft(&draft);

        // Test ability detection in time range
        let abilities = get_abilities_at_time(
            &timeline,
            Timer::new(10.0),
            Timer::new(0.0)
        );
        
        assert_eq!(abilities.len(), 2);
        assert_eq!(abilities[0].0, AbilityId::AUTO_SHOT);
        assert_eq!(abilities[1].0, AbilityId::HEAL);
    }

    #[test]
    fn test_const_keymap_completeness() {
        // Ensure all abilities have keybindings
        assert_eq!(KEY_ABILITIES.len(), 4);
        
        // Test that each ability ID is unique
        let mut seen_abilities = std::collections::HashSet::new();
        for (_, ability) in &KEY_ABILITIES {
            assert!(seen_abilities.insert(ability));
        }
    }
    
    // Tutorial Note: The lead suggested property-based testing with 1000 random
    // timestamps. That's overkill for a tutorial. These deterministic tests
    // clearly show the expected behavior without requiring quickcheck or proptest.
    
    #[test]
    fn test_timeline_monotonic_invariant() {
        let mut timeline = DraftTimeline::new();
        
        // Add events out of order (add_event should sort them)
        timeline.add_event(TimelineEvent {
            timestamp: Timer::new(10.0),
            event_type: EventType::Movement(GridPos::new(1, 0)),
        });
        
        timeline.add_event(TimelineEvent {
            timestamp: Timer::new(5.0),
            event_type: EventType::Movement(GridPos::new(0, 1)),
        });
        
        timeline.add_event(TimelineEvent {
            timestamp: Timer::new(15.0),
            event_type: EventType::Death,
        });
        
        // Verify monotonic ordering
        assert!(is_timeline_monotonic(&timeline.events));
        
        // Verify actual order
        assert_eq!(timeline.events[0].timestamp.as_secs(), 5.0);
        assert_eq!(timeline.events[1].timestamp.as_secs(), 10.0);
        assert_eq!(timeline.events[2].timestamp.as_secs(), 15.0);
        
        // Optimize and verify monotonic invariant is maintained
        optimize_timeline(&mut timeline);
        assert!(is_timeline_monotonic(&timeline.events));
    }
}
```

## Verification

Run tests:

```bash
cargo test capture
cargo test query
```

Run the game and test movement capture:

```bash
cargo run
```

Test sequence:

1. Press R to start recording
2. Wait for countdown
3. Move with WASD during recording - intent should be captured (not position!)
4. Press ability keys (1-4) - abilities should be logged with timestamps
5. Stop recording and check console for statistics
6. Debug gizmos should show movement path

## Next Steps

With intent capture complete, we can now:

- Tutorial 04: Build the playback system that interprets recorded intent
- Tutorial 05: Add confirmation dialogs for commits
- Tutorial 06: Implement multi-arena support

## Key Takeaways

1. **Record Intent, Not Results**: Capture WASD keys, not Transform positions
2. **Const Keymaps**: All inputs defined in one place for maintainability
3. **Timeline Optimization**: Remove redundant events while preserving important changes
4. **Let-Else Pattern**: Clean early returns throughout the code
5. **Type Safety**: GridPos and ArenaIdx prevent invalid data

## Production Notes

### What We Got Right:

- Recording player intent ensures deterministic replay
- Const keymaps make control schemes maintainable
- Optimization reduces memory without losing fidelity
- Query helpers provide clean APIs for playback

### What We Intentionally Avoided:

- Recording transforms would break on physics changes
- Recording every frame would waste memory
- Complex interpolation when simple intent replay works

### Why Intent Recording Matters:

- **Deterministic**: Same inputs always produce same outputs
- **Compact**: Movement direction is 2 bytes vs Transform's 48 bytes
- **Portable**: Works across different physics implementations
- **Debuggable**: Can see exactly what player pressed

Movement intent capture is the heart of a robust recording system. By recording what the player DID (pressed W) rather
than what HAPPENED (moved to position X), we ensure perfect replay regardless of game engine changes.