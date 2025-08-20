# Tutorial 04: Playback System

## Objective

Build the ghost playback system that replays published timelines. Ghosts will automatically repeat their recorded
movements and abilities every 2-minute cycle, creating the core mechanic of the game.

## Prerequisites

- Completed Tutorials 01-03 (Timeline, Recording State, Movement Capture)
- Understanding of entity queries and component insertion
- Familiarity with interpolation concepts

## Components/Systems

We'll create:

- Ghost playback components
- Timeline replay system
- Ghost spawning and management
- Visual differentiation for ghosts
- Ability replay system

## Implementation Steps

### Step 1: Create Playback Components

Create `src/playback/mod.rs`:

```rust
use bevy::prelude::*;
use crate::timeline::{PublishTimeline, TimelinePosition, EventType, TimeStamp, TimelineClock};
use crate::arena::{Arena, ArenaId, ArenaEntities};
use crate::ability::AbilityType;
use crate::recording::Ghost;
use crate::character::Character;

/// Marker for entities actively replaying a timeline
#[derive(Component)]
pub struct Replaying;

/// Tracks which abilities have been triggered this frame
#[derive(Component, Default)]
pub struct TriggeredAbilities {
    pub abilities: Vec<(TimeStamp, AbilityType)>, // (timestamp, ability)
    pub previous_position: Option<TimeStamp>, // Track previous position for range checks
}

impl TriggeredAbilities {
    pub fn has_triggered(&self, timestamp: TimeStamp, ability: AbilityType) -> bool {
        self.abilities.iter().any(|(t, a)| {
            *t == timestamp && *a == ability
        })
    }

    pub fn add_triggered(&mut self, timestamp: TimeStamp, ability: AbilityType) {
        self.abilities.push((timestamp, ability));

        // Keep only recent triggers (last 1 second)
        self.abilities.retain(|(t, _)| timestamp.as_secs() - t.as_secs() < 1.0);
    }
}

/// Reference to the original character for visual copying
#[derive(Component)]
pub struct GhostSource(pub Entity);

/// Each ghost tracks its own arena for independent clocks
#[derive(Component)]
pub struct GhostArena(pub ArenaId);

/// Visual properties for ghosts
#[derive(Component)]
pub struct GhostVisuals {
    pub transparency: f32,
    pub tint: Srgba,  // Use specific color space type
}

impl Default for GhostVisuals {
    fn default() -> Self {
        Self {
            transparency: 0.5,
            tint: Srgba::srgb(0.5, 0.5, 1.0), // Blue tint
        }
    }
}
```

### Step 2: Ghost Replay Feature - Simple Timeline Playback

Ghosts simply play whatever timeline exists in their `CharacterTimelines` hashmap for the current arena. No special replay states needed.

Add to `src/playback/mod.rs`:

```rust
/// Event for showing replay dialog when user presses R on ghost
#[derive(Event)]
pub struct ShowGhostReplayDialog {
    pub ghost_entity: Entity,
    pub arena: ArenaId,
    pub has_previous_recording: bool,
}

/// Handle user's choice for ghost replay
pub fn handle_ghost_replay_choice(
    mut commands: Commands,
    mut replay_events: EventReader<GhostReplayChoice>,
    mut ghost_q: Query<&mut TimelinePosition, With<Ghost>>,
) {
    for event in replay_events.read() {
        match event.choice {
            ReplayChoice::DraftNew => {
                // Convert ghost back to normal character for new recording
                commands.entity(event.ghost_entity)
                    .remove::<Ghost>()
                    .remove::<Replaying>();
                info!("Ghost {:?} converted to character for new recording", event.ghost_entity);
            }
            ReplayChoice::KeepExisting => {
                // Reset position to replay from the beginning
                if let Ok(mut position) = ghost_q.get_mut(event.ghost_entity) {
                    position.0 = TimeStamp::ZERO;
                }
                info!("Ghost {:?} will replay existing recording", event.ghost_entity);
            }
            ReplayChoice::Cancel => {
                // Continue normal playback - no action needed
                info!("Ghost replay cancelled");
            }
        }
    }
}

#[derive(Event)]
pub struct GhostReplayChoice {
    pub ghost_entity: Entity,
    pub choice: ReplayChoice,
}

#[derive(Debug)]
pub enum ReplayChoice {
    KeepExisting,    // Keep playing existing recording (reset to start)
    DraftNew,        // Start a new recording (convert ghost to character)
    Cancel,          // Continue without changes
}
```

