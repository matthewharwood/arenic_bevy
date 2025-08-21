use bevy::ecs::change_detection::DetectChanges;
use bevy::log::trace;
use bevy::prelude::*;
use bevy::time::Virtual;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::sync::Arc;
use std::time::Duration;
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
    /// Time when this event occurred
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
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
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
        let safe_seconds = if seconds.is_nan() {
            Self::ZERO.0
        } else {
            seconds
        };
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
        let safe_seconds = if seconds.is_nan() {
            Self::ZERO.0
        } else {
            seconds
        };
        Self(safe_seconds.rem_euclid(Self::MAX.0))
    }
}

impl Display for TimeStamp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}s", self.0)
    }
}

/// Types of events that can be recorded - ALL use value types
#[derive(Clone, Debug)]
pub enum EventType {
    /// Movement intent from input - uses VALUE type GridPosData
    Movement(GridPosData),
    /// Ability cast with optional target
    Ability(AbilityType, Option<TargetData>),
    /// Character death event
    Death,
}

/// Target data for events
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TargetData {
    Entity(Entity),
    Position(GridPosData),
}

use crate::ability::AbilityType;
use crate::arena::{Arena, ArenaName, CurrentArenaEntity};

// === TYPE DOMAIN SEPARATION ===

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
    pub fn add_event(&mut self, event: TimelineEvent) -> TimelineResult<()> {
        let pos = self.events.binary_search_by(|e| {
            e.timestamp
                .partial_cmp(&event.timestamp)
                .ok_or_else(|| TimelineError::InvalidComparison)
                .unwrap_or(Ordering::Equal)
        });

        match pos {
            Ok(pos) | Err(pos) => {
                if pos <= self.events.len() {
                    self.events.insert(pos, event);
                    Ok(())
                } else {
                    Err(TimelineError::OperationFailed {
                        message: format!(
                            "Insert position {} exceeds timeline length {}",
                            pos,
                            self.events.len()
                        ),
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
    /// Returns events where start <= timestamp < end
    #[must_use]
    pub fn events_in_range(
        &self,
        start: TimeStamp,
        end: TimeStamp,
    ) -> TimelineResult<impl Iterator<Item = &TimelineEvent> + '_> {
        if start.as_secs() < 0.0 || end.as_secs() < 0.0 {
            return Err(TimelineError::OperationFailed {
                message: format!(
                    "Invalid range: start={:.1}s, end={:.1}s",
                    start.as_secs(),
                    end.as_secs()
                ),
            });
        }

        let start_idx = self
            .events
            .binary_search_by(|e| {
                e.timestamp
                    .partial_cmp(&start)
                    .ok_or_else(|| TimelineError::InvalidComparison)
                    .unwrap_or(Ordering::Equal)
            })
            .unwrap_or_else(|idx| idx);

        let end_idx = self
            .events
            .binary_search_by(|e| {
                e.timestamp
                    .partial_cmp(&end)
                    .ok_or_else(|| TimelineError::InvalidComparison)
                    .unwrap_or(Ordering::Equal)
            })
            .unwrap_or_else(|idx| idx);

        if start_idx > self.events.len() || end_idx > self.events.len() {
            return Err(TimelineError::OperationFailed {
                message: format!(
                    "Binary search failed on timeline with {} events",
                    self.events.len()
                ),
            });
        }

        Ok(self.events[start_idx..end_idx].iter())
    }

    /// Safe event lookup with error context
    #[must_use]
    pub fn next_event_after(&self, timestamp: TimeStamp) -> TimelineResult<Option<&TimelineEvent>> {
        if timestamp.as_secs() < 0.0 {
            return Err(TimelineError::OperationFailed {
                message: format!("Invalid timestamp: {:.1}s", timestamp.as_secs()),
            });
        }

        let search_result = self.events.binary_search_by(|e| {
            e.timestamp
                .partial_cmp(&timestamp)
                .ok_or_else(|| TimelineError::InvalidComparison)
                .unwrap_or(Ordering::Equal)
        });

        let idx = match search_result {
            Ok(exact_idx) => exact_idx + 1,
            Err(insert_idx) => insert_idx,
        };

        Ok(self.events.get(idx))
    }

    /// Get a slice of the timeline events
    #[must_use]
    pub fn slice(&self, start: usize, end: usize) -> &[TimelineEvent] {
        &self.events[start.min(self.events.len())..end.min(self.events.len())]
    }

    /// Safe previous event lookup
    #[must_use]
    pub fn prev_event_before(
        &self,
        timestamp: TimeStamp,
    ) -> TimelineResult<Option<&TimelineEvent>> {
        if timestamp.as_secs() < 0.0 {
            return Err(TimelineError::OperationFailed {
                message: format!("Invalid timestamp: {:.1}s", timestamp.as_secs()),
            });
        }

        let search_result = self.events.binary_search_by(|e| {
            e.timestamp
                .partial_cmp(&timestamp)
                .ok_or_else(|| TimelineError::InvalidComparison)
                .unwrap_or(Ordering::Equal)
        });

        let idx = match search_result {
            Ok(exact_idx) => Some(exact_idx),
            Err(insert_idx) => insert_idx.checked_sub(1),
        };

        Ok(idx.and_then(|i| self.events.get(i)))
    }
}

/// Clock for 2-minute arena cycles
/// RULE 1 COMPLIANCE: TimelineClock is a Component, not Resource
/// Each arena entity has its own clock for independent timing
/// PR Gate: Using bevy::time::Timer for proper time handling
/// Virtual time integration ensures pause-safe operation
#[derive(Component)]
pub struct TimelineClock {
    /// Internal timer that processes virtual time deltas
    pub timer: bevy::time::Timer,
    pub is_paused: bool, // Local pause state (separate from global)
}

impl TimelineClock {
    pub fn new() -> Self {
        Self {
            timer: bevy::time::Timer::new(
                Duration::from_secs(120),
                bevy::time::TimerMode::Repeating,
            ),
            is_paused: false,
        }
    }

    pub fn pause(&mut self) {
        self.is_paused = true;
    }

    pub fn resume(&mut self) {
        self.is_paused = false;
    }

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

/// Current playback position for a character's timeline
#[derive(Component)]
pub struct TimelinePosition(pub TimeStamp);

impl TimelinePosition {
    pub fn sync_with_clock(&mut self, clock: &TimelineClock) {
        self.0 = clock.current();
    }
}

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
pub fn update_timeline_clocks(
    virtual_time: Res<Time<Virtual>>,
    mut arena_q: Query<(&Arena, &mut TimelineClock)>,
) {
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

pub struct TimelinePlugin;

impl Plugin for TimelinePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GlobalTimelinePause>()
            .add_event::<TimelineCheckpoint>()
            .add_systems(
                Update,
                (
                    // Control virtual time pause state BEFORE updating clocks
                    control_virtual_time_pause,
                    update_timeline_clocks,
                    debug_timeline_clocks,
                )
                    .chain(),
            ); // chain() ensures sequential execution
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draft_timeline_adds_events_sorted() {
        let mut timeline = DraftTimeline::new();

        timeline
            .add_event(TimelineEvent {
                timestamp: TimeStamp::new(5.0),
                event_type: EventType::Movement(GridPosData::new(1, 0)),
            })
            .expect("Failed to add event");

        timeline
            .add_event(TimelineEvent {
                timestamp: TimeStamp::new(2.0),
                event_type: EventType::Ability(AbilityType::AutoShot, None),
            })
            .expect("Failed to add event");

        timeline
            .add_event(TimelineEvent {
                timestamp: TimeStamp::new(10.0),
                event_type: EventType::Death,
            })
            .expect("Failed to add event");

        assert_eq!(timeline.events.len(), 3);
        assert_eq!(timeline.events[0].timestamp, TimeStamp::new(2.0));
        assert_eq!(timeline.events[1].timestamp, TimeStamp::new(5.0));
        assert_eq!(timeline.events[2].timestamp, TimeStamp::new(10.0));
    }

    #[test]
    fn test_timeline_clock_loops_at_120_seconds() {
        let mut clock = TimelineClock::new();

        clock.tick(Duration::from_secs(125));

        assert_eq!(clock.current().as_secs(), 5.0);
    }

    #[test]
    fn test_timestamp_wrap_around() {
        let timestamp = TimeStamp::wrapped(TimeStamp::MAX.0);
        assert_eq!(timestamp.as_secs(), TimeStamp::ZERO.0);

        let timestamp = TimeStamp::wrapped(365.0);
        assert_eq!(timestamp.as_secs(), 5.0);

        let timestamp = TimeStamp::wrapped(-10.0);
        assert_eq!(timestamp.as_secs(), 110.0);
    }

    #[test]
    fn test_timeline_clock_pause_resume() {
        let mut clock = TimelineClock::new();

        clock.tick(Duration::from_secs(10));
        assert_eq!(clock.current().as_secs(), 10.0);

        clock.pause();
        clock.tick(Duration::from_secs(10));
        assert_eq!(clock.current().as_secs(), 10.0);

        clock.resume();
        clock.tick(Duration::from_secs(10));
        assert_eq!(clock.current().as_secs(), 20.0);
    }

    #[test]
    fn test_publish_timeline_get_events_in_range() {
        let mut draft = DraftTimeline::new();

        for i in 0..10 {
            draft
                .add_event(TimelineEvent {
                    timestamp: TimeStamp::new(i as f32 * 2.0),
                    event_type: EventType::Movement(GridPosData::new(i as i32, 0)),
                })
                .expect("Failed to add event");
        }

        let published = PublishTimeline::from_draft(draft);

        let events: Vec<_> = published
            .events_in_range(TimeStamp::new(5.0), TimeStamp::new(10.0))
            .unwrap()
            .collect();

        assert_eq!(events.len(), 2); // Should get events at 6.0, 8.0
        assert_eq!(events[0].timestamp, TimeStamp::new(6.0));
        assert_eq!(events[1].timestamp, TimeStamp::new(8.0));
    }

    #[test]
    fn test_next_event_after_edge_cases() {
        let mut draft = DraftTimeline::new();

        draft
            .add_event(TimelineEvent {
                timestamp: TimeStamp::new(10.0),
                event_type: EventType::Movement(GridPosData::new(0, 0)),
            })
            .expect("Failed to add event");
        draft
            .add_event(TimelineEvent {
                timestamp: TimeStamp::new(20.0),
                event_type: EventType::Ability(AbilityType::AutoShot, None),
            })
            .expect("Failed to add event");
        draft
            .add_event(TimelineEvent {
                timestamp: TimeStamp::new(30.0),
                event_type: EventType::Movement(GridPosData::new(1, 0)),
            })
            .expect("Failed to add event");

        let published = PublishTimeline::from_draft(draft);

        let next = published.next_event_after(TimeStamp::new(15.0)).unwrap();
        assert!(next.is_some());
        assert_eq!(next.unwrap().timestamp, TimeStamp::new(20.0));

        let next = published.next_event_after(TimeStamp::new(20.0)).unwrap();
        assert!(next.is_some());
        assert_eq!(next.unwrap().timestamp, TimeStamp::new(30.0));

        let next = published.next_event_after(TimeStamp::new(30.0)).unwrap();
        assert!(next.is_none());

        let next = published.next_event_after(TimeStamp::new(35.0)).unwrap();
        assert!(next.is_none());

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

        // Test Arena component with ArenaName enum
        let arena = Arena(ArenaName::Bastion);
        assert_eq!(arena.0.as_u8(), 4);
        assert_eq!(arena.0, ArenaName::Bastion);

        let pos_data = GridPosData::new(5, -3);
        assert_eq!(pos_data.x, 5);
        assert_eq!(pos_data.y, -3);
        assert_eq!(pos_data.to_string(), "(5, -3)");

        let pos_component = GridPositionComponent::new(5, -3);
        assert_eq!(pos_component.x, 5);
        assert_eq!(pos_component.y, -3);
        assert_eq!(pos_component.to_string(), "(5, -3)");

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
        draft_labyrinth
            .add_event(TimelineEvent {
                timestamp: TimeStamp::new(10.0),
                event_type: EventType::Movement(GridPosData::new(0, 0)),
            })
            .expect("Failed to add event");
        let timeline_labyrinth = PublishTimeline::from_draft(draft_labyrinth);

        let mut draft_gala = DraftTimeline::new();
        draft_gala
            .add_event(TimelineEvent {
                timestamp: TimeStamp::new(30.0),
                event_type: EventType::Ability(AbilityType::AutoShot, None),
            })
            .expect("Failed to add event");
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
