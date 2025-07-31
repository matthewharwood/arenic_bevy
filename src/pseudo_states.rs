use bevy::prelude::Component;

#[derive(Component)]
pub struct Selected;
#[derive(Component)]
pub struct Focused;

#[derive(Component)]
pub struct Hovered;

#[derive(Component)]
pub struct Active;

#[derive(Component)]
pub struct Highlighted;
#[derive(Component)]
pub struct Checked;

#[derive(Component)]
pub struct Disabled;

/// used for loading or pending states.. it's like disabled but timebased
#[derive(Component)]
pub struct Indeterminate;

#[derive(Component)]
pub struct Enabled;

#[derive(Component)]
pub struct Required;

#[derive(Component)]
pub struct Valid;

#[derive(Component)]
pub struct Invalid;
