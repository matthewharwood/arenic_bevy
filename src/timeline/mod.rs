mod tests;

use crate::ability::AbilityType;
use crate::arena::{Arena, CurrentArena};
use bevy::ecs::change_detection::DetectChanges;
use bevy::log::trace;
use bevy::math::IVec2;
use bevy::prelude::*;
use bevy::time::{Time, Timer, TimerMode, Virtual};
use std::convert::identity;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::sync::Arc;
use std::time::Duration;

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

    pub fn add_event(&mut self, event: TimelineEvent) {
        match self
            .events
            .binary_search_by(|e| e.timestamp.partial_cmp(&event.timestamp).unwrap())
        {
            Ok(pos) | Err(pos) => self.events.insert(pos, event),
        }
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}

pub struct PublishTimeline {
    pub events: Arc<[TimelineEvent]>,
}

impl PublishTimeline {
    pub fn from_draft(draft: DraftTimeline) -> Self {
        Self {
            events: draft.events.into(),
        }
    }
    #[must_use]
    pub fn events_in_range(
        &self,
        start: TimeStamp,
        end: TimeStamp,
    ) -> impl Iterator<Item = &TimelineEvent> + '_ {
        // Simple range query - wrap-around handling comes in Tutorial 04
        let start_idx = self
            .events
            .binary_search_by(|e| e.timestamp.partial_cmp(&start).unwrap())
            .unwrap_or_else(identity);
        let end_idx = self
            .events
            .binary_search_by(|e| e.timestamp.partial_cmp(&end).unwrap())
            .unwrap_or_else(identity);

        self.events[start_idx..end_idx].iter()
    }

    /// Get movement intent at a specific timestamp using partition_point for optimal boundary finding
    /// Returns the most recent movement event before or at the timestamp
    ///
    /// Uses partition_point which directly finds where the predicate changes from true to false,
    /// clarifying the logic than binary_search_by with its Ok/Err handling
    #[must_use]
    pub fn get_movement_intent_at(&self, timestamp: TimeStamp) -> Option<GridPos> {
        // partition_point finds the first index where timestamp > t, so we work backwards from there
        // This is more idiomatic than binary_search_by for finding boundaries in sorted sequences
        let mut i = self.events.partition_point(|e| e.timestamp <= timestamp);
        while i > 0 {
            i -= 1; // Move to the last index with ts ≤ timestamp
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
        end: TimeStamp,
    ) -> impl Iterator<Item = &TimelineEvent> + '_ {
        self.events_in_range(start, end)
            .filter(|e| matches!(e.event_type, EventType::Ability(_, _)))
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
    pub fn tick_secs(&mut self, seconds: f32) {
        self.tick(Duration::from_secs_f32(seconds));
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
        .find(|(arena, _)| arena.as_u8() == current_arena.0)
    else {
        return;
    };

    // PR Gate: Using trace! for per-frame logs instead of info!
    if (clock.elapsed_secs() % 1.0) < 0.02 {
        trace!("{}: {:.1}s", arena, clock.elapsed_secs());
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
