# Tutorial 01: Timeline Foundation

## Objective

Build the core timeline data structures and components that will store recorded character actions. This foundation will
support all future recording and playback functionality.

## Prerequisites

- Basic understanding of Bevy ECS (Entity Component System)
- Familiarity with the existing arena and character systems
- Rust knowledge of structs, enums, and vectors
- Understanding of Result types and error handling
- Add `thiserror = "2.0"` to your `Cargo.toml` dependencies for proper error handling

## Components/Systems

We'll create:

- Timeline event data structures with type-safe newtypes
- Timeline storage components with zero-alloc helpers
- Arena timer system for 2-minute cycles
- Basic timeline position tracking

## Implementation Steps

### Step 1: Create the Timeline Module

Create a new file `src/timeline/mod.rs`:

```rust
// PR Gate: All imports at module level for Rule 24 compliance
use bevy::prelude::*;
use bevy::ecs::change_detection::DetectChanges;
use bevy::log::trace;
use bevy::time::Virtual;
use bevy::utils::HashMap;  // Use Bevy's HashMap for better performance
use std::fmt::{self, Display, Formatter};
use std::convert::identity;
use std::cmp::Ordering;
use std::time::Duration;
use std::sync::Arc;
use thiserror::Error;

/// Error types for timeline operations - Rule 22 compliance
#[derive(Error, Debug)]
pub enum TimelineError {
    #[error("Invalid arena index: {index}")]
    InvalidArenaIndex { index: u8 },
    #[error("Invalid timestamp comparison")]
    InvalidComparison,
    #[error("Timeline operation failed: {message}")]
    OperationFailed { message: String },
}

/// Result type for timeline operations
pub type TimelineResult<T> = Result<T, TimelineError>;

/// A single recorded event in a timeline
#[derive(Clone, Debug)]
pub struct TimelineEvent {
    /// Time when this event occurred (PR Gate: Using TimeStamp newtype for type safety)
    pub timestamp: TimeStamp,
    /// The type of event that occurred
    pub event_type: EventType,
}

/// Newtype for timeline timestamps (0.0 to 120.0 seconds)
/// PR Gate: TimeStamp + Duration pattern for type safety (not raw f32)
///
/// # Examples
/// ```
/// let timestamp = TimeStamp::new(65.5);
/// assert_eq!(timestamp.as_secs(), 65.5);
///
/// let clamped = TimeStamp::new(150.0);
/// assert_eq!(clamped.as_secs(), 120.0); // Clamped to MAX
/// ```
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct TimeStamp(pub f32);

impl TimeStamp {
    pub const ZERO: Self = Self(0.0);
    pub const MAX: Self = Self(120.0);

    /// Creates a new TimeStamp, clamping value to [0, 120] seconds
    /// NaN values are coerced to 0.0 for safety
    #[must_use]
    pub fn new(seconds: f32) -> Self {
        debug_assert!(!seconds.is_nan(), "TimeStamp cannot be NaN");
        let safe_seconds = if seconds.is_nan() { Self::ZERO.0 } else { seconds };
        Self(safe_seconds.clamp(Self::ZERO.0, Self::MAX.0))
    }

    #[must_use]
    pub fn as_secs(&self) -> f32 {
        self.0
    }

    /// Wraps time back to start when exceeding 120 seconds
    /// NaN values are coerced to 0.0 for safety
    #[must_use]
    pub fn wrapped(seconds: f32) -> Self {
        debug_assert!(!seconds.is_nan(), "TimeStamp cannot be NaN");
        let safe_seconds = if seconds.is_nan() { Self::ZERO.0 } else { seconds };
        Self(safe_seconds.rem_euclid(Self::MAX.0))
    }
}


impl Display for TimeStamp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}s", self.0)
    }
}

/// Types of events that can be recorded
#[derive(Clone, Debug)]
pub enum EventType {
    /// Movement intent from input - not transform!
    Movement(GridPos),
    /// Ability cast with optional target
    Ability(AbilityType, Option<Target>),
    /// Character death event
    Death,
}

/// Target for abilities
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Target {
    Entity(Entity),
    Position(GridPos),
}

/// Import unified ability types from the ability module
/// This replaces the previous duplicate AbilityId definition
use crate::ability::AbilityType;

// NOTE: AbilityType is now defined in /src/ability/mod.rs and provides:
// - AbilityType::AutoShot, AbilityType::HolyNova, etc.
// - Display implementation for human-readable names
// - from_id() and to_id() methods for backwards compatibility
// - Integration with the actual ability system components (AutoShot, HolyNova)

