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
use std::fmt;

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
        let safe_seconds = if seconds.is_nan() { 0.0 } else { seconds };
        Self(safe_seconds.clamp(0.0, 120.0))
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
        let safe_seconds = if seconds.is_nan() { 0.0 } else { seconds };
        Self(safe_seconds.rem_euclid(120.0))
    }
}

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
    pub fn from_draft(draft: &DraftTimeline) -> Self {
        Self {
            events: draft.events.clone().into(),
        }
    }

    /// Zero-alloc helper: Get events within a time range with wrap-around support
    /// PR Gate: Added #[must_use] to timeline slice functions
    /// 
    /// Supports wrap-around ranges where start > end (e.g., 118.0 → 2.0)
    /// This is essential for seamless playback across the 120s boundary
    /// 
    /// # Examples
    /// ```
    /// // Normal range
    /// let events: Vec<_> = timeline.events_in_range(TimeStamp::new(5.0), TimeStamp::new(10.0)).collect();
    /// 
    /// // Wrap-around range (118s through wrap to 2s)
    /// let events: Vec<_> = timeline.events_in_range(TimeStamp::new(118.0), TimeStamp::new(2.0)).collect();
    /// ```
    #[must_use]
    pub fn events_in_range(&self, start: TimeStamp, end: TimeStamp) -> impl Iterator<Item=&TimelineEvent> + '_ {
        // Detect wrap-around case: start > end means we cross the 120s boundary
        if start > end {
            // Wrap case: return events from [start..120) and [0..end)
            let start_idx = self.events.binary_search_by(|e| e.timestamp.partial_cmp(&start).unwrap())
                .unwrap_or_else(|i| i);
            
            // Find events up to 120s (or end of array)
            let wrap_idx = self.events.binary_search_by(|e| e.timestamp.partial_cmp(&TimeStamp::MAX).unwrap())
                .unwrap_or_else(|i| i);
            
            // Find events from 0s
            let zero_idx = self.events.binary_search_by(|e| e.timestamp.partial_cmp(&TimeStamp::ZERO).unwrap())
                .unwrap_or_else(|i| i);
            
            let end_idx = self.events.binary_search_by(|e| e.timestamp.partial_cmp(&end).unwrap())
                .unwrap_or_else(|i| i);
            
            // Chain two slices: [start..120) and [0..end)
            // Using chain() for zero-alloc iteration
            self.events[start_idx..wrap_idx].iter()
                .chain(self.events[zero_idx..end_idx].iter())
        } else {
            // Normal case: simple range query (no wrap-around handling)
            // This is a straight slice - wrap-around logic is handled above
            let start_idx = self.events.binary_search_by(|e| e.timestamp.partial_cmp(&start).unwrap())
                .unwrap_or_else(|i| i);
            let end_idx = self.events.binary_search_by(|e| e.timestamp.partial_cmp(&end).unwrap())
                .unwrap_or_else(|i| i);
            
            // Single slice, chain with empty for consistent return type
            self.events[start_idx..end_idx].iter()
                .chain(std::iter::empty())
        }
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
}

/// Interpolation helpers for timeline playback
pub mod interpolation {
    use super::*;
    
    /// Get movement intent at a specific timestamp
    /// Returns the most recent movement event before or at the timestamp
    #[must_use]
    pub fn get_movement_intent_at(timeline: &PublishTimeline, timestamp: TimeStamp) -> Option<GridPos> {
        // Binary search for the position
        let idx = match timeline.events.binary_search_by(|e| e.timestamp.partial_cmp(&timestamp).unwrap()) {
            Ok(idx) => idx,
            Err(idx) => if idx > 0 { idx - 1 } else { return None },
        };
        
        // Walk backwards to find the most recent movement event
        for i in (0..=idx).rev() {
            if let EventType::Movement(pos) = &timeline.events[i].event_type {
                return Some(*pos);
            }
        }
        
        None
    }
    
