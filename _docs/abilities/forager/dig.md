# Dig

A complete implementation guide for the Forager's resource gathering and terrain preparation utility ability.

## Overview

The **Dig** ability serves as the Forager's primary resource generation and terrain modification tool, allowing excavation of up to 2 tiles per activation to gather loot and rocks. This fundamental ability creates the resource foundation for other Forager abilities while providing valuable materials for crafting and trading. The ability's efficiency and multi-tile capability make it essential for both economic gameplay and battlefield preparation.

## Game Design Philosophy

This ability demonstrates foundational resource generation design with strategic terrain modification:

**Multi-Purpose Utility**: The ability serves both immediate resource needs and long-term tactical preparation, creating interesting decisions about dig site selection and timing.

**Efficiency Through Batching**: The 2-tile excavation limit encourages thoughtful site selection while providing meaningful resource generation in single activations.

**Environmental Preparation**: Creating dug terrain enables other abilities (Border, Mushroom), adding strategic depth to seemingly simple resource gathering.

## Implementation Architecture

### Component-Based Design

```rust
Dig {
    max_excavations: 2,                 // Up to 2 tiles per activation
    base_loot_chance: 0.7,              // 70% chance for loot per tile
    rock_generation: 1,                 // +1 rock per successfully dug tile
    cast_time: 0.8,                     // 0.8 second per tile excavation
    cooldown: 3.0,                      // 3 second ability cooldown
    range: 1.5,                         // 1.5 tile reach for excavation
}

DigOperation {
    target_positions: Vec<GridPos>,
    current_excavation: usize,
    dig_progress: f32,
    loot_generated: Vec<ItemDrop>,
    rocks_gained: u32,
    visual_effects: Vec<Entity>,
}

TerrainModification {
    position: GridPos,
    previous_state: TerrainType,
    new_state: TerrainType,
    excavation_depth: u8,
    preparation_level: PreparationLevel, // Prepared for Border/Mushroom placement
}
```

### Event-Driven Systems

The ability operates through five coordinated systems:
1. **Target Selection** - Handles up to 2 tile selection with range validation
2. **Sequential Excavation** - Manages dig timing and progress through selected tiles
3. **Loot Generation** - Calculates and spawns appropriate resource rewards
4. **Terrain Modification** - Changes ground state to enable future ability placement
5. **Visual Coordination** - Manages digging animation and particle effects

## Step-by-Step Gameplay

### Phase 1: Site Selection (Double-Tap Activation)
- **Input Method**: Double-tap to begin excavation site selection
- **Multi-Target**: Select up to 2 adjacent tiles within 1.5-tile range
- **Terrain Analysis**: Evaluate ground type for excavation potential
- **Strategic Planning**: Consider resource needs versus tactical terrain preparation

### Phase 2: First Excavation (0.8 Second Duration)
- **Excavation Start**: Forager begins digging first selected tile
- **Progress Animation**: Digging animation with soil and debris effects
- **Loot Calculation**: 70% chance to generate valuable materials
- **Rock Generation**: +1 rock automatically added to inventory

### Phase 3: Second Excavation (0.8 Second Duration)
- **Continuation**: If second tile selected, Forager moves and begins second dig
- **Independent Rewards**: Each tile calculated separately for loot generation
- **Cumulative Benefits**: Additional +1 rock gained from second successful dig
- **Terrain Completion**: Both tiles marked as dug terrain for future abilities

### Phase 4: Resource Collection (Instant Resolution)
- **Loot Distribution**: Generated items appear on dug tiles for collection
- **Rock Integration**: Gained rocks immediately available for Boulder/Border abilities
- **Inventory Update**: Resources automatically collected or marked for pickup
- **Terrain Readiness**: Dug tiles now valid for Border and Mushroom placement

## Loot Generation System

### Resource Tables by Terrain Type
```rust
fn generate_dig_loot(terrain_type: TerrainType, luck_modifier: f32) -> Vec<ItemType> {
    let base_chance = 0.7 + luck_modifier;
    let mut loot = Vec::new();
    
    if random() < base_chance {
        match terrain_type {
            TerrainType::Grassland => {
                loot.push(choose_random(&[
                    ItemType::Seeds,
                    ItemType::Herbs,
                    ItemType::SmallGems,
                    ItemType::WormCasting,
                ]));
            },
            TerrainType::Rocky => {
                loot.push(choose_random(&[
                    ItemType::IronOre,
                    ItemType::Crystals,
                    ItemType::PreciousStones,
                    ItemType::Fossils,
                ]));
            },
            TerrainType::Forest => {
                loot.push(choose_random(&[
                    ItemType::RareWood,
                    ItemType::Mushrooms,
                    ItemType::AnimalBones,
                    ItemType::MagicRoots,
                ]));
            },
            TerrainType::Wetland => {
                loot.push(choose_random(&[
                    ItemType::Clay,
                    ItemType::WaterCrystals,
                    ItemType::AquaticHerbs,
                    ItemType::PurifiedWater,
                ]));
            },
        }
    }
    
    // Always generate rocks regardless of loot success
    loot.push(ItemType::Rock);
    loot
}
```

