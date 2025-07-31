use bevy::prelude::*;

// Module declarations for each boss type
pub mod alchemist;
pub mod cardinal;
pub mod collector;
pub mod gatherer;
pub mod guild_master;
pub mod bard;
pub mod tank;
pub mod thief;
pub mod trapper;

#[derive(Component)]
pub struct BossAnimationConfig {
    pub first_frame: usize,
    pub last_frame: usize,
    pub timer: Timer,
}

pub trait Boss {
    const NAME: &'static str;
    const TEXTURE_PATH: &'static str;
    const FRAME_COUNT: usize = 14;
    const FRAME_WIDTH: u32 = 115;
    const FRAME_HEIGHT: u32 = 115;
    const ANIMATION_FPS: f32 = 10.0;

    fn animation_config() -> BossAnimationConfig {
        BossAnimationConfig {
            first_frame: 0,
            last_frame: Self::FRAME_COUNT - 1,
            timer: Timer::from_seconds(1.0 / Self::ANIMATION_FPS, TimerMode::Repeating),
        }
    }

    fn create_atlas_layout() -> TextureAtlasLayout {
        TextureAtlasLayout::from_grid(
            UVec2::new(Self::FRAME_WIDTH, Self::FRAME_HEIGHT),
            Self::FRAME_COUNT as u32,
            1,
            None,
            None,
        )
    }
}
