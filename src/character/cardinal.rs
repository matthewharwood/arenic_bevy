use crate::character::Character;
use crate::utils::AudioClips;
use bevy::prelude::Component;

#[derive(Component, Default)]
pub struct CharacterCardinal;

impl Character for CharacterCardinal {
    const CLASS_NAME: &'static str = "Cardinal";
    const AUDIO: AudioClips<4> = [
        ("character/cardinal-0.mp3", "Light shall heal and harm."),
        ("character/cardinal-1.mp3", "Sacred power flows through me."),
        ("character/cardinal-2.mp3", "I bring divine judgment."),
        (
            "character/cardinal-greet.mp3",
            "Blessings upon you, traveler.",
        ),
    ];
    const ICON: (&'static str, &'static str) = ("characters/cardinal_icon.png", "characters/cardinal_icon_selected.png");
    const PORTRAIT: &'static str = "characters/cardinal.png";
    const ABILITY_1: (&'static str, &'static str) = ("Barrier", "Applies a defense and barrier on the nearest round robin hero within a 8x8 grid space 5s cooldown");
    const ABILITY_2: (&'static str, &'static str) = ("Beam", "Fires a straight beam 1x8 grid that hurts foes. Cannot move while casting");
    const ABILITY_3: (&'static str, &'static str) = ("Heal", "Applies a heal nearest and weakest hero within a 8x8 grid space 5s cooldown");
    const ABILITY_4: (&'static str, &'static str) = ("Resurrect", "Resurrects any nearby hero within 4x4 grid 1min cooldown");
}