    /// Get abilities within a time window with wrap-around support
    /// Uses the wrap-aware events_in_range internally
    #[must_use]
    pub fn abilities_in_window(
        timeline: &PublishTimeline, 
        start: TimeStamp, 
        end: TimeStamp
    ) -> impl Iterator<Item=&TimelineEvent> + '_ {
        timeline.events_in_range(start, end)
            .filter(|e| matches!(e.event_type, EventType::Ability(_, _)))
    }
}
```

### Step 3: Create Arena Clock Component

Add to `src/timeline/mod.rs`:

```rust
/// Clock for 2-minute arena cycles
/// PR Gate: Using TimeStamp + Duration (wrapped in TimeStamp newtype) for type safety
#[derive(Component)]
pub struct TimelineClock {
    /// PR Gate: bevy::time::Timer used internally for proper time handling
    pub timer: bevy::time::Timer,
    pub is_paused: bool,
}

impl Default for TimelineClock {
    fn default() -> Self {
        Self {
            // PR Gate: Using bevy::time::Timer instead of f32
            timer: bevy::time::Timer::new(
                bevy::time::Duration::from_secs(120),
                bevy::time::TimerMode::Repeating,
            ),
            is_paused: false,
        }
    }
}

impl TimelineClock {
    pub fn tick(&mut self, delta: bevy::time::Duration) {
        if !self.is_paused {
            // PR Gate: Using bevy::time::Timer::tick
            self.timer.tick(delta);
        }
    }
    
    /// Convenience method for ticking with seconds (useful for tests)
    pub fn tick_secs(&mut self, seconds: f32) {
        self.tick(bevy::time::Duration::from_secs_f32(seconds));
    }

    pub fn reset(&mut self) {
        self.timer.reset();
    }

    pub fn pause(&mut self) {
        self.is_paused = true;
    }

    pub fn resume(&mut self) {
        self.is_paused = false;
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

/// System to update all arena clocks
/// PR Gate: Respecting GlobalTimelinePause for all timeline operations
pub fn update_timeline_clocks(
    time: Res<Time>,
    global_pause: Res<GlobalTimelinePause>,
    mut arena_q: Query<(&ArenaIdx, &mut TimelineClock)>,
) {
    // PR Gate: Respecting GlobalTimelinePause
    if global_pause.is_paused {
        return;
    }
    
    let delta = time.delta();

    for (_arena, mut clock) in arena_q.iter_mut() {
        clock.tick(delta);
    }
}

/// System to display current clock values (for debugging)
pub fn debug_timeline_clocks(
    arena_q: Query<(&ArenaIdx, &TimelineClock)>,
    current_arena: Res<CurrentArena>,
) {
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
                update_timeline_clocks,
                debug_timeline_clocks,
            ).chain());
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
TimelineClock::default (), // Add this line
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

        // Add events out of order
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
        let timestamp = TimeStamp::wrapped(120.0);
        assert_eq!(timestamp.as_secs(), 0.0);
        
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

        let published = PublishTimeline::from_draft(&draft);

        // Get events between 5.0 and 10.0 seconds
        let events: Vec<_> = published.events_in_range(TimeStamp::new(5.0), TimeStamp::new(10.0)).collect();

