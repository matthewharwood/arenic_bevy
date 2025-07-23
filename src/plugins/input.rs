//! Input handling plugin with event-driven architecture.
//! 
//! This plugin converts raw input into game events, decoupling input detection
//! from game logic for better testability and remappability.

use bevy::prelude::*;
use crate::input::{
    ArenaAction, ArenaNavigationHandler, CharacterAction, CharacterControlHandler, InputHandler,
};

/// Events for arena-related actions
#[derive(Event, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArenaActionEvent {
    NextArena,
    PreviousArena,
    ToggleZoom,
}

/// Events for character-related actions
#[derive(Event, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterActionEvent {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    CycleCharacter,
}

/// Plugin responsible for input processing and event generation
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ArenaActionEvent>()
            .add_event::<CharacterActionEvent>()
            .add_systems(
                Update,
                (process_arena_input, process_character_input),
            );
    }
}

/// System that converts arena input to events
fn process_arena_input(
    input: Res<ButtonInput<KeyCode>>,
    mut arena_events: EventWriter<ArenaActionEvent>,
) {
    let handler = ArenaNavigationHandler;
    if let Some(action) = handler.handle_input(&input) {
        let event = match action {
            ArenaAction::NextArena => ArenaActionEvent::NextArena,
            ArenaAction::PreviousArena => ArenaActionEvent::PreviousArena,
            ArenaAction::ToggleZoom => ArenaActionEvent::ToggleZoom,
        };
        arena_events.write(event);
    }
}

/// System that converts character input to events
fn process_character_input(
    input: Res<ButtonInput<KeyCode>>,
    mut character_events: EventWriter<CharacterActionEvent>,
) {
    let handler = CharacterControlHandler;
    if let Some(action) = handler.handle_input(&input) {
        let event = match action {
            CharacterAction::MoveLeft => CharacterActionEvent::MoveLeft,
            CharacterAction::MoveRight => CharacterActionEvent::MoveRight,
            CharacterAction::MoveUp => CharacterActionEvent::MoveUp,
            CharacterAction::MoveDown => CharacterActionEvent::MoveDown,
            CharacterAction::CycleCharacter => CharacterActionEvent::CycleCharacter,
        };
        character_events.write(event);
    }
}