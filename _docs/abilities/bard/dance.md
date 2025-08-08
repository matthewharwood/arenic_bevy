# Dance

A complete implementation guide for the Bard's rhythm-based offensive ability using single-component architecture.

## Overview

The **Dance** ability transforms combat into a musical performance through a quick-time event sequence that requires precise timing and rhythm mastery. Players must execute a series of key inputs in sync with musical beats to unleash powerful damage combinations. This ability rewards both mechanical skill and musical timing, creating a unique fusion of rhythm game mechanics with traditional RPG combat.

## Game Design Philosophy

This ability demonstrates innovative skill expression through rhythm-based gameplay:

**Musical Timing Over Button Mashing**: Success depends on synchronization with the beat rather than input speed, creating a more musical and less frantic experience.

**Escalating Complexity**: The sequence starts simple and builds complexity, allowing players to learn the pattern while rewarding mastery with increased damage output.

**Performance Anxiety Design**: The public nature of the sequence creates social pressure that mirrors real musical performance, adding emotional weight to execution.

## Component Architecture

### Entity Composition Pattern

Following the single-component architecture, Dance uses simple, single-value components:

```rust
// Dance Sequence Entity
commands.spawn((
    // Marker Components
    DanceSequence,
    RhythmGame,
    
    // Sequence Components
    SequenceLength(8),
    CurrentBeat(0),
    Tempo(120.0),  // BPM
    
    // Timing Components
    TimingWindow(0.15),
    NextBeatTime(0.0),
    ElapsedTime(0.0),
    
    // Scoring Components
    PerfectMultiplier(2.0),
    GoodMultiplier(1.5),
    MissPenalty(0.1),
    ComboMultiplier(1.0),
    
    // Damage Components
    BaseDamage(150.0),
    AccumulatedScore(0.0),
));

// Beat Indicator Entity (per beat)
commands.spawn((
    // Marker Components
    BeatIndicator,
    
    // Beat Components
    BeatIndex(index),
    RequiredKey(key),
    BeatTime(beat_time),
    
    // Visual Components
    Transform::from_translation(ui_pos),
    EmissiveColor(Color::srgb(0.8, 0.2, 0.8)),
    EmissiveIntensity(1.0),
));

// Input Result Entity (spawned on input)
commands.spawn((
    // Marker Components
    InputResult,
    
    // Result Components
    TimingRating(rating),
    ScoreValue(score),
    BeatIndex(beat),
    
    // Visual Components
    FlashColor(result_color),
    FlashIntensity(2.0),
    Duration(0.3),
    ElapsedTime(0.0),
));
```

### Core Components Used

All components are single-value/single-purpose:

```rust
// Sequence Components
pub struct SequenceLength(pub u8);
pub struct CurrentBeat(pub u8);
pub struct Tempo(pub f32);
pub struct BeatIndex(pub u8);
pub struct RequiredKey(pub KeyCode);
pub struct BeatTime(pub f32);

// Timing Components
pub struct TimingWindow(pub f32);
pub struct NextBeatTime(pub f32);
pub struct ElapsedTime(pub f32);
pub struct Duration(pub f32);

// Scoring Components
pub struct PerfectMultiplier(pub f32);
pub struct GoodMultiplier(pub f32);
pub struct MissPenalty(pub f32);
pub struct ComboMultiplier(pub f32);
pub struct AccumulatedScore(pub f32);
pub struct ScoreValue(pub f32);

// Rating Components
pub struct TimingRating(pub TimingGrade);
pub enum TimingGrade { Perfect, Good, Okay, Miss }

// Damage Components
pub struct BaseDamage(pub f32);
pub struct FinalDamage(pub f32);

// Visual Components
pub struct FlashColor(pub Color);
pub struct FlashIntensity(pub f32);
pub struct EmissiveColor(pub Color);
pub struct EmissiveIntensity(pub f32);

// Marker Components (zero-sized)
pub struct DanceSequence;
pub struct RhythmGame;
pub struct BeatIndicator;
pub struct InputResult;
```

### System Implementation

Systems query only the components they need:

```rust
// Beat generation system
fn beat_generation_system(
    mut commands: Commands,
    time: Res<Time>,
    mut sequences: Query<(
        Entity,
        &SequenceLength,
        &mut CurrentBeat,
        &Tempo,
        &mut NextBeatTime,
        &mut ElapsedTime
    ), With<DanceSequence>>,
) {
    for (entity, length, mut current, tempo, mut next_beat, mut elapsed) in sequences.iter_mut() {
        elapsed.0 += time.delta_secs();
        
        let beat_interval = 60.0 / tempo.0;  // Convert BPM to seconds per beat
        
        if elapsed.0 >= next_beat.0 {
            if current.0 < length.0 {
                // Spawn beat indicator
                let key = get_key_for_beat(current.0);
                commands.spawn((
                    BeatIndicator,
                    BeatIndex(current.0),
                    RequiredKey(key),
                    BeatTime(elapsed.0),
                    Duration(beat_interval),
                    ElapsedTime(0.0),
                ));
                
                current.0 += 1;
                next_beat.0 += beat_interval;
            } else {
                // Sequence complete
                commands.entity(entity).insert(SequenceComplete);
            }
        }
    }
}

// Input detection system
fn input_detection_system(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    beats: Query<(&BeatIndex, &RequiredKey, &BeatTime), With<BeatIndicator>>,
    mut sequences: Query<(&ElapsedTime, &TimingWindow, &mut AccumulatedScore, &mut ComboMultiplier)>,
) {
    for (elapsed, window, mut score, mut combo) in sequences.iter_mut() {
        for (beat_index, required_key, beat_time) in beats.iter() {
            if input.just_pressed(required_key.0) {
                let timing_diff = (elapsed.0 - beat_time.0).abs();
                
                let rating = if timing_diff <= 0.05 {
                    TimingGrade::Perfect
                } else if timing_diff <= window.0 {
                    TimingGrade::Good
                } else if timing_diff <= window.0 * 2.0 {
                    TimingGrade::Okay
                } else {
                    TimingGrade::Miss
                };
                
                // Apply scoring
                let beat_score = match rating {
                    TimingGrade::Perfect => {
                        combo.0 += 0.1;
                        2.0 * combo.0
                    },
                    TimingGrade::Good => {
                        1.5 * combo.0
                    },
                    TimingGrade::Okay => {
                        combo.0 = 1.0;
                        1.0
                    },
                    TimingGrade::Miss => {
                        combo.0 = 1.0;
                        -0.1
                    }
                };
                
                score.0 += beat_score;
                
                // Spawn result indicator
                commands.spawn((
                    InputResult,
                    TimingRating(rating),
                    ScoreValue(beat_score),
                    BeatIndex(beat_index.0),
                    FlashColor(rating_to_color(rating)),
                    FlashIntensity(2.0),
                    Duration(0.3),
                    ElapsedTime(0.0),
                ));
            }
        }
    }
}

// Damage application system
fn dance_damage_system(
    mut commands: Commands,
    sequences: Query<(&BaseDamage, &AccumulatedScore), (With<DanceSequence>, With<SequenceComplete>)>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (base_damage, score) in sequences.iter() {
        let final_damage = base_damage.0 * score.0.max(0.1);
        
        // Apply damage to all enemies in range
        for (enemy_entity, enemy_transform) in enemies.iter() {
            if enemy_transform.translation.distance(Vec3::ZERO) <= 3.0 * TILE_SIZE {
                commands.entity(enemy_entity).with_child((
                    DamageInstance,
                    Damage(final_damage),
                    DamageType::Musical,
                ));
            }
        }
        
        // Spawn damage effect
        commands.spawn((
            DanceFinale,
            AreaEffect,
            Radius(3.0),
            FinalDamage(final_damage),
            Duration(1.0),
            ElapsedTime(0.0),
            ParticleCount(200),
        ));
    }
}

// Visual feedback system
fn beat_visual_system(
    mut commands: Commands,
    time: Res<Time>,
    mut indicators: Query<(
        Entity,
        &mut Transform,
        &mut ElapsedTime,
        &Duration
    ), With<BeatIndicator>>,
) {
    for (entity, mut transform, mut elapsed, duration) in indicators.iter_mut() {
        elapsed.0 += time.delta_secs();
        
        // Scroll indicator toward timing window
        let progress = elapsed.0 / duration.0;
        transform.translation.x = lerp(100.0, 0.0, progress);
        
        if elapsed.0 >= duration.0 {
            commands.entity(entity).despawn();
        }
    }
}

// Bard animation system
fn bard_animation_system(
    results: Query<&TimingRating, Added<InputResult>>,
    mut bard: Query<&mut AnimationState, With<Bard>>,
) {
    for rating in results.iter() {
        if let Ok(mut anim) = bard.get_single_mut() {
            match rating.0 {
                TimingGrade::Perfect => anim.0 = "dance_flourish",
                TimingGrade::Good => anim.0 = "dance_step",
                TimingGrade::Okay => anim.0 = "dance_basic",
                TimingGrade::Miss => anim.0 = "dance_stumble",
            }
        }
    }
}
```

