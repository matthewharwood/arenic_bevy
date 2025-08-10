use bevy::prelude::*;

/// Marker component identifying an arena entity in the world.
/// Each arena entity should have this component along with ArenaId.
#[derive(Component, Debug)]
pub struct Arena(pub u8);

/// Marker component for arena tile entities.
#[derive(Component, Debug)]
pub struct ArenaTile;

#[derive(Component, Debug, Clone)]
pub struct CurrentArena(pub u8);

impl CurrentArena {
    /// Increment arena index cyclically (0-8)
    pub fn increment(value: u8) -> u8 {
        (value + 1) % 9
    }

    /// Decrement arena index cyclically (0-8)
    pub fn decrement(value: u8) -> u8 {
        if value == 0 { 8 } else { value - 1 }
    }
}

pub fn decrement_current_arena(
    keycode: Res<ButtonInput<KeyCode>>,
    current_arena_q: Single<&mut CurrentArena>,
) {
    if keycode.just_pressed(KeyCode::BracketLeft) {
        let mut current_arena = current_arena_q.into_inner();
        current_arena.0 = CurrentArena::decrement(current_arena.0);
    }
}

pub fn increment_current_arena(
    keycode: Res<ButtonInput<KeyCode>>,
    current_arena_q: Single<&mut CurrentArena>,
) {
    if keycode.just_pressed(KeyCode::BracketRight) {
        let mut current_arena = current_arena_q.into_inner();
        current_arena.0 = CurrentArena::increment(current_arena.0);
    }
}