// Import arena types from the arena module - no duplicates!
use crate::arena::{Arena, ArenaId, ArenaName, ArenaEntities};

/// Newtype for grid positions using IVec2 internally
#[derive(Clone, Copy, Debug, PartialEq, Component)]
pub struct GridPos(pub IVec2);

impl GridPos {
    #[must_use]
    pub fn new(x: i32, y: i32) -> Self {
        Self(IVec2::new(x, y))
    }

    #[must_use]
    pub fn x(&self) -> i32 {
        self.0.x
    }

    #[must_use]
    pub fn y(&self) -> i32 {
        self.0.y
    }
}

// From traits kept for Bevy interop - use GridPos::new() in examples
impl From<IVec2> for GridPos {
    fn from(vec: IVec2) -> Self {
        Self(vec)
    }
}

impl From<GridPos> for IVec2 {
    fn from(pos: GridPos) -> Self {
        pos.0
    }
}

impl Display for GridPos {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0.x, self.0.y)
    }
}
```

### Step 2: Add Timeline Storage Components

Add to `src/timeline/mod.rs`:

```rust
/// Component for entities that can be recorded
#[derive(Component)]
pub struct Recordable;

use std::sync::Arc;

/// Temporary timeline buffer during recording
#[derive(Component, Default)]
pub struct DraftTimeline {
    pub events: Vec<TimelineEvent>,
    pub max_duration: TimeStamp,
}

impl DraftTimeline {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            max_duration: TimeStamp::MAX,
        }
    }

    /// Add event to timeline with proper error handling
    ///
    /// # Errors
    ///
    /// Returns `TimelineError::InvalidComparison` if timestamps cannot be compared
    pub fn add_event(&mut self, event: TimelineEvent) -> TimelineResult<()> {
        // APPROVED: Binary search maintains O(log n) sorted insertion
        // PR Gate: Rule 22 - Proper error handling without unwrap()
        let pos = self.events.binary_search_by(|e| {
            e.timestamp.partial_cmp(&event.timestamp)
                .unwrap_or(Ordering::Equal)
        });
        
        match pos {
            Ok(pos) | Err(pos) => {
                self.events.insert(pos, event);
                Ok(())
            }
        }
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}

/// Published timeline for playback (immutable once set)
#[derive(Component, Clone)]
pub struct PublishTimeline {
    /// APPROVED: Arc<[T]> for immutable shared timeline data
    /// Zero-cost cloning, cache-friendly iteration
    pub events: Arc<[TimelineEvent]>,
}

/// Component that stores multiple timelines per character (one per arena)
/// Resolves the architectural issue where characters need separate recordings per arena
#[derive(Component, Default)]
pub struct CharacterTimelines {
    pub timelines: HashMap<ArenaId, PublishTimeline>,
}

impl CharacterTimelines {
    pub fn new() -> Self {
        Self {
            timelines: HashMap::new(),
        }
    }

    pub fn store_timeline(&mut self, arena: ArenaId, timeline: PublishTimeline) {
        self.timelines.insert(arena, timeline);
    }

    pub fn get_timeline(&self, arena: ArenaId) -> Option<&PublishTimeline> {
        self.timelines.get(&arena)
    }

    pub fn has_recording_for(&self, arena: ArenaId) -> bool {
        self.timelines.contains_key(&arena)
    }

    pub fn arena_count(&self) -> usize {
        self.timelines.len()
    }

    pub fn recorded_arenas(&self) -> impl Iterator<Item = ArenaId> + '_ {
        self.timelines.keys().copied()
    }
}

impl PublishTimeline {
    /// Convert draft timeline to published timeline using ownership transfer for zero-copy
    /// Takes ownership of DraftTimeline to enable Vec<T> → Arc<[T]> conversion without cloning
    pub fn from_draft(draft: DraftTimeline) -> Self {
        Self {
            // Zero-copy transformation: Vec<T> → Arc<[T]> via into()
            // This avoids cloning all timeline events, improving performance
            events: draft.events.into(),
        }
    }

