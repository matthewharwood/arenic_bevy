use super::CharacterData;

pub const ALCHEMIST_DATA: CharacterData = CharacterData {
    name: "Alchemist",
    default_name: "Giuseppe",
    audio: [
        ("characters/alchemist-0.mp3", "Science conquers all."),
        ("characters/alchemist-1.mp3", "Every element serves me."),
        (
            "characters/alchemist-2.mp3",
            "Behold the power of transmutation.",
        ),
        (
            "characters/alchemist-greet.mp3",
            "Need a Potion? A Transmutation?",
        ),
    ],
    icon: (
        "characters/alchemist_icon.png",
        "characters/alchemist_icon_selected.png",
    ),
    portrait: "characters/alchemist.png",
    ability_1: (
        "Ironskin Draft",
        "Quickly drinks a concoction, granting increased defense for a short time. Enemies carry on as normal, but their attacks hurt you less during this window.",
    ),
};
