I have this type:

type AudioPath = &'static str;
type AudioCaption = &'static str;
type AudioClip = (AudioPath, AudioCaption);
pub type AudioClips<const N: usize> = [AudioClip; N];

const AUDIO: AudioClips<
4> = [("character/hunter-0.mp3", "Patience. Precision. Perfection."), ("character/hunter-1.mp3", "The hunt begins now"), ("character/hunter-2.mp3", "My traps never miss."), ("character/hunter-greet.mp3", "Tracking prey? Or tracking treasure? Either way, I never miss.")];

Where the 0-2nd indexes are always number 0
where the 3rd index is always path is suffixed in -greet

## Hunter - michel

### Audio Captions

1. Patience. Precision. Perfection."
2. "The hunt begins now."
3. "My traps never miss."
4. "Tracking prey? Or tracking treasure? Either way, I never miss."

## Alchemist - Kallixis

1. "Science conquers all."
2. "Every element serves me."
3. "Behold the power of transmutation."
4. "Need a Potion? A Transmutation?"

## Bard - Chris

1. "Let the music guide us."
2. "Together we are unstoppable."
3. "I'll amplify your strength."
4. "Ah, a new face! Let me play you the song of our people."

## Forager - Ember

1. "From earth comes power."
2. "Nature provides everything."
3. "I'll dig deep for victory."
4. "The Earth whispers Secrets to those who listen."

## Warrior - Adam

1. "My shield protects all."
2. "Stand behind me!"
3. "Honor guides my blade."
4. "Hail, friend!"

## Cardinal - Pastor

1. "Light shall heal and harm."
2. "Sacred power flows through me."
3. "I bring divine judgment."
4. "Blessings upon you, traveler."

## Merchant - Cocky

1. "Fortune favors the bold."
2. "Everything has a price."
3. "Luck is my greatest weapon."
4. "Every deal's a gamble, but I always win."

## Thief - Jessica Voice

1. "Shadows hide my strikes."
2. "You'll never see me coming."
3. "Quick and silent death."
4. "Oh, you actually saw me?"