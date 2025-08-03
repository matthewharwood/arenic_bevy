# Acid Flask

A complete implementation guide for the Alchemist's area denial offensive ability.

## Overview

The **Acid Flask** is a tactical area-of-effect ability that creates persistent damage zones on the battlefield. The Alchemist throws a volatile flask of corrosive acid onto a targeted tile, creating a puddle that deals damage over time to any enemy passing through. This ability excels at controlling enemy movement paths and maximizing damage through predictive placement on high-traffic areas.

## Game Design Philosophy

This ability exemplifies strategic offensive design through area control and timing mastery:

**Predictive Targeting Over Reactive**: Success requires understanding enemy pathing and timing rather than twitch reflexes. Players must study enemy routes and place acid where enemies will be, not where they are.

**Persistent Area Control**: The acid pool remains active for its full duration regardless of enemy interaction, creating ongoing tactical value and battlefield control.

**Risk-Reward Positioning**: Effective placement requires the Alchemist to position themselves for optimal throw angles, creating interesting risk-reward decisions about battlefield positioning.

## Implementation Architecture

### Component-Based Design

```rust
AcidFlask {
    throw_range: 6.0,           // 6 tile maximum throw distance
    pool_duration: 15.0,        // 15 second acid pool lifetime
    damage_per_tick: 25.0,      // Damage every 0.5 seconds
    tick_interval: 0.5,         // Damage application frequency
    pool_radius: 1.5,           // 1.5 tile radius coverage
    cooldown: 8.0,              // 8 second ability cooldown
}

AcidPool {
    position: Vec2,
    remaining_duration: f32,
    damage_per_tick: f32,
    affected_tiles: HashSet<GridPos>,
}
```

### Event-Driven Systems

The ability utilizes five coordinated systems:
1. **Targeting System** - Handles aim assist and valid placement detection
2. **Projectile Physics** - Manages flask arc trajectory and impact
3. **Pool Creation** - Spawns acid pool on flask impact
4. **Damage Application** - Applies DOT to enemies in affected area
5. **Environmental Cleanup** - Removes expired pools and restores tiles

## Step-by-Step Gameplay

### Phase 1: Targeting (Tap and Hold)
- **Input Method**: Tap and hold to enter targeting mode
- **Visual Feedback**: Targeting reticle appears with range indicator
- **Strategic Planning**: Analyze enemy pathing for optimal placement
- **Range Limitation**: 6-tile maximum range prevents overextension

### Phase 2: Trajectory (Hold Duration)
- **Arc Visualization**: Parabolic line shows projected throw path
- **Obstacle Detection**: Red indicators show blocked trajectories  
- **Timing Window**: Hold duration doesn't affect throw power
- **Commitment Point**: Release triggers immediate throw with no cancellation

### Phase 3: Flask Flight (0.8 Second Travel)
- **Projectile Physics**: Flask follows realistic arc with gravity
- **Visual Trail**: Green particle trail indicates corrosive contents
- **Audio Feedback**: Glass container whistling through air
- **Impact Anticipation**: Landing zone highlighted during flight

### Phase 4: Pool Formation (Instant Impact)
- **Splash Effect**: Flask shatters with acidic splash animation
- **Pool Creation**: 1.5-tile radius acid pool forms immediately
- **Tile Marking**: Affected grid squares show acid texture overlay
- **Damage Activation**: DOT begins immediately upon pool formation

### Phase 5: Persistent Damage (15 Second Duration)
- **Tick Damage**: 25 damage every 0.5 seconds to occupying enemies
- **Movement Independence**: Enemies continue normal movement patterns
- **Visual Persistence**: Bubbling acid animation throughout duration
- **Strategic Value**: Forces enemy routing or accepts damage penalty

## Upgrade Paths

### Tier 1: Concentrated Acid
- **Damage Increase**: 25 damage per tick → 35 damage per tick
- **Visual Enhancement**: Deeper green coloration with more aggressive bubbling
- **Strategic Impact**: Increases punishment for enemies using affected routes
- **Balance Consideration**: Higher damage makes positioning even more critical

### Tier 2: Expanding Puddle
- **Radius Growth**: Pool expands from 1.5 → 2.5 tiles over first 3 seconds
- **Tactical Evolution**: Creates larger area denial zones
- **Visual Design**: Acid spreads outward with flowing animation
- **Strategic Depth**: Rewards placement near chokepoints for maximum expansion

### Tier 3: Caustic Chain Reaction
- **Chain Effect**: Enemy death in acid pool creates new 1-tile pool at death location
- **Duration**: Chain pools last 8 seconds with reduced damage (15 per tick)
- **Strategic Complexity**: Creates potential for cascading area denial
- **Visual Spectacle**: Chain reactions marked with bright green splash effects

