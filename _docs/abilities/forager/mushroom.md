# Mushroom

A complete implementation guide for the Forager's healing garden creation utility ability.

## Overview

The **Mushroom** ability showcases the Forager's mastery over natural healing through fast-growing medicinal fungi. When planted on dug terrain, the mushroom takes several seconds to mature before releasing healing spores that benefit the first ally to make contact. This ability requires strategic terrain preparation and timing to maximize healing effectiveness while creating interesting risk-reward decisions about positioning and resource allocation.

## Game Design Philosophy

This ability demonstrates delayed gratification design with environmental prerequisite requirements:

**Preparation-Dependent Strategy**: The requirement for dug ground creates interesting pre-planning phases where Foragers must prepare healing locations before they're needed.

**Temporal Risk-Reward**: The growth delay creates tension between immediate placement and optimal timing, rewarding players who can anticipate healing needs.

**First-Touch Economics**: The single-use nature per mushroom creates tactical decisions about which ally receives the healing benefit.

## Implementation Architecture

### Component-Based Design

```rust
Mushroom {
    growth_time: 3.0,                   // 3 second maturation period
    heal_amount: 100.0,                 // 100 HP restoration to first ally
    placement_requirement: DugGround,    // Must be planted on excavated terrain
    duration_after_growth: 30.0,        // 30 second availability window
    cast_time: 0.5,                     // 0.5 second planting time
    cooldown: 8.0,                      // 8 second ability cooldown
}

MushroomGrowth {
    position: GridPos,
    growth_progress: f32,
    growth_stage: GrowthStage,          // Seed, Sprouting, Mature, Depleted
    healing_available: bool,
    expiration_timer: f32,
    visual_effect: Entity,
}

HealingSpores {
    mushroom_entity: Entity,
    heal_amount: f32,
    affected_ally: Option<Entity>,
    spore_effect: Entity,
    consumption_time: f32,
}
```

### Event-Driven Systems

The ability coordinates through five growth-based systems:
1. **Placement Validation** - Verifies dug ground requirement and site suitability
2. **Growth Management** - Handles 3-second maturation with visual progression
3. **Healing Delivery** - Applies restoration to first ally contact
4. **Duration Tracking** - Manages 30-second availability window after maturation
5. **Visual Evolution** - Shows mushroom growth stages and healing effects

## Step-by-Step Gameplay

### Phase 1: Site Preparation and Planting (Tap Activation)
- **Ground Validation**: Target tile must be previously dug terrain
- **Strategic Placement**: Consider ally movement patterns and anticipated healing needs
- **Planting Cast**: 0.5-second animation as Forager plants mushroom spores
- **Resource Investment**: Ability enters cooldown, committing to selected location

### Phase 2: Growth Period (3 Second Maturation)
- **Sprouting Animation**: Mushroom visibly grows from tiny spore to full size
- **Vulnerability Window**: Mushroom can be destroyed during growth if targeted
- **Strategic Tension**: Team must protect growing mushroom while anticipating healing timing
- **Visual Progression**: Clear growth stages indicate approaching maturation

### Phase 3: Mature Availability (30 Second Window)
- **Healing Readiness**: Mushroom glows with healing energy, ready for ally contact
- **First-Touch Activation**: Any ally walking over mushroom triggers healing
- **Immediate Effect**: Selected ally receives 100 HP restoration instantly
- **Single Use**: Mushroom depletes after first healing activation

### Phase 4: Depletion or Expiration (Natural End)
- **Post-Healing**: Mushroom withers after successful healing delivery
- **Expiration**: If unused, mushroom expires after 30-second availability window
- **Visual Decay**: Depleted mushrooms visibly wither and sink into ground
- **Site Restoration**: Tile returns to normal dug ground state

## Growth Mechanics and Timing

### Growth Stage Progression
```rust
enum GrowthStage {
    Planted,        // 0.0 - 0.5 seconds: Initial spore placement
    Sprouting,      // 0.5 - 1.5 seconds: Visible sprouting begins
    Developing,     // 1.5 - 2.5 seconds: Mushroom takes shape
    Mature,         // 2.5+ seconds: Ready for healing activation
    Depleted,       // Post-healing: Withered and non-functional
}

fn update_mushroom_growth(mushroom: &mut MushroomGrowth, delta_time: f32) {
    mushroom.growth_progress += delta_time;
    
    mushroom.growth_stage = match mushroom.growth_progress {
        t if t < 0.5 => GrowthStage::Planted,
        t if t < 1.5 => GrowthStage::Sprouting,
        t if t < 2.5 => GrowthStage::Developing,
        _ => GrowthStage::Mature,
    };
    
    if mushroom.growth_stage == GrowthStage::Mature {
        mushroom.healing_available = true;
    }
}
```

