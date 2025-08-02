use super::CharacterData;

pub const FORAGER_DATA: CharacterData = CharacterData {
    name: "Forager",
    default_name: "Daisy",
    audio: [
        ("characters/forager-0.mp3", "From earth comes power."),
        ("characters/forager-1.mp3", "Nature provides everything."),
        ("characters/forager-2.mp3", "I'll dig deep for victory."),
        (
            "characters/forager-greet.mp3",
            "The Earth whispers Secrets to those who listen.",
        ),
    ],
    icon: (
        "characters/forager_icon.png",
        "characters/forager_icon_selected.png",
    ),
    portrait: "characters/forager.png",
    ability_1: (
        "Border",
        "Places a 1x1 Border on the ground that will deflect any projectiles for the 1min requires -1 rock. Requires ground to be dug.",
    ),
};