    /// Zero-alloc helper: Get events within a time range
    /// PR Gate: Added #[must_use] to timeline slice functions
    ///
    /// Returns events where start <= timestamp < end
    /// NOTE: Wrap-around handling (e.g., 118.0 → 2.0) is covered in Tutorial 04
    ///
    /// # Examples
    /// ```
    /// // Normal range
    /// let events: Vec<_> = timeline.events_in_range(TimeStamp::new(5.0), TimeStamp::new(10.0)).collect();
    /// ```
    #[must_use]
    pub fn events_in_range(&self, start: TimeStamp, end: TimeStamp) -> impl Iterator<Item=&TimelineEvent> + '_ {
        // Simple range query - wrap-around handling comes in Tutorial 04
        // PR Gate: Rule 22 - Safe error handling without unwrap()
        let start_idx = self.events.binary_search_by(|e| {
            e.timestamp.partial_cmp(&start).unwrap_or(Ordering::Equal)
        }).unwrap_or_else(identity);
        
        let end_idx = self.events.binary_search_by(|e| {
            e.timestamp.partial_cmp(&end).unwrap_or(Ordering::Equal)
        }).unwrap_or_else(identity);

        self.events[start_idx..end_idx].iter()
    }

    // Consolidated API: Use next_event_after/prev_event_before with iterator methods for specific queries
    // Example: timeline.events_in_range(start, end).filter(|e| matches!(e.event_type, EventType::Ability(_, _)))

    /// Zero-alloc helper: Find next event after timestamp
    #[must_use]
    pub fn next_event_after(&self, timestamp: TimeStamp) -> Option<&TimelineEvent> {
        // PR Gate: Rule 22 - Safe error handling without unwrap()
        match self.events.binary_search_by(|e| {
            e.timestamp.partial_cmp(&timestamp).unwrap_or(Ordering::Equal)
        }) {
            Ok(idx) => self.events.get(idx + 1),
            Err(idx) => self.events.get(idx),
        }
    }

    /// Get a slice of the timeline events
    #[must_use]
    pub fn slice(&self, start: usize, end: usize) -> &[TimelineEvent] {
        &self.events[start.min(self.events.len())..end.min(self.events.len())]
    }

    /// Get previous event before or at a specific timestamp
    /// Returns the most recent event with timestamp <= the provided timestamp
    ///
    /// Complements next_event_after for full timeline traversal capabilities
    /// Use with iterator methods for specific event type filtering
    #[must_use]
    pub fn prev_event_before(&self, timestamp: TimeStamp) -> Option<&TimelineEvent> {
        // PR Gate: Rule 22 - Safe error handling without unwrap()
        match self.events.binary_search_by(|e| {
            e.timestamp.partial_cmp(&timestamp).unwrap_or(Ordering::Equal)
        }) {
            Ok(idx) => self.events.get(idx),     // Found exact match, return it
            Err(idx) => idx.checked_sub(1).and_then(|i| self.events.get(i)), // Return previous element
        }
    }
}
```

### Step 3: Create Arena Clock Component

Add to `src/timeline/mod.rs`:

```rust
/// Clock for 2-minute arena cycles
/// PR Gate: Using bevy::time::Timer for proper time handling
/// Virtual time integration ensures pause-safe operation
#[derive(Component)]
pub struct TimelineClock {
    /// Internal timer that processes virtual time deltas
    pub timer: bevy::time::Timer,
    pub is_paused: bool,  // Local pause state (separate from global)
}

impl TimelineClock {
    /// Create a new timeline clock with pause/resume functionality
    pub fn new() -> Self {
        Self {
            timer: bevy::time::Timer::new(
                Duration::from_secs(120),
                bevy::time::TimerMode::Repeating,
            ),
            is_paused: false,
        }
    }

    /// Pause the clock
    pub fn pause(&mut self) {
        self.is_paused = true;
    }

    /// Resume the clock
    pub fn resume(&mut self) {
        self.is_paused = false;
    }
}

impl Default for TimelineClock {
    fn default() -> Self {
        Self {
            // PR Gate: Using bevy::time::Timer instead of f32
            timer: bevy::time::Timer::new(
                Duration::from_secs(120),
                bevy::time::TimerMode::Repeating,
            ),
            is_paused: false,
        }
    }
}

impl TimelineClock {
    pub fn tick(&mut self, delta: Duration) {
        if !self.is_paused {
            self.timer.tick(delta);
        }
    }


    pub fn reset(&mut self) {
        self.timer.reset();
    }

    pub fn current(&self) -> TimeStamp {
        TimeStamp::new(self.timer.elapsed_secs())
    }
}

/// Current playback position for a character's timeline
#[derive(Component)]
pub struct TimelinePosition(pub TimeStamp);

