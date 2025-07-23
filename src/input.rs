//! Input handling traits and types for modular input processing.
//! 
//! This module provides traits for standardizing input handling across
//! different game systems, making input processing more modular and testable.

use bevy::prelude::*;

/// Trait for handling input events and converting them to actions
pub trait InputHandler {
    /// The type of action this handler produces
    type Action;
    /// The input context type this handler works with
    type Context<'a> = &'a ButtonInput<KeyCode>;
    /// Optional state that can be maintained between input handling calls
    type State: Default = ();
    
    /// Process input and return an optional action
    fn handle_input(&self, input: Self::Context<'_>) -> Option<Self::Action> {
        self.handle_input_with_state(input, &Self::State::default())
    }
    
    /// Process input with persistent state
    fn handle_input_with_state(
        &self, 
        input: Self::Context<'_>, 
        _state: &Self::State
    ) -> Option<Self::Action> {
        self.handle_input(input)
    }
}

/// Actions that can be performed for arena navigation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArenaAction {
    /// Navigate to the next arena
    NextArena,
    /// Navigate to the previous arena
    PreviousArena,
    /// Toggle camera zoom
    ToggleZoom,
}

/// Handler for arena navigation input
pub struct ArenaNavigationHandler;

impl InputHandler for ArenaNavigationHandler {
    type Action = ArenaAction;
    
    fn handle_input(&self, input: &ButtonInput<KeyCode>) -> Option<Self::Action> {
        if input.just_pressed(KeyCode::BracketRight) {
            Some(ArenaAction::NextArena)
        } else if input.just_pressed(KeyCode::BracketLeft) {
            Some(ArenaAction::PreviousArena)
        } else if input.just_pressed(KeyCode::KeyP) {
            Some(ArenaAction::ToggleZoom)
        } else {
            None
        }
    }
}

/// Actions that can be performed for character control
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterAction {
    /// Move left
    MoveLeft,
    /// Move right
    MoveRight,
    /// Move up
    MoveUp,
    /// Move down
    MoveDown,
    /// Cycle to next character
    CycleCharacter,
}

/// Handler for character control input
pub struct CharacterControlHandler;

impl InputHandler for CharacterControlHandler {
    type Action = CharacterAction;
    
    fn handle_input(&self, input: &ButtonInput<KeyCode>) -> Option<Self::Action> {
        if input.just_pressed(KeyCode::KeyA) {
            Some(CharacterAction::MoveLeft)
        } else if input.just_pressed(KeyCode::KeyD) {
            Some(CharacterAction::MoveRight)
        } else if input.just_pressed(KeyCode::KeyW) {
            Some(CharacterAction::MoveUp)
        } else if input.just_pressed(KeyCode::KeyS) {
            Some(CharacterAction::MoveDown)
        } else if input.just_pressed(KeyCode::Tab) {
            Some(CharacterAction::CycleCharacter)
        } else {
            None
        }
    }
}