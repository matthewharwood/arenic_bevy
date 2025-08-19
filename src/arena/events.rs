use bevy::prelude::*;

#[derive(Event, Debug, Clone)]
pub struct CameraUpdate;

#[derive(Event, Debug, Clone)]
pub struct CharacterMoved {
    pub character_entity: Entity,
    pub from_arena: crate::arena::ArenaName,
    pub to_arena: crate::arena::ArenaName,
}
