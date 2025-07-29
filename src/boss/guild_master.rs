use super::Boss;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct GuildMaster;

impl Boss for GuildMaster {
    const NAME: &'static str = "The Guild Master";
    const TEXTURE_PATH: &'static str = "bosses/guild_master.png";
    const ANIMATION_FPS: f32 = 10.0;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_guild_master_constants() {
        assert_eq!(GuildMaster::NAME, "The Guild Master");
        assert_eq!(GuildMaster::TEXTURE_PATH, "bosses/guild_master.png");
        assert_eq!(GuildMaster::ANIMATION_FPS, 10.0);
    }
    #[test]
    fn test_animation_config() {
        let config = GuildMaster::animation_config();
        assert_eq!(config.first_frame, 0);
        assert_eq!(config.last_frame, 14);
        assert_eq!(config.timer.duration().as_secs_f32(), 0.1);
    }
}
