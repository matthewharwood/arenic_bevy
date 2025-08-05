use bevy::prelude::*;
use bevy::pbr::{NotShadowCaster, NotShadowReceiver};
use crate::battleground::BattleGround;

use super::{
    ActiveArena, Arena, ArenaId, ArenaTile, GridPosition,
    ARENA_HEIGHT, ARENA_WIDTH,
    GRID_HEIGHT, GRID_WIDTH, HALF_TILE, HALF_WINDOW_HEIGHT,
    HALF_WINDOW_WIDTH, TILE_SIZE, TOTAL_ARENAS,
};

/// Calculate the world position of an arena by its ArenaId  
pub fn get_arena_position(arena_id: ArenaId) -> Vec3 {
    let col = arena_id.col() as f32;
    let row = arena_id.row() as f32;
    
    // Start from window top-left corner and offset by arena size
    let x = -HALF_WINDOW_WIDTH + (col * ARENA_WIDTH) + HALF_TILE;
    let y = HALF_WINDOW_HEIGHT - (row * ARENA_HEIGHT) - HALF_TILE;
    
    Vec3::new(x, y, 0.0)
}

/// Spawn the tiles for a single arena as children of the arena entity
pub fn spawn_arena_tiles(
    commands: &mut Commands,
    arena_entity: Entity,
    tile_mesh: Handle<Mesh>,
    inset_mesh: Handle<Mesh>,
    gray_material: Handle<StandardMaterial>,
    pink_material: Handle<StandardMaterial>,
) {
    commands.entity(arena_entity).with_children(|parent| {
        for x in 0..GRID_WIDTH {
            for y in 0..GRID_HEIGHT {
                let world_x = x as f32 * TILE_SIZE;
                let world_y = -(y as f32 * TILE_SIZE);

                parent
                    .spawn((
                        Transform::from_xyz(world_x, world_y, 0.0),
                        GridPosition::new(x, y),
                        ArenaTile,
                    ))
                    .with_children(|tile| {
                        tile.spawn((
                            Mesh3d(tile_mesh.clone()),
                            MeshMaterial3d(gray_material.clone()),
                        ));
                        tile.spawn((
                            Mesh3d(inset_mesh.clone()),
                            MeshMaterial3d(pink_material.clone()),
                            NotShadowCaster,
                            NotShadowReceiver,
                        ));
                    });
            }
        }
    });
}

/// Setup the complete 3x3 grid of arenas
pub fn setup_arena_grid(
    commands: &mut Commands,
    tile_mesh: Handle<Mesh>,
    inset_mesh: Handle<Mesh>,
    gray_material: Handle<StandardMaterial>,
    pink_material: Handle<StandardMaterial>,
) {
    // Set up 3x3 grid of arenas (9 total)
    for arena_index in 0..TOTAL_ARENAS {
        let arena_id = ArenaId::new(arena_index as u8).expect("Arena index should be valid");
        let position = get_arena_position(arena_id);

        // Create arena entity
        let mut arena_entity = commands.spawn((
            Transform::from_translation(position),
            InheritedVisibility::default(),
            BattleGround,
            Arena,
            arena_id,
        ));

        // Set the first arena (index 0) as active by default
        if arena_index == 0 {
            arena_entity.insert(ActiveArena);
        }

        let arena_entity_id = arena_entity.id();

        // Spawn the tiles for this arena
        spawn_arena_tiles(
            commands,
            arena_entity_id,
            tile_mesh.clone(),
            inset_mesh.clone(),
            gray_material.clone(),
            pink_material.clone(),
        );
    }
}