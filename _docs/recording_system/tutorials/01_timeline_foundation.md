# Tutorial 01: Timeline Foundation

## Objective

Build the core timeline data structures and components that will store recorded character actions. This foundation will
support all future recording and playback functionality.

## Prerequisites

- Basic understanding of Bevy ECS (Entity Component System)
- Familiarity with the existing arena and character systems
- Rust knowledge of structs, enums, and vectors

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
use bevy::prelude::*;
use bevy::ecs::change_detection::DetectChanges;
use bevy::log::trace;
use bevy::time::Virtual;
use std::fmt;
use std::convert::identity;

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

// From trait kept only for Bevy interop - use TimeStamp::new() in examples
impl From<f32> for TimeStamp {
    fn from(seconds: f32) -> Self {
        Self::new(seconds)
    }
}

impl fmt::Display for TimeStamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}s", self.0)
    }
}

/// Types of events that can be recorded
#[derive(Clone, Debug)]
pub enum EventType {
    /// Movement intent from input - not transform!
    Movement(GridPos),
    /// Ability cast with optional target
    Ability(AbilityId, Option<Target>),
    /// Character death event
    Death,
}

/// Target for abilities
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Target {
    Entity(Entity),
    Position(GridPos),
}

/// Identifier for character abilities
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AbilityId(pub u8);

impl AbilityId {
    pub const AUTO_SHOT: Self = Self(1);
    pub const HOLY_NOVA: Self = Self(2);
    pub const POISON_SHOT: Self = Self(3);
    pub const HEAL: Self = Self(4);
}

impl fmt::Display for AbilityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::AUTO_SHOT => "AutoShot",
            Self::HOLY_NOVA => "HolyNova",
            Self::POISON_SHOT => "PoisonShot",
            Self::HEAL => "Heal",
            _ => "Unknown",
        };
        write!(f, "{}", name)
    }
}

/// Newtype for arena indices (0-8)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Component)]
pub struct ArenaIdx(pub u8);

impl ArenaIdx {
    /// Creates new ArenaIdx if value is valid (0-8)
    #[must_use]
    pub fn new(idx: u8) -> Option<Self> {
        (idx < 9).then(|| Self(idx))
    }
    
    #[must_use]
    pub fn as_u8(&self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for ArenaIdx {
    type Error = &'static str;
    
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value).ok_or("Arena index must be 0-8")
    }
}

impl fmt::Display for ArenaIdx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Arena {}", self.0)
    }
}

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