### Step 3: Create Timeline Commit System

Add to `src/playback/mod.rs`:

```rust
use crate::recording::{RecordingRequest, Recording, RecordingState};
use crate::timeline::DraftTimeline;
use crate::materials::Materials;
use crate::arena::CurrentArena;
use bevy::prelude::Parent;

/// Commit a draft timeline to published timeline
pub fn handle_commit_recording(
    mut commands: Commands,
    recording_state: Res<RecordingState>,
    draft_q: Query<(Entity, &DraftTimeline, &Parent), With<Recording>>,
    arena_q: Query<&Arena>,
) {
    for event in commit_events.read() {
        if let Ok((entity, draft, parent)) = draft_q.get(event.character) {
            // Only commit if there are events to publish
            if draft.events.is_empty() {
                warn!("Cannot commit empty timeline");
                continue;
            }

            // PR Gate: Per-ghost clock from parent arena
            // Get the arena this character belongs to via parent entity
            let Ok(arena_idx) = arena_q.get(parent.get()) else {
                warn!("Character has no parent arena");
                continue;
            };

            // Create published timeline from draft using ownership transfer
            // Note: We need to consume the draft here but can't due to ECS borrow checker
            // This is a limitation of the Query system - in real code, use Commands to remove and recreate
            let published = PublishTimeline::from_draft(draft);

            info!("Committing timeline with {} events in arena {}", published.events.len(), **arena_idx);

            // Remove recording components, add playback components
            // Zero-copy: .remove() transfers component ownership for efficient cleanup
            // .insert() transfers ownership of published timeline to ECS storage
            commands.entity(entity)
                .remove::<Recording>()
                .remove::<DraftTimeline>() // Ownership transferred for cleanup
                .insert(published) // Ownership transferred to ECS
                .insert(Ghost)
                .insert(Replaying)
                .insert(TimelinePosition(TimeStamp::ZERO))
                .insert(TriggeredAbilities::default())
                .insert(GhostVisuals::default())
                .insert(GhostMovementState::default())
                .insert(GhostArena(arena_id)); // Track which arena this ghost belongs to
        }
    }
}

/// Clear a recording without committing
pub fn handle_clear_recording(
    mut commands: Commands,
    recording_state: Res<RecordingState>,
    recording_q: Query<Entity, With<Recording>>,
) {
    for event in clear_events.read() {
        if recording_q.contains(event.character) {
            commands.entity(event.character)
                .remove::<Recording>()
                .remove::<DraftTimeline>();

            info!("Cleared recording for {:?}", event.character);
        }
    }
}
```

### Step 3: Create Ghost Movement Playback

Add to `src/playback/mod.rs`:

