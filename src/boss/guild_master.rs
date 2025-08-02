use super::Boss;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct GuildMaster;

impl Boss for GuildMaster {
    const TEXTURE_PATH: &'static str = "bosses/guild_master.png";
    const FRAME_WIDTH: u32 = 115; // 4x export
    const FRAME_HEIGHT: u32 = 115; // 4x export
    const ANIMATION_FPS: f32 = 10.0;
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_guild_master_constants() {
        assert_eq!(GuildMaster::TEXTURE_PATH, "bosses/guild_master.png");
        assert_eq!(GuildMaster::ANIMATION_FPS, 10.0);
    }
    #[test]
    fn test_animation_config() {
        let config = GuildMaster::animation_config();
        assert_eq!(config.first_frame, 0);
        assert_eq!(config.last_frame, 13);
        assert_eq!(config.timer.duration().as_secs_f32(), 0.1);
        assert!(matches!(config.timer.mode(), TimerMode::Repeating));
    }
    #[test]
    fn test_atlas_layout_math() {
        let layout = GuildMaster::create_atlas_layout();

        assert_eq!(layout.textures.len(), 14);

        let frame_0 = &layout.textures[0];
        assert_eq!(frame_0.min, UVec2::new(0, 0));
        assert_eq!(frame_0.max, UVec2::new(115, 115));
        let frame_7 = &layout.textures[7];
        assert_eq!(frame_7.min, UVec2::new(115 * 7, 0));
        assert_eq!(frame_7.max, UVec2::new(115 * 8, 115));
        assert_eq!(layout.size, UVec2::new(1610, 115));
    }
}