## Visual & Audio Design

### Lighting Design Philosophy
The Alchemist's Acid Flask employs **corrosive chemistry lighting** that emphasizes dangerous chemical reactions and toxic hazards. The color palette centers on **sickly greens and caustic yellows** to convey chemical danger and environmental contamination.

**Technical Implementation:**
- **Key Light**: Harsh, localized lighting from acid pool (concentrated illumination)
- **Fill Light**: Sickly green ambient bounce from affected surfaces
- **Caustic Light**: Animated light patterns simulating acid bubble refractions
- **Color Temperature**: 5500K-7000K range for harsh, unnatural chemical feeling
- **PBR Materials**: High metallic values (0.6-0.8) with animated roughness for liquid surfaces

### Targeting Phase

**Lighting Design:**
- **Range Illumination**: Dim green spotlights showing throw range boundaries
- **Target Zone**: Pulsing caustic light at aimed location (RGB: 0.4, 0.8, 0.2, Intensity: 2.0)
- **Flask Preparation**: Alchemist's hands glow with green chemical residue
- **Performance**: Static range lighting, dynamic target marker with animated caustics

**Visual Effects:**
- **Trajectory Visualization**: Dotted green arc showing flask flight path
- **Area Indicator**: Translucent green circle with caustic edge effects
- **Flask Animation**: Glass container with swirling liquid and vapor wisps
- **Shader Optimization**: Screen-space trajectory overlay, minimal geometry

**Audio Design:**
- **Flask Preparation**: Glass clink with liquid sloshing (200-400Hz)
- **Chemical Reaction**: Subtle bubbling and fizzing sounds
- **Spatial Audio**: Point source at Alchemist position, 6-unit range
- **Performance**: Looped liquid sounds with randomized pitch variations

### Projectile Phase

**Lighting Design:**
- **Flask Illumination**: Moving green point light following projectile (Intensity: 3.0)
- **Chemical Glow**: Visible liquid contents emit caustic green light
- **Trail Lighting**: Fading light trail marking flight path
- **Performance**: Single dynamic light with distance-based intensity scaling

**Visual Effects:**
- **Glass Flask**: Realistic glass material with visible liquid interior
- **Particle Trail**: 10-15 green droplets following arc trajectory
- **Vapor Effects**: Wisps of green vapor trailing behind flask
- **Liquid Motion**: Animated liquid surface responding to projectile physics

**Audio Design:**
- **Flight Sound**: Glass whistling through air (1-3kHz frequency)
- **Liquid Audio**: Periodic sloshing with Doppler effect
- **3D Tracking**: Audio source follows projectile with spatial accuracy
- **Performance**: Compressed audio with real-time pitch shifting

### Impact and Pool Phase

**Lighting Design:**
- **Shatter Flash**: Brief bright green burst (Intensity: 12, Duration: 0.1s)
- **Pool Illumination**: Persistent area light covering 1.5-tile radius (RGB: 0.3, 0.7, 0.1)
- **Caustic Patterns**: Animated refractive patterns on nearby surfaces
- **Bubble Lighting**: Randomized light intensity variations simulating chemical activity
- **Performance**: Area light with animated light cookies for caustic effects

**Visual Effects:**
- **Glass Explosion**: 20-30 glass fragments with realistic physics
- **Acid Splash**: 40-60 green droplets with corrosive particle effects
- **Pool Formation**: Expanding liquid surface with animated meniscus edges
- **Environmental Damage**: Progressive corrosion texture on affected tiles

**Audio Design:**
- **Glass Shatter**: Sharp crack with high-frequency components (3-8kHz)
- **Acid Splash**: Liquid impact with chemical sizzling (broadband noise)
- **Ongoing Bubbling**: Continuous random bubble pops and fizzing
- **Performance**: Layered audio with convolution reverb for environmental response

### Damage Application

**Lighting Design:**
- **Damage Flash**: Brief red-orange light indicating acid burns (Intensity: 5.0)
- **Enemy Illumination**: Affected enemies gain green caustic outline lighting
- **Pool Activity**: Intensified bubble lighting when enemies take damage
- **Performance**: Temporary damage lights with 0.2-second duration

**Visual Effects:**
- **Corrosion Damage**: Green particle bursts at enemy contact points
- **Health Reduction**: Bright green damage numbers with acid drop styling
- **Enemy State**: Green caustic aura around affected enemies
- **Pool Interaction**: Additional bubbling and vapor when enemies enter

