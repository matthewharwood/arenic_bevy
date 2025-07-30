use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Battleground;

impl Battleground {
    pub const MAX_ARENAS: usize = 9;
}
