use crate::arena::{CameraUpdate, CharacterMoved, LastActiveHero};
use crate::arena_camera::{position_camera_for_arena, ZoomOut, ZOOM};
use crate::character::Character;
use crate::materials::Materials;
use crate::selectors::Active;
use bevy::prelude::*;
use std::fmt::{Display, Formatter, Result as FmtResult};
use crate::timeline::TimelineError;

/// Arena names enum with explicit discriminants for all 9 arenas
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    const MAX_ARENAS: u8 = 9;
    
    /// Returns the arena's numeric index (0-8)
    #[must_use]
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
    
    /// Creates ArenaName from u8 index if valid (0-8)
    #[must_use]
    pub fn from_u8(idx: u8) -> Result<Self, TimelineError> {
        match idx {
            0 => Ok(Self::Labyrinth),
            1 => Ok(Self::GuildHouse),
            2 => Ok(Self::Sanctum),
            3 => Ok(Self::Mountain),
            4 => Ok(Self::Bastion),
            5 => Ok(Self::Pawnshop),
            6 => Ok(Self::Crucible),
            7 => Ok(Self::Casino),
            8 => Ok(Self::Gala),
            _ => Err(TimelineError::InvalidArenaIndex { index: idx }),
        }
    }
    
    /// Creates ArenaName with fallback to Labyrinth if invalid
    /// Use this for startup/initialization where you need guaranteed success
    #[must_use]
    pub fn from_u8_clamped(idx: u8) -> Self {
        Self::from_u8(idx.min(Self::MAX_ARENAS - 1)).unwrap_or(Self::Labyrinth)
    }
    
    /// Iterator over all arena names in order
    pub fn all() -> impl Iterator<Item = Self> {
        [
            Self::Labyrinth,
            Self::GuildHouse,
            Self::Sanctum,
            Self::Mountain,
            Self::Bastion,
            Self::Pawnshop,
            Self::Crucible,
            Self::Casino,
            Self::Gala,
        ].into_iter()
    }
}

impl TryFrom<u8> for ArenaName {
    type Error = TimelineError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value)
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

/// Arena component using ArenaName enum instead of raw u8
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Arena(pub ArenaName);

impl Arena {
    /// Creates new Arena from ArenaName
    #[must_use]
    pub fn new(name: ArenaName) -> Self {
        Self(name)
    }
    
    /// Creates new Arena from u8 index if valid (0-8)
    #[must_use]
    pub fn from_u8(idx: u8) -> Result<Self, TimelineError> {
        Ok(Self(ArenaName::from_u8(idx)?))
    }
    
