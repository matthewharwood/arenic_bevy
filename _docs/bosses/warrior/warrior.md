# The Ironwall - Warrior Boss

## Overview
- **Theme**: Immovable defender who wins through attrition
- **Difficulty**: Resource management marathon
- **Arena**: Fortress courtyard with defensive positions
- **Unique Verb**: OUTLAST - Depletes player resources through endurance

## Phase Structure (2 minutes total)

### Phase 1: Siege Stance (0:00-0:30)
**Core Mechanic**: Introduction to attrition warfare

**Boss Abilities**:
- **Shield Wall** (continuous): 50% damage reduction from front
- **Retaliation** (passive): Returns 25% melee damage
- **War Cry** (at 0:20): Reduces all healing by 30%

**Environmental**:
- Defensive barricades provide boss cover
- Ammunition/mana regeneration reduced by 20%
- Fatigue meter begins accumulating

**Counter-play**:
- Flank for full damage
- Use ranged attacks carefully
- Conserve resources early

### Phase 2: Attrition Escalation (0:30-1:00)
**Core Mechanic**: Accelerating resource drain

**Boss Abilities**:
- **Stamina Burn** (every 10s): Abilities cost 50% more for 5s
- **Iron Discipline** (every 8s): Immune to debuffs, transfers them to players
- **Fortress Mode** (at 0:45): Immobile but 75% damage reduction

**Environmental**:
- Healing zones become damage zones
- Maximum resources reduced by 1% per second
- Respawn timers increased by 50%

**Counter-play**:
- Rotate resource usage among team
- Save burst for vulnerability windows
- Manage debuff applications carefully

### Phase 3: Last Stand (1:00-1:30)
**Core Mechanic**: Desperation resource management

**Boss Abilities**:
- **Unyielding** (at 1:00): Cannot drop below 1 HP for 10s
- **Resource Vampirism** (every 7s): Steals 20% of all resources
- **Bulwark** (continuous): Gains 1% damage reduction per second

**Environmental**:
- All resource generation stops
- Fatigue causes ability failures
- Arena slowly fills with exhaustion gas

**Counter-play**:
- Time burst for after Unyielding
- Share resources strategically
- Use consumables wisely

### Phase 4: War of Attrition (1:30-2:00)
**Core Mechanic**: Pure endurance test

**Boss Abilities**:
- **Infinite Endurance** (at 1:30): Boss stops taking fatigue
- **Final Bastion** (every 5s): Heals for damage dealt
- **Victory or Death** (at 1:50): Instantly defeats most fatigued player

**Environmental**:
- Player health/resources cap at 25%
- All regeneration becomes degeneration
- Time itself seems to slow

**Counter-play**:
- Perfect resource efficiency required
- Coordinate final push
- Manage fatigue distribution

## Orthogonal Design Analysis

### Unique Mechanics
- **Resource Attrition**: Gradual depletion warfare
- **Endurance Contest**: Outlasting rather than outdamaging
- **Efficiency Focus**: Resource management over burst

### Taxonomy Mapping
- **Verb**: OUTLAST (endurance attrition)
- **Modifier**: Resource drain, defensive scaling
- **Cost**: Resource efficiency, sustained output

### OC Score: 0.20
- Lowest overlap with: Shadowdancer (0.14) - attrition vs deception
- Highest overlap with: Forager (0.28) - both control space

### Strategic Niche
Creates a unique marathon experience where victory comes through perfect resource management and efficiency rather than burst damage.

## Component Architecture

```rust
// Attrition Component
commands.spawn((
    AttritionAura,
    DrainRate(0.01), // 1% per second
    ResourceType(ResourceType::All),
    AuraRadius(50.0),
    Stacking(true),
));

// Fatigue Tracker
commands.spawn((
    FatigueLevel(0.0),
    FatigueRate(0.005),
    MaxFatigue(1.0),
    FailureThreshold(0.75),
));
```