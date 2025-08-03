# Heal

A complete implementation guide for the Cardinal's targeted restoration utility ability.

## Overview

The **Heal** ability represents the Cardinal's primary healing support through divine restoration magic. Using intelligent targeting within an 8x8 grid area, the ability automatically selects the ally with the lowest health percentage, ensuring healing resources reach those most in need. This smart targeting system eliminates targeting confusion during combat while providing substantial health restoration to keep the team operational.

## Game Design Philosophy

This ability demonstrates intelligent automation design that enhances rather than replaces player skill:

**Need-Based Targeting**: Automatic selection of the most injured ally removes micro-management burden while ensuring optimal resource distribution.

**Substantial Restoration**: The significant healing amount makes each cast meaningful and impactful rather than requiring repeated minor applications.

**Consistent Availability**: The 5-second cooldown provides reliable access to healing without overwhelming combat with constant restoration.

## Implementation Architecture

### Component-Based Design

```rust
Heal {
    range: GridArea::new(8, 8),     // 8x8 grid coverage area
    heal_amount: 150.0,             // 150 HP restoration
    cast_time: 0.5,                 // 0.5 second channel time
    cooldown: 5.0,                  // 5 second ability cooldown
    targeting: SmartTarget::LowestHealthPercent,
    mana_cost: 25.0,                // 25 MP per cast
}

HealTargeting {
    eligible_allies: Vec<Entity>,
    health_percentages: HashMap<Entity, f32>,
    selected_target: Option<Entity>,
    targeting_override: Option<Entity>, // Manual override if needed
}

HealEffect {
    target: Entity,
    heal_amount: f32,
    channel_progress: f32,
    visual_effect: Entity,
    audio_source: Entity,
}
```

### Event-Driven Systems

The ability operates through five integrated systems:
1. **Smart Targeting** - Identifies ally with lowest health percentage within range
2. **Cast Management** - Handles 0.5-second channel with interruption potential
3. **Healing Application** - Applies health restoration upon successful cast completion
4. **Resource Management** - Tracks mana consumption and cooldown timing
5. **Visual Coordination** - Manages divine healing effects and status feedback

## Step-by-Step Gameplay

### Phase 1: Automatic Target Selection (Pre-Activation)
- **Range Scanning**: System identifies all allies within 8x8 grid area
- **Health Assessment**: Calculate health percentage for each eligible ally
- **Priority Selection**: Choose ally with lowest health percentage
- **Visual Preview**: Selected target highlighted with gentle healing indicator

### Phase 2: Cast Initiation (Tap Activation)
- **Input Method**: Single tap begins 0.5-second healing channel
- **Mana Check**: Verify sufficient mana (25 MP) before initiating cast
- **Channel Start**: Cardinal begins divine invocation with raised hands
- **Visual Buildup**: Golden healing energy accumulates around Cardinal

### Phase 3: Channel Completion (0.5 Second Duration)
- **Uninterrupted Channel**: Cardinal must avoid damage or movement during cast
- **Visual Connection**: Golden light stream connects Cardinal to target
- **Audio Building**: Divine harmony builds in intensity toward completion
- **Interruption Risk**: Taking damage or forced movement cancels heal

### Phase 4: Healing Resolution (Instant Application)
- **Health Restoration**: Target gains 150 HP instantly upon channel completion
- **Visual Impact**: Brilliant golden light envelops target with healing particles
- **Audio Climax**: Divine chime indicates successful healing completion
- **Status Update**: Target's health bar updates with healing amount displayed

## Smart Targeting Algorithm

### Selection Logic
```rust
fn select_heal_target(allies: &[Entity], cardinal_pos: GridPos) -> Option<Entity> {
    let eligible_allies: Vec<Entity> = allies.iter()
        .filter(|ally| is_within_range(**ally, cardinal_pos, 8))
        .filter(|ally| is_alive(**ally))
        .filter(|ally| !is_at_full_health(**ally))
        .cloned()
        .collect();
    
    if eligible_allies.is_empty() {
        return None;
    }
    
    // Find ally with lowest health percentage
    eligible_allies.into_iter()
        .min_by(|a, b| {
            let a_percent = health_percentage(*a);
            let b_percent = health_percentage(*b);
            a_percent.partial_cmp(&b_percent).unwrap_or(std::cmp::Ordering::Equal)
        })
}

fn health_percentage(entity: Entity) -> f32 {
    let current = get_current_health(entity);
    let maximum = get_maximum_health(entity);
    current / maximum
}
```

### Priority Factors
- **Critical Health**: Allies below 25% health gain selection priority
- **Damage Trends**: Recently damaged allies weighted slightly higher
- **Role Consideration**: Tanks and healers gain minor priority weighting
- **Distance Optimization**: Among equal health percentages, closer allies preferred

## Upgrade Paths

### Tier 1: Empowered Restoration
- **Heal Amount**: 150 HP → 200 HP per cast
- **Cast Speed**: 0.5 seconds → 0.3 seconds channel time
- **Mana Efficiency**: 25 MP → 20 MP cost reduction
- **Strategic Value**: More powerful healing with faster delivery and better resource economy

