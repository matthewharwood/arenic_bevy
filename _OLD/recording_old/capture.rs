// Standard library and external crates
use bevy::input::ButtonInput;
use bevy::log::trace;
use bevy::math::{IVec2, IVec3};
use bevy::prelude::{KeyCode, Query, Res, ResMut};

// Local crate modules
use crate::ability::AbilityType;
use crate::arena::{Arena, CurrentArenaEntity};
use crate::recording::{RecordingMode, RecordingState};
use crate::timeline::{
    DraftTimeline, EventType, GlobalTimelinePause, TimelineClock, TimelineEvent,
};

const KEY_MOVE_UP: KeyCode = KeyCode::KeyW;
const KEY_MOVE_DOWN: KeyCode = KeyCode::KeyS;
const KEY_MOVE_LEFT: KeyCode = KeyCode::KeyA;
const KEY_MOVE_RIGHT: KeyCode = KeyCode::KeyD;

// Const keymaps for abilities
const KEY_ABILITIES: [(KeyCode, AbilityType); 4] = [
    (KeyCode::Digit1, AbilityType::AutoShot),
    (KeyCode::Digit2, AbilityType::HolyNova),
    (KeyCode::Digit3, AbilityType::PoisonShot),
    (KeyCode::Digit4, AbilityType::Heal),
];

pub fn capture_movement_intent(
    keyboard: Res<ButtonInput<KeyCode>>,
    global_pause: Res<GlobalTimelinePause>,
    recording_state: Res<RecordingState>,
    mut draft_timeline: ResMut<DraftTimeline>,
    arena_q: Query<(&Arena, &TimelineClock)>,
    current: CurrentArenaEntity,
) {
    if global_pause.is_paused {
        return;
    }
    if recording_state.mode != RecordingMode::Recording {
        return;
    }
    let movement_dir = get_movement_direction(&keyboard);
    if movement_dir == IVec2::ZERO {
        return;
    }

    let Ok((_, clock)) = arena_q.get(current.get()) else {
        return;
    };
    let timestamp = clock.current();
    let event = TimelineEvent {
        timestamp,
        event_type: EventType::Movement(IVec3::new(movement_dir.x, movement_dir.y, 0)),
    };

    draft_timeline
        .add_event(event)
        .expect("TODO: panic message");
    trace!(
        "Recorded movement intent at {}: {}",
        timestamp,
        IVec3::new(movement_dir.x, movement_dir.y, 0)
    );
}

fn get_movement_direction(keyboard: &ButtonInput<KeyCode>) -> IVec2 {
    let mut dir = IVec2::ZERO;

    // Check each movement key - only on the frame it was pressed
    if keyboard.just_pressed(KEY_MOVE_UP) {
        dir.y += 1;
    }
    if keyboard.just_pressed(KEY_MOVE_DOWN) {
        dir.y -= 1;
    }
    if keyboard.just_pressed(KEY_MOVE_LEFT) {
        dir.x -= 1;
    }
    if keyboard.just_pressed(KEY_MOVE_RIGHT) {
        dir.x += 1;
    }

    dir
}

pub fn capture_ability_intent(
    keyboard: Res<ButtonInput<KeyCode>>,
    global_pause: Res<GlobalTimelinePause>,
    recording_state: Res<RecordingState>,
    mut draft_timeline: ResMut<DraftTimeline>,
    arena_q: Query<(&Arena, &TimelineClock)>,
    current: CurrentArenaEntity,
) {
    if global_pause.is_paused {
        return;
    }
    if recording_state.mode != RecordingMode::Recording {
        return;
    }

    let Some(ability_type) = KEY_ABILITIES
        .iter()
        .find(|(key, _)| keyboard.just_pressed(*key))
        .map(|(_, ability)| *ability)
    else {
        return; // No ability key was pressed, just return early
    };

    let Ok((_, clock)) = arena_q.get(current.get()) else {
        return;
    };
    let timestamp = clock.current();
    let event = TimelineEvent {
        timestamp,
        event_type: EventType::Ability(ability_type, None),
    };

    draft_timeline
        .add_event(event)
        .expect("TODO: panic message");
    // info!("Recorded ability intent {} at {}", ability_type, timestamp);
}
