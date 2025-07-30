use bevy::prelude::Component;

#[derive(Component)]
pub struct Tile;

#[derive(Component, Debug)]
pub struct TilePosition {
    pub row: usize,
    pub col: usize,
}