### Tier 2: Radiant Recovery
- **Overheal Shield**: Excess healing creates temporary shield (maximum 50 HP)
- **Range Extension**: 8x8 grid → 10x10 grid coverage area
- **Heal Over Time**: Additional 20 HP/second for 5 seconds after initial heal
- **Enhanced Utility**: Provides protection beyond maximum health and sustained recovery

### Tier 3: Divine Grace
- **Group Blessing**: Heals selected target plus all allies within 2 tiles of target
- **Resurrection Touch**: Can target and revive recently fallen allies (within 10 seconds)
- **Perfect Timing**: Successful heal on ally below 25% health reduces cooldown by 2 seconds
- **Master Support**: Transforms single-target heal into powerful group restoration tool

## Positioning and Team Coordination

### Optimal Cardinal Placement
- **Central Coverage**: Position to maximize 8x8 grid coverage of team formation
- **Tank Support**: Maintain range to primary tank for consistent healing access
- **Line of Sight**: Ensure clear view of all allies for targeting and channel completion
- **Safety Positioning**: Balance healing range with personal safety from enemy threats

### Team Formation Benefits
- **Cluster Healing**: Tight formations ensure all allies within healing range
- **Tank Priority**: Damaged tanks typically receive healing priority due to health pool size
- **Support Chain**: Other support characters receive healing to maintain team sustainability
- **Damage Dealer Care**: Glass cannon allies benefit from smart targeting when injured

## Visual & Audio Design

### Lighting Design Philosophy
The Cardinal's Heal ability employs **divine radiance lighting** that evokes warmth, comfort, and spiritual restoration. The color palette centers on **warm golds and soft whites** to convey benevolent divine power and healing energy.

**Technical Implementation:**
- **Key Light**: Warm, omnidirectional lighting from Cardinal position (360-degree diffusion)
- **Fill Light**: Soft ambient lighting with increased intensity during healing
- **Divine Light**: Volumetric lighting column connecting Cardinal to target
- **Color Temperature**: 2700K-3500K range for warm, comforting feeling
- **PBR Materials**: High emission values (2.0-4.0) with soft falloff curves for divine energy

### Target Selection

**Lighting Design:**
- **Target Highlighting**: Warm golden rim light (4-unit intensity) around selected ally
- **Range Visualization**: Subtle golden particle field showing 8x8 grid coverage
- **Cardinal Aura**: Soft omnidirectional glow (RGB: 1.0, 0.8, 0.4, Intensity: 1.5)
- **Performance**: Baked area lighting for grid, dynamic rim light for target

**Visual Effects:**
- **Health Assessment**: Color-coded health bar overlays (red→yellow→green gradient)
- **Selection Indicator**: Gentle golden particle ring around target (30 particles, 2-second cycle)
- **UI Enhancement**: Soft-glow typography with divine symbol watermarks
- **Shader Optimization**: Screen-space selection outline, no geometry duplication

**Audio Design:**
- **Target Lock**: Gentle bell chime (C5 note, 0.3s duration, 0.4 volume)
- **Harmonic Layer**: Subtle choir pad (C major chord, sustained background)
- **Spatial Audio**: Omnidirectional source at Cardinal position, 12-unit range
- **Performance**: Compressed streaming for choir, cached samples for bells

### Channel Phase

**Lighting Design:**
- **Divine Column**: Volumetric light beam (Intensity: 8, Height: 10 units, Width: 1 unit)
- **Energy Buildup**: Graduated light intensity (0.0→8.0) over 0.5-second channel
- **Color Progression**: Soft gold (RGB: 1.0, 0.8, 0.4) to brilliant white (RGB: 1.0, 1.0, 0.9)
- **Sacred Geometry**: Subtle caustic patterns projected from Cardinal's hands
- **Performance**: Single volumetric light with animated intensity curve

**Visual Effects:**
- **Hand Positioning**: Cardinal's arms raised with palms facing target
- **Energy Manifestation**: 50-75 golden particles spiraling upward from hands
- **Robe Animation**: Cloth simulation with divine wind forces applied
- **Symbol Appearance**: Rotating holy symbols (5 total) orbiting Cardinal at 2-unit radius

**Audio Design:**
- **Divine Harmony**: Building orchestral chord progression (C-F-G-C)
- **Vocal Layer**: Ethereal choir with growing intensity
- **Frequency Range**: Fundamental at 261Hz (C4) with harmonic overtones
- **Performance**: Layered audio with crossfaded intensity stages

### Healing Application

**Lighting Design:**
- **Healing Flash**: Brilliant white burst (Intensity: 15, Duration: 0.2s)
- **Target Illumination**: Target becomes light source (RGB: 1.0, 0.9, 0.7, Intensity: 6.0)
- **Radiance Spread**: Light expands from target in 3-unit radius
- **Divine Blessing**: Soft golden afterglow (2-second fade to ambient)
- **Performance**: Burst lighting uses light cookies for consistent pattern