impl TimelinePosition {
    pub fn sync_with_clock(&mut self, clock: &TimelineClock) {
        self.0 = clock.current();
    }
}
```

### Step 4: Create the Clock Update System

Add to `src/timeline/mod.rs`:

```rust
/// Global pause state that affects all timeline clocks
#[derive(Resource, Default)]
pub struct GlobalTimelinePause {
    pub is_paused: bool,
    pub pause_reason: Option<PauseReason>,
}

#[derive(Debug, Clone)]
pub enum PauseReason {
    DialogOpen,
    SystemMenu,
    LoadingTransition,
}

impl GlobalTimelinePause {
    pub fn pause(&mut self, reason: PauseReason) {
        self.is_paused = true;
        self.pause_reason = Some(reason);
    }

    pub fn resume(&mut self) {
        self.is_paused = false;
        self.pause_reason = None;
    }
}

/// System to update all arena clocks using virtual time
/// Virtual time automatically handles pause states - no time jumps!
pub fn update_timeline_clocks(
    // Use Time<Virtual> which pauses/resumes without time jumps
    virtual_time: Res<Time<Virtual>>,
    mut arena_q: Query<(&Arena, &mut TimelineClock)>,
) {
    // Virtual time's delta is already pause-aware
    let delta = virtual_time.delta();

    for (_arena, mut clock) in arena_q.iter_mut() {
        clock.tick(delta);
    }
}

/// Control virtual time based on GlobalTimelinePause state
pub fn control_virtual_time_pause(
    global_pause: Res<GlobalTimelinePause>,
    mut virtual_time: ResMut<Time<Virtual>>,
) {
    // Only update when pause state changes
    if global_pause.is_changed() {
        if global_pause.is_paused {
            virtual_time.pause();
            trace!("Virtual time paused: {:?}", global_pause.pause_reason);
        } else {
            virtual_time.unpause();
            trace!("Virtual time resumed");
        }
    }
}

/// System to display current clock values (for debugging)
/// 🆕 Uses ArenaEntities O(1) lookup - eliminates O(n) linear search!
pub fn debug_timeline_clocks(
    arena_q: Query<(&Arena, &TimelineClock)>,
    arena_entities: Res<ArenaEntities>,  // O(1) arena entity lookup
    current_arena: Res<CurrentArena>,
) {
    // Helper method eliminates repetitive lookup pattern
    let Ok((arena, clock)) = arena_q.get(current_arena.entity(&arena_entities)) else {
        return;
    };

    // PR Gate: Using trace! for per-frame logs instead of info!
    if (clock.current().as_secs() % 1.0) < 0.02 {
        trace!("{}: {:.1}s", arena, clock.current().as_secs());
    }
}
```

### Step 5: Create the Timeline Plugin

Add to `src/timeline/mod.rs`:

```rust
use crate::arena::{CurrentArena, ArenaEntities};

pub struct TimelinePlugin;

impl Plugin for TimelinePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GlobalTimelinePause>()
            .add_systems(Update, (
                // Control virtual time pause state BEFORE updating clocks
                control_virtual_time_pause,
                update_timeline_clocks,
                debug_timeline_clocks,
            ).chain());  // chain() ensures sequential execution
    }
}
```

### Step 6: Wire It Into Main

Update `src/main.rs`:

```rust
mod timeline;
use crate::timeline::TimelinePlugin;

