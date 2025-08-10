use bevy::prelude::*;
// pub fn setup_arena_grid(
//     commands: &mut Commands,
//     tile_scene: Handle<Scene>,
//     materials: &mut ResMut<Assets<StandardMaterial>>,
// ) {
//     // Create materials for each arena (debug colors)
//     let _arena_materials: Vec<Handle<StandardMaterial>> = DEBUG_COLORS
//         .iter()
//         .map(|&color| {
//             materials.add(StandardMaterial {
//                 base_color: color,
//                 ..default()
//             })
//         })
//         .collect();
//
//     // Set up 3x3 grid of arenas (9 total)
//     for arena_index in 0..TOTAL_ARENAS {
//         let arena_id = ArenaId::new(arena_index).expect("Arena index should be valid");
//         let position = get_arena_position(arena_id);
//         let class_type = ClassType::index_of(arena_index);
//         let arena_name = ClassType::index_of(arena_index).name();
//         let is_guild_house = arena_index == 1;
//         let mut arena_entity = commands.spawn((
//             Transform::from_translation(position),
//             InheritedVisibility::default(),
//             BattleGround,
//             Arena,
//             arena_id,
//             class_type,
//             Name::new(arena_name),
//         ));
//
//         // Set the first arena (index 0) as active by default
//         if is_guild_house {
//             arena_entity.insert(Active);
//         }
//
//         let arena_entity_id = arena_entity.id();
//
//         // Spawn the tiles for this arena
//         // spawn_arena_tiles(commands, arena_entity_id, tile_scene.clone());
//     }
// }

