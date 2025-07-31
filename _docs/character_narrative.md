Hey Jon, Please do this data entry work source code of each `src/character/*.rs` from my data below.

## Context

in `src/utils/mod.rs` i've made a generic audio type alas:

```rust
type AudioPath = &'static str;
type AudioCaption = &'static str;
type AudioClip = (AudioPath, AudioCaption);
pub type AudioClips<const N: usize> = [AudioClip; N];
```

in `src/characters/mod.rs`

```rust
pub trait Character {
    const AUDIO: AudioClips<4>;
}
```

for each character type in the `src/character/*.rs` I want you to apply this Character Trait and fill in the data

```rust
impl Character for CharacterAlchemist {
    const AUDIO: AudioClips<4> = [("character/hunter-0.mp3", "Patience. Precision. Perfection."), ("character/hunter-1.mp3", "The hunt begins now"), ("character/hunter-2.mp3", "My traps never miss."), ("character/hunter-greet.mp3", "Tracking prey? Or tracking treasure? Either way, I never miss.")];
}
```

## Data Notes:

- The data will have a markdown pattern of `# ClassName` and you'll use that to match the file path and ECS
  ComponentName
- The 0 to 2nd indexes are always numbered in the type `AudioPath` `1.` shall be `-0` and `2.` shall be `-1` and `3.`
  shall be `-2`.
- The 3rd index or "4." is always suffixed in `-greet` in the type `AudioPath`.
- The text AFTER the bulletpoint number e.g. `1.` or `2.` or `3.` is the type `AudioCaption`

---

# Data

## Hunter

1. Patience. Precision. Perfection."
2. "The hunt begins now."
3. "My traps never miss."
4. "Tracking prey? Or tracking treasure? Either way, I never miss."

## Alchemist

1. "Science conquers all."
2. "Every element serves me."
3. "Behold the power of transmutation."
4. "Need a Potion? A Transmutation?"

## Bard

1. "Let the music guide us."
2. "Together we are unstoppable."
3. "I'll amplify your strength."
4. "Ah, a new face! Let me play you the song of our people."

## Forager

1. "From earth comes power."
2. "Nature provides everything."
3. "I'll dig deep for victory."
4. "The Earth whispers Secrets to those who listen."

## Warrior

1. "My shield protects all."
2. "Stand behind me!"
3. "Honor guides my blade."
4. "Hail, friend!"

## Cardinal

1. "Light shall heal and harm."
2. "Sacred power flows through me."
3. "I bring divine judgment."
4. "Blessings upon you, traveler."

## Merchant

1. "Fortune favors the bold."
2. "Everything has a price."
3. "Luck is my greatest weapon."
4. "Every deal's a gamble, but I always win."

## Thief

1. "Shadows hide my strikes."
2. "You'll never see me coming."
3. "Quick and silent death."
4. "Oh, you actually saw me?"