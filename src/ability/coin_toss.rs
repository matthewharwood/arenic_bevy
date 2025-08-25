use bevy::prelude::Component;

/// Marker component for Coin Toss ability
#[derive(Component, Debug)]
pub struct CoinToss;

impl CoinToss {
    pub fn new() -> Self {
        Self
    }
}