impl fmt::Display for GridPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

    pub fn add_event(&mut self, event: TimelineEvent) {
        // APPROVED: Binary search maintains O(log n) sorted insertion
        // This is the right balance of performance and simplicity for tutorials
        match self.events.binary_search_by(|e| {
            e.timestamp.partial_cmp(&event.timestamp).unwrap()
        }) {
            Ok(pos) | Err(pos) => self.events.insert(pos, event),
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
        // Using identity instead of |i| i for clearer intent - no transformation needed
        // identity is a standard library function that simply returns its input unchanged,
        // making the code more self-documenting than a trivial closure
        let start_idx = self.events.binary_search_by(|e| e.timestamp.partial_cmp(&start).unwrap())
            .unwrap_or_else(identity);
        let end_idx = self.events.binary_search_by(|e| e.timestamp.partial_cmp(&end).unwrap())
            .unwrap_or_else(identity);
        
        self.events[start_idx..end_idx].iter()
    }

    /// Zero-alloc helper: Find next event after timestamp
    #[must_use]
    pub fn next_event_after(&self, timestamp: TimeStamp) -> Option<&TimelineEvent> {
        match self.events.binary_search_by(|e| e.timestamp.partial_cmp(&timestamp).unwrap()) {
            Ok(idx) => self.events.get(idx + 1),
            Err(idx) => self.events.get(idx),
        }
    }
    
    /// Get a slice of the timeline events
    #[must_use]
    pub fn slice(&self, start: usize, end: usize) -> &[TimelineEvent] {
        &self.events[start.min(self.events.len())..end.min(self.events.len())]
    }

    /// Get movement intent at a specific timestamp using partition_point for optimal boundary finding
    /// Returns the most recent movement event before or at the timestamp
    /// 
    /// # Why partition_point is superior to binary_search_by:
    /// - **Clearer Intent**: partition_point directly finds the boundary where predicate changes
    /// - **Simpler Logic**: No Ok/Err pattern matching needed
    /// - **More Idiomatic**: Specifically designed for finding boundaries in sorted sequences
    /// - **Same Performance**: O(log n) complexity but cleaner implementation
    #[must_use]
    pub fn get_movement_intent_at(&self, timestamp: TimeStamp) -> Option<GridPos> {
        // partition_point finds first index where timestamp > target, so work backwards from there
        // This directly gives us the boundary we want without complex Ok/Err handling
        let mut i = self.events.partition_point(|e| e.timestamp <= timestamp);
        while i > 0 {
            i -= 1; // Move to last index with ts ≤ timestamp
            if let EventType::Movement(pos) = self.events[i].event_type {
                return Some(pos);
            }
        }
        None
    }
    
    /// Get abilities within a time window
    /// Returns an iterator over events that contain abilities within the specified range
    #[must_use]
    pub fn abilities_in_window(
        &self,
        start: TimeStamp, 
        end: TimeStamp
    ) -> impl Iterator<Item=&TimelineEvent> + '_ {
        self.events_in_range(start, end)
            .filter(|e| matches!(e.event_type, EventType::Ability(_, _)))
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

impl Default for TimelineClock {
    fn default() -> Self {
        Self {
            // PR Gate: Using bevy::time::Timer instead of f32
            timer: bevy::time::Timer::new(
                std::time::Duration::from_secs(120),
                bevy::time::TimerMode::Repeating,
            ),
            is_paused: false,
        }
    }
}

impl TimelineClock {
    pub fn tick(&mut self, delta: std::time::Duration) {
        if !self.is_paused {
            self.timer.tick(delta);
        }
    }
    
    /// Convenience method for ticking with seconds (useful for tests)
    pub fn tick_secs(&mut self, seconds: f32) {
        self.tick(std::time::Duration::from_secs_f32(seconds));
    }

    pub fn reset(&mut self) {
        self.timer.reset();
    }
    
    pub fn elapsed_secs(&self) -> f32 {
        self.timer.elapsed_secs()
    }
    
    pub fn current(&self) -> TimeStamp {
        TimeStamp::wrapped(self.timer.elapsed_secs())
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
    mut arena_q: Query<(&ArenaIdx, &mut TimelineClock)>,
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
pub fn debug_timeline_clocks(
    arena_q: Query<(&ArenaIdx, &TimelineClock)>,
    current_arena_q: Query<&CurrentArena>,  // CurrentArena is a Component, not Resource
) {
    // Get the current arena entity
    let Ok(current_arena) = current_arena_q.single() else {
        return;
    };
    
    // Use let-else for early return pattern - more idiomatic Rust
    let Some((arena, clock)) = arena_q
        .iter()
        .find(|(arena, _)| arena.as_u8() == current_arena.0)
    else {
        return;
    };
    
    // PR Gate: Using trace! for per-frame logs instead of info!
    if (clock.elapsed_secs() % 1.0) < 0.02 {
        trace!("{}: {:.1}s", arena, clock.elapsed_secs());
    }
}
```

### Step 5: Create the Timeline Plugin

Add to `src/timeline/mod.rs`:

```rust
use crate::arena::CurrentArena;

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
ArenaIdx::new(arena_index).unwrap(),
TimelineClock::default(), // Add this line
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
        });

        timeline.add_event(TimelineEvent {
            timestamp: TimeStamp::new(2.0),
            event_type: EventType::Ability(AbilityId::AUTO_SHOT, None),
        });

        timeline.add_event(TimelineEvent {
            timestamp: TimeStamp::new(10.0),
            event_type: EventType::Death,
        });

        // Verify events are sorted by timestamp
        assert_eq!(timeline.events.len(), 3);
        assert_eq!(timeline.events[0].timestamp, TimeStamp::new(2.0));
        assert_eq!(timeline.events[1].timestamp, TimeStamp::new(5.0));
        assert_eq!(timeline.events[2].timestamp, TimeStamp::new(10.0));
    }

    #[test]
    fn test_timeline_clock_loops_at_120_seconds() {
        let mut clock = TimelineClock::default();

        // Tick past 120 seconds
        clock.tick_secs(125.0);

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
        let mut clock = TimelineClock::default();

        clock.tick_secs(10.0);
        assert_eq!(clock.current().as_secs(), 10.0);

        clock.pause();
        clock.tick_secs(10.0); // Should not advance while paused
        assert_eq!(clock.current().as_secs(), 10.0);

        clock.resume();
        clock.tick_secs(10.0); // Should advance again
        assert_eq!(clock.current().as_secs(), 20.0);
    }

    #[test]
    fn test_publish_timeline_get_events_in_range() {
        let mut draft = DraftTimeline::new();

        for i in 0..10 {
            draft.add_event(TimelineEvent {
                timestamp: TimeStamp::new(i as f32 * 2.0),
                event_type: EventType::Movement(GridPos::new(i as i32, 0)),
            });
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
        });
        draft.add_event(TimelineEvent {
            timestamp: TimeStamp::new(20.0),
            event_type: EventType::Ability(AbilityId::AUTO_SHOT, None),
        });
        draft.add_event(TimelineEvent {
            timestamp: TimeStamp::new(30.0),
            event_type: EventType::Movement(GridPos::new(1, 0)),
        });
        
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
        
        // Test ArenaIdx::new() as primary constructor
        let idx = ArenaIdx::new(3).unwrap();
        assert_eq!(idx.as_u8(), 3);
        assert_eq!(idx.to_string(), "Arena 3");
        
        let err = ArenaIdx::new(10);
        assert!(err.is_none());
        
        // Test GridPos::new() as primary constructor
        let pos = GridPos::new(5, -3);
        assert_eq!(pos.x(), 5);
        assert_eq!(pos.y(), -3);
        assert_eq!(pos.to_string(), "(5, -3)");
        
        // From traits still work for Bevy interop
        let vec: IVec2 = pos.into();
        assert_eq!(vec, IVec2::new(5, -3));
    }
}
```

## Verification

Run the tests to verify implementation:

```bash
cargo test timeline
```

Run the game and observe timer logs:

```bash
cargo run
```

You should see:

- TimelineClock counting from TimeStamp::ZERO to TimeStamp::MAX for the current arena  
- TimelineClock looping back to TimeStamp::ZERO after reaching TimeStamp::MAX
- No crashes or panics

## Next Steps

With the timeline foundation in place, we can now:

- Tutorial 02: Build the recording state machine with event-driven transitions
- Tutorial 03: Implement movement capture (recording intent, not transforms!)
- Tutorial 04: Create the playback system with strict ordering

## Key Takeaways

1. **Type-Safe Newtypes**: TimeStamp, ArenaIdx, GridPos provide compile-time safety
2. **Intent Not Transform**: Recording Movement(GridPos) not Transform(Vec3)
3. **Zero-Alloc Helpers**: events_in_range(), next_event_after(), slice() avoid allocations
4. **Explicit Constructors**: TimeStamp::new(), GridPos::new(), ArenaIdx::new() as primary API
5. **Binary Search**: Efficient O(log n) operations on sorted timelines
6. **Zero-Copy Ownership Transfer**: PublishTimeline::from_draft(draft) consumes for efficient Vec→Arc conversion
7. **Idiomatic Helpers**: Use `std::convert::identity` over trivial closures for clearer intent
8. **partition_point over binary_search_by**: More idiomatic boundary finding with clearer intent and simpler logic
9. **Virtual Time for Pause Safety**: Time<Virtual> prevents time jumps when pausing/unpausing

## Production Notes

### What We Got Right:

- Binary search insertion is perfect for small-to-medium event counts
- Immutable PublishTimeline with Arc<[T]> prevents mutations and enables cheap cloning
- Type-safe newtypes with Display/From implementations for clean APIs
- Recording intent (Movement/Ability) not results (Transform)

### What We Intentionally Simplified:

- No event compression (covered in Tutorial 08)
- No custom memory pools (unnecessary complexity)
- No event sourcing patterns (wrong tool for the job)

### Why These Patterns Matter:

- **Newtypes**: Catch unit errors at compile time (can't mix TimeStamp with f32)
- **Explicit Constructors**: TimeStamp::new() makes the common case obvious and discoverable
- **Intent Recording**: Deterministic replay regardless of physics/interpolation
- **Arc<[T]>**: Share timeline across systems without cloning the data
- **Binary Search**: Fast lookups for playback position queries
- **Zero-Copy Ownership Transfer**: When data flows one-way (draft→publish), consume instead of borrow to enable efficient transformations
- **partition_point Preference**: Choose partition_point over binary_search_by when finding boundaries - it's designed exactly for this pattern and eliminates Ok/Err handling complexity

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

This transformation avoids cloning potentially thousands of events when transitioning from draft to published state. The key insight is that draft timelines naturally flow one-way into published timelines, so consuming ownership enables efficient Vec<T> → Arc<[T]> conversion.

This foundation provides a robust base for the entire recording system. The type-safe APIs prevent common errors, the sorted event storage makes playback efficient, and recording intent ensures perfect replay fidelity.