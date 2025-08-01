use super::CharacterData;

pub const ALCHEMIST_DATA: CharacterData = CharacterData {
    name: "Alchemist",
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
    ability_2: (
        "Acid Flask",
        "Throws a bottle of acid onto a target tile, dealing damage over time to any enemy passing through. Their movement/attack schedule stays the same, but they suffer DOT.",
    ),
    ability_3: (
        "Transmute",
        "Converts an on-ground item or loot pile into a random useful resource. Doesn't interrupt enemies at all; it purely affects picked-up or environment items.",
    ),
    ability_4: ("Siphon", "Channel damage closes hero over time"),
};
