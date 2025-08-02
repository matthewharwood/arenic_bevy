use super::CharacterData;

pub const HUNTER_DATA: CharacterData = CharacterData {
    name: "Hunter",
    default_name: "Dean",
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
        "Auto Shot",
        "Gun will auto fire against closest enemy within a 3x3 area",
    ),
};
