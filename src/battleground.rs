use bevy::prelude::Component;

/// Parent entity for the arena grid, positioned at the top-left of the screen (-640, 360).
/// This allows tiles to use positive coordinates (0,0 at top-left) matching typical 2D conventions.

#[derive(Component)]
pub struct BattleGround;