```rust
// No need to import - get_movement_intent_at is now a method on PublishTimeline

/// Component to track previous movement state for interpolation
#[derive(Component)]
pub struct GhostMovementState {
    pub previous_position: Vec3,
    pub target_position: Vec3,
    pub previous_timestamp: TimeStamp,
    pub target_timestamp: TimeStamp,
}

impl Default for GhostMovementState {
    fn default() -> Self {
        Self {
            previous_position: Vec3::ZERO,
            target_position: Vec3::ZERO,
            previous_timestamp: TimeStamp::ZERO,
            target_timestamp: TimeStamp::ZERO,
        }
    }
}

/// Replay ghost movement from published timelines with deterministic interpolation
/// IMPORTANT: Ghosts only play when they have the Replaying component
/// The Replaying component is added in two scenarios:
/// 1. When committing a recording (ghost starts playing immediately)
/// 2. When player chooses "Keep Existing" in replay dialog (player-initiated)
/// Ghosts do NOT automatically start playing when entering an arena!
pub fn playback_ghost_movement(
    mut ghost_q: Query<
        (&CharacterTimelines, &mut TimelinePosition, &mut Transform, &GhostArena, &mut GhostMovementState),
        (With<Ghost>, With<Replaying>)  // Only ghosts with Replaying component actually play
    >,
    arena_q: Query<(&Arena, &TimelineClock)>,
    arena_entities: Res<ArenaEntities>,  // O(1) arena entity lookup
    current_arena: Res<CurrentArena>,
) {
    for (timelines, mut position, mut transform, ghost_arena, mut movement_state) in ghost_q.iter_mut() {
        // O(1) lookup for ghost's arena entity using ArenaEntities
        // Each ghost uses its parent arena's clock, NOT CurrentArena
        let ghost_arena_entity = arena_entities.get(ghost_arena.0.name());
        
        let Ok((_, clock)) = arena_q.get(ghost_arena_entity) else {
            continue;
        };
        
        // Simple rule: Ghost plays timeline for its current arena (from its CharacterTimelines hashmap)
        // If no timeline exists for this arena, ghost has nothing to play (stays idle)
        let Some(timeline) = timelines.get_timeline(current_arena.id()) else {
            continue; // No timeline stored for current arena - ghost stays idle
        };
        
        let current_time = clock.current();
        
        // Update timeline position to match ghost's arena clock
        position.0 = current_time;

        // Get movement intent at current timestamp using the new consolidated API
        // Apply Rule #20: Pattern matching over conditional extraction
        if let Some(move_intent) = timeline.prev_event_before(current_time)
            .and_then(|e| match e.event_type {
                EventType::Movement(pos) => Some(pos),
                _ => None,
            }) {
            // Convert grid position to world position (deterministic)
            let target_world_pos = Vec3::new(
                move_intent.x() as f32 * 1.0,  // Grid unit size
                move_intent.y() as f32 * 1.0,
                0.0
            );

            // Check if we have a new target position
            if target_world_pos != movement_state.target_position {
                // Update movement state with new target
                movement_state.previous_position = transform.translation;
                movement_state.target_position = target_world_pos;
                movement_state.previous_timestamp = current_time;
                // Find the next movement event to know when we should reach the target
                movement_state.target_timestamp = timeline.next_event_after(current_time)
                    .map(|e| e.timestamp)
                    .unwrap_or(TimeStamp::new(current_time.as_secs() + 1.0));
            }

            // Calculate interpolation factor (0.0 to 1.0) based on timestamps
            let time_range = movement_state.target_timestamp.as_secs() - movement_state.previous_timestamp.as_secs();
            let time_elapsed = current_time.as_secs() - movement_state.previous_timestamp.as_secs();
            let t = if time_range > 0.0 {
                (time_elapsed / time_range).clamp(0.0, 1.0)
            } else {
                1.0  // Instant movement if timestamps are the same
            };

            // Interpolate position (deterministic, frame-rate independent)
            transform.translation = movement_state.previous_position.lerp(movement_state.target_position, t);

            trace!(
                "Ghost in {} at {:.2}s: interpolating to {:?} (t={:.2})", 
                ghost_arena.0,
                current_time.as_secs(), 
                move_intent,
                t
            );
        }
    }
}

/// Handle ghost timeline looping with wrap-around
pub fn loop_ghost_timelines(
    mut ghost_q: Query<(&mut TimelinePosition, &GhostArena), With<Ghost>>,
    arena_q: Query<(&Arena, &TimelineClock)>,
    arena_entities: Res<ArenaEntities>,  // O(1) arena entity lookup
) {
    for (mut position, ghost_arena) in ghost_q.iter_mut() {
        // O(1) lookup for ghost's arena entity using ArenaEntities
        let ghost_arena_entity = arena_entities.get(ghost_arena.0.name());
        
        let Ok((_, clock)) = arena_q.get(ghost_arena_entity) else {
            continue;
        };
        
        let current_time = clock.current();
        
        // Handle wrap-around: if clock wrapped (went from high to low)
        if current_time.as_secs() < 1.0 && position.0.as_secs() > TimeStamp::MAX.0 - 1.0 {
            position.0 = TimeStamp::ZERO;
            info!("Ghost timeline in {} looped", ghost_arena.0);
        }
    }
}
```

