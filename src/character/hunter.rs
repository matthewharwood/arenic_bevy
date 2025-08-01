use super::CharacterData;

pub const HUNTER_DATA: CharacterData = CharacterData {
    name: "Hunter",
    audio: [
        (
            "characters/hunter-0.mp3",
            "Patience. Precision. Perfection.",
        ),
        ("characters/hunter-1.mp3", "The hunt begins now."),
        ("characters/hunter-2.mp3", "My traps never miss."),
        (
            "characters/hunter-greet.mp3",
            "Tracking prey? Or tracking treasure? Either way, I never miss.",
        ),
    ],
    icon: (
        "characters/hunter_icon.png",
        "characters/hunter_icon_selected.png",
    ),
    portrait: "characters/hunter.png",
    ability_1: (
        "Poison Shot",
        "A shot that pushes hero back one prev square does DOT Damage for next 20s with 12s cooldown.",
    ),
    ability_2: ("Auto Shot", "Gun will auto fire against closest enemy"),
    ability_3: (
        "Trap",
        "Lays a trap down on current grid space. If enemy touch it small AOE explosion 2x2.",
    ),
    ability_4: (
        "Sniper",
        "Fires any distance always at the boss 4 second cd, instant cast",
    ),
};
