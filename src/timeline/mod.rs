mod tests;

use crate::ability::AbilityType;
use crate::arena::{Arena, ArenaId, CurrentArena};
use bevy::ecs::change_detection::DetectChanges;
use bevy::log::trace;
use bevy::math::IVec2;
use bevy::prelude::*;
use bevy::time::{Time, Timer, TimerMode, Virtual};
use std::cmp::Ordering::Equal;
use std::collections::HashMap;
use std::convert::identity;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::sync::Arc;
use std::time::Duration;

/// Error types for timeline operations
#[derive(Debug, thiserror::Error)]
pub enum TimelineError {
    #[error("Invalid timestamp: {timestamp} (must be between 0.0 and 120.0)")]
    InvalidTimestamp { timestamp: TimeStamp },

    #[error("Timeline position corrupted: {position}")]
    CorruptedPosition { position: TimeStamp },

    #[error("Timeline is empty")]
    EmptyTimeline,

    #[error("Event comparison failed - invalid timestamp")]
    InvalidComparison,

    #[error("Grid position out of bounds: ({x}, {y})")]
    InvalidGridPosition { x: i32, y: i32 },
}

/// Result type for timeline operations
pub type TimelineResult<T> = Result<T, TimelineError>;

#[derive(Clone, Debug)]
pub struct TimelineEvent {
    pub timestamp: TimeStamp,
    pub event_type: EventType,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
pub struct TimeStamp(pub f32);

impl TimeStamp {
    pub const ZERO: Self = Self(0.0);
    pub const MAX: Self = Self(120.0);

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

    /// Wraps time back to start when, exceeding 120 seconds,
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

impl From<f32> for TimeStamp {
    fn from(seconds: f32) -> Self {
        Self::new(seconds)
    }
}
impl Display for TimeStamp {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:.1}s", self.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Target {
    Entity(Entity),
    Position(GridPos),
}

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
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "({}, {})", self.0.x, self.0.y)
    }
}

#[derive(Clone, Debug)]
pub enum EventType {
    Movement(GridPos),
    Ability(AbilityType, Option<Target>),
    Death,
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