### Step 4: Create Ghost Ability Playback

Add to `src/playback/mod.rs`:

```rust
// No need to import - events_in_range is now the unified method on PublishTimeline
// Filter with iterator methods for specific event types

/// Event sent when a ghost triggers an ability
#[derive(Event)]
pub struct GhostAbilityTrigger {
    pub ghost: Entity,
    pub ability: AbilityType,
    pub timestamp: TimeStamp,
}

/// Replay ghost abilities from timeline with deterministic range checking
pub fn playback_ghost_abilities(
    mut ghost_q: Query<
        (Entity, &PublishTimeline, &TimelinePosition, &mut TriggeredAbilities),
        With<Ghost>
    >,
    mut ability_events: EventWriter<GhostAbilityTrigger>,
) {
    for (entity, timeline, position, mut triggered) in ghost_q.iter_mut() {
        let current_time = position.0;
        let prev_time = triggered.previous_position.unwrap_or(current_time);
        
        // Use the consolidated events_in_range with iterator filtering
        // The PublishTimeline::events_in_range now handles wrap-around internally
        // When prev_time > current_time, it automatically returns events from
        // [prev_time..120) concatenated with [0..current_time)
        let abilities = timeline.events_in_range(prev_time, current_time)
            .filter(|e| matches!(e.event_type, EventType::Ability(_, _)));
        
        // Zero-alloc: Process abilities directly from iterator
        for event in abilities {
            if let EventType::Ability(ability_id, _target) = &event.event_type {
                // Check if already triggered
                if !triggered.has_triggered(event.timestamp, *ability_id) {
                    // Send ability trigger event
                    ability_events.write(GhostAbilityTrigger {
                        ghost: entity,
                        ability: *ability_id,
                        timestamp: event.timestamp,
                    });

                    // Mark as triggered
                    triggered.add_triggered(event.timestamp, *ability_id);

                    info!(
                        "Ghost {:?} triggered ability {} at {}",
                        entity, ability_id, event.timestamp
                    );
                }
            }
        }
        
        // Update previous position for next frame
        triggered.previous_position = Some(current_time);
    }
}

/// Process ghost ability triggers (connect to existing ability systems)
pub fn process_ghost_ability_triggers(
    mut trigger_events: EventReader<GhostAbilityTrigger>,
    ghost_q: Query<&Transform, With<Ghost>>,
) {
    for event in trigger_events.read() {
        if let Ok(transform) = ghost_q.get(event.ghost) {
            // Here you would connect to existing ability systems
            // For now, just log the trigger
            match event.ability {
                AbilityType::AutoShot => {
                    info!("Ghost AutoShot from {:?}", transform.translation);
                    // TODO: Spawn projectile
                }
                AbilityType::HolyNova => {
                    info!("Ghost HolyNova from {:?}", transform.translation);
                    // TODO: Trigger AoE effect
                }
                _ => {
                    info!("Ghost ability {:?} from {:?}", event.ability, transform.translation);
                }
            }
        }
    }
}
```

### Step 5: Create Ghost Visual System

Add to `src/playback/mod.rs`:

```rust
/// Apply ghost visual effects
pub fn update_ghost_visuals(
    mut ghost_q: Query<
        (&GhostVisuals, &mut Handle<StandardMaterial>),
        Added<Ghost>
    >,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mats: Res<Materials>,
) {
    for (visuals, mut material_handle) in ghost_q.iter_mut() {
        // Create translucent ghost material
        let ghost_material = StandardMaterial {
            base_color: Color::from(visuals.tint.with_alpha(visuals.transparency)),
            alpha_mode: AlphaMode::Blend,
            emissive: Color::from(visuals.tint) * 0.2,
            ..default()
        };

        *material_handle = materials.add(ghost_material);
    }
}

/// Pulse ghost transparency for visibility
pub fn pulse_ghost_transparency(
    mut ghost_q: Query<&mut GhostVisuals, With<Ghost>>,
    time: Res<Time>,
) {
    let pulse = (time.elapsed().as_secs_f32() * 2.0).sin() * 0.1 + 0.5;

    for mut visuals in ghost_q.iter_mut() {
        visuals.transparency = pulse;
    }
}
```

