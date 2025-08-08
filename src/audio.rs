// audio.rs
use bevy::prelude::*;

#[derive(Resource)]
pub struct Audio {
    pub autoshot: Handle<AudioSource>,
    pub holy_nova: Handle<AudioSource>,
}

impl Audio {
    pub fn new(asset_server: &Res<AssetServer>) -> Self {
        Self {
            autoshot: asset_server.load("abilities/autoshot.mp3"),
            holy_nova: asset_server.load("abilities/holy_nova.mp3"),
        }
    }
}