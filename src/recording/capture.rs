use crate::ability::AbilityType;
use crate::arena::{Arena, CurrentArenaEntity};
use crate::recording::{Recording, RecordingMode, RecordingState};
use crate::timeline::{
    DraftTimeline, EventType, GlobalTimelinePause, TimelineClock, TimelineEvent,
};
use bevy::input::ButtonInput;
use bevy::log::{info, trace};
use bevy::math::IVec2;
use bevy::prelude::{KeyCode, Query, Res, Single, With};

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
    recording_timeline: Single<&mut DraftTimeline, With<Recording>>,
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
    let mut timeline = recording_timeline.into_inner();
    let event = TimelineEvent {
        timestamp,
        event_type: EventType::Movement(IVec2::new(movement_dir.x, movement_dir.y)),
    };

    timeline.add_event(event).expect("TODO: panic message");
    trace!(
        "Recorded movement intent at {}: {}",
        timestamp,
        IVec2::new(movement_dir.x, movement_dir.y)
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
    recording_timeline: Single<&mut DraftTimeline, With<Recording>>,
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
        todo!()
    };

    let Ok((_, clock)) = arena_q.get(current.get()) else {
        return;
    };
    let mut timeline = recording_timeline.into_inner();
    let timestamp = clock.current();
    let event = TimelineEvent {
        timestamp,
        event_type: EventType::Ability(ability_type, None),
    };

    timeline.add_event(event).expect("TODO: panic message");
    // info!("Recorded ability intent {} at {}", ability_type, timestamp);
}