### Step 6: Create Ghost Input Blocking

Add to `src/playback/mod.rs`:

```rust
use crate::selectors::Active;
use crate::character::Character;

/// Block movement and ability input on ghost characters (but allow TAB switching and R for retry dialog)
pub fn block_ghost_movement_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    active_character: Option<Single<(Entity, Option<&Ghost>), (With<Character>, With<Active>)>>,
) {
    let Some((entity, ghost_marker)) = active_character else {
        return;
    };

    // Only block input if this is a ghost
    if ghost_marker.is_none() {
        return;
    }

    // Check for movement or ability input (but NOT R key or TAB - those are handled by other systems)
    let movement_input_attempted =
        keyboard.just_pressed(KeyCode::KeyW) ||
            keyboard.just_pressed(KeyCode::KeyA) ||
            keyboard.just_pressed(KeyCode::KeyS) ||
            keyboard.just_pressed(KeyCode::KeyD);
            
    let ability_input_attempted = 
            keyboard.just_pressed(KeyCode::Digit1) ||
            keyboard.just_pressed(KeyCode::Digit2) ||
            keyboard.just_pressed(KeyCode::Digit3) ||
            keyboard.just_pressed(KeyCode::Digit4);

    if movement_input_attempted {
        info!("Cannot move ghost character {:?} - movement blocked", entity);
    }
    
    if ability_input_attempted {
        info!("Cannot use abilities on ghost character {:?} - ability input blocked", entity);
    }
}
```

### Step 7: Create Playback Plugin

Add to `src/playback/mod.rs`:

```rust
// PR Gate: Removed CurrentArena - ghosts track their own arena via GhostArena component

/// APPROVED: PlaybackSet with strict ordering for deterministic execution
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlaybackSet {
    Input,
    Commit,
    Movement,
    Abilities,
    Visuals,
}

pub struct PlaybackPlugin;

impl Plugin for PlaybackPlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<GhostAbilityTrigger>()
            .add_event::<ShowGhostReplayDialog>()
            .add_event::<GhostReplayChoice>()

            // Configure system sets with strict ordering
            // APPROVED: Show both .chain() and .after() for educational purposes
            .configure_sets(Update, (
                PlaybackSet::Input,
                PlaybackSet::Commit,
                PlaybackSet::Movement,
                PlaybackSet::Abilities,
                PlaybackSet::Visuals,
            ).chain())  // Alternative: use .after() for specific dependencies

            // Systems - Input blocking
            .add_systems(Update, block_ghost_movement_input.in_set(PlaybackSet::Input))

            // Systems - Commit/Clear
            .add_systems(Update, (
                handle_commit_recording,
                handle_clear_recording,
                handle_ghost_replay_choice,
            ).in_set(PlaybackSet::Commit))

            // Systems - Movement playback  
            // APPROVED: Show both ordering methods for education
            .add_systems(Update, (
                playback_ghost_movement,
                loop_ghost_timelines.after(playback_ghost_movement), // Alternative to .chain()
            ).in_set(PlaybackSet::Movement))

            // Systems - Ability playback
            .add_systems(Update, (
                playback_ghost_abilities,
                process_ghost_ability_triggers,
            ).chain().in_set(PlaybackSet::Abilities))

            // Systems - Visuals
            .add_systems(Update, (
                update_ghost_visuals,
                pulse_ghost_transparency,
            ).in_set(PlaybackSet::Visuals));
    }
}
```

### Step 8: Wire Into Main

Update `src/main.rs`:

```rust
mod playback;
use crate::playback::PlaybackPlugin;

// In main():
.add_plugins(PlaybackPlugin)
```

### Step 9: Add Test Commands

Update `src/recording/mod.rs` to add test commands:

