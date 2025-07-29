use bevy::prelude::*;

// Module declarations for each boss type
mod alchemist;
mod cardinal;
mod collector;
mod gatherer;
mod guild_master;
mod sprinter;
mod tank;
mod thief;
mod trapper;

#[derive(Component)]
pub struct BossAnimationConfig {
    pub first_frame: usize,
    pub last_frame: usize,
    pub timer: Timer,
}

pub trait Boss {
    const NAME: &'static str;
    const TEXTURE_PATH: &'static str;
    const FRAME_COUNT: usize = 15;
    const FRAME_WIDTH: u32 = 64;
    const FRAME_HEIGHT: u32 = 64;
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