**Audio Design:**
- **Burn Sound**: Sizzling audio with organic damage undertones (400-2kHz)
- **Pain Response**: Enemy audio reactions with chemical burn characteristics
- **Pool Interaction**: Enhanced bubbling when damage is dealt
- **Performance**: Damage audio with dynamic range compression

### Pool Persistence (15-Second Duration)

**Lighting Design:**
- **Activity Cycles**: Light intensity varies with bubble activity (1.0-3.0 intensity)
- **Chemical Glow**: Gradual dimming over duration (exponential decay curve)
- **Caustic Animation**: Rotating caustic patterns at 0.2 rotations/second
- **Performance**: Animated shader parameters, no additional light sources after minute 1

**Visual Effects:**
- **Continuous Bubbling**: 5-10 bubbles per second with randomized timing
- **Vapor Wisps**: Occasional green vapor rising from pool surface
- **Liquid Surface**: Animated normal maps creating realistic acid texture
- **Environmental Wear**: Progressive tile damage accumulating over time

**Audio Design:**
- **Ambient Bubbling**: Low-level chemical activity (50-200Hz)
- **Periodic Pops**: Random bubble bursts every 0.5-2.0 seconds
- **Chemical Hiss**: Subtle acid evaporation sound (high-frequency noise)
- **Performance**: Looped ambient with randomized one-shot overlays

### Performance Optimization for Mass Combat

**Lighting Optimization:**
- **Pool Light LOD**:
  - **High (0-25 units)**: Full caustic effects and dynamic shadows
  - **Medium (25-50 units)**: Simple area light, no caustics
  - **Low (50+ units)**: Emissive materials only, no dynamic lighting
- **Light Culling**: Acid pool lights disabled when outside camera frustum
- **Batch Rendering**: Multiple acid pools rendered with instanced geometry

**Visual Effects Optimization:**
- **Particle Scaling**: Bubble count reduced by distance from camera
- **Shader LOD**: Simplified caustic shaders for distant pools
- **Pool Pooling**: Pre-allocated acid pool effect objects (maximum 15 concurrent)
- **Texture Compression**: Acid textures use BC7 compression for quality/size balance

**Audio Optimization:**
- **Audio LOD**: Simplified audio beyond 20-unit range from camera
- **Voice Limiting**: Maximum 6 concurrent acid pool audio sources
- **Streaming Audio**: Long-duration ambient tracks use compressed streaming
- **Memory Management**: Shared audio buffers for similar chemical sounds

### Deterministic Recording Compatibility

**Visual Synchronization:**
- **Physics Determinism**: Flask trajectory calculated using fixed-point math
- **Bubble Timing**: Randomized effects use seeded RNG for consistency
- **Pool Formation**: Impact location precisely determined by collision system
- **Replay Accuracy**: Identical visual sequence across all replay instances

**Audio Synchronization:**
- **Event Triggers**: Audio cues tied to specific physics and damage events
- **Position Tracking**: Audio sources update with exact entity coordinates
- **Timing Precision**: Audio scheduling synchronized with simulation timesteps
- **Replay Fidelity**: Consistent audio playback in all replay modes

### Accessibility Considerations

**Visual Accessibility:**
- **Colorblind Support**: Green acid effects include brightness intensity variations
- **High Contrast**: Alternative orange-red palette for improved visibility
- **Motion Sensitivity**: Reduced bubble animations option available
- **Text Scaling**: Damage numbers support 150%-250% size scaling

**Audio Accessibility:**
- **Hearing Impaired**: Visual bubble indicators for audio cues
- **Frequency Alternatives**: Lower frequency chemical sounds available
- **Subtitle Support**: Text descriptions for environmental audio
- **Volume Independence**: Visual hazard clarity maintained at any audio level

### Alchemist Class Visual Identity

The Acid Flask ability establishes the Alchemist's **chemical science** visual language:
- **Primary Colors**: Caustic green (RGB: 0.3, 0.7, 0.1) and toxic yellow (RGB: 0.8, 0.8, 0.2)
- **Secondary Accents**: Chemical orange (RGB: 0.9, 0.5, 0.1) for reactions and danger
- **Material Palette**: Glass, liquid, and corroded metal surfaces with realistic wear
- **Lighting Character**: Harsh, localized lighting emphasizing chemical danger
- **Animation Style**: Scientific precision with realistic chemical reaction physics

### Environmental Interaction

**Surface Chemistry:**
- **Material Reactions**: Different tile types show appropriate corrosion patterns
- **Caustic Spread**: Acid effects interact realistically with surface materials
- **Vapor Dynamics**: Realistic evaporation rates based on environmental temperature
- **Chemical Persistence**: Visual degradation follows real-world acid chemistry