## Upgrade Paths

### Tier 1: Extended Performance
Adds sequence enhancement components:
```rust
// Additional components
SequenceLengthBonus(4)     // +4 beats
DamagePerBeat(15.0)        // +15 damage per beat
MusicalVariety             // New patterns
```

### Tier 2: Harmonic Resonance
Adds team buff components:
```rust
// Additional components
AllyBuff
BuffStrength(0.2)          // 20% damage boost
BuffDuration(10.0)         // 10 seconds
SharedRhythm               // Allies see indicators
```

### Tier 3: Maestro's Mastery
Adds mastery components:
```rust
// Additional components
AdaptiveDifficulty
PerfectStreak(0)
StreakBonus(2.0)           // Double damage on streak
EncoreMode                 // Unlocked by streak
MovementSlow(0.5)          // Enemies slowed during performance
AuraDuration(30.0)         // Persistent damage aura
```

## Musical Theory Integration

Rhythm patterns use component-based timing:

```rust
// Beat pattern components
fn get_key_for_beat(beat: u8) -> KeyCode {
    match beat {
        0 => KeyCode::ArrowUp,      // Downbeat
        1 => KeyCode::ArrowDown,    // Upbeat
        2 => KeyCode::ArrowLeft,    // Syncopation
        3 => KeyCode::ArrowRight,   // Resolution
        4 => KeyCode::Space,        // Hold
        5 => KeyCode::Space,        // Hold
        6 => KeyCode::ArrowUp,      // Triplet start
        7 => KeyCode::ArrowDown,    // Triplet end
        _ => KeyCode::Space,
    }
}

// Tempo variations
commands.spawn((
    TempoVariation,
    BaseTempo(120.0),
    CurrentTempo(120.0),
    TempoChange(0.0),  // Accelerando/ritardando
));
```

## Visual & Audio Design

Visual effects use single-value components:

```rust
// Performance setup
commands.spawn((
    MusicalStaff,
    StaffPosition(Vec3::new(0.0, 100.0, 0.0)),
    StaffWidth(400.0),
    NoteSpeed(100.0),
    Duration(10.0),
));

// Beat indicators
commands.spawn((
    NoteVisual,
    NoteType(note_type),
    NoteColor(Color::srgb(0.8, 0.2, 0.8)),
    GlowIntensity(1.0),
    ScrollSpeed(100.0),
));

// Performance climax
commands.spawn((
    MusicalExplosion,
    ParticleCount(500),
    ParticleColors(vec![Color::srgb(0.8, 0.2, 0.8)]),
    ExplosionRadius(5.0),
    Duration(2.0),
    AudioVolume(1.0),
));

// Score display
commands.spawn((
    ScoreCard,
    Accuracy(0.85),
    PerfectCount(6),
    GoodCount(2),
    TotalDamage(final_damage),
    DisplayDuration(3.0),
));
```

## Recording Integration

All components are deterministic and recordable:

```rust
commands.spawn((
    RecordableAction,
    ActionType::DanceSequence,
    Timestamp(recording.current_time),
    InputSequence(recorded_inputs),
    TimingSequence(recorded_timings),
));
```

This single-component architecture ensures:
- **Frame-perfect timing** with single timing window values
- **Clean combo tracking** through single multiplier components
- **Flexible beat patterns** with index-based key mapping
- **Efficient visual scrolling** with single-value positions
- **Deterministic rhythm** for recording system