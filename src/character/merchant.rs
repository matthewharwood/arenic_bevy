use super::CharacterData;

pub const MERCHANT_DATA: CharacterData = CharacterData {
    name: "Merchant",
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
    ability_2: (
        "Coin Toss",
        "Spend money to give a skill shot. Big damage if hits, get money back. Up to 5s cast time. 10second cd",
    ),
    ability_3: (
        "Fortune",
        "Aura: The merchant and any adjacent hero's will have a luck % increase to gain gold on offensive attack (luck stacks)",
    ),
    ability_4: (
        "TBD",
        "4x4 grid that 2x Crit damage. 10 secs and 30s cd. (Stacks with other merchant)",
    ),
};
