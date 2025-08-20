use crate::arena::{CameraUpdate, CharacterMoved, LastActiveHero};
use crate::arena_camera::{ZOOM, ZoomOut, position_camera_for_arena};
use crate::character::Character;
use crate::materials::Materials;
use crate::selectors::Active;
use bevy::prelude::*;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Arena names enum with explicit discriminants for all 9 arenas
/// This is a VALUE TYPE only - not a Component
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ArenaName {
    Labyrinth = 0,
    GuildHouse = 1,
    Sanctum = 2,
    Mountain = 3,
    Bastion = 4,
    Pawnshop = 5,
    Crucible = 6,
    Casino = 7,
    Gala = 8,
}

impl ArenaName {
    /// All arenas in index order for compile-time safe iteration
    pub const ALL_ARENAS: [Self; 9] = [
        Self::Labyrinth,
        Self::GuildHouse,
        Self::Sanctum,
        Self::Mountain,
        Self::Bastion,
        Self::Pawnshop,
        Self::Crucible,
        Self::Casino,
        Self::Gala,
    ];

    const MAX_ARENAS: u8 = 9;

    /// Returns the arena's numeric index (0-8)
    #[must_use]
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }

    /// Creates ArenaName from index with compile-time safety (clamps to valid range)
    /// Use this for internal code where we control the input
    #[must_use]
    pub fn from_index_safe(idx: u8) -> Self {
        let clamped_idx = idx.min(Self::MAX_ARENAS - 1);
        Self::ALL_ARENAS[clamped_idx as usize]
    }

    /// Iterator over all arena names in order
    pub fn all() -> impl Iterator<Item = Self> {
        Self::ALL_ARENAS.into_iter()
    }
}

impl Display for ArenaName {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Labyrinth => write!(f, "Labyrinth"),
            Self::GuildHouse => write!(f, "Guild House"),
            Self::Sanctum => write!(f, "Sanctum"),
            Self::Mountain => write!(f, "Mountain"),
            Self::Bastion => write!(f, "Bastion"),
            Self::Pawnshop => write!(f, "Pawnshop"),
            Self::Crucible => write!(f, "Crucible"),
            Self::Casino => write!(f, "Casino"),
            Self::Gala => write!(f, "Gala"),
        }
    }
}

/// Arena ID value type for passing arena identifiers in events and data
/// This is a VALUE TYPE only - not a Component
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ArenaId(pub ArenaName);

/// Arena component that marks arena entities
/// This is a COMPONENT TYPE only - for attaching to arena entities
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Arena(pub ArenaName);

impl ArenaId {
    /// Creates new ArenaId from ArenaName
    #[must_use]
    pub fn new(name: ArenaName) -> Self {
        Self(name)
    }

    /// Creates ArenaId from index with compile-time safety (clamps to valid range)
    /// Use this for internal code where we control the input
    #[must_use]
    pub fn from_index_safe(idx: u8) -> Self {
        Self(ArenaName::from_index_safe(idx))
    }

    /// Returns the arena's numeric index (0-8)
    #[must_use]
    pub fn as_u8(&self) -> u8 {
        self.0.as_u8()
    }

    /// Returns the ArenaName enum value
    #[must_use]
    pub fn name(&self) -> ArenaName {
        self.0
    }
}

impl Arena {
    /// Creates Arena from index with compile-time safety (clamps to valid range)
    /// Use this for internal code where we control the input
    #[must_use]
    pub fn from_index_safe(idx: u8) -> Self {
        Self(ArenaName::from_index_safe(idx))
    }

    /// Returns the arena's numeric index (0-8)
    #[must_use]
    pub fn as_u8(&self) -> u8 {
        self.0.as_u8()
    }
}

impl Display for ArenaId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{} ({})", self.0, self.as_u8())
    }
}

impl Display for Arena {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{} ({})", self.0, self.as_u8())
    }
}

/// Marker component for arena tile entities.
#[derive(Component, Debug)]
pub struct ArenaTile;

#[derive(Resource, Debug, Clone)]
pub struct CurrentArena(pub ArenaId);

/// O(1) arena entity lookup resource - eliminates linear searches
/// Maps ArenaName enum values (0-8) to their corresponding arena entities
#[derive(Resource, Debug)]
pub struct ArenaEntities {
    entities: [Entity; 9],
}

