use crate::character::Character;
use crate::utils::AudioClips;
use bevy::prelude::Component;

#[derive(Component, Debug, Default)]
pub struct CharacterHunter;

impl Character for CharacterHunter {
    const CLASS_NAME: &'static str = "Hunter";
    const AUDIO: AudioClips<4> = [
        ("character/hunter-0.mp3", "Patience. Precision. Perfection."),
        ("character/hunter-1.mp3", "The hunt begins now."),
        ("character/hunter-2.mp3", "My traps never miss."),
        (
            "character/hunter-greet.mp3",
            "Tracking prey? Or tracking treasure? Either way, I never miss.",
        ),
    ];
    const ICON: (&'static str, &'static str) = ("characters/hunter_icon.png", "characters/hunter_icon_selected.png");
    const PORTRAIT: &'static str = "characters/hunter.png";
    const ABILITY_1: (&'static str, &'static str) = ("Poison Shot", "A shot that pushes hero back one prev square does DOT Damage for next 20s with 12s cooldown.");
    const ABILITY_2: (&'static str, &'static str) = ("Auto Shot", "Gun will auto fire against closest enemy");
    const ABILITY_3: (&'static str, &'static str) = ("Trap", "Lays a trap down on current grid space. If enemy touch it small AOE explosion 2x2.");
    const ABILITY_4: (&'static str, &'static str) = ("Sniper", "Fires any distance always at the boss 4 second cd, instant cast");
}
