use crate::character::Character;
use crate::utils::AudioClips;
use bevy::prelude::Component;

#[derive(Component)]
pub struct CharacterThief;

impl Character for CharacterThief {
    const CLASS_NAME: &'static str = "Thief";
    const AUDIO: AudioClips<4> = [
        ("character/thief-0.mp3", "Shadows hide my strikes."),
        ("character/thief-1.mp3", "You'll never see me coming."),
        ("character/thief-2.mp3", "Quick and silent death."),
        ("character/thief-greet.mp3", "Oh, you actually saw me?"),
    ];
    const ICON: (&'static str, &'static str) = ("characters/thief_icon.png", "characters/thief_icon_selected.png");
    const PORTRAIT: &'static str = "characters/thief.png";
}