impl ArenaEntities {
    /// Creates new ArenaEntities from array of (ArenaName, Entity) pairs
    /// Uses ArenaName enum discriminants as indices for O(1) access
    pub fn new(arena_entities: [(ArenaName, Entity); 9]) -> Self {
        let mut entities = [Entity::PLACEHOLDER; 9];
        for (name, entity) in arena_entities {
            entities[name as usize] = entity;
        }
        Self { entities }
    }

    /// Get arena entity by name - O(1) lookup using enum discriminant as index
    #[must_use]
    pub fn get(&self, name: ArenaName) -> Entity {
        self.entities[name as usize]
    }

    /// Find which arena an entity belongs to - O(n) but only 9 entities max
    /// Returns None if entity is not found in any arena
    #[must_use]
    pub fn find_arena_for_entity(&self, entity: Entity) -> Option<ArenaName> {
        self.entities
            .iter()
            .position(|&e| e == entity)
            .map(|idx| ArenaName::from_index_safe(idx as u8))
    }
}

impl CurrentArena {
    /// Increment arena cyclically (Labyrinth -> GuildHouse -> ... -> Gala -> Labyrinth)
    pub fn increment(arena_id: ArenaId) -> ArenaId {
        let next_idx = (arena_id.as_u8() + 1) % 9;
        ArenaId::from_index_safe(next_idx)
    }

    /// Decrement arena cyclically (Gala -> Casino -> ... -> Labyrinth -> Gala)
    pub fn decrement(arena_id: ArenaId) -> ArenaId {
        let prev_idx = if arena_id.as_u8() == 0 {
            8
        } else {
            arena_id.as_u8() - 1
        };
        ArenaId::from_index_safe(prev_idx)
    }

    /// Get the arena's numeric index (0-8)
    pub fn as_u8(&self) -> u8 {
        self.0.as_u8()
    }

    /// Get the ArenaName enum value
    pub fn name(&self) -> ArenaName {
        self.0.name()
    }

    /// Get the ArenaId value type
    pub fn id(&self) -> ArenaId {
        self.0
    }
}

pub fn decrement_current_arena(
    keycode: Res<ButtonInput<KeyCode>>,
    mut current_arena: ResMut<CurrentArena>,
    mut arena_refresh_event: EventWriter<CameraUpdate>,
) {
    if keycode.just_pressed(KeyCode::BracketLeft) {
        current_arena.0 = CurrentArena::decrement(current_arena.0);

        // Send event
        arena_refresh_event.write(CameraUpdate);
    }
}

pub fn increment_current_arena(
    keycode: Res<ButtonInput<KeyCode>>,
    mut current_arena: ResMut<CurrentArena>,
    mut arena_refresh_event: EventWriter<CameraUpdate>,
) {
    if keycode.just_pressed(KeyCode::BracketRight) {
        current_arena.0 = CurrentArena::increment(current_arena.0);

        // Send event
        arena_refresh_event.write(CameraUpdate);
    }
}

pub fn handle_character_moved(
    mut commands: Commands,
    mut character_moved_events: EventReader<CharacterMoved>,
    current_arena: Res<CurrentArena>,
    camera: Single<(Entity, &mut Transform, Option<&ZoomOut>), With<Camera3d>>,
    arena_entities: Res<ArenaEntities>,
) {
    // Only process if there are events to handle
    if character_moved_events.is_empty() {
        return;
    }

    let current_arena = &*current_arena;
    let (camera_entity, mut camera_transform, zoom) = camera.into_inner();

    // Handle character movement between arenas
    for event in character_moved_events.read() {
        // Only update camera if we're not zoomed out and the target arena is the current arena
        if zoom.is_none() && event.to_arena.name() == current_arena.name() {
            // Update camera position to the new arena
            position_camera_for_arena(&mut camera_transform, event.to_arena.as_u8(), ZOOM.0);
            commands.entity(camera_entity).remove::<ZoomOut>();
        }

        // Update LastActiveHero for the target arena - O(1) lookup
        let arena_entity = arena_entities.get(event.to_arena.name());
        commands
            .entity(arena_entity)
            .insert(LastActiveHero(Some(event.character_entity)));
        println!(
            "Updated LastActiveHero for {} to {:?}",
            event.to_arena, event.character_entity
        );

        println!(
            "Character {:?} moved from {} to {} (preserved Active status)",
            event.character_entity, event.from_arena, event.to_arena
        );
    }
}

