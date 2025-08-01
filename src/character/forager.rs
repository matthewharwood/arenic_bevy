use crate::character::Character;
use crate::utils::AudioClips;
use bevy::prelude::Component;

#[derive(Component, Default)]
pub struct CharacterForager;

impl Character for CharacterForager {
    const CLASS_NAME: &'static str = "Forager";
    const AUDIO: AudioClips<4> = [
        ("character/forager-0.mp3", "From earth comes power."),
        ("character/forager-1.mp3", "Nature provides everything."),
        ("character/forager-2.mp3", "I'll dig deep for victory."),
        (
            "character/forager-greet.mp3",
            "The Earth whispers Secrets to those who listen.",
        ),
    ];
    const ICON: (&'static str, &'static str) = ("characters/forager_icon.png", "characters/forager_icon_selected.png");
    const PORTRAIT: &'static str = "characters/forager.png";
    const ABILITY_1: (&'static str, &'static str) = ("Border", "Places a 1x1 Border on the ground that will deflect any projectiles for the 1min requires -1 rock. Requires ground to be dug.");
    const ABILITY_2: (&'static str, &'static str) = ("Bolder", "Cast a bolder that can roll across the entire screen if unobstructed. requires -2 rocks from digging 2 grid spots");
    const ABILITY_3: (&'static str, &'static str) = ("Dig", "Dig up to 2 times on a grid square to gain loot and +1 rock");
    const ABILITY_4: (&'static str, &'static str) = ("Mushroom", "Plants a fast-growing mushroom on a tile. The first hero to touch the mushroom will get a heal. Requires ground to be dug.");
}
