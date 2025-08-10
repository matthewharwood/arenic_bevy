use bevy::prelude::*;

/// Marker component identifying an arena entity in the world.
/// Each arena entity should have this component along with ArenaId.
#[derive(Component, Debug)]
pub struct Arena(pub u8);

/// Marker component for arena tile entities.
#[derive(Component, Debug)]
pub struct ArenaTile;
