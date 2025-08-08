# The Purifier - Cardinal Boss

## Overview
- **Theme**: Holy zealot who inverts healing and harm
- **Difficulty**: Paradigm adaptation challenge
- **Arena**: Cathedral with stained glass windows
- **Unique Verb**: INVERT - Reverses beneficial and harmful effects

## Phase Structure (2 minutes total)

### Phase 1: Blessed Corruption (0:00-0:30)
**Core Mechanic**: Introduction to effect inversion

**Boss Abilities**:
- **Inversion Field** (every 10s): Healing becomes damage for 3s
- **Corrupted Prayer** (every 7s): Buffs on players become debuffs
- **False Sanctuary** (at 0:20): Creates healing zone that actually damages

**Environmental**:
- Stained glass light creates inversion zones
- Holy water pools deal damage instead of healing
- Consecrated ground burns instead of blessing

**Counter-play**:
- Time healing between inversions
- Cleanse buffs before corruption
- Identify true safe zones

### Phase 2: Paradigm Shift (0:30-1:00)
**Core Mechanic**: Rapid polarity switches

**Boss Abilities**:
- **Polarity Pulse** (every 8s): Swaps all positive/negative effects
- **Zealot's Paradox** (every 12s): Damage increases boss healing
- **Divine Confusion** (at 0:45): Randomizes effect polarities

**Environmental**:
- Light and shadow zones swap properties
- Damage zones become healing zones randomly
- Status effect icons show opposite effects

**Counter-play**:
- Track current polarity state
- Use damage strategically during paradox
- Adapt to randomized effects

### Phase 3: Sacred Reversal (1:00-1:30)
**Core Mechanic**: Complete effect inversion

**Boss Abilities**:
- **Mirror of Truth** (continuous): All effects work opposite
- **Baptism of Fire** (every 10s): Healing abilities deal damage
- **Martyr's Blessing** (at 1:15): Boss takes damage to heal players, players must damage selves

**Environmental**:
- Entire arena becomes inverted space
- Gravity occasionally reverses
- Time-based effects run backward

**Counter-play**:
- Embrace reversed mechanics
- Damage self strategically during Martyr's
- Use harmful abilities as healing

### Phase 4: Absolute Paradox (1:30-2:00)
**Core Mechanic**: Logic breakdown

**Boss Abilities**:
- **Quantum Prayer** (at 1:30): Effects both heal and harm simultaneously
- **Faith Crisis** (every 6s): Random abilities do opposite of intended
- **Final Judgment** (at 1:50): Players must heal boss to damage it

**Environmental**:
- Reality constantly inverts
- All effects have dual nature
- Victory conditions reverse

**Counter-play**:
- Accept paradoxical mechanics
- Heal boss during Final Judgment
- Trust inverted logic

## Orthogonal Design Analysis

### Unique Mechanics
- **Effect Inversion**: Reverses game logic
- **Paradigm Adaptation**: Challenges core assumptions
- **Paradoxical Victory**: Win by healing enemy

### Taxonomy Mapping
- **Verb**: INVERT (reverse effects)
- **Modifier**: Polarity swaps, paradigm shifts
- **Cost**: Cognitive adaptation, logic reversal

### OC Score: 0.23
- Lowest overlap with: Hunter (0.17) - inversion vs prediction
- Highest overlap with: Alchemist (0.31) - both transform effects

### Strategic Niche
Creates a mind-bending experience where players must constantly question and reverse their instincts, challenging fundamental game logic.

## Component Architecture

```rust
// Inversion Field Entity
commands.spawn((
    InversionField,
    FieldRadius(10.0),
    InversionType(InversionType::HealingDamage),
    Duration(3.0),
    PolarityFlip(true),
));

// Paradox Effect Entity
commands.spawn((
    ParadoxEffect,
    DualNature(true),
    Healing(100.0),       // Using shared component
    Damage(100.0),        // Using shared component
    SimultaneousApplication(true),
    Duration(5.0),
));
```