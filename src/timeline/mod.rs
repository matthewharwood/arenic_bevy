// Timeline module - implements unified event architecture
// Based on Tutorial 01: Timeline Foundation (refactored version)

use bevy::prelude::*;
use bevy::ecs::change_detection::DetectChanges;
use bevy::log::trace;
use bevy::time::Virtual;
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::convert::identity;
use std::cmp::Ordering;
use std::time::Duration;
use std::sync::Arc;
use thiserror::Error;

/// Error types for timeline operations
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
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
pub struct TimeStamp(pub f32);

impl TimeStamp {
    pub const ZERO: Self = Self(0.0);
    pub const MAX: Self = Self(120.0);

    /// Creates a new TimeStamp, clamping value to [0, 120] seconds
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
use crate::ability::AbilityType;

/// Import arena types from the arena module
use crate::arena::{Arena, ArenaId, ArenaEntities, CurrentArena};

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

impl Display for GridPos {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0.x, self.0.y)
    }
}

/// Component for entities that can be recorded
#[derive(Component)]
pub struct Recordable;

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
    pub fn add_event(&mut self, event: TimelineEvent) -> TimelineResult<()> {
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
    pub events: Arc<[TimelineEvent]>,
}

/// Component that stores multiple timelines per character (one per arena)
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
    pub fn from_draft(draft: DraftTimeline) -> Self {
        Self {
            events: draft.events.into(),
        }
    }

    /// Zero-alloc helper: Get events within a time range
    #[must_use]
    pub fn events_in_range(&self, start: TimeStamp, end: TimeStamp) -> impl Iterator<Item=&TimelineEvent> + '_ {
        let start_idx = self.events.binary_search_by(|e| {
            e.timestamp.partial_cmp(&start).unwrap_or(Ordering::Equal)
        }).unwrap_or_else(identity);
        
        let end_idx = self.events.binary_search_by(|e| {
            e.timestamp.partial_cmp(&end).unwrap_or(Ordering::Equal)
        }).unwrap_or_else(identity);

        self.events[start_idx..end_idx].iter()
    }

    /// Zero-alloc helper: Find next event after timestamp
    #[must_use]
    pub fn next_event_after(&self, timestamp: TimeStamp) -> Option<&TimelineEvent> {
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
    #[must_use]
    pub fn prev_event_before(&self, timestamp: TimeStamp) -> Option<&TimelineEvent> {
        match self.events.binary_search_by(|e| {
            e.timestamp.partial_cmp(&timestamp).unwrap_or(Ordering::Equal)
        }) {
            Ok(idx) => self.events.get(idx),
            Err(idx) => idx.checked_sub(1).and_then(|i| self.events.get(i)),
        }
    }
}

/// Clock for 2-minute arena cycles
#[derive(Component)]
pub struct TimelineClock {
    pub timer: bevy::time::Timer,
    pub is_paused: bool,
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
        Self::new()
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
pub fn debug_timeline_clocks(
    arena_q: Query<(&Arena, &TimelineClock)>,
    arena_entities: Res<ArenaEntities>,
    current_arena: Res<CurrentArena>,
) {
    let current_arena_entity = arena_entities.get(current_arena.name());
    
    let Ok((arena, clock)) = arena_q.get(current_arena_entity) else {
        return;
    };

    if (clock.current().as_secs() % 1.0) < 0.02 {
        trace!("{}: {:.1}s", arena, clock.current().as_secs());
    }
}

pub struct TimelinePlugin;

impl Plugin for TimelinePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GlobalTimelinePause>()
            .add_systems(Update, (
                control_virtual_time_pause,
                update_timeline_clocks,
                debug_timeline_clocks,
            ).chain());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draft_timeline_adds_events_sorted() {
        let mut timeline = DraftTimeline::new();

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
    fn test_character_timelines_multi_arena_storage() {
        let mut character_timelines = CharacterTimelines::new();
        
        let mut draft_labyrinth = DraftTimeline::new();
        draft_labyrinth.add_event(TimelineEvent {
            timestamp: TimeStamp::new(10.0),
            event_type: EventType::Movement(GridPos::new(0, 0)),
        }).expect("Failed to add event");
        let timeline_labyrinth = PublishTimeline::from_draft(draft_labyrinth);
        
        let mut draft_gala = DraftTimeline::new();
        draft_gala.add_event(TimelineEvent {
            timestamp: TimeStamp::new(30.0),
            event_type: EventType::Ability(AbilityType::AutoShot, None),
        }).expect("Failed to add event");
        let timeline_gala = PublishTimeline::from_draft(draft_gala);
        
        let labyrinth_id = ArenaId::from_index_safe(0);
        let gala_id = ArenaId::from_index_safe(8);
        
        character_timelines.store_timeline(labyrinth_id, timeline_labyrinth);
        character_timelines.store_timeline(gala_id, timeline_gala);
        
        assert_eq!(character_timelines.arena_count(), 2);
        assert!(character_timelines.has_recording_for(labyrinth_id));
        assert!(character_timelines.has_recording_for(gala_id));
        
        let labyrinth_timeline = character_timelines.get_timeline(labyrinth_id).unwrap();
        assert_eq!(labyrinth_timeline.events.len(), 1);
        assert_eq!(labyrinth_timeline.events[0].timestamp, TimeStamp::new(10.0));
    }
}