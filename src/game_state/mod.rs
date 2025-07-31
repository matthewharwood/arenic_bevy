use bevy::prelude::*;

// Module declarations
mod character_create;
mod intro;
mod title;

// Re-exports
pub use character_create::CharacterCreatePlugin;
pub use intro::IntroPlugin;
pub use title::TitlePlugin;

/// The main game states
#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Title,
    #[default]
    CharacterCreate,
    Intro,
}

/// Plugin that manages game state transitions
pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins((
            TitlePlugin,
            CharacterCreatePlugin,
            IntroPlugin,
        ));
    }
}
