use crate::character::Character;
use crate::utils::AudioClips;
use bevy::prelude::Component;

#[derive(Component, Default)]
pub struct CharacterAlchemist;

impl Character for CharacterAlchemist {
    const CLASS_NAME: &'static str = "Alchemist";
    const AUDIO: AudioClips<4> = [
        ("character/alchemist-0.mp3", "Science conquers all."),
        ("character/alchemist-1.mp3", "Every element serves me."),
        (
            "character/alchemist-2.mp3",
            "Behold the power of transmutation.",
        ),
        (
            "character/alchemist-greet.mp3",
            "Need a Potion? A Transmutation?",
        ),
    ];
    const ICON: (&'static str, &'static str) = ("characters/alchemist_icon.png", "characters/alchemist_icon_selected.png");
    const PORTRAIT: &'static str = "characters/alchemist.png";
    const ABILITY_1: (&'static str, &'static str) = ("Ironskin Draft", "Quickly drinks a concoction, granting increased defense for a short time. Enemies carry on as normal, but their attacks hurt you less during this window.");
    const ABILITY_2: (&'static str, &'static str) = ("Acid Flask", "Throws a bottle of acid onto a target tile, dealing damage over time to any enemy passing through. Their movement/attack schedule stays the same, but they suffer DOT.");
    const ABILITY_3: (&'static str, &'static str) = ("Transmute", "Converts an on-ground item or loot pile into a random useful resource. Doesn't interrupt enemies at all; it purely affects picked-up or environment items.");
    const ABILITY_4: (&'static str, &'static str) = ("Siphon", "Channel damage closes hero over time");
}
