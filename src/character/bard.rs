use super::CharacterData;

pub const BARD_DATA: CharacterData = CharacterData {
    name: "Bard",
    audio: [
        ("character/bard-0.mp3", "Let the music guide us."),
        ("character/bard-1.mp3", "Together we are unstoppable."),
        ("character/bard-2.mp3", "I'll amplify your strength."),
        ("character/bard-greet.mp3", "Ah, a new face! Let me play you the song of our people."),
    ],
    icon: ("characters/bard_icon.png", "characters/bard_icon_selected.png"),
    portrait: "characters/bard.png",
    ability_1: ("Cleanse", "Removes any debuffs from any heros within a 4x4 grid"),
    ability_2: ("Dance", "Quick Time Event sequence of keyclicks"),
    ability_3: ("Helix", "haste or heal"),
    ability_4: ("Mimic", "Bard will instant cast mimic the previous damage spell casted by adjacent hero"),
};