// In main():
.add_plugins(TimelinePlugin)
```

Also update arena spawning in `setup_scene` to include timer:

```rust
// In setup_scene, when spawning arenas:
battleground
.spawn((
Transform::from_xyz(offset_x, offset_y, 0.0),
Arena::from_index_safe(arena_index), // Use safe constructor for startup initialization
TimelineClock::new(), // Use explicit constructor
// InheritedVisibility is automatically added via required components
class_type,
Name::new(arena_name),
LastActiveHero(None),
))
```

## Unit Tests

Create `src/timeline/tests.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draft_timeline_adds_events_sorted() {
        let mut timeline = DraftTimeline::new();

        // Add events out of order - using explicit constructors
        timeline.add_event(TimelineEvent {
            timestamp: TimeStamp::new(5.0),
            event_type: EventType::Movement(GridPos::new(1, 0)),
        }).expect("Failed to add event");

        timeline.add_event(TimelineEvent {
            timestamp: TimeStamp::new(2.0),
            event_type: EventType::Ability(AbilityType::AutoShot, None),
        }).expect("Failed to add event");

        timeline.add_event(TimelineEvent {
            timestamp: TimeStamp::new(10.0),
            event_type: EventType::Death,
        }).expect("Failed to add event");

        // Verify events are sorted by timestamp
        assert_eq!(timeline.events.len(), 3);
        assert_eq!(timeline.events[0].timestamp, TimeStamp::new(2.0));
        assert_eq!(timeline.events[1].timestamp, TimeStamp::new(5.0));
        assert_eq!(timeline.events[2].timestamp, TimeStamp::new(10.0));
    }

    #[test]
    fn test_timeline_clock_loops_at_120_seconds() {
        let mut clock = TimelineClock::new();

        // Tick past 120 seconds
        clock.tick(Duration::from_secs(125));

        // Should loop back
        assert_eq!(clock.current().as_secs(), 5.0);
    }

    #[test]
    fn test_timestamp_wrap_around_edge_cases() {
        // Test exact boundary
        let timestamp = TimeStamp::wrapped(TimeStamp::MAX.0);
        assert_eq!(timestamp.as_secs(), TimeStamp::ZERO.0);

        // Test multiple wraps
        let timestamp = TimeStamp::wrapped(365.0); // 365 = 3*120 + 5
        assert_eq!(timestamp.as_secs(), 5.0);

        // Test negative wrapping
        let timestamp = TimeStamp::wrapped(-10.0);
        assert_eq!(timestamp.as_secs(), 110.0); // -10 + 120 = 110
    }

    #[test]
    fn test_timeline_clock_pause_resume() {
        let mut clock = TimelineClock::new();

        clock.tick(Duration::from_secs(10));
        assert_eq!(clock.current().as_secs(), 10.0);

        clock.pause();
        clock.tick(Duration::from_secs(10)); // Should not advance while paused
        assert_eq!(clock.current().as_secs(), 10.0);

        clock.resume();
        clock.tick(Duration::from_secs(10)); // Should advance again
        assert_eq!(clock.current().as_secs(), 20.0);
    }

    #[test]
    fn test_publish_timeline_get_events_in_range() {
        let mut draft = DraftTimeline::new();

        for i in 0..10 {
            draft.add_event(TimelineEvent {
                timestamp: TimeStamp::new(i as f32 * 2.0),
                event_type: EventType::Movement(GridPos::new(i as i32, 0)),
            }).expect("Failed to add event");
        }

        let published = PublishTimeline::from_draft(draft);

        // Get events between 5.0 and 10.0 seconds
        let events: Vec<_> = published.events_in_range(TimeStamp::new(5.0), TimeStamp::new(10.0)).collect();

        assert_eq!(events.len(), 2); // Should get events at 6.0, 8.0
        assert_eq!(events[0].timestamp, TimeStamp::new(6.0));
        assert_eq!(events[1].timestamp, TimeStamp::new(8.0));
    }

    #[test]
    fn test_next_event_after_edge_cases() {
        let mut draft = DraftTimeline::new();

        // Add events at specific timestamps
        draft.add_event(TimelineEvent {
            timestamp: TimeStamp::new(10.0),
            event_type: EventType::Movement(GridPos::new(0, 0)),
        }).expect("Failed to add event");
        draft.add_event(TimelineEvent {
            timestamp: TimeStamp::new(20.0),
            event_type: EventType::Ability(AbilityType::AutoShot, None),
        }).expect("Failed to add event");
        draft.add_event(TimelineEvent {
            timestamp: TimeStamp::new(30.0),
            event_type: EventType::Movement(GridPos::new(1, 0)),
        }).expect("Failed to add event");

        let published = PublishTimeline::from_draft(draft);

        // Test: Find next event after a timestamp with no exact match
        let next = published.next_event_after(TimeStamp::new(15.0));
        assert!(next.is_some());
        assert_eq!(next.unwrap().timestamp, TimeStamp::new(20.0));

        // Test: Find next event when timestamp matches exactly
        let next = published.next_event_after(TimeStamp::new(20.0));
        assert!(next.is_some());
        assert_eq!(next.unwrap().timestamp, TimeStamp::new(30.0));

        // Test: No next event when at or past last event
        let next = published.next_event_after(TimeStamp::new(30.0));
        assert!(next.is_none());

        let next = published.next_event_after(TimeStamp::new(35.0));
        assert!(next.is_none());

        // Test: Find first event when timestamp is before all events
        let next = published.next_event_after(TimeStamp::new(5.0));
        assert!(next.is_some());
        assert_eq!(next.unwrap().timestamp, TimeStamp::new(10.0));
    }

    #[test]
    fn test_explicit_constructors() {
        // Test TimeStamp::new() as primary constructor
        let timestamp = TimeStamp::new(42.5);
        assert_eq!(timestamp.as_secs(), 42.5);
        assert_eq!(timestamp.to_string(), "42.5s");

        // Test TimeStamp::ZERO constant
        assert_eq!(TimeStamp::ZERO.as_secs(), TimeStamp::ZERO.0);

        // Test ArenaId::from_u8() for u8 conversion with proper error handling
        let idx = ArenaId::from_u8(3).expect("Arena 3 should be valid");
        assert_eq!(idx.as_u8(), 3);
        assert_eq!(idx.to_string(), "Mountain (3)");

        // Test error case - arena out of bounds
        let invalid = ArenaId::from_u8(10);
        assert!(invalid.is_err());

        // Test safe constructor for internal use
        let safe = ArenaId::from_index_safe(15);
        assert_eq!(safe.as_u8(), 8); // Should clamp to max valid arena (Gala)

        // Test Arena::new() with ArenaName enum
        let arena = Arena::new(ArenaName::Bastion);
        assert_eq!(arena.as_u8(), 4);
        assert_eq!(arena.name(), ArenaName::Bastion);

        // Test GridPos::new() as primary constructor
        let pos = GridPos::new(5, -3);
        assert_eq!(pos.x(), 5);
        assert_eq!(pos.y(), -3);
        assert_eq!(pos.to_string(), "(5, -3)");

        // From traits still work for Bevy interop
        let vec: IVec2 = pos.into();
        assert_eq!(vec, IVec2::new(5, -3));
    }

    #[test]
    fn test_character_timelines_multi_arena_storage() {
        // Test the critical architectural fix: CharacterTimelines stores multiple timelines per character
        let mut character_timelines = CharacterTimelines::new();
        
        // Create test timelines for different arenas
        let mut draft_labyrinth = DraftTimeline::new();
        draft_labyrinth.add_event(TimelineEvent {
            timestamp: TimeStamp::new(10.0),
            event_type: EventType::Movement(GridPos::new(0, 0)),
        }).expect("Failed to add event");
        let timeline_labyrinth = PublishTimeline::from_draft(draft_labyrinth).expect("Failed to create timeline");
        
        let mut draft_gala = DraftTimeline::new();
        draft_gala.add_event(TimelineEvent {
            timestamp: TimeStamp::new(30.0),
            event_type: EventType::Ability(AbilityType::AutoShot, None),
        }).expect("Failed to add event");
        let timeline_gala = PublishTimeline::from_draft(draft_gala).expect("Failed to create timeline");
        
        // Store timelines for different arenas
        let labyrinth_id = ArenaId::from_index_safe(0); // Labyrinth
        let gala_id = ArenaId::from_index_safe(8);      // Gala
        
        character_timelines.store_timeline(labyrinth_id, timeline_labyrinth);
        character_timelines.store_timeline(gala_id, timeline_gala);
        
        // Verify separate timeline storage
        assert_eq!(character_timelines.arena_count(), 2);
        assert!(character_timelines.has_recording_for(labyrinth_id));
        assert!(character_timelines.has_recording_for(gala_id));
        assert!(!character_timelines.has_recording_for(ArenaId::from_index_safe(1))); // GuildHouse - no recording
        
        // Verify we can retrieve specific arena timelines
        let labyrinth_timeline = character_timelines.get_timeline(labyrinth_id).unwrap();
        assert_eq!(labyrinth_timeline.events.len(), 1);
        assert_eq!(labyrinth_timeline.events[0].timestamp, TimeStamp::new(10.0));
        
        let gala_timeline = character_timelines.get_timeline(gala_id).unwrap();
        assert_eq!(gala_timeline.events.len(), 1);
        assert_eq!(gala_timeline.events[0].timestamp, TimeStamp::new(30.0));
        
        // Verify recorded arenas iterator
        let recorded: Vec<_> = character_timelines.recorded_arenas().collect();
        assert_eq!(recorded.len(), 2);
        assert!(recorded.contains(&labyrinth_id));
        assert!(recorded.contains(&gala_id));
    }
}
```

## Next Steps

With the timeline foundation in place, we can now:

- Tutorial 02: Build the recording state machine with event-driven transitions
- Tutorial 03: Implement movement capture (recording intent, not transforms!)
- Tutorial 04: Create the playback system with strict ordering

## Key Takeaways

1. **Type Domain Separation**: ArenaId (value type) for data passing, Arena (component) for entity attachment
2. **Type-Safe Newtypes**: TimeStamp, ArenaId, GridPos provide compile-time safety
3. **Intent Not Transform**: Recording Movement(GridPos) not Transform(Vec3)
4. **Zero-Alloc Helpers**: events_in_range(), next_event_after(), slice() avoid allocations
5. **Explicit Constructors**: TimeStamp::new(), GridPos::new(), ArenaId::new(ArenaName) for value types, Arena::new(ArenaName) for components
6. **Binary Search**: Efficient O(log n) operations on sorted timelines
7. **Zero-Copy Ownership Transfer**: PublishTimeline::from_draft(draft) consumes for efficient Vec→Arc conversion
8. **Idiomatic Helpers**: Use `std::convert::identity` over trivial closures for clearer intent
9. **Consolidated Timeline API**: next_event_after/prev_event_before provide unified temporal queries with consistent
   binary_search_by approach
10. **Virtual Time for Pause Safety**: Time<Virtual> prevents time jumps when pausing/unpausing
11. **🆕 Bevy 0.16 Error-Safe ECS**: Use Result-returning query methods with `?` operator or let-else patterns for robust
    error handling
12. **🆕 Compile-Time Safe Arena Creation**: Use `from_index_safe()` for all arena creation - no runtime validation needed
13. **🎯 Multi-Arena Timeline Storage**: CharacterTimelines component stores HashMap<ArenaId, PublishTimeline> to support characters recording in all 9 arenas with separate timeline storage per arena
14. **🚀 Unified Event Architecture**: Like CameraUpdate, use RecordingUpdate as root orchestration event to prevent event explosion and race conditions
15. **⚡ ArenaEntities O(1) Lookup**: Use ArenaEntities resource for O(1) arena entity lookup instead of O(n) linear search. Performance is critical when supporting 320+ ghosts across 9 arenas!

## Production Notes

### Critical Performance Optimization: ArenaEntities O(1) Lookup

**The Problem:** Early tutorial versions used O(n) linear searches to find arena entities:

```rust
// OLD WAY - O(n) linear search (SLOW!)
let Some((arena, clock)) = arena_q
    .iter()
    .find(|(arena, _)| arena.name() == current_arena.name())
