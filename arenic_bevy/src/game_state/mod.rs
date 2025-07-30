use bevy::prelude::*;

// Module declarations
mod title;
mod character_create;
mod intro;

// Re-exports
pub use title::TitlePlugin;
pub use character_create::CharacterCreatePlugin;
pub use intro::IntroPlugin;

/// The main game states
#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    Title,
    CharacterCreate,
    Intro,
}

/// Plugin that manages game state transitions
pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<GameState>()
            .add_plugins((
                TitlePlugin,
                CharacterCreatePlugin,
                IntroPlugin,
            ));
    }
}