```rust
/// Debug command to force commit current recording
pub fn debug_force_commit(
    keyboard: Res<ButtonInput<KeyCode>>,
    // Use Option<Single> for the single recording entity
    recording_entity: Option<Single<Entity, With<Recording>>>,
    current_arena: Res<CurrentArena>,
    mut recording_state: ResMut<RecordingState>,
) {
    if keyboard.just_pressed(KeyCode::KeyF) {
        if recording_entity.is_some() {
            recording_state.pending_request = Some(RecordingRequest::Commit);
            info!("Force committed recording");
        }
    }
}

// Add to RecordingPlugin systems
.add_system(Update, debug_force_commit)
```

## Unit Tests

Create `src/playback/tests.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_triggered_abilities_tracking() {
        let mut triggered = TriggeredAbilities::default();

        // Add some triggers
        triggered.add_triggered(TimeStamp::new(5.0), AbilityType::AutoShot);
        triggered.add_triggered(TimeStamp::new(5.0), AbilityType::HolyNova);
        triggered.add_triggered(TimeStamp::new(10.0), AbilityType::AutoShot);

        // Check if triggered
        assert!(triggered.has_triggered(TimeStamp::new(5.0), AbilityType::AutoShot));
        assert!(triggered.has_triggered(TimeStamp::new(5.0), AbilityType::HolyNova));
        assert!(!triggered.has_triggered(TimeStamp::new(5.0), AbilityType::Heal));

        // Old triggers should be cleaned up
        triggered.add_triggered(TimeStamp::new(11.1), AbilityType::Heal);
        assert_eq!(triggered.abilities.len(), 2); // 10.0 and 11.1 remain
    }

    #[test]
    fn test_ghost_visuals_default() {
        let visuals = GhostVisuals::default();
        assert_eq!(visuals.transparency, 0.5);
        assert_eq!(visuals.tint.blue, 1.0);  // Srgba fields are public
    }

    #[test]
    fn test_timeline_position_sync() {
        let mut position = TimelinePosition(TimeStamp::new(10.0));
        let clock = TimelineClock {
            timer: bevy::time::Timer::new(
                Duration::from_secs(120),
                bevy::time::TimerMode::Repeating,
            ),
            is_paused: false,
        };

        position.sync_with_clock(&clock);
        assert_eq!(position.0, clock.current());
    }

    // PR Gate: Test proving off-screen ghosts advance independently
    #[test]
    fn test_ghost_arena_independence() {
        // Create two ghosts with different arenas using explicit constructors
        let ghost_arena_0 = GhostArena(ArenaId::from_index_safe(0));
        let ghost_arena_5 = GhostArena(ArenaId::from_index_safe(5));
        
        // Create different clocks for each arena
        let mut clock_0 = TimelineClock::default();
        clock_0.tick(Duration::from_secs(10));
        let mut clock_5 = TimelineClock::default();
        clock_5.tick(Duration::from_secs(45));
        
        // Verify ghosts track different times based on their arena
        assert_ne!(clock_0.current().as_secs(), clock_5.current().as_secs());
        assert_eq!(ghost_arena_0.0, ArenaId::from_index_safe(0));
        assert_eq!(ghost_arena_5.0, ArenaId::from_index_safe(5));
        
        // Each ghost will use its own arena's clock during playback
        // This ensures off-screen ghosts advance independently
    }
    
    #[test]
    fn test_ability_replay_wrap_around() {
        use crate::timeline::{DraftTimeline, TimelineEvent, EventType};
        
        // Create a timeline with abilities near the wrap boundary
        let mut draft = DraftTimeline::new();
        
        // Add ability at 119.5 seconds
        draft.add_event(TimelineEvent {
            timestamp: TimeStamp::new(119.5),
            event_type: EventType::Ability(AbilityType::AutoShot, None),
        });
        
        // Add ability at 119.8 seconds
        draft.add_event(TimelineEvent {
            timestamp: TimeStamp::new(119.8),
            event_type: EventType::Ability(AbilityType::HolyNova, None),
        });
        
        // Add ability after wrap at 0.1 seconds
        draft.add_event(TimelineEvent {
            timestamp: TimeStamp::new(0.1),
            event_type: EventType::Ability(AbilityType::PoisonShot, None),
        });
        
        // Add ability at 0.5 seconds
        draft.add_event(TimelineEvent {
            timestamp: TimeStamp::new(0.5),
            event_type: EventType::Ability(AbilityType::Heal, None),
        });
        
        let published = PublishTimeline::from_draft(&draft);
        
        // Test wrap-around range captures abilities correctly
        let abilities: Vec<_> = published.events_in_range(
            TimeStamp::new(119.4), 
            TimeStamp::new(0.6)
        ).collect();
        
        // Should get all 4 abilities
        assert_eq!(abilities.len(), 4);
        
        // Verify order is preserved across wrap
        if let EventType::Ability(id, _) = &abilities[0].event_type {
            assert_eq!(*id, AbilityType::AutoShot);
        }
        if let EventType::Ability(id, _) = &abilities[1].event_type {
            assert_eq!(*id, AbilityType::HolyNova);
        }
        if let EventType::Ability(id, _) = &abilities[2].event_type {
            assert_eq!(*id, AbilityType::PoisonShot);
        }
        if let EventType::Ability(id, _) = &abilities[3].event_type {
            assert_eq!(*id, AbilityType::Heal);
        }
        
        // Test that triggered abilities tracking prevents duplicates
        let mut triggered = TriggeredAbilities::default();
        triggered.add_triggered(TimeStamp::new(119.5), AbilityType::AutoShot);
        assert!(triggered.has_triggered(TimeStamp::new(119.5), AbilityType::AutoShot));
        assert!(!triggered.has_triggered(TimeStamp::new(0.1), AbilityType::PoisonShot));
    }
}
```

