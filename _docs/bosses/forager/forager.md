# The Earthshaper - Forager Boss

## Overview
- **Theme**: Geological architect who reshapes the battlefield
- **Difficulty**: High spatial adaptation through terrain transformation
- **Arena**: Cavern with malleable stone and earth
- **Unique Verb**: RESHAPE - Dynamically alters arena topology

## Phase Structure (2 minutes total)

### Phase 1: Tectonic Awakening (0:00-0:30)
**Core Mechanic**: Introduction to terrain manipulation

**Boss Abilities**:
- **Earth Spike** (every 6s): Raises damaging pillars from ground
- **Fissure** (every 10s): Creates line of cracking earth
- **Stone Skin** (at 0:20): Gains damage reduction, sheds as projectiles

**Environmental**:
- Floor elevation begins changing
- Cracks appear warning of future changes
- Dust clouds reduce visibility

**Counter-play**:
- Watch for crack patterns
- Use elevation changes for advantage
- Position away from walls

### Phase 2: Geological Upheaval (0:30-1:00)
**Core Mechanic**: Rapid terrain transformation

**Boss Abilities**:
- **Tectonic Shift** (every 12s): Rotates entire arena sections
- **Gravity Well** (every 8s): Creates zones of altered gravity
- **Avalanche** (at 0:45): Rockslide from elevated areas

**Environmental**:
- Arena becomes multi-level battlefield
- Bridges form and collapse dynamically
- Magnetic zones affect metal equipment

**Counter-play**:
- Memorize rotation patterns
- Use gravity wells for mobility
- Avoid predictable avalanche paths

### Phase 3: Living Mountain (1:00-1:30)
**Core Mechanic**: Arena becomes hostile entity

**Boss Abilities**:
- **Earthquake** (continuous): Constant tremors disrupt movement
- **Terraforming** (every 10s): Reshapes arena to trap players
- **Crystal Growth** (every 7s): Spawns reflecting crystal formations

**Environmental**:
- Entire arena pulses like heartbeat
- Walls close in and expand rhythmically
- Crystal formations create laser hazards

**Counter-play**:
- Adapt to unstable footing
- Break crystals before laser buildup
- Use terraforming against boss

### Phase 4: Continental Drift (1:30-2:00)
**Core Mechanic**: Complete geological chaos

**Boss Abilities**:
- **Pangaea Break** (at 1:30): Splits arena into floating islands
- **Volcanic Eruption** (every 8s): Lava geysers from cracks
- **Mountain's Heart** (at 1:50): Channels to reset arena, healing fully if successful

**Environmental**:
- Islands drift and collide
- Lava rises gradually
- Gravity becomes directional

**Counter-play**:
- Jump between islands carefully
- Interrupt Mountain's Heart channel
- Use environmental damage on boss

## Orthogonal Design Analysis

### Unique Mechanics
- **Topology Manipulation**: 3D battlefield reshaping
- **Environmental Personification**: Arena as enemy
- **Geological Timing**: Slow but inevitable changes

### Taxonomy Mapping
- **Verb**: RESHAPE (alter terrain)
- **Modifier**: Elevation changes, topology shifts
- **Cost**: Spatial adaptation, positioning challenge

### OC Score: 0.21
- Lowest overlap with: Bard (0.15) - terrain vs rhythm
- Highest overlap with: Warrior (0.28) - both control space

### Strategic Niche
Creates a constantly evolving 3D puzzle where the battlefield itself becomes the primary opponent, requiring continuous spatial adaptation.

## Component Architecture

```rust
// Terrain Transformation Entity
commands.spawn((
    TerrainTransform,
    StartElevation(0.0),
    TargetElevation(5.0),
    TransformDuration(2.0),
    AffectedTiles(tile_list),
));

// Gravity Well Component
commands.spawn((
    GravityWell,
    WellCenter(Vec3),
    GravityStrength(-9.8),
    WellRadius(5.0),
    Duration(10.0),
));
```