### Vulnerability During Growth
- **Environmental Damage**: Growing mushrooms can be destroyed by area attacks
- **Enemy Targeting**: Some enemies may actively target growing mushrooms
- **Protection Strategy**: Team positioning to shield vulnerable growing fungi
- **Risk Assessment**: Balance mushroom placement safety with healing accessibility

## Strategic Placement and Timing

### Optimal Placement Locations
- **Traffic Paths**: Plant along common ally movement routes for easy access
- **Defensive Positions**: Create healing stations in safe retreat areas
- **Chokepoints**: Position mushrooms where allies naturally cluster during combat
- **Emergency Zones**: Pre-place mushrooms in anticipated high-damage areas

### Timing Strategies
- **Predictive Planting**: Start growth before damage is expected to occur
- **Combat Rhythm**: Sync mushroom maturation with expected enemy attack patterns
- **Resource Conservation**: Plant during low-activity periods to avoid waste
- **Team Coordination**: Communicate mushroom placement and maturation timing

## Upgrade Paths

### Tier 1: Enhanced Fungi
- **Heal Amount**: 100 HP → 150 HP restoration per mushroom
- **Growth Speed**: 3.0 → 2.0 seconds maturation time
- **Duration Extension**: 30 → 45 seconds availability window after maturation
- **Strategic Value**: More powerful healing with faster availability and longer use window

### Tier 2: Spore Network
- **Multi-Heal**: Each mushroom can heal up to 3 allies before depletion
- **Healing Range**: Allies within 1 tile of mushroom receive healing (not just contact)
- **Chain Growth**: Mature mushrooms occasionally spawn additional mushrooms nearby
- **Tactical Evolution**: Transforms from single-use to sustained healing infrastructure

### Tier 3: Mycelial Mastery
- **Instant Growth**: Mushrooms mature immediately upon planting
- **Regenerative**: Depleted mushrooms regrow after 15 seconds instead of disappearing
- **Area Healing**: Mature mushrooms create 2x2 healing zones with continuous regeneration
- **Ultimate Healing**: Creates permanent healing infrastructure with massive coverage

## Environmental Synergies

### Dig Ability Integration
- **Terrain Preparation**: Dig creates necessary foundation for mushroom placement
- **Garden Planning**: Strategic digging patterns enable mushroom farming networks
- **Resource Efficiency**: Coordinate dig and mushroom usage for maximum battlefield coverage
- **Defensive Synergy**: Dug areas with mushrooms create fortified healing positions

### Border Ability Coordination
- **Protected Gardens**: Border barriers can protect growing mushrooms from projectiles
- **Healing Sanctuaries**: Combine barriers and mushrooms for ultimate safe zones
- **Resource Management**: Balance rock usage between defensive borders and healing mushrooms
- **Tactical Flexibility**: Create protected healing areas during intense combat phases

## Visual & Audio Design

### Planting Phase
- **Visual**: Forager kneels and places glowing spores into prepared earth
- **Animation**: Gentle earth magic swirls around hands during planting
- **Audio**: Soft nature sounds with magical undertones
- **Ground Effect**: Dug earth shows slight luminescence where spores are planted

### Growth Progression
- **Visual**: Time-lapse style growth with realistic mushroom development
- **Stage Effects**: Different particle effects for sprouting, developing, and maturation
- **Audio**: Subtle natural growth sounds with increasing magical resonance
- **Size Scaling**: Mushroom visibly grows from tiny to full size over 3 seconds

### Mature Readiness
- **Visual**: Pulsing golden glow indicates healing readiness and availability
- **Particle**: Gentle healing spores drift upward from mature mushroom cap
- **Audio**: Soft magical humming with nature-inspired harmonics
- **Interaction**: Clear visual feedback when allies are in healing activation range

### Healing Activation
- **Visual**: Burst of golden healing energy when ally makes contact
- **Animation**: Mushroom releases concentrated healing spores in directed stream
- **Audio**: Satisfying healing chime with natural restoration sounds
- **Effect**: Ally surrounded by golden healing particles during restoration

### Depletion and Decay
- **Visual**: Mushroom gradually withers and shrinks back into ground
- **Animation**: Natural decay process over 2-3 seconds post-healing
- **Audio**: Gentle natural sounds as mushroom completes its lifecycle
- **Ground Restoration**: Tile returns to normal dug state ready for future use