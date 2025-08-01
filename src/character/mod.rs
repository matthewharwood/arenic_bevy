use crate::utils::AudioClips;
use bevy::prelude::*;

/// Data structure containing all static information for a character type
#[derive(Clone, Copy, Debug)]
pub struct CharacterData {
    pub name: &'static str,
    pub audio: AudioClips<4>,
    pub icon: (&'static str, &'static str),
    pub portrait: &'static str,
    pub ability_1: (&'static str, &'static str),
    pub ability_2: (&'static str, &'static str),
    pub ability_3: (&'static str, &'static str),
    pub ability_4: (&'static str, &'static str),
}

/// Unified character system using enum-based architecture
#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum CharacterType {
    Warrior,
    #[default]
    Hunter,
    Thief,
    Alchemist,
    Bard,
    Cardinal,
    Forager,
    Merchant,
}

/// Const lookup table containing all character data
const CHARACTER_DATA: [CharacterData; 8] = [
    // Warrior
    CharacterData {
        name: "Warrior",
        audio: [
            ("character/warrior-0.mp3", "My shield protects all."),
            ("character/warrior-1.mp3", "Stand behind me!"),
            ("character/warrior-2.mp3", "Honor guides my blade."),
            ("character/warrior-greet.mp3", "Hail, friend!"),
        ],
        icon: ("characters/warrior_icon.png", "characters/warrior_icon_selected.png"),
        portrait: "characters/warrior.png",
        ability_1: ("Block", "Brings out shield to reflect any projectiles that come at the Tank"),
        ability_2: ("Bash", "Raises a spike shield for next attack to thrust at an enemy."),
        ability_3: ("Taunt", "Forces nearby enemies in a 2x2 to direct projectiles towards the Tank"),
        ability_4: ("Bulwark", "Instantly raises a frontal barrier That absorbs all projectiles and cone attacks"),
    },
    // Hunter
    CharacterData {
        name: "Hunter",
        audio: [
            ("character/hunter-0.mp3", "Patience. Precision. Perfection."),
            ("character/hunter-1.mp3", "The hunt begins now."),
            ("character/hunter-2.mp3", "My traps never miss."),
            ("character/hunter-greet.mp3", "Tracking prey? Or tracking treasure? Either way, I never miss."),
        ],
        icon: ("characters/hunter_icon.png", "characters/hunter_icon_selected.png"),
        portrait: "characters/hunter.png",
        ability_1: ("Poison Shot", "A shot that pushes hero back one prev square does DOT Damage for next 20s with 12s cooldown."),
        ability_2: ("Auto Shot", "Gun will auto fire against closest enemy"),
        ability_3: ("Trap", "Lays a trap down on current grid space. If enemy touch it small AOE explosion 2x2."),
        ability_4: ("Sniper", "Fires any distance always at the boss 4 second cd, instant cast"),
    },
    // Thief
    CharacterData {
        name: "Thief",
        audio: [
            ("character/thief-0.mp3", "Shadows hide my strikes."),
            ("character/thief-1.mp3", "You'll never see me coming."),
            ("character/thief-2.mp3", "Quick and silent death."),
            ("character/thief-greet.mp3", "Oh, you actually saw me?"),
        ],
        icon: ("characters/thief_icon.png", "characters/thief_icon_selected.png"),
        portrait: "characters/thief.png",
        ability_1: ("Smoke Screen", "Any hero in the smoke screen can walk through enemies without taking damage"),
        ability_2: ("Backstab", "any positional move is an attack. Back attacks do more damage"),
        ability_3: ("Pickpocket", "Lets you steal gold, buffs, or minor items from an enemy without interrupting its sequence. The enemy doesn't break stride; you just gain extra resources if you succeed."),
        ability_4: ("Shadow Step", "A forward dash that grants brief invulnerability. You pass through hazards, enemy spells, or boss patterns without altering the enemy's routine. Can result in a bump attack can be a backstab"),
    },
    // Alchemist
    CharacterData {
        name: "Alchemist",
        audio: [
            ("character/alchemist-0.mp3", "Science conquers all."),
            ("character/alchemist-1.mp3", "Every element serves me."),
            ("character/alchemist-2.mp3", "Behold the power of transmutation."),
            ("character/alchemist-greet.mp3", "Need a Potion? A Transmutation?"),
        ],
        icon: ("characters/alchemist_icon.png", "characters/alchemist_icon_selected.png"),
        portrait: "characters/alchemist.png",
        ability_1: ("Ironskin Draft", "Quickly drinks a concoction, granting increased defense for a short time. Enemies carry on as normal, but their attacks hurt you less during this window."),
        ability_2: ("Acid Flask", "Throws a bottle of acid onto a target tile, dealing damage over time to any enemy passing through. Their movement/attack schedule stays the same, but they suffer DOT."),
        ability_3: ("Transmute", "Converts an on-ground item or loot pile into a random useful resource. Doesn't interrupt enemies at all; it purely affects picked-up or environment items."),
        ability_4: ("Siphon", "Channel damage closes hero over time"),
    },
    // Bard
    CharacterData {
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
    },
    // Cardinal
    CharacterData {
        name: "Cardinal",
        audio: [
            ("character/cardinal-0.mp3", "Light shall heal and harm."),
            ("character/cardinal-1.mp3", "Sacred power flows through me."),
            ("character/cardinal-2.mp3", "I bring divine judgment."),
            ("character/cardinal-greet.mp3", "Blessings upon you, traveler."),
        ],
        icon: ("characters/cardinal_icon.png", "characters/cardinal_icon_selected.png"),
        portrait: "characters/cardinal.png",
        ability_1: ("Barrier", "Applies a defense and barrier on the nearest round robin hero within a 8x8 grid space 5s cooldown"),
        ability_2: ("Beam", "Fires a straight beam 1x8 grid that hurts foes. Cannot move while casting"),
        ability_3: ("Heal", "Applies a heal nearest and weakest hero within a 8x8 grid space 5s cooldown"),
        ability_4: ("Resurrect", "Resurrects any nearby hero within 4x4 grid 1min cooldown"),
    },
    // Forager
    CharacterData {
        name: "Forager",
        audio: [
            ("character/forager-0.mp3", "From earth comes power."),
            ("character/forager-1.mp3", "Nature provides everything."),
            ("character/forager-2.mp3", "I'll dig deep for victory."),
            ("character/forager-greet.mp3", "The Earth whispers Secrets to those who listen."),
        ],
        icon: ("characters/forager_icon.png", "characters/forager_icon_selected.png"),
        portrait: "characters/forager.png",
        ability_1: ("Border", "Places a 1x1 Border on the ground that will deflect any projectiles for the 1min requires -1 rock. Requires ground to be dug."),
        ability_2: ("Bolder", "Cast a bolder that can roll across the entire screen if unobstructed. requires -2 rocks from digging 2 grid spots"),
        ability_3: ("Dig", "Dig up to 2 times on a grid square to gain loot and +1 rock"),
        ability_4: ("Mushroom", "Plants a fast-growing mushroom on a tile. The first hero to touch the mushroom will get a heal. Requires ground to be dug."),
    },
    // Merchant
    CharacterData {
        name: "Merchant",
        audio: [
            ("character/merchant-0.mp3", "Fortune favors the bold."),
            ("character/merchant-1.mp3", "Everything has a price."),
            ("character/merchant-2.mp3", "Luck is my greatest weapon."),
            ("character/merchant-greet.mp3", "Every deal's a gamble, but I always win."),
        ],
        icon: ("characters/merchant_icon.png", "characters/merchant_icon_selected.png"),
        portrait: "characters/merchant.png",
        ability_1: ("Dice", "Stackable 1% Chance to increase Critical strike on yourself for next attack. instant cast stacks"),
        ability_2: ("Coin Toss", "Spend money to give a skill shot. Big damage if hits, get money back. Up to 5s cast time. 10second cd"),
        ability_3: ("Fortune", "Aura: The merchant and any adjacent hero's will have a luck % increase to gain gold on offensive attack (luck stacks)"),
        ability_4: ("TBD", "4x4 grid that 2x Crit damage. 10 secs and 30s cd. (Stacks with other merchant)"),
    },
];

