use super::{
    get_local_tile_space, Arena, ArenaId, ArenaTile, GridPosition, ARENA_HEIGHT, ARENA_HEIGHT_HALF,
    ARENA_WIDTH, ARENA_WIDTH_HALF, DEBUG_COLORS, GRID_HEIGHT, GRID_WIDTH, TOTAL_ARENAS,
};
use crate::arena_camera::CAMERA_CENTER;
use crate::battleground::BattleGround;
use crate::class_type::ClassType;
use crate::selectors::Active;
use bevy::prelude::*;

/// Calculate the world position of an arena by its ArenaId  
pub fn get_arena_position(arena_id: ArenaId) -> Vec3 {
    let col = arena_id.col() as f32;
    let row = arena_id.row() as f32;

    // Start from window top-left corner and offset by arena size
    let x = (col * ARENA_WIDTH);
    let y = (row * ARENA_HEIGHT);

    Vec3::new(x, y, 0.0)
}

/// Spawn the tiles for a single arena as children of the arena entity
pub fn spawn_arena_tiles(commands: &mut Commands, arena_entity: Entity, tile_scene: Handle<Scene>) {
    commands.entity(arena_entity).with_children(|parent| {
        for row in 0..GRID_HEIGHT {
            for col in 0..GRID_WIDTH {
                let local = get_local_tile_space(row, col);

                // Spawn each tile as a child
                parent.spawn((
                    SceneRoot(tile_scene.clone()),
                    Transform::from_xyz(local.x, local.y, 0.0),
                    GridPosition { row, col },
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
        let is_guild_house = arena_index == 1;
        let mut arena_entity = commands.spawn((
            Transform::from_translation(position),
            InheritedVisibility::default(),
            BattleGround,
            Arena,
            arena_id,
            class_type,
            Name::new(arena_name),
        ));

        // Set the first arena (index 0) as active by default
        if is_guild_house {
            arena_entity.insert(Active);
        }

        let arena_entity_id = arena_entity.id();

        // Spawn the tiles for this arena
        // spawn_arena_tiles(commands, arena_entity_id, tile_scene.clone());
    }
}

pub fn spawn_lights(mut commands: Commands) {
    commands.spawn(DirectionalLight {
        illuminance: 10000.0,
        color: Color::WHITE,
        shadows_enabled: true,
        ..default()
    });

    commands.spawn((
        SpotLight {
            intensity: 10000000.0, // lumens
            color: Color::srgb(1.0, 0.0, 0.0),
            shadows_enabled: true,
            inner_angle: 0.6,
            outer_angle: 0.6,
            ..default()
        },
        Transform::from_xyz(ARENA_WIDTH_HALF, ARENA_HEIGHT_HALF, 9.0)
            .looking_at(CAMERA_CENTER, Vec3::Y),
    ));
}
