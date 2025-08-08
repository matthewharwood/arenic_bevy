# The Mad Alchemist - Alchemist Boss

## Overview
- **Theme**: Volatile chemist creating cascading reaction chains
- **Difficulty**: High tactical complexity through reaction management
- **Arena**: Laboratory with chemical pools and reaction chambers
- **Unique Verb**: CASCADE - Creates chemical chain reactions that multiply

## Phase Structure (2 minutes total)

### Phase 1: Chemical Introduction (0:00-0:30)
**Core Mechanic**: Basic chemical combination system

**Boss Abilities**:
- **Catalyst Throw** (every 6s): Throws vials creating elemental pools
- **Reaction Trigger** (every 10s): Combines two pools for area damage
- **Toxic Cloud** (at 0:20): Releases damaging gas in a cone

**Environmental**:
- Chemical pools appear in primary colors (red, blue, yellow)
- Pools slowly expand until triggered
- Ventilation systems periodically clear gases

**Counter-play**:
- Avoid standing between pools
- Trigger reactions early to control damage
- Use wind effects to redirect clouds

### Phase 2: Compound Complexity (0:30-1:00)
**Core Mechanic**: Secondary and tertiary reactions

**Boss Abilities**:
- **Chain Catalyst** (every 12s): Creates reactions that trigger more reactions
- **Unstable Formula** (every 8s): Random pools become volatile, exploding after 3s
- **Transmutation Field** (at 0:45): Converts player buffs into debuffs

**Environmental**:
- Pools now mix automatically when touching
- Secondary reactions create new element types
- Reaction explosions leave residue pools

**Counter-play**:
- Control pool placement through positioning
- Time buff usage around transmutation
- Create controlled reaction chains

### Phase 3: Critical Mass (1:00-1:30)
**Core Mechanic**: Reaction threshold system

**Boss Abilities**:
- **Cascade Overload** (continuous): All reactions now trigger adjacent pools
- **Philosopher's Stone** (at 1:15): Drops item that amplifies all reactions 3x
- **Elemental Fusion** (every 10s): Combines all pools of same color

**Environmental**:
- Arena temperature rises, accelerating reactions
- Critical mass meter shows total reaction potential
- Reaching 100% causes arena-wide explosion

**Counter-play**:
- Carefully manage total pool count
- Destroy Philosopher's Stone quickly
- Use cleanses to remove pool effects

### Phase 4: Alchemical Apocalypse (1:30-2:00)
**Core Mechanic**: Uncontrolled cascade system

**Boss Abilities**:
- **Infinite Catalyst** (at 1:30): Continuous pool creation
- **Reaction Reversal** (every 7s): Healing pools become damage
- **Final Experiment** (at 1:50): Attempts to create gold, success heals boss to full

**Environmental**:
- Entire floor becomes reactive surface
- Random transmutation waves sweep arena
- Pools begin creating autonomous reactions

**Counter-play**:
- Focus on interrupting Final Experiment
- Use immunity windows strategically
- Create safe zones through controlled reactions

## Orthogonal Design Analysis

### Unique Mechanics
- **Chemical Cascades**: Multiplicative reaction chains
- **Pool Management**: Spatial resource control
- **Reaction Prediction**: Calculate chain outcomes

### Taxonomy Mapping
- **Verb**: CASCADE (multiply effects)
- **Modifier**: Chemical combinations, reaction chains
- **Cost**: Spatial management, prediction complexity

### OC Score: 0.25
- Lowest overlap with: Shadowdancer (0.16) - chemistry vs deception
- Highest overlap with: Cardinal (0.31) - both involve area effects

### Strategic Niche
Creates a dynamic battlefield puzzle where player actions have multiplicative consequences, requiring careful spatial and temporal planning.

## Component Architecture

```rust
// Chemical Pool Entity
commands.spawn((
    ChemicalPool,
    PoolElement(ElementType::Fire),
    PoolRadius(2.0),
    ReactionPotential(1.0),
    Volatile(false),
    ExpansionRate(0.1),
));

// Cascade Reaction Component
commands.spawn((
    CascadeReaction,
    TriggerPool(pool_entity),
    ChainDepth(3),
    ReactionMultiplier(1.5),
    PropagationDelay(0.5),
));
```