impl CharacterType {
    /// Convert enum to index for array lookup
    const fn index(self) -> usize {
        self as usize
    }
    
    /// Get all character data for this type
    pub const fn data(self) -> &'static CharacterData {
        &CHARACTER_DATA[self.index()]
    }
    
    /// Get character class name
    pub const fn class_name(self) -> &'static str {
        self.data().name
    }

    /// Get character audio clips
    pub const fn audio(self) -> AudioClips<4> {
        self.data().audio
    }

    /// Get character icon paths (normal, selected)
    pub const fn icon(self) -> (&'static str, &'static str) {
        self.data().icon
    }

    /// Get character portrait path
    pub const fn portrait(self) -> &'static str {
        self.data().portrait
    }

    /// Get ability 1 (name, description)
    pub const fn ability_1(self) -> (&'static str, &'static str) {
        self.data().ability_1
    }

    /// Get ability 2 (name, description)
    pub const fn ability_2(self) -> (&'static str, &'static str) {
        self.data().ability_2
    }

    /// Get ability 3 (name, description)
    pub const fn ability_3(self) -> (&'static str, &'static str) {
        self.data().ability_3
    }

    /// Get ability 4 (name, description)
    pub const fn ability_4(self) -> (&'static str, &'static str) {
        self.data().ability_4
    }

    /// Returns all character types in the order they should appear in the UI
    pub fn all() -> [CharacterType; 8] {
        [
            CharacterType::Warrior,
            CharacterType::Hunter,
            CharacterType::Thief,
            CharacterType::Alchemist,
            CharacterType::Bard,
            CharacterType::Cardinal,
            CharacterType::Forager,
            CharacterType::Merchant,
        ]
    }
}
