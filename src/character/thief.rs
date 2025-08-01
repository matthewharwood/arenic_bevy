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
    ability_2: (
        "Backstab",
        "any positional move is an attack. Back attacks do more damage",
    ),
    ability_3: (
        "Pickpocket",
        "Lets you steal gold, buffs, or minor items from an enemy without interrupting its sequence. The enemy doesn't break stride; you just gain extra resources if you succeed.",
    ),
    ability_4: (
        "Shadow Step",
        "A forward dash that grants brief invulnerability. You pass through hazards, enemy spells, or boss patterns without altering the enemy's routine. Can result in a bump attack can be a backstab",
    ),
};
