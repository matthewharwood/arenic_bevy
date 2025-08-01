use crate::character::Character;
use crate::utils::AudioClips;
use bevy::prelude::Component;

#[derive(Component, Default)]
pub struct CharacterWarrior;

impl Character for CharacterWarrior {
    const CLASS_NAME: &'static str = "Warrior";
    const AUDIO: AudioClips<4> = [
        ("character/warrior-0.mp3", "My shield protects all."),
        ("character/warrior-1.mp3", "Stand behind me!"),
        ("character/warrior-2.mp3", "Honor guides my blade."),
        ("character/warrior-greet.mp3", "Hail, friend!"),
    ];
    const ICON: (&'static str, &'static str) = ("characters/warrior_icon.png", "characters/warrior_icon_selected.png");
    const PORTRAIT: &'static str = "characters/warrior.png";
    const ABILITY_1: (&'static str, &'static str) = ("Block", "Brings out shield to reflect any projectiles that come at the Tank");
    const ABILITY_2: (&'static str, &'static str) = ("Bash", "Raises a spike shield for next attack to thrust at an enemy.");
    const ABILITY_3: (&'static str, &'static str) = ("Taunt", "Forces nearby enemies in a 2x2 to direct projectiles towards the Tank");
    const ABILITY_4: (&'static str, &'static str) = ("Bulwark", "Instantly raises a frontal barrier That absorbs all projectiles and cone attacks");
}
