use super::CharacterData;

pub const WARRIOR_DATA: CharacterData = CharacterData {
    name: "Warrior",
    default_name: "King",
    audio: [
        ("characters/warrior-0.mp3", "My shield protects all."),
        ("characters/warrior-1.mp3", "Stand behind me!"),
        ("characters/warrior-2.mp3", "Honor guides my blade."),
        ("characters/warrior-greet.mp3", "Hail, friend!"),
    ],
    icon: (
        "characters/warrior_icon.png",
        "characters/warrior_icon_selected.png",
    ),
    portrait: "characters/warrior.png",
    ability_1: (
        "Block",
        "Brings out shield to reflect any projectiles that come at the Tank",
    ),
};
