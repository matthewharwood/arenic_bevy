use super::CharacterData;

pub const MERCHANT_DATA: CharacterData = CharacterData {
    name: "Merchant",
    default_name: "Calvin",
    audio: [
        ("characters/merchant-0.mp3", "Fortune favors the bold."),
        ("characters/merchant-1.mp3", "Everything has a price."),
        ("characters/merchant-2.mp3", "Luck is my greatest weapon."),
        (
            "characters/merchant-greet.mp3",
            "Every deal's a gamble, but I always win.",
        ),
    ],
    icon: (
        "characters/merchant_icon.png",
        "characters/merchant_icon_selected.png",
    ),
    portrait: "characters/merchant.png",
    ability_1: (
        "Dice",
        "Stackable 1% Chance to increase Critical strike on yourself for next attack. instant cast stacks",
    ),
};
