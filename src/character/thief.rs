use crate::character::Character;
use crate::utils::AudioClips;
use bevy::prelude::Component;

#[derive(Component, Default)]
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
    const ABILITY_1: (&'static str, &'static str) = ("Smoke Screen", "Any hero in the smoke screen can walk through enemies without taking damage");
    const ABILITY_2: (&'static str, &'static str) = ("Backstab", "any positional move is an attack. Back attacks do more damage");
    const ABILITY_3: (&'static str, &'static str) = ("Pickpocket", "Lets you steal gold, buffs, or minor items from an enemy without interrupting its sequence. The enemy doesn't break stride; you just gain extra resources if you succeed.");
    const ABILITY_4: (&'static str, &'static str) = ("Shadow Step", "A forward dash that grants brief invulnerability. You pass through hazards, enemy spells, or boss patterns without altering the enemy's routine. Can result in a bump attack can be a backstab");
}
