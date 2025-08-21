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

// RULE 3 COMPLIANCE: Events for timeline communication
/// Event to notify systems when timeline reaches major checkpoints
#[derive(Event)]
pub struct TimelineCheckpoint {
    pub arena: ArenaName,
    pub timestamp: TimeStamp,
    pub checkpoint_type: CheckpointType,
}

#[derive(Debug, Clone, Copy)]
pub enum CheckpointType {
    QuarterTime,  // 30 seconds
    HalfTime,     // 60 seconds
    ThreeQuarter, // 90 seconds
    FullCycle,    // 120 seconds (reset)
}

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
    /// RULE 2 COMPLIANCE: Static data lookup with const values
    pub const ZERO: Self = Self(0.0);
    pub const MAX: Self = Self(120.0);
    
    /// Common timeline positions as static lookups
    pub const QUARTER: Self = Self(30.0);
    pub const HALF: Self = Self(60.0);
    pub const THREE_QUARTER: Self = Self(90.0);

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

// DOMAIN: VALUE TYPES - Timeline events use value types only
/// Types of events that can be recorded - ALL use value types
#[derive(Clone, Debug)]
pub enum EventType {
    /// Movement intent from input - uses VALUE type GridPosData
    Movement(GridPosData),  // ✅ CORRECT: Value type in event
    /// Ability cast with optional target
    Ability(AbilityType, Option<TargetData>),  // ✅ CORRECT: Value types
    /// Character death event
    Death,
}

