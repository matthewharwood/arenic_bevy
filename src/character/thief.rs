use super::CharacterData;

pub const THIEF_DATA: CharacterData = CharacterData {
    name: "Thief",
    default_name: "Ginger",
    audio: [
        ("characters/thief-0.mp3", "Shadows hide my strikes."),
        ("characters/thief-1.mp3", "You'll never see me coming."),
        ("characters/thief-2.mp3", "Quick and silent death."),
        ("characters/thief-greet.mp3", "Oh, you actually saw me?"),
    ],
    icon: (
        "characters/thief_icon.png",
        "characters/thief_icon_selected.png",
    ),
    portrait: "characters/thief.png",
    ability_1: (
        "Smoke Screen",
        "Any hero in the smoke screen can walk through enemies without taking damage",
    ),
};
