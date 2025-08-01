use super::CharacterData;

pub const CARDINAL_DATA: CharacterData = CharacterData {
    name: "Cardinal",
    audio: [
        ("characters/cardinal-0.mp3", "Light shall heal and harm."),
        (
            "characters/cardinal-1.mp3",
            "Sacred power flows through me.",
        ),
        ("characters/cardinal-2.mp3", "I bring divine judgment."),
        (
            "characters/cardinal-greet.mp3",
            "Blessings upon you, traveler.",
        ),
    ],
    icon: (
        "characters/cardinal_icon.png",
        "characters/cardinal_icon_selected.png",
    ),
    portrait: "characters/cardinal.png",
    ability_1: (
        "Barrier",
        "Applies a defense and barrier on the nearest round robin hero within a 8x8 grid space 5s cooldown",
    ),
    ability_2: (
        "Beam",
        "Fires a straight beam 1x8 grid that hurts foes. Cannot move while casting",
    ),
    ability_3: (
        "Heal",
        "Applies a heal nearest and weakest hero within a 8x8 grid space 5s cooldown",
    ),
    ability_4: (
        "Resurrect",
        "Resurrects any nearby hero within 4x4 grid 1min cooldown",
    ),
};