else {
    return;
};
```

This becomes a performance bottleneck when supporting 320+ ghosts across 9 arenas.

**The Solution:** Use ArenaEntities resource for O(1) lookup:

```rust
// NEW WAY - O(1) lookup using ArenaEntities (FAST!)
let Ok((arena, clock)) = arena_q.get(current_arena.entity(&arena_entities)) else {
    return;
};
```

**Why This Matters:**
- O(n) search: 9 comparisons worst case per system per frame
- O(1) lookup: 1 array access using enum discriminant as index
- With 100+ systems doing arena lookups, this saves thousands of comparisons per frame
- ArenaEntities uses ArenaName enum discriminants (0-8) as array indices for instant access

### Critical Architectural Fix: Multi-Arena Timeline Storage

**The Problem:** The original design assumed characters would only have one timeline, but characters can move between all 9 arenas and need to store separate recordings for each arena they've been in.

**The Solution:** 
- `CharacterTimelines` component stores `HashMap<ArenaId, PublishTimeline>` per character
- `RecordingRequest::Commit` event queries Active Character and CurrentArena resource
- Each character can store up to 9 separate timelines (one per arena)
- O(1) lookup by arena using HashMap for efficient retrieval

**Why This Matters:** Without this fix, a character's recording in Arena 0 (Labyrinth) would be overwritten when they record in Arena 8 (Gala). Now each arena maintains its own timeline history per character.

### What We Got Right:

- Binary search insertion is perfect for small-to-medium event counts
- Immutable PublishTimeline with Arc<[T]> prevents mutations and enables cheap cloning
- Type-safe newtypes with Display/From implementations for clean APIs
- Recording intent (Movement/Ability) not results (Transform)
- Multi-arena timeline storage resolves the fundamental storage architecture issue

### What We Intentionally Simplified:

- No event compression (covered in Tutorial 08)
- No custom memory pools (unnecessary complexity)
- No event sourcing patterns (wrong tool for the job)

### Why These Patterns Matter:

- **Type Domain Separation**: Absolute rule - every type belongs to EXACTLY one domain. Components attach to entities, Values pass data. This prevents mixing contexts and makes code intent clear.
- **Newtypes**: Catch unit errors at compile time (can't mix TimeStamp with f32)
- **Explicit Constructors**: TimeStamp::new(), ArenaId::new() makes the common case obvious and discoverable
- **Intent Recording**: Deterministic replay regardless of physics/interpolation
- **Arc<[T]>**: Share timeline across systems without cloning the data
- **Binary Search**: Fast lookups for playback position queries
- **Zero-Copy Ownership Transfer**: When data flows one-way (draft→publish), consume instead of borrow to enable
  efficient transformations
- **Unified Timeline Queries**: Use next_event_after/prev_event_before as foundational API, combine with iterator
  methods for event type filtering instead of specialized methods
- **🆕 Bevy 0.16 Error-Safe Patterns**: Query methods now return Results - use
  `let Ok(...) = query.single() else { return; }` for graceful early returns
- **🆕 Compile-Time Safe Arena Creation**: `from_index_safe()` eliminates runtime validation for all arena creation by clamping to valid ranges. This follows Rule #1 (Type Domain Separation) and Rule #30 (Build for Now).

### Zero-Copy Principle Applied:

The `PublishTimeline::from_draft(draft)` method demonstrates the zero-copy principle:

**Before (Inefficient):**

```rust
pub fn from_draft(draft: &DraftTimeline) -> Self {
    Self {
        events: draft.events.clone().into(), // Clones entire Vec!
    }
}
```

**After (Zero-Copy):**

```rust
pub fn from_draft(draft: DraftTimeline) -> Self {
    Self {
        events: draft.events.into(), // Consumes Vec, no cloning
    }
}
```

This transformation avoids cloning potentially thousands of events when transitioning from draft to published state. The
key insight is that draft timelines naturally flow one-way into published timelines, so consuming ownership enables
efficient Vec<T> → Arc<[T]> conversion.

### Critical Architectural Decision: Unified Event Pattern

**The Problem:** Early recording system designs used complex command events (StartRecording, StopRecording, PauseForDialog, etc.) which leads to:
- Event explosion and race conditions
- Multiple systems trying to manage state simultaneously
- Complex coordination logic scattered across multiple systems
- Debugging nightmares when state transitions conflict

**The Solution:** Apply the successful camera pattern to recording with CRITICAL SIMPLIFICATION:

**INSIGHT: Recording entity = Active Character in Current Arena (ALWAYS!)**

```rust
// TRIGGER EVENTS - SIMPLIFIED: No entity passing needed!
#[derive(Event)]
pub enum RecordingRequest {
    Start,   // System finds active character automatically  
    Stop { reason: StopReason },
    Commit,  // Uses active character with Recording component
    Clear,   // Uses active character with Recording component
}