    /// Creates new Arena with fallback to Labyrinth if invalid
    /// Use this for startup/initialization where you need guaranteed success
    #[must_use]
    pub fn from_u8_clamped(idx: u8) -> Self {
        Self(ArenaName::from_u8_clamped(idx))
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

impl TryFrom<u8> for Arena {
    type Error = TimelineError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value)
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

#[derive(Component, Debug, Clone)]
pub struct CurrentArena(pub ArenaName);

impl CurrentArena {
    /// Increment arena cyclically (Labyrinth -> GuildHouse -> ... -> Gala -> Labyrinth)
    pub fn increment(arena: ArenaName) -> ArenaName {
        let next_idx = (arena.as_u8() + 1) % 9;
        ArenaName::from_u8_clamped(next_idx)
    }

    /// Decrement arena cyclically (Gala -> Casino -> ... -> Labyrinth -> Gala)
    pub fn decrement(arena: ArenaName) -> ArenaName {
        let prev_idx = if arena.as_u8() == 0 { 8 } else { arena.as_u8() - 1 };
        ArenaName::from_u8_clamped(prev_idx)
    }

    /// Go to specific arena, clamps invalid values
    pub fn go_to(arena: ArenaName) -> ArenaName {
        arena
    }
    
    /// Get the arena's numeric index (0-8)
    pub fn as_u8(&self) -> u8 {
        self.0.as_u8()
    }
    
    /// Get the ArenaName enum value
    pub fn name(&self) -> ArenaName {
        self.0
    }
}

pub fn decrement_current_arena(
    keycode: Res<ButtonInput<KeyCode>>,
    current_arena_q: Single<&mut CurrentArena>,
    mut arena_refresh_event: EventWriter<CameraUpdate>,
) {
    if keycode.just_pressed(KeyCode::BracketLeft) {
        let mut current_arena = current_arena_q.into_inner();
        current_arena.0 = CurrentArena::decrement(current_arena.0);

        // Send event
        arena_refresh_event.write(CameraUpdate);
    }
}

pub fn increment_current_arena(
    keycode: Res<ButtonInput<KeyCode>>,
    current_arena_q: Single<&mut CurrentArena>,
    mut arena_refresh_event: EventWriter<CameraUpdate>,
) {
    if keycode.just_pressed(KeyCode::BracketRight) {
        let mut current_arena = current_arena_q.into_inner();
        current_arena.0 = CurrentArena::increment(current_arena.0);

        // Send event
        arena_refresh_event.write(CameraUpdate);
    }
}

pub fn handle_character_moved(
    mut commands: Commands,
    mut character_moved_events: EventReader<CharacterMoved>,
    current_arena_q: Single<&CurrentArena>,
    camera: Single<(Entity, &mut Transform, Option<&ZoomOut>), With<Camera3d>>,
    arena_q: Query<(Entity, &Arena), With<Arena>>,
) {
    // Only process if there are events to handle
    if character_moved_events.is_empty() {
        return;
    }
    
    let current_arena = current_arena_q.into_inner();
    let (camera_entity, mut camera_transform, zoom) = camera.into_inner();
    
    // Handle character movement between arenas
    for event in character_moved_events.read() {
        // Only update camera if we're not zoomed out and the target arena is the current arena
        if zoom.is_none() && event.to_arena == current_arena.name() {
            // Update camera position to the new arena
            position_camera_for_arena(&mut camera_transform, event.to_arena.as_u8(), ZOOM.0);
            commands.entity(camera_entity).remove::<ZoomOut>();
        }
        
        // Update LastActiveHero for the target arena
        if let Some((arena_entity, _)) = arena_q.iter().find(|(_, arena)| arena.name() == event.to_arena) {
            commands.entity(arena_entity).insert(LastActiveHero(Some(event.character_entity)));
            println!("Updated LastActiveHero for {} to {:?}", event.to_arena, event.character_entity);
        }
        
        println!("Character {:?} moved from {} to {} (preserved Active status)", 
                 event.character_entity, event.from_arena, event.to_arena);
    }
}

pub fn arena_update(
    mut commands: Commands,
    mut arena_refresh_events: EventReader<CameraUpdate>,
    current_arena_q: Single<&CurrentArena>,
    camera: Single<(Entity, &mut Transform, Option<&ZoomOut>), With<Camera3d>>,
    arena_q: Query<(Entity, &Arena, &Children, Option<&LastActiveHero>), With<Arena>>,
    characters_q: Query<(Entity, Option<&Active>), With<Character>>,
    mats: Res<Materials>,
) {
    // Only run when CameraUpdate event is triggered
    if arena_refresh_events.is_empty() {
        return;
    }
    arena_refresh_events.clear();
    let current_arena = current_arena_q.into_inner();
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
        let current_arena_name = current_arena.0;
        position_camera_for_arena(&mut camera_transform, current_arena_name.as_u8(), ZOOM.0);
        commands.entity(camera_entity).remove::<ZoomOut>();

        for (arena_entity, arena, children, last_active_hero) in arena_q.iter() {
            if arena.0 == current_arena_name {
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
                    println!(
                        "Found {} characters in {}",
                        characters_data.len(),
                        arena
                    );

                    // Find the active character (if any)
                    let active_character = characters_data
                        .iter()
                        .find(|(_, active)| active.is_some())
                        .map(|(entity, _)| *entity);

                    if let Some(active_entity) = active_character {
                        println!(
                            "Found active character in {}: {:?}",
                            arena, active_entity
                        );
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
                                .entity(arena_entity)
                                .insert(LastActiveHero(Some(first_character)));
                        }
                    }
                }
            }
        }
    }
}
