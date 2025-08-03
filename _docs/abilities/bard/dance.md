# Dance

A complete implementation guide for the Bard's rhythm-based offensive ability.

## Overview

The **Dance** ability transforms combat into a musical performance through a quick-time event sequence that requires precise timing and rhythm mastery. Players must execute a series of key inputs in sync with musical beats to unleash powerful damage combinations. This ability rewards both mechanical skill and musical timing, creating a unique fusion of rhythm game mechanics with traditional RPG combat.

## Game Design Philosophy

This ability demonstrates innovative skill expression through rhythm-based gameplay:

**Musical Timing Over Button Mashing**: Success depends on synchronization with the beat rather than input speed, creating a more musical and less frantic experience.

**Escalating Complexity**: The sequence starts simple and builds complexity, allowing players to learn the pattern while rewarding mastery with increased damage output.

**Performance Anxiety Design**: The public nature of the sequence creates social pressure that mirrors real musical performance, adding emotional weight to execution.

## Implementation Architecture

### Component-Based Design

```rust
Dance {
    sequence_length: 8,             // 8-beat musical phrase
    base_damage: 150.0,             // Base damage for perfect sequence
    timing_window: 0.15,            // 150ms timing tolerance per beat
    tempo: 120.0,                   // 120 BPM (beats per minute)
    perfect_multiplier: 2.0,        // 2x damage for perfect timing
    good_multiplier: 1.5,           // 1.5x damage for good timing
    miss_penalty: 0.1,              // 10% damage reduction per miss
}

DanceSequence {
    current_beat: u8,
    required_inputs: Vec<InputKey>,
    timing_scores: Vec<TimingRating>,
    total_score: f32,
    combo_multiplier: f32,
}
```

### Event-Driven Systems

The ability coordinates through five rhythm-synchronized systems:
1. **Beat Generation** - Maintains precise 120 BPM timing and visual metronome
2. **Input Detection** - Captures player key presses and evaluates timing accuracy
3. **Sequence Management** - Progresses through the 8-beat pattern with increasing complexity
4. **Score Calculation** - Computes damage multipliers based on timing performance
5. **Visual Choreography** - Synchronizes Bard animation with musical performance

## Step-by-Step Gameplay

### Phase 1: Performance Initiation (Ability Activation)
- **Input Method**: Activate ability to begin musical sequence
- **Visual Setup**: Musical staff appears with scrolling note indicators
- **Audio Preparation**: Introductory musical phrase establishes tempo
- **UI Display**: Timing windows and required key sequence preview

### Phase 2: Beat Sequence (8-Beat Pattern)
- **Beat 1-2**: Simple alternating arrow keys (↑ ↓) to establish rhythm
- **Beat 3-4**: Add diagonal inputs (↖ ↗) for basic complexity
- **Beat 5-6**: Introduce held notes requiring sustained key presses
- **Beat 7-8**: Rapid triplet combination (↑↓↑) for finale flourish

### Phase 3: Timing Evaluation (Per Beat)
- **Perfect Timing**: Within 50ms of beat center (2.0x multiplier)
- **Good Timing**: Within 150ms of beat center (1.5x multiplier)
- **Acceptable**: Within 300ms of beat center (1.0x multiplier)
- **Miss**: Outside timing window (0.1x damage reduction)

### Phase 4: Damage Application (Sequence Completion)
- **Score Calculation**: Base damage × perfect multiplier × combo score
- **Area Effect**: Damage affects all enemies within 3-tile radius
- **Visual Climax**: Spectacular musical explosion with note particle effects
- **Performance Rating**: UI displays timing accuracy and total damage dealt

## Timing Windows and Scoring

### Precision Ratings
```
Perfect: ±50ms   | Visual: Golden note flash  | Multiplier: 2.0x
Good:    ±150ms  | Visual: Blue note flash    | Multiplier: 1.5x
Okay:    ±300ms  | Visual: White note flash   | Multiplier: 1.0x
Miss:    >300ms  | Visual: Red X indicator    | Penalty: -10% total
```

### Combo System
- **Consecutive Perfects**: Each perfect hit increases combo multiplier by 0.1x
- **Combo Preservation**: Good hits maintain combo but don't increase it
- **Combo Breaking**: Miss or okay rating resets combo multiplier to 1.0x
- **Maximum Combo**: 8 consecutive perfects = 1.8x final damage multiplier

## Upgrade Paths

### Tier 1: Extended Performance
- **Sequence Length**: 8 beats → 12 beats with additional complexity patterns
- **Damage Scaling**: Each additional beat increases potential damage by 15%
- **Musical Variety**: Introduces new rhythm patterns and key combinations
- **Visual Enhancement**: More elaborate Bard choreography and particle effects

### Tier 2: Harmonic Resonance
- **Ally Buff**: Perfect performance grants 20% damage boost to nearby allies for 10 seconds
- **Team Synchronization**: Nearby allies gain visual rhythm indicators
- **Shared Glory**: Team benefits create incentive for perfect execution
- **Social Pressure**: Adds positive team dependency to performance outcome

### Tier 3: Maestro's Mastery
- **Adaptive Difficulty**: Sequence complexity adjusts based on player's historical performance
- **Perfect Streak**: 3 consecutive perfect sequences unlocks "Encore Mode" with doubled effects
- **Crowd Favorite**: Enemies within range suffer movement speed reduction during performance
- **Legendary Performance**: Maximum combo creates persistent damage aura for 30 seconds

## Musical Theory Integration

### Rhythm Patterns (8-Beat Sequence)
```
Beat 1: ↑     (Downbeat - strong emphasis)
Beat 2: ↓     (Upbeat - light touch)
Beat 3: ↖     (Syncopation - off-beat accent)
Beat 4: ↗     (Resolution - return to pattern)
Beat 5: ↑─    (Sustained note - hold for full beat)
Beat 6: ↓─    (Sustained note - hold for full beat)
Beat 7: ↑↓↑   (Triplet - three inputs in one beat)
Beat 8: (Rest) (Musical pause - no input required)
```

### Tempo Variations
- **Adagio**: 60 BPM - Slower tempo for learning (Practice Mode)
- **Moderato**: 90 BPM - Standard tempo for casual play
- **Allegro**: 120 BPM - Default competitive tempo
- **Presto**: 150 BPM - Advanced tempo for expert players

## Visual & Audio Design

### Performance Setup
- **Visual**: Musical staff materializes in front of Bard with scrolling notation
- **UI**: Timing meter shows beat positions and required input indicators
- **Audio**: Introductory musical phrase establishes key signature and tempo
- **Animation**: Bard assumes performance stance with instrument raised

### Beat-by-Beat Execution
- **Visual**: Notes light up as they approach timing window
- **Animation**: Bard performs corresponding dance moves for each input
- **Audio**: Musical notes play with perfect pitch for accurate timing
- **Feedback**: Timing rating appears instantly above each note

### Performance Climax
- **Visual**: Explosive musical finale with rainbow note particles
- **Animation**: Bard strikes dramatic final pose with instrument gleaming
- **Audio**: Orchestral crescendo matching the accumulated score
- **Effect**: Damage numbers appear with musical note styling

### Aftermath
- **Visual**: Lingering musical sparkles around Bard for 3 seconds
- **UI**: Performance score card displays with accuracy percentages
- **Audio**: Satisfied audience applause sound for good performances
- **Feedback**: Ability cooldown begins with musical note countdown timer