pub fn arena_update(
    mut commands: Commands,
    mut arena_refresh_events: EventReader<CameraUpdate>,
    current_arena: Res<CurrentArena>,
    camera: Single<(Entity, &mut Transform, Option<&ZoomOut>), With<Camera3d>>,
    arena_entities: Res<ArenaEntities>,
    arena_q: Query<(&Arena, &Children, Option<&LastActiveHero>), With<Arena>>,
    characters_q: Query<(Entity, Option<&Active>), With<Character>>,
    mats: Res<Materials>,
) {
    // Only run when CameraUpdate event is triggered
    if arena_refresh_events.is_empty() {
        return;
    }
    arena_refresh_events.clear();
    let current_arena = &*current_arena;
    let (camera_entity, mut camera_transform, zoom) = camera.into_inner();
    if zoom.is_some() {
        for (entity, active) in characters_q.iter() {
            if active.is_some() {
                commands
                    .entity(entity)
                    .insert(MeshMaterial3d(mats.gray.clone()))
                    .remove::<Active>();
            }
        }
    } else {
        position_camera_for_arena(&mut camera_transform, current_arena.as_u8(), ZOOM.0);
        commands.entity(camera_entity).remove::<ZoomOut>();

        // O(1) lookup for current arena entity
        let current_arena_entity = arena_entities.get(current_arena.name());

        // Direct query for the current arena - no iteration needed
        if let Ok((arena, children, last_active_hero)) = arena_q.get(current_arena_entity) {
            // Check if arena has characters
            let characters_data: Vec<(Entity, Option<&Active>)> =
                characters_q.iter_many(children).collect();

            if characters_data.is_empty() {
                println!("No characters in {}", arena);
                for (entity, active) in characters_q.iter() {
                    if active.is_some() {
                        commands
                            .entity(entity)
                            .insert(MeshMaterial3d(mats.gray.clone()))
                            .remove::<Active>();
                    }
                }
                return;
            } else {
                println!("Found {} characters in {}", characters_data.len(), arena);

                // Find the active character (if any)
                let active_character = characters_data
                    .iter()
                    .find(|(_, active)| active.is_some())
                    .map(|(entity, _)| *entity);

                if let Some(active_entity) = active_character {
                    println!("Found active character in {}: {:?}", arena, active_entity);
                    // This character is already active - handle this case
                    return;
                } else {
                    println!("No active character in {}", arena);

                    // Check if LastActiveHero has a character present and still exists
                    let use_last_active_hero =
                        last_active_hero
                            .and_then(|hero| hero.0)
                            .filter(|&hero_entity| {
                                characters_data
                                    .iter()
                                    .any(|(entity, _)| *entity == hero_entity)
                            });

                    if let Some(hero_entity) = use_last_active_hero {
                        println!("Using LastActiveHero: {:?}", hero_entity);
                        // Remove Active from all currently active characters across all arenas
                        for (entity, active) in characters_q.iter() {
                            if active.is_some() {
                                commands
                                    .entity(entity)
                                    .insert(MeshMaterial3d(mats.gray.clone()))
                                    .remove::<Active>();
                            }
                        }
                        // Now set the new active character
                        commands
                            .entity(hero_entity)
                            .insert(MeshMaterial3d(mats.blue.clone()))
                            .insert(Active);
                    } else {
                        println!("Using first character");
                        let first_character = characters_data[0].0;
                        // Remove Active from all currently active characters across all arenas
                        for (entity, active) in characters_q.iter() {
                            if active.is_some() {
                                commands
                                    .entity(entity)
                                    .insert(MeshMaterial3d(mats.gray.clone()))
                                    .remove::<Active>();
                            }
                        }
                        // Now set the new active character
                        commands
                            .entity(first_character)
                            .insert(MeshMaterial3d(mats.blue.clone()))
                            .insert(Active);
                        commands
                            .entity(current_arena_entity)
                            .insert(LastActiveHero(Some(first_character)));
                    }
                }
            }
        }
    }
}
