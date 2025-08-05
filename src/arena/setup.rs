use bevy::prelude::*;
use crate::battleground::BattleGround;
use crate::class_type::ClassType;
use crate::selectors::Active;
use super::{get_local_tile_space, Arena, ArenaId, ArenaTile, GridPosition, ARENA_HEIGHT, ARENA_WIDTH, DEBUG_COLORS, GRID_HEIGHT, GRID_WIDTH, HALF_TILE, HALF_WINDOW_HEIGHT, HALF_WINDOW_WIDTH, TOTAL_ARENAS};

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
    tile_scene: Handle<Scene>,
) {
    commands.entity(arena_entity).with_children(|parent| {
        for row in 0..GRID_WIDTH {
            for col in 0..GRID_HEIGHT {
                let local = get_local_tile_space(row, col);

                // Spawn each tile as a child
                parent.spawn((
                    SceneRoot(tile_scene.clone()),
                    Transform::from_xyz(local.x, local.y, 0.0),
                    GridPosition{row, col},
                    ArenaTile,
                ));
            }
        }
    });
}

/// Setup the complete 3x3 grid of arenas
pub fn setup_arena_grid(
    commands: &mut Commands,
    tile_scene: Handle<Scene>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    // Create materials for each arena (debug colors)
    let _arena_materials: Vec<Handle<StandardMaterial>> = DEBUG_COLORS
        .iter()
        .map(|&color| {
            materials.add(StandardMaterial {
                base_color: color,
                ..default()
            })
        })
        .collect();

    // Set up 3x3 grid of arenas (9 total)
    for arena_index in 0..TOTAL_ARENAS {
        let arena_id = ArenaId::new(arena_index).expect("Arena index should be valid");
        let position = get_arena_position(arena_id);
        let class_type = ClassType::index_of(arena_index);
        let arena_name = ClassType::index_of(arena_index).name();

        let mut arena_entity = commands.spawn((
            Transform::from_translation(position),
            InheritedVisibility::default(),
            BattleGround,
            Arena,
            arena_id,
            class_type,
            Name::new(arena_name)
        ));

        // Set the first arena (index 0) as active by default
        if arena_index == 0 {
            arena_entity.insert(Active);
        }

        let arena_entity_id = arena_entity.id();

        // Spawn the tiles for this arena
        spawn_arena_tiles(commands, arena_entity_id, tile_scene.clone());
    }
}