// DOMAIN: VALUE TYPES - Target data for events
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TargetData {
    Entity(Entity),           // ✅ CORRECT: Entity reference in value context
    Position(GridPosData),    // ✅ CORRECT: Value type
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
use crate::arena::{Arena, ArenaName, ArenaEntities, CurrentArenaEntity};

// === TYPE DOMAIN SEPARATION - RULE 1 COMPLIANCE ===

// DOMAIN: VALUE TYPES (for events, function parameters, data passing)
/// Grid position data for event payloads and function parameters
/// NEVER used as a Component - this is pure value type
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GridPosData {
    pub x: i32,
    pub y: i32,
}

impl GridPosData {
    #[must_use]
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

// DOMAIN: COMPONENT TYPES (for entity state only)
/// Component for entity's grid position - NEVER used in events
/// Only attached to entities, never in event payloads
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct GridPositionComponent {
    pub x: i32,
    pub y: i32,
}

impl GridPositionComponent {
    #[must_use]
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    
    /// Convert to value type for event emission
    pub fn to_data(&self) -> GridPosData {
        GridPosData::new(self.x, self.y)
    }
}

// === CONVERSION UTILITIES ===
impl From<GridPositionComponent> for GridPosData {
    fn from(component: GridPositionComponent) -> Self {
        component.to_data()
    }
}

impl From<GridPosData> for GridPositionComponent {
    fn from(data: GridPosData) -> Self {
        Self::new(data.x, data.y)
    }
}

impl Display for GridPosData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Display for GridPositionComponent {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```

### Step 2: Add Timeline Storage Components

Add to `src/timeline/mod.rs`:

```rust
/// RULE 5 COMPLIANCE: Marker component for entity categorization
/// Unit struct without data - perfect for simple entity filtering in queries
#[derive(Component)]
pub struct Recordable;

/// RULE 5 COMPLIANCE: Additional timeline markers for fine-grained filtering
#[derive(Component)]
pub struct TimelineReady; // Entity has valid timeline data

#[derive(Component)]
pub struct TimelineActive; // Entity's timeline is currently ticking

#[derive(Component)]
pub struct TimelineComplete; // Entity reached full 120 second cycle

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

    /// Add event to timeline with comprehensive error handling
    /// RULE 15 COMPLIANCE: Never ignore Result, always handle errors with context
    pub fn add_event(&mut self, event: TimelineEvent) -> TimelineResult<()> {
        // FIXED: Safe comparison without unwrap()
        let pos = self.events.binary_search_by(|e| {
            e.timestamp.partial_cmp(&event.timestamp)
                .ok_or_else(|| TimelineError::InvalidComparison)
                .unwrap_or(Ordering::Equal)  // Safe fallback for NaN cases
        });
        
        match pos {
            Ok(pos) | Err(pos) => {
                if pos <= self.events.len() {
                    self.events.insert(pos, event);
                    Ok(())
                } else {
                    Err(TimelineError::OperationFailed {
                        message: format!("Insert position {} exceeds timeline length {}", 
                                      pos, self.events.len())
                    })
                }
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
    pub timelines: HashMap<ArenaName, PublishTimeline>,
}

impl CharacterTimelines {
    pub fn new() -> Self {
        Self {
            timelines: HashMap::new(),
        }
    }

    pub fn store_timeline(&mut self, arena: ArenaName, timeline: PublishTimeline) {
        self.timelines.insert(arena, timeline);
    }

    pub fn get_timeline(&self, arena: ArenaName) -> Option<&PublishTimeline> {
        self.timelines.get(&arena)
    }

    pub fn has_recording_for(&self, arena: ArenaName) -> bool {
        self.timelines.contains_key(&arena)
    }

    pub fn arena_count(&self) -> usize {
        self.timelines.len()
    }

    pub fn recorded_arenas(&self) -> impl Iterator<Item = ArenaName> + '_ {
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
    /// RULE 15 COMPLIANCE: Comprehensive error handling with context
    /// 
    /// Returns events where start <= timestamp < end
    /// NOTE: Wrap-around handling (e.g., 118.0 → 2.0) is covered in Tutorial 04
    ///
    /// # Examples
    /// ```
    /// // Normal range
    /// let events: Vec<_> = timeline.events_in_range(TimeStamp::new(5.0), TimeStamp::new(10.0)).unwrap().collect();
    /// ```
    #[must_use]
    pub fn events_in_range(&self, start: TimeStamp, end: TimeStamp) -> TimelineResult<impl Iterator<Item=&TimelineEvent> + '_> {
        // Validate input parameters
        if start.as_secs() < 0.0 || end.as_secs() < 0.0 {
            return Err(TimelineError::OperationFailed {
                message: format!("Invalid range: start={:.1}s, end={:.1}s", start.as_secs(), end.as_secs())
            });
        }

        // FIXED: Safe binary search without unwrap()  
        let start_idx = self.events.binary_search_by(|e| {
            e.timestamp.partial_cmp(&start)
                .ok_or_else(|| TimelineError::InvalidComparison)
                .unwrap_or(Ordering::Equal)
        }).unwrap_or_else(|idx| idx);  // Convert Err(idx) to idx safely
        
        let end_idx = self.events.binary_search_by(|e| {
            e.timestamp.partial_cmp(&end)
                .ok_or_else(|| TimelineError::InvalidComparison)
                .unwrap_or(Ordering::Equal)
        }).unwrap_or_else(|idx| idx);  // Convert Err(idx) to idx safely

        // Bounds checking
        if start_idx > self.events.len() || end_idx > self.events.len() {
            return Err(TimelineError::OperationFailed {
                message: format!("Binary search failed on timeline with {} events", self.events.len())
            });
        }

        Ok(self.events[start_idx..end_idx].iter())
    }

    // Consolidated API: Use next_event_after/prev_event_before with iterator methods for specific queries
    // Example: timeline.events_in_range(start, end).filter(|e| matches!(e.event_type, EventType::Ability(_, _)))

    /// Safe event lookup with error context
    /// RULE 15 COMPLIANCE: All Result types handled with context
    #[must_use]
    pub fn next_event_after(&self, timestamp: TimeStamp) -> TimelineResult<Option<&TimelineEvent>> {
        if timestamp.as_secs() < 0.0 {
            return Err(TimelineError::OperationFailed {
                message: format!("Invalid timestamp: {:.1}s", timestamp.as_secs())
            });
        }

        // FIXED: Safe binary search with error handling
        let search_result = self.events.binary_search_by(|e| {
            e.timestamp.partial_cmp(&timestamp)
                .ok_or_else(|| TimelineError::InvalidComparison)
                .unwrap_or(Ordering::Equal)
        });

        let idx = match search_result {
            Ok(exact_idx) => exact_idx + 1,  // Next event after exact match
            Err(insert_idx) => insert_idx,   // First event after timestamp
        };

        Ok(self.events.get(idx))
    }

    /// Get a slice of the timeline events
    #[must_use]
    pub fn slice(&self, start: usize, end: usize) -> &[TimelineEvent] {
        &self.events[start.min(self.events.len())..end.min(self.events.len())]
    }

    /// Safe previous event lookup
    /// RULE 15 COMPLIANCE: Error-safe implementation
    #[must_use]
    pub fn prev_event_before(&self, timestamp: TimeStamp) -> TimelineResult<Option<&TimelineEvent>> {
        if timestamp.as_secs() < 0.0 {
            return Err(TimelineError::OperationFailed {
                message: format!("Invalid timestamp: {:.1}s", timestamp.as_secs())
            });
        }

        // FIXED: Safe binary search
        let search_result = self.events.binary_search_by(|e| {
            e.timestamp.partial_cmp(&timestamp)
                .ok_or_else(|| TimelineError::InvalidComparison)
                .unwrap_or(Ordering::Equal)
        });

        let idx = match search_result {
            Ok(exact_idx) => Some(exact_idx),           // Exact match found
            Err(insert_idx) => insert_idx.checked_sub(1), // Previous element if exists
        };

        Ok(idx.and_then(|i| self.events.get(i)))
    }
}
```

### Step 3: Create Arena Clock Component

Add to `src/timeline/mod.rs`:

```rust
/// Clock for 2-minute arena cycles
/// RULE 1 COMPLIANCE: TimelineClock is a Component, not Resource
/// Each arena entity has its own clock for independent timing
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
/// RULE 1 COMPLIANCE: GlobalTimelinePause is appropriately a Resource
/// This affects ALL timeline clocks globally - true singleton behavior
/// Individual arena clocks are Components, global pause is Resource
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
/// 🆕 Uses CurrentArenaEntity SystemParam - eliminates O(n) linear search!
pub fn debug_timeline_clocks(
    arena_q: Query<(&Arena, &TimelineClock)>,
    current: CurrentArenaEntity,
) {
    // CurrentArenaEntity provides O(1) arena entity lookup
    let Ok((arena, clock)) = arena_q.get(current.get()) else {
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
use crate::arena::{CurrentArena, ArenaEntities, CurrentArenaEntity};

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
Arena(ArenaName::from_index_safe(arena_index)), // ArenaName provides safe constructor
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

        // Add events out of order - using explicit constructors with value types
        timeline.add_event(TimelineEvent {
            timestamp: TimeStamp::new(5.0),
            event_type: EventType::Movement(GridPosData::new(1, 0)),
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
                event_type: EventType::Movement(GridPosData::new(i as i32, 0)),
            }).expect("Failed to add event");
        }

        let published = PublishTimeline::from_draft(draft);

        // Get events between 5.0 and 10.0 seconds
        let events: Vec<_> = published.events_in_range(TimeStamp::new(5.0), TimeStamp::new(10.0)).unwrap().collect();

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
            event_type: EventType::Movement(GridPosData::new(0, 0)),
        }).expect("Failed to add event");
        draft.add_event(TimelineEvent {
            timestamp: TimeStamp::new(20.0),
            event_type: EventType::Ability(AbilityType::AutoShot, None),
        }).expect("Failed to add event");
        draft.add_event(TimelineEvent {
            timestamp: TimeStamp::new(30.0),
            event_type: EventType::Movement(GridPosData::new(1, 0)),
        }).expect("Failed to add event");

        let published = PublishTimeline::from_draft(draft);

        // Test: Find next event after a timestamp with no exact match
        let next = published.next_event_after(TimeStamp::new(15.0)).unwrap();
        assert!(next.is_some());
        assert_eq!(next.unwrap().timestamp, TimeStamp::new(20.0));

        // Test: Find next event when timestamp matches exactly
        let next = published.next_event_after(TimeStamp::new(20.0)).unwrap();
        assert!(next.is_some());
        assert_eq!(next.unwrap().timestamp, TimeStamp::new(30.0));

        // Test: No next event when at or past last event
        let next = published.next_event_after(TimeStamp::new(30.0)).unwrap();
        assert!(next.is_none());

        let next = published.next_event_after(TimeStamp::new(35.0)).unwrap();
        assert!(next.is_none());

        // Test: Find first event when timestamp is before all events
        let next = published.next_event_after(TimeStamp::new(5.0)).unwrap();
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

        // Test ArenaName safe constructor for internal use
        let safe_arena_name = ArenaName::from_index_safe(15);
        assert_eq!(safe_arena_name.as_u8(), 8); // Should clamp to max valid arena (Gala)

        // Test Arena component with ArenaName enum
        let arena = Arena(ArenaName::Bastion);
        assert_eq!(arena.0.as_u8(), 4);
        assert_eq!(arena.0, ArenaName::Bastion);

        // Test GridPosData::new() as primary constructor for value types
        let pos_data = GridPosData::new(5, -3);
        assert_eq!(pos_data.x, 5);
        assert_eq!(pos_data.y, -3);
        assert_eq!(pos_data.to_string(), "(5, -3)");

        // Test GridPositionComponent::new() for entity components
        let pos_component = GridPositionComponent::new(5, -3);
        assert_eq!(pos_component.x, 5);
        assert_eq!(pos_component.y, -3);
        assert_eq!(pos_component.to_string(), "(5, -3)");
        
        // Test conversion between domains
        let converted_data: GridPosData = pos_component.into();
        assert_eq!(converted_data.x, 5);
        assert_eq!(converted_data.y, -3);
    }

    #[test]
    fn test_character_timelines_multi_arena_storage() {
        // Test the critical architectural fix: CharacterTimelines stores multiple timelines per character
        let mut character_timelines = CharacterTimelines::new();
        
        // Create test timelines for different arenas
        let mut draft_labyrinth = DraftTimeline::new();
        draft_labyrinth.add_event(TimelineEvent {
            timestamp: TimeStamp::new(10.0),
            event_type: EventType::Movement(GridPosData::new(0, 0)),
        }).expect("Failed to add event");
        let timeline_labyrinth = PublishTimeline::from_draft(draft_labyrinth);
        
        let mut draft_gala = DraftTimeline::new();
        draft_gala.add_event(TimelineEvent {
            timestamp: TimeStamp::new(30.0),
            event_type: EventType::Ability(AbilityType::AutoShot, None),
        }).expect("Failed to add event");
        let timeline_gala = PublishTimeline::from_draft(draft_gala);
        
        // Store timelines for different arenas using ArenaName directly
        let labyrinth_name = ArenaName::Labyrinth;
        let gala_name = ArenaName::Gala;
        
        character_timelines.store_timeline(labyrinth_name, timeline_labyrinth);
        character_timelines.store_timeline(gala_name, timeline_gala);
        
        // Verify separate timeline storage
        assert_eq!(character_timelines.arena_count(), 2);
        assert!(character_timelines.has_recording_for(labyrinth_name));
        assert!(character_timelines.has_recording_for(gala_name));
        assert!(!character_timelines.has_recording_for(ArenaName::GuildHouse)); // GuildHouse - no recording
        
        // Verify we can retrieve specific arena timelines
        let labyrinth_timeline = character_timelines.get_timeline(labyrinth_name).unwrap();
        assert_eq!(labyrinth_timeline.events.len(), 1);
        assert_eq!(labyrinth_timeline.events[0].timestamp, TimeStamp::new(10.0));
        
        let gala_timeline = character_timelines.get_timeline(gala_name).unwrap();
        assert_eq!(gala_timeline.events.len(), 1);
        assert_eq!(gala_timeline.events[0].timestamp, TimeStamp::new(30.0));
        
        // Verify recorded arenas iterator
        let recorded: Vec<_> = character_timelines.recorded_arenas().collect();
        assert_eq!(recorded.len(), 2);
        assert!(recorded.contains(&labyrinth_name));
        assert!(recorded.contains(&gala_name));
    }
}
```

## Next Steps

With the timeline foundation in place, we can now:

- Tutorial 02: Build the recording state machine with event-driven transitions
- Tutorial 03: Implement movement capture (recording intent, not transforms!)
- Tutorial 04: Create the playback system with strict ordering

## Key Takeaways

1. **🎯 RULE 1 - Components First**: TimelineClock, DraftTimeline, CharacterTimelines are Components (entity state), GlobalTimelinePause is Resource (global singleton)
2. **🎯 RULE 2 - Static Data Lookup**: TimeStamp::ZERO/MAX constants, ArenaName enum indices for efficient arena data access
3. **🎯 RULE 3 - Events for Communication**: TimelineCheckpoint events notify systems of timeline milestones, decoupled communication
4. **🎯 RULE 5 - Marker Components**: Recordable, TimelineReady, TimelineActive unit structs for efficient entity filtering
5. **Type Domain Separation**: ArenaName (domain logic) for arena identification, Arena (component) for entity attachment
3. **Type-Safe Newtypes**: TimeStamp, ArenaName, GridPos provide compile-time safety
4. **Intent Not Transform**: Recording Movement(GridPos) not Transform(Vec3)
5. **Zero-Alloc Helpers**: events_in_range(), next_event_after(), slice() avoid allocations
6. **Explicit Constructors**: TimeStamp::new(), GridPos::new(), ArenaName methods for domain logic, Arena(ArenaName) for components
7. **Binary Search**: Efficient O(log n) operations on sorted timelines
8. **Zero-Copy Ownership Transfer**: PublishTimeline::from_draft(draft) consumes for efficient Vec→Arc conversion
9. **Idiomatic Helpers**: Use `std::convert::identity` over trivial closures for clearer intent
10. **Consolidated Timeline API**: next_event_after/prev_event_before provide unified temporal queries with consistent
   binary_search_by approach
11. **Virtual Time for Pause Safety**: Time<Virtual> prevents time jumps when pausing/unpausing
12. **🆕 Bevy 0.16 Error-Safe ECS**: Use Result-returning query methods with `?` operator or let-else patterns for robust
    error handling
13. **🆕 Compile-Time Safe Arena Creation**: Use `from_index_safe()` for all arena creation - no runtime validation needed
14. **🎯 Multi-Arena Timeline Storage**: CharacterTimelines component stores HashMap<ArenaName, PublishTimeline> to support characters recording in all 9 arenas with separate timeline storage per arena
15. **🚀 Unified Event Architecture**: Like CameraUpdate, use RecordingUpdate as root orchestration event to prevent event explosion and race conditions
16. **⚡ CurrentArenaEntity SystemParam**: Use CurrentArenaEntity SystemParam for O(1) current arena entity access instead of repetitive lookups. Performance is critical when supporting 320+ ghosts across 9 arenas!

## Production Notes

### Critical Performance Optimization: ArenaEntities O(1) Lookup

**The Problem:** Early tutorial versions used O(n) linear searches to find arena entities:

```rust
// OLD WAY - O(n) linear search (SLOW!)
let Some((arena, clock)) = arena_q
    .iter()
    .find(|(arena, _)| arena.0 == current_arena.0)
else {
    return;
};
```

This becomes a performance bottleneck when supporting 320+ ghosts across 9 arenas.

**The Solution:** Use CurrentArenaEntity SystemParam for O(1) lookup:

```rust
// NEW WAY - O(1) lookup using CurrentArenaEntity (FAST!)
let Ok((arena, clock)) = arena_q.get(current.get()) else {
    return;
};
```

**Why This Matters:**
- O(n) search: 9 comparisons worst case per system per frame
- O(1) lookup: 1 array access using enum discriminant as index
- With 100+ systems doing arena lookups, this saves thousands of comparisons per frame
- CurrentArenaEntity uses ArenaEntities resource which maps ArenaName enum discriminants (0-8) to entities for instant access

### Critical Architectural Fix: Multi-Arena Timeline Storage

**The Problem:** The original design assumed characters would only have one timeline, but characters can move between all 9 arenas and need to store separate recordings for each arena they've been in.

**The Solution:** 
- `CharacterTimelines` component stores `HashMap<ArenaName, PublishTimeline>` per character
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
- **🆕 Compile-Time Safe Arena Creation**: `ArenaName::from_index_safe()` eliminates runtime validation for all arena creation by clamping to valid ranges. This follows Rule #1 (Type Domain Separation) and Rule #30 (Build for Now).

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