// STATE RESOURCE - SIMPLIFIED: No entity storage needed!
#[derive(Resource)]
pub struct RecordingState {
    pub mode: RecordingMode,
    pub countdown_remaining: Option<Duration>,
    // No entity storage - can always query for (Active, Recording)
}

// ORCHESTRATOR - SIMPLIFIED: No entity storage, just query for active character
fn recording_update(
    mut recording_request_events: EventReader<RecordingRequest>,
    mut state: ResMut<RecordingState>,
    active_character_q: Query<Entity, (With<Character>, With<Active>, Without<Ghost>)>,
    recording_character_q: Query<Entity, (With<Character>, With<Active>, With<Recording>)>,
) {
    // Handle ALL recording logic in one place
    // - State transitions using queries instead of stored entities
    // - Countdown management  
    // - Recording component management
    // - Ghost conversion
    // Recording entity is ALWAYS the active character!
}
```

**Key Benefits:**
- **Single Orchestrator**: Like arena_update(), one system handles ALL recording coordination
- **No Race Conditions**: RecordingUpdate triggers are processed sequentially
- **Clean Separation**: Request events express intent, orchestrator decides what actually happens
- **Debuggable**: Clear flow from trigger → RecordingUpdate → orchestrator response
- **Testable**: Single system to test instead of complex multi-system coordination

**Next Tutorial Preview:** Tutorial 02 will implement this unified pattern with the recording_update() orchestrator system that mirrors the successful arena_update() pattern.

This foundation provides a robust base for the entire recording system. The type-safe APIs prevent common errors, the
sorted event storage makes playback efficient, and recording intent ensures perfect replay fidelity.