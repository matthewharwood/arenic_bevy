# The Shadowdancer - Thief Boss

## Overview
- **Theme**: Master of deception who manipulates information and perception
- **Difficulty**: High cognitive load through information warfare
- **Arena**: Hall of mirrors with shifting dimensional phases
- **Unique Verb**: MISDIRECT - Creates false information and hides true threats

## Phase Structure (2 minutes total)

### Phase 1: Veil of Deception (0:00-0:30)
**Core Mechanic**: Introduction to illusion and misdirection system

**Boss Abilities**:
- **Shadow Clone** (every 10s): Creates 3 identical copies, only one takes damage
- **False Telegraph** (every 6s): Shows fake attack indicator, real attack comes from different angle
- **Smoke Bomb** (at 0:20): Obscures vision, teleports to random position

**Environmental**:
- Mirrors create false reflections of boss and players
- Lighting flickers, briefly hiding boss position
- False health bars appear above clones

**Counter-play**:
- Track the real boss through damage numbers
- Learn to recognize subtle differences in clone behavior
- Use AoE abilities to hit multiple targets

### Phase 2: Information Chaos (0:30-1:00)
**Core Mechanic**: UI elements become unreliable

**Boss Abilities**:
- **Ability Shuffle** (at 0:30, 0:45): Swaps player ability positions/icons
- **Phantom Strike** (every 8s): Invisible attacks that only show damage after hitting
- **Decoy Protocol** (every 12s): All players see different boss positions

**Environmental**:
- False UI elements appear (fake debuffs, incorrect cooldowns)
- Arena sections phase in/out of visibility
- Damage numbers randomly show wrong values

**Counter-play**:
- Memorize ability positions before shuffle
- Listen for audio cues instead of visual
- Communicate to verify boss position

### Phase 3: Dimensional Fracture (1:00-1:30)
**Core Mechanic**: Multiple overlapping realities

**Boss Abilities**:
- **Reality Split** (at 1:00): Each player fights a different version of the boss
- **Cross-Phase Strike** (every 7s): Attacks from one dimension affect another
- **Information Leak** (every 10s): Randomly reveals one true piece of information

**Environmental**:
- Players phase between 3 different arena versions
- Actions in one dimension affect others with delay
- True boss location rotates between dimensions

**Counter-play**:
- Coordinate attacks across dimensions
- Time abilities to hit during phase convergence
- Use information leaks strategically

### Phase 4: Perfect Deception (1:30-2:00)
**Core Mechanic**: Complete information blackout

**Boss Abilities**:
- **Total Eclipse** (at 1:30): All visual information becomes false
- **Backstab Protocol** (every 5s): Instantly appears behind lowest HP player
- **Final Gambit** (at 1:50): Players must identify real boss among 10 clones or take massive damage

**Environmental**:
- Complete darkness except for misleading light sources
- All UI elements show random/false information
- Sound becomes the only reliable sense

**Counter-play**:
- Rely entirely on audio cues and communication
- Group together to prevent isolation
- Use process of elimination for Final Gambit

## Orthogonal Design Analysis

### Unique Mechanics
- **Information Warfare**: Only boss that manipulates UI/perception
- **Dimensional Phasing**: Players experience different realities
- **Trust Destruction**: Forces reliance on communication over game feedback

### Taxonomy Mapping
- **Verb**: MISDIRECT (false information creation)
- **Modifier**: Perception manipulation, dimensional splitting
- **Cost**: Cognitive load (information processing), trust tax

### OC Score: 0.29
- Lowest overlap with: Ironwall (0.14) - deception vs direct confrontation
- Highest overlap with: Webweaver (0.29) - both involve prediction/anticipation

### Strategic Niche
The Shadowdancer creates a unique psychological challenge where players must question everything they see and develop alternative information channels through teamwork.

## Component Architecture

```rust
// Shadow Clone Entity
commands.spawn((
    ShadowClone,
    CloneSource(boss_entity),
    IsRealBoss(false),
    FakeHealthBar(1000.0),
    DamageImmune,
    CloneBehavior(BehaviorType::Aggressive),
    Duration(15.0),
));

// False Telegraph Marker
commands.spawn((
    FalseTelegraph,
    TelegraphPosition(fake_pos),
    RealAttackAngle(real_angle),
    MisdirectionDelay(1.5),
    TelegraphIntensity(1.0),
));

// Dimensional Phase Component
commands.spawn((
    DimensionalPhase,
    PlayerPhase(phase_id),
    BossPhase(boss_phase_id),
    PhaseVisibility(0.3),
    CrossPhaseBleed(0.2),
));

// Information Manipulation System
fn manipulate_ui_system(
    mut ui_query: Query<&mut Text, With<AbilityText>>,
    shuffle_events: EventReader<AbilityShuffleEvent>,
    time: Res<Time>,
) {
    for event in shuffle_events.iter() {
        // Randomly swap UI text elements
        for mut text in ui_query.iter_mut() {
            if rand::random::<f32>() < 0.5 {
                text.sections[0].value = random_ability_name();
            }
        }
    }
}
```