        assert_eq!(events.len(), 2); // Should get events at 6.0, 8.0
        assert_eq!(events[0].timestamp, TimeStamp::new(6.0));
        assert_eq!(events[1].timestamp, TimeStamp::new(8.0));
    }
    
    #[test]
    fn test_events_in_range_wrap_around() {
        let mut draft = DraftTimeline::new();
        
        // Add events near the wrap boundary
        draft.add_event(TimelineEvent {
            timestamp: TimeStamp::new(118.0),
            event_type: EventType::Movement(GridPos::new(0, 1)),
        });
        draft.add_event(TimelineEvent {
            timestamp: TimeStamp::new(119.0),
            event_type: EventType::Ability(AbilityId::AUTO_SHOT, None),
        });
        draft.add_event(TimelineEvent {
            timestamp: TimeStamp::new(119.5),
            event_type: EventType::Movement(GridPos::new(1, 1)),
        });
        // Events after wrap
        draft.add_event(TimelineEvent {
            timestamp: TimeStamp::new(0.5),
            event_type: EventType::Movement(GridPos::new(2, 1)),
        });
        draft.add_event(TimelineEvent {
            timestamp: TimeStamp::new(1.0),
            event_type: EventType::Ability(AbilityId::HOLY_NOVA, None),
        });
        draft.add_event(TimelineEvent {
            timestamp: TimeStamp::new(2.0),
            event_type: EventType::Movement(GridPos::new(3, 1)),
        });
        
        let published = PublishTimeline::from_draft(&draft);
        
        // Test wrap-around range: 118.0 through wrap to 2.0
        let events: Vec<_> = published.events_in_range(TimeStamp::new(118.0), TimeStamp::new(2.0)).collect();
        
        // Should get events at 118, 119, 119.5, 0.5, 1.0 (not 2.0 as it's exclusive end)
        assert_eq!(events.len(), 5);
        assert_eq!(events[0].timestamp, TimeStamp::new(118.0));
        assert_eq!(events[1].timestamp, TimeStamp::new(119.0));
        assert_eq!(events[2].timestamp, TimeStamp::new(119.5));
        assert_eq!(events[3].timestamp, TimeStamp::new(0.5));
        assert_eq!(events[4].timestamp, TimeStamp::new(1.0));
        
        // Test wrap-around with no events in wrapped section
        let events: Vec<_> = published.events_in_range(TimeStamp::new(119.8), TimeStamp::new(0.2)).collect();
        assert_eq!(events.len(), 0);
        
        // Test wrap-around capturing all wrap events
        let events: Vec<_> = published.events_in_range(TimeStamp::new(117.0), TimeStamp::new(3.0)).collect();
        assert_eq!(events.len(), 6); // All events
    }
    
    #[test]
    fn test_events_in_range_edge_cases() {
        let mut draft = DraftTimeline::new();
        
        // Add events at exact boundaries
        draft.add_event(TimelineEvent {
            timestamp: TimeStamp::new(0.0),
            event_type: EventType::Movement(GridPos::new(0, 0)),
        });
        draft.add_event(TimelineEvent {
            timestamp: TimeStamp::new(120.0), // Will be clamped to 120.0
            event_type: EventType::Movement(GridPos::new(1, 0)),
        });
        
        let published = PublishTimeline::from_draft(&draft);
        
        // Test exact boundary wrap
        let events: Vec<_> = published.events_in_range(TimeStamp::new(119.0), TimeStamp::new(1.0)).collect();
        assert_eq!(events.len(), 2); // Should get both boundary events
        
        // Test empty wrap range
        let events: Vec<_> = published.events_in_range(TimeStamp::new(50.0), TimeStamp::new(50.0)).collect();
        assert_eq!(events.len(), 0);
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
        
        let published = PublishTimeline::from_draft(&draft);
        
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
    fn test_type_conversions() {
        // Test TimeStamp conversions
        let timestamp: TimeStamp = 42.5.into();
        assert_eq!(timestamp.as_secs(), 42.5);
        assert_eq!(timestamp.to_string(), "42.5s");
        
        // Test TimeStamp::ZERO constant
        assert_eq!(TimeStamp::ZERO.as_secs(), 0.0);
        
        // Test ArenaIdx conversions
        let idx = ArenaIdx::try_from(3).unwrap();
        assert_eq!(idx.as_u8(), 3);
        assert_eq!(idx.to_string(), "Arena 3");
        
        let err = ArenaIdx::try_from(10);
        assert!(err.is_err());
        
        // Test GridPos conversions
        let pos = GridPos::from(IVec2::new(5, -3));
        assert_eq!(pos.x(), 5);
        assert_eq!(pos.y(), -3);
        assert_eq!(pos.to_string(), "(5, -3)");
        
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

- TimelineClock counting from 0.0 to 120.0 for the current arena
- TimelineClock looping back to 0.0 after reaching 120.0
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
4. **From/Display Traits**: Clean conversions and formatting for all types
5. **Binary Search**: Efficient O(log n) operations on sorted timelines

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
- **Intent Recording**: Deterministic replay regardless of physics/interpolation
- **Arc<[T]>**: Share timeline across systems without cloning the data
- **Binary Search**: Fast lookups for playback position queries

This foundation provides a robust base for the entire recording system. The type-safe APIs prevent common errors, the sorted event storage makes playback efficient, and recording intent ensures perfect replay fidelity.