**Visual Effects:**
- **Particle Shower**: 100-150 golden sparkles descending on target
- **Health Restoration**: Green healing numbers with golden particle trail
- **Target Glow**: Full-body luminescence with soft edge falloff
- **Blessing Aura**: Expanding golden ring effect (0.5→3.0 units over 1 second)

**Audio Design:**
- **Divine Bell**: Crystal-clear chime (C6 note, 0.5s duration)
- **Harmonic Resolution**: Major chord progression resolving to tonic
- **Frequency Spectrum**: Bright harmonics (1-8kHz) for divine clarity
- **Performance**: High-quality samples with natural reverb tail

### Post-Healing Effects

**Lighting Design:**
- **Blessing Afterglow**: Soft golden aura around healed ally (RGB: 1.0, 0.8, 0.4, Intensity: 0.8)
- **Fade Curve**: Exponential decay over 3-second duration
- **Ambient Boost**: Temporary increase in local ambient lighting
- **Performance**: Animated material emission, no additional light sources

**Visual Effects:**
- **Lingering Sparkles**: 20-30 particles with slow upward drift
- **Health Bar Animation**: Smooth green fill with golden highlight
- **Status Indicator**: Subtle divine blessing icon above target
- **Accessibility**: Health restoration clearly visible in colorblind-safe palette

**Audio Design:**
- **Echo Harmony**: Reverberant choir fade with 2-second decay
- **Comfort Tone**: Sustained warm pad (C major chord, 3-second fade)
- **Frequency Warmth**: Emphasized low-mid frequencies (200-800Hz)
- **Performance**: Convolution reverb for realistic sacred space acoustics

### Performance Optimization for Mass Combat

**Lighting Optimization:**
- **Healing Light LOD**: 
  - **High (0-15 units)**: Full volumetric lighting and shadows
  - **Medium (15-40 units)**: Point lights only, no volumetrics
  - **Low (40+ units)**: Emissive materials only, no dynamic lighting
- **Light Culling**: Cardinal healing lights disabled when not visible
- **Batch Processing**: Similar healing effects rendered in single pass

**Visual Effects Optimization:**
- **Particle Reduction**: Scaled particle counts based on camera distance
- **Shader LOD**: Simplified shaders for background healing effects
- **Effect Pooling**: Pre-allocated pools for 20 concurrent healing effects
- **Texture Streaming**: Divine symbols use compressed texture streaming

**Audio Optimization:**
- **Voice Management**: Maximum 4 concurrent healing audio sources
- **Distance Attenuation**: Healing audio fades beyond 25-unit range
- **Dynamic Range**: Compressed audio for consistent volume levels
- **Memory Optimization**: Shared audio buffers for similar healing sounds

### Deterministic Recording Compatibility

**Visual Synchronization:**
- **Component-Driven Animation**: All effects triggered by healing component state
- **Deterministic Particles**: Seeded random number generation for particle placement
- **Frame-Locked Timing**: Visual progression based on simulation steps
- **Replay Accuracy**: Identical visual sequence across all replay instances

**Audio Synchronization:**
- **Event-Based Triggers**: Audio cues linked to specific healing events
- **Position Precision**: Audio sources track exact entity positions
- **Timing Consistency**: Audio scheduling based on fixed timestep updates
- **Replay Fidelity**: Synchronized audio playback in all replay modes

### Accessibility Considerations

**Visual Accessibility:**
- **Colorblind Support**: Golden effects accompanied by brightness variations
- **High Contrast**: Alternative blue-white palette for better visibility
- **Motion Reduction**: Option to disable particle effects while maintaining core visuals
- **Text Scaling**: Healing numbers support up to 300% size scaling

**Audio Accessibility:**
- **Hearing Impaired**: Visual indicators for all audio cues
- **Frequency Alternatives**: Lower frequency healing tones available
- **Subtitle Support**: Text descriptions for divine audio cues
- **Volume Independence**: Visual clarity maintained at all audio levels

### Cardinal Class Visual Identity

The Heal ability establishes the Cardinal's **divine radiance** visual language:
- **Primary Colors**: Warm gold (RGB: 1.0, 0.8, 0.4) and soft white (RGB: 1.0, 1.0, 0.9)
- **Secondary Accents**: Sacred purple (RGB: 0.6, 0.4, 0.8) for enhanced abilities
- **Material Palette**: High emission surfaces with soft, organic falloff patterns
- **Lighting Character**: Omnidirectional, warm lighting emphasizing comfort and protection
- **Animation Style**: Fluid, graceful movements with divine wind effects

### Sacred Geometry Integration

**Divine Patterns:**
- **Healing Circles**: Sacred geometric patterns appear during channeling
- **Symbol Rotation**: Holy symbols rotate at mathematically significant ratios (φ golden ratio)
- **Light Refraction**: Caustic patterns based on medieval stained glass designs
- **Fractal Elements**: Self-similar patterns in particle distribution for divine complexity