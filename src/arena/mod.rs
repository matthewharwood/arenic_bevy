use crate::config::arena::{GRID_HEIGHT, GRID_WIDTH};
use crate::config::display::TILE_SIZE;
use crate::config::{arena, display};
use bevy::asset::AssetServer;
use bevy::math::Vec2;
use bevy::prelude::{default, Commands, Component, Entity, Sprite, Transform};

// Module declarations for each arena type
mod bastion;
mod casino;
mod crucible;
mod gala;
mod guildhouse;
mod labyrinth;
mod mountain;
mod pawnshop;
mod sanctum;

// Re-export all arena types
pub use bastion::Bastion;
pub use casino::Casino;
pub use crucible::Crucible;
pub use gala::Gala;
pub use guildhouse::GuildHouse;
pub use labyrinth::Labyrinth;
pub use mountain::Mountain;
pub use pawnshop::Pawnshop;
pub use sanctum::Sanctum;

#[derive(Component, Debug)]
pub struct Arena;

impl Arena {
    pub fn spawn_tile_grid(
        &self,
        commands: &mut Commands,
        arena_entity: Entity,
        asset_server: &AssetServer,
        _arena_index: usize,
    ) {
        let image_path = format!("{}.png", "default_grid_tile");

        commands.entity(arena_entity).with_children(|parent| {
            for row in 0..GRID_HEIGHT {
                for col in 0..GRID_WIDTH {
                    parent.spawn((
                        Sprite {
                            image: asset_server.load(&image_path),
                            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                            ..default()
                        },
                        Transform::from_xyz(col as f32 * TILE_SIZE, -(row as f32 * TILE_SIZE), 0.0),
                    ));
                }
            }
        });
    }
}

pub trait ArenaTransform {
    /// The index of this arena in the grid (0-8)
    const INDEX: usize;

    /// Calculate the transform position for this arena type
    fn transform() -> Transform {
        const ARENAS_PER_ROW: usize = 3;

        let arena_col = Self::INDEX % ARENAS_PER_ROW;
        let arena_row = Self::INDEX / ARENAS_PER_ROW;

        let x_offset = arena_col as f32 * arena::ARENA_WIDTH;
        let y_offset = arena_row as f32 * arena::ARENA_HEIGHT;

        Transform::from_xyz(
            -display::HALF_WINDOW_WIDTH + display::HALF_TILE_SIZE + x_offset,
            display::HALF_WINDOW_HEIGHT - display::HALF_TILE_SIZE - y_offset,
            0.0,
        )
    }

}

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
