use bevy::prelude::Component;

#[derive(Component, Debug)]
struct Arena {
    name: String,
    max_characters: usize,
}
