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
    ability_2: (
        "Bash",
        "Raises a spike shield for next attack to thrust at an enemy.",
    ),
    ability_3: (
        "Taunt",
        "Forces nearby enemies in a 2x2 to direct projectiles towards the Tank",
    ),
    ability_4: (
        "Bulwark",
        "Instantly raises a frontal barrier That absorbs all projectiles and cone attacks",
    ),
};