    pub fn add_event(&mut self, event: TimelineEvent) -> TimelineResult<()> {
        // Use safe comparison with proper error handling
        let comparison = |e: &TimelineEvent| {
            e.timestamp
                .partial_cmp(&event.timestamp)
                .ok_or(TimelineError::InvalidComparison)
        };

        match self
            .events
            .binary_search_by(|e| comparison(e).unwrap_or(Equal))
        {
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

pub struct PublishTimeline {
    pub events: Arc<[TimelineEvent]>,
}

/// Component to store multiple timelines per character (one per arena)
/// This solves the critical architectural issue: characters can record in all 9 arenas
/// and need separate timeline storage for each arena they've recorded in
#[derive(Component, Default)]
pub struct CharacterTimelines {
    /// Map from ArenaId to PublishTimeline - supports up to 9 timelines per character
    /// Using HashMap for O(1) lookup by arena, typically 1-3 arenas per character
    pub timelines: HashMap<ArenaId, PublishTimeline>,
}

impl CharacterTimelines {
    pub fn new() -> Self {
        Self {
            timelines: HashMap::new(),
        }
    }

    /// Store a timeline for a specific arena
    pub fn store_timeline(&mut self, arena: ArenaId, timeline: PublishTimeline) {
        self.timelines.insert(arena, timeline);
    }

    /// Get timeline for a specific arena
    pub fn get_timeline(&self, arena: ArenaId) -> Option<&PublishTimeline> {
        self.timelines.get(&arena)
    }

    /// Remove timeline for a specific arena
    pub fn remove_timeline(&mut self, arena: ArenaId) -> Option<PublishTimeline> {
        self.timelines.remove(&arena)
    }

    /// Get all arenas this character has recordings for
    pub fn recorded_arenas(&self) -> impl Iterator<Item = ArenaId> + '_ {
        self.timelines.keys().copied()
    }

    /// Total number of recorded arenas
    pub fn arena_count(&self) -> usize {
        self.timelines.len()
    }

    /// Check if character has a recording for the given arena
    pub fn has_recording_for(&self, arena: ArenaId) -> bool {
        self.timelines.contains_key(&arena)
    }
}

impl PublishTimeline {
    pub fn from_draft(draft: DraftTimeline) -> TimelineResult<Self> {
        if draft.events.is_empty() {
            return Err(TimelineError::EmptyTimeline);
        }

        Ok(Self {
            events: draft.events.into(),
        })
    }
    #[must_use]
    pub fn events_in_range(
        &self,
        start: TimeStamp,
        end: TimeStamp,
    ) -> TimelineResult<impl Iterator<Item = &TimelineEvent> + '_> {
        // Safe comparison with proper error handling
        let safe_compare = |e: &TimelineEvent, target: TimeStamp| {
            e.timestamp
                .partial_cmp(&target)
                .ok_or(TimelineError::InvalidComparison)
        };

        let start_idx = self
            .events
            .binary_search_by(|e| safe_compare(e, start).unwrap_or(Equal))
            .unwrap_or_else(identity);
        let end_idx = self
            .events
            .binary_search_by(|e| safe_compare(e, end).unwrap_or(Equal))
            .unwrap_or_else(identity);

        Ok(self.events[start_idx..end_idx].iter())
    }

    /// Get next event after a specific timestamp
    /// Returns the first event with timestamp > the provided timestamp
    #[must_use]
    pub fn next_event_after(&self, timestamp: TimeStamp) -> TimelineResult<Option<&TimelineEvent>> {
        let safe_compare = |e: &TimelineEvent| {
            e.timestamp
                .partial_cmp(&timestamp)
                .ok_or(TimelineError::InvalidComparison)
        };

        match self
            .events
            .binary_search_by(|e| safe_compare(e).unwrap_or(Equal))
        {
            Ok(idx) => Ok(self.events.get(idx + 1)), // Found exact match, return next
            Err(idx) => Ok(self.events.get(idx)), // Found insertion point, return event at that position
        }
    }

    /// Get a previous event before or at a specific timestamp
    /// Returns the most recent event with timestamp <= the provided timestamp
    ///
    /// Complements next_event_after for full timeline traversal capabilities
    #[must_use]
    pub fn prev_event_before(
        &self,
        timestamp: TimeStamp,
    ) -> TimelineResult<Option<&TimelineEvent>> {
        let safe_compare = |e: &TimelineEvent| {
            e.timestamp
                .partial_cmp(&timestamp)
                .ok_or(TimelineError::InvalidComparison)
        };

        match self
            .events
            .binary_search_by(|e| safe_compare(e).unwrap_or(Equal))
        {
            Ok(idx) => Ok(self.events.get(idx)), // Found an exact match, return it
            Err(idx) => Ok(idx.checked_sub(1).and_then(|i| self.events.get(i))), // Return previous element
        }
    }
}

/// Clock Component for 2-minute arena cycles
#[derive(Component)]
pub struct TimelineClock {
    pub timer: Timer,
    pub is_paused: bool,
}

impl Default for TimelineClock {
    fn default() -> Self {
        Self {
            timer: Timer::new(Duration::from_secs(120), TimerMode::Repeating),
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
    pub fn pause(&mut self) {
        self.is_paused = true;
    }
    pub fn resume(&mut self) {
        self.is_paused = false;
    }
    pub fn current(&self) -> TimeStamp {
        TimeStamp::wrapped(self.timer.elapsed_secs())
    }
}

pub struct TimelinePosition(pub TimeStamp);

impl TimelinePosition {
    pub fn sync_with_clock(&mut self, clock: &TimelineClock) {
        self.0 = clock.current();
    }
}

#[derive(Debug, Clone)]
pub enum PauseReason {
    DialogOpen,
    SystemMenu,
    LoadingTransition,
}
#[derive(Resource, Default)]
pub struct GlobalTimelinePause {
    pub is_paused: bool,
    pub pause_reason: Option<PauseReason>,
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

pub fn update_timeline_clocks(
    // Use Time<Virtual> which automatically handles pause states
    virtual_time: Res<Time<Virtual>>,
    mut arena_q: Query<(&Arena, &mut TimelineClock)>,
) {
    // Virtual time's delta is already pause-aware - no need to check GlobalTimelinePause
    let delta = virtual_time.delta();

    for (_arena, mut clock) in arena_q.iter_mut() {
        clock.tick(delta);
    }
}
/// Control virtual time based on the GlobalTimelinePause state
pub fn control_virtual_time_pause(
    global_pause: Res<GlobalTimelinePause>,
    mut virtual_time: ResMut<Time<Virtual>>,
) {
    // Only update when pause state changes to avoid unnecessary mutations
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
    current_arena_q: Query<&CurrentArena>,
) {
    // Get the current arena entity
    let Ok(current_arena) = current_arena_q.single() else {
        return;
    };

    // Use let-else for early return pattern - more idiomatic Rust
    let Some((arena, clock)) = arena_q
        .iter()
        .find(|(arena, _)| arena.name() == current_arena.name())
    else {
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
        app.init_resource::<GlobalTimelinePause>().add_systems(
            Update,
            (
                control_virtual_time_pause,
                update_timeline_clocks,
                debug_timeline_clocks,
            )
                .chain(),
        );
    }
}