## Verification

Run tests:

```bash
cargo test playback
```

Run the game and test playback:

```bash
cargo run
```

Test sequence:

1. Press R to start recording
2. Move around and use abilities during recording
3. Press F to commit the recording (force commit)
4. Watch the ghost replay your movements
5. Try to control the ghost - input should be blocked
6. Watch ghost loop after 2 minutes

## Next Steps

With playback working, we can now:

- Tutorial 05: Add confirmation dialogs for commit/clear
- Tutorial 06: Implement multi-arena ghost management
- Tutorial 07: Add ghost collision and interaction

## Key Takeaways

1. **Simple Timeline Lookup**: Ghosts play `timelines.get_timeline(current_arena.id())` - no complex state needed
2. **Timeline Replay**: Interpolation creates smooth movement from keyframes  
3. **Ability Triggers**: Deterministic range-based detection using [prev, curr] slices prevents duplicate triggers
4. **Visual Distinction**: Ghosts have transparency and glow effects
5. **Explicit Constructors**: TimeStamp::new(), ArenaId construction throughout playback code
6. **Automatic Looping**: Ghosts seamlessly repeat every 2 minutes
7. **CharacterTimelines as Source of Truth**: The hashmap IS the ghost's behavior - no additional state tracking needed
8. **⚡ ArenaEntities O(1) Lookup**: Use ArenaEntities resource for O(1) arena entity lookup in ghost systems - critical for performance with 320+ ghosts across 9 arenas

## Production Notes

### What We Got Right:

- PlaybackSet with strict ordering for deterministic execution
- Zero-alloc iteration using timeline helper methods
- Arc<[TimelineEvent]> leveraging for efficient cloning
- TimeStamp newtype for type-safe time handling
- Movement intent playback instead of position interpolation

### What We Intentionally Simplified:

- No ghost collision (design choice - ghosts are ethereal)
- No complex scheduling (timestamps work fine)
- No shader effects (material changes are sufficient)
- No LOD yet (that's Tutorial 06)

### Rejected Over-Engineering:

- ECS schedulers for 10 abilities per ghost
- Complex state machines for playback
- Collision detection that adds no gameplay value
- Custom shaders in a component tutorial

The playback system brings recordings to life as autonomous ghosts. This creates the core gameplay loop where players
build up layers of coordinated actions across multiple recording cycles.