### Loot Quality Scaling
- **Common Items (60%)**: Basic crafting materials and resources
- **Uncommon Items (30%)**: Enhanced materials with better properties
- **Rare Items (9%)**: Valuable components for advanced crafting
- **Epic Items (1%)**: Extremely rare materials with unique properties

## Multi-Tile Excavation Strategy

### Optimal Dig Patterns
- **Adjacent Placement**: Dig connected tiles for efficient Border/Mushroom coverage
- **Resource Maximization**: Target terrain types matching current resource needs
- **Tactical Preparation**: Create defensive positions in anticipation of combat
- **Economic Efficiency**: Balance immediate needs with long-term strategic value

### Site Selection Considerations
- **Terrain Diversity**: Different ground types yield different resource categories
- **Future Ability Synergy**: Consider Border and Mushroom placement requirements
- **Safety Positioning**: Dig in areas that won't expose Forager to enemy attacks
- **Team Coordination**: Prepare terrain that benefits overall team strategy

## Upgrade Paths

### Tier 1: Efficient Excavation
- **Excavation Count**: 2 → 3 tiles per activation
- **Loot Chance**: 70% → 85% chance for valuable materials per tile
- **Speed Improvement**: 0.8 → 0.6 seconds per tile excavation time
- **Strategic Value**: Higher efficiency and more terrain preparation per use

### Tier 2: Treasure Hunter
- **Quality Boost**: +1 tier chance for all loot (Common → Uncommon, etc.)
- **Rare Discovery**: 5% chance for epic-tier materials regardless of terrain
- **Resource Bonus**: Each dig grants 2 rocks instead of 1
- **Economic Evolution**: Dramatically improves resource generation potential

### Tier 3: Master Excavator
- **Area Dig**: Can excavate 3x3 area in single activation
- **Instant Preparation**: Dug terrain immediately gains maximum preparation level
- **Loot Magnet**: Automatically collects all generated loot within 3-tile radius
- **Ultimate Efficiency**: Transforms dig into large-scale terrain and resource operation

## Terrain Preparation Benefits

### Ground State Changes
- **Unprepared → Dug**: Enables Border and Mushroom ability placement
- **Preparation Levels**: Multiple digs on same tile increase effectiveness
- **Tactical Terrain**: Dug areas provide slight movement speed bonus to allies
- **Environmental Integration**: Modified terrain affects enemy pathing slightly

### Strategic Terrain Usage
- **Defensive Networks**: Create interconnected dug areas for Border barrier chains
- **Garden Plots**: Prepared terrain for multiple Mushroom healing stations
- **Resource Corridors**: Dig paths that funnel Boulder trajectory through valuable areas
- **Emergency Positions**: Pre-prepared safe zones for tactical retreats

## Visual & Audio Design

### Excavation Animation
- **Visual**: Forager kneels and uses natural digging motions with earth magic
- **Tool Effects**: Magical energy enhances digging tools with earthen glow
- **Audio**: Satisfying soil digging sounds mixed with magical earth tones
- **Particle**: Dirt and debris fly realistically from excavation sites

### Resource Discovery
- **Visual**: Items appear with sparkling emergence effects from freshly dug earth
- **Rarity Indication**: Different particle colors indicate resource quality tiers
- **Audio**: Distinct discovery chimes for different material types
- **Collection**: Loot items pulse gently to indicate availability for pickup

### Terrain Transformation
- **Visual**: Ground texture changes from normal to excavated appearance
- **Depth Indication**: Subtle shading shows dug areas versus normal terrain
- **Audio**: Soft earth settling sounds as terrain modification completes
- **Status**: UI indicator shows which areas are prepared for ability placement

### Multi-Tile Coordination
- **Visual**: Connecting particle streams show dig site relationships
- **Progress**: Sequential excavation clearly shows which tile is currently active
- **Audio**: Layered digging sounds create satisfying work rhythm
- **Completion**: Final flourish indicates successful excavation sequence completion