# Holy Nova

A complete implementation guide for the Cardinal's divine area-of-effect ability that simultaneously heals allies and
damages enemies.

## Overview

**Holy Nova** represents the Cardinal's signature divine intervention through explosive sacred energy. This
area-of-effect ability creates a brilliant nova of holy power centered on the Cardinal, healing all allied units within
range while simultaneously damaging enemy targets in the same area. The dual nature of this ability creates compelling
tactical decisions around positioning, timing, and risk management that define high-level Cardinal play.

## Game Design Philosophy

Holy Nova exemplifies **high-risk, high-reward positioning design** that transforms the Cardinal from a passive
supporter into an active tactical participant:

**Tactical Positioning**: Forces Cardinals to make calculated decisions about proximity to combat, balancing safety with
healing efficiency.

**Dual-Purpose Impact**: Simultaneous healing and damage creates opportunities for game-changing moments where
positioning determines massive swing potential.

**Team Coordination Catalyst**: Requires team coordination around Cardinal positioning, creating natural interdependence
and communication opportunities.

**Mastery Through Risk**: Rewards players who master positioning mechanics with powerful impact, while providing clear
counterplay opportunities for opponents.

## Mechanical Innovation Analysis

### First-Principles Design

Rather than traditional "point and click" healing, Holy Nova reimagines support as **spatial control**. The ability
asks: "What if healing required the same positional mastery as dealing damage?" This creates a new skill expression
where support players must read battlefield flow and position proactively rather than reactively.

### Skill Atom Architecture

- **Action/Trigger**: Cardinal activates nova (tap/click input)
- **Simulation**: System calculates all units within 2-tile radius, applies healing/damage based on allegiance
- **Feedback**: Visual nova explosion with distinct healing/damage effects on targets
- **Insight**: Players learn that positioning near combat zones amplifies impact but increases risk

### Strategic Innovation

Holy Nova introduces **proximity-based support efficiency**, where getting closer to danger increases healing potential.
This inverts traditional support positioning wisdom and creates unique tactical scenarios.

## Implementation Architecture

### Component-Based Design (Single-Use Pattern)

```rust
// Holy Nova ability composition (following holy_nova.rs pattern)
commands.spawn((
    HolyNova,                           // Marker
    Radius(2.0 * TILE_SIZE),            // 2-tile radius
    Healing(120.0),                     // Heal amount per ally
    Damage(80.0),                       // Damage per enemy
    ChannelTime(0.8),                   // Channel duration
    Cooldown(9.0),                      // Cooldown duration
    ManaCost(40.0),                     // Mana required
    TargetSelf,                         // Centered on caster
    IgnoresLineOfSight,                 // Penetrates obstacles
));

// Holy Nova pulse entity (spawned on cast completion)
commands.spawn((
    HolyNova,                           // Ability marker
    Pulse,                              // Pulse effect marker
    Origin(cardinal_pos),               // Center position
    Radius(2.0 * TILE_SIZE),            // Effect radius
    Healing(120.0),                     // Heal per ally
    Damage(80.0),                       // Damage per enemy
    Duration(0.1),                      // Brief pulse
    ElapsedTime(0.0),                   // Timer
));

// Holy Nova VFX entity (from holy_nova.rs)
commands.entity(character_entity).with_child((
    HolyNovaVfx,                        // VFX marker
    ElapsedTime(0.0),                   // Time tracking
    Duration(0.225),                    // VFX duration
    StartRadius(4.0),                   // Initial radius
    EndRadius(32.0),                    // Final radius
    Transform::from_scale(Vec3::splat(4.0)),
    Mesh3d(vfx_mesh),
    MeshMaterial3d(mats.yellow.clone()),
));
```

### Event-Driven Systems

The ability operates through six integrated systems:

1. **Area Calculation** - Determines all units within 2-tile radius of Cardinal
2. **Allegiance Processing** - Separates targets into allies (heal) and enemies (damage)
3. **Cast Management** - Handles 0.8-second channel with movement/damage interruption
4. **Simultaneous Application** - Applies healing and damage simultaneously upon completion
5. **Resource Management** - Tracks mana consumption and cooldown timing
6. **Visual Coordination** - Manages explosive nova effects with distinct ally/enemy feedback

## Step-by-Step Gameplay

### Phase 1: Area Preview (Pre-Activation)

- **Range Visualization**: 2-tile radius highlighted around Cardinal position
- **Target Assessment**: All allies and enemies within area marked with preview indicators
- **Efficiency Calculation**: UI shows potential healing total and damage total
- **Positioning Feedback**: Visual cues indicate optimal positioning opportunities

### Phase 2: Cast Initiation (Activation)

- **Input Method**: Single tap begins 0.8-second channeling ritual
- **Mana Check**: Verify sufficient mana (40 MP) before initiating cast
- **Channel Start**: Cardinal raises both hands skyward, divine energy building
- **Vulnerability Window**: Cardinal becomes stationary and vulnerable to interruption

### Phase 3: Channel Buildup (0.8 Second Duration)

- **Uninterrupted Channel**: Movement or damage taken cancels the ability
- **Visual Buildup**: Brilliant white-gold energy sphere expands around Cardinal
- **Audio Crescendo**: Divine chorus builds to crescendo with harmonic resonance
- **Strategic Tension**: Enemies can attempt to interrupt or flee the area

### Phase 4: Nova Explosion (Instant Resolution)

- **Simultaneous Impact**: All targets receive healing or damage instantly
- **Visual Detonation**: Brilliant explosion of divine light affecting all targets
- **Audio Climax**: Thunderous divine resonance with distinct ally/enemy audio cues
- **Battle State Change**: Potentially massive health swings across multiple units

## Area-of-Effect Mechanics

### Targeting Algorithm

```rust
fn calculate_holy_nova_targets(cardinal_pos: GridPos) -> (Vec<Entity>, Vec<Entity>) {
    let affected_tiles = get_tiles_in_radius(cardinal_pos, 2);
    let all_units: Vec<Entity> = affected_tiles.iter()
        .flat_map(|tile| get_units_on_tile(*tile))
        .filter(|unit| is_alive(**unit))
        .collect();

    let (allies, enemies): (Vec<Entity>, Vec<Entity>) = all_units.iter()
        .partition(|unit| is_ally(**unit, cardinal_pos.team));

    // Exclude the Cardinal from healing (cannot self-target)
    let heal_targets = allies.into_iter()
        .filter(|ally| **ally != cardinal_entity)
        .collect();

    (heal_targets, enemies)
}

fn calculate_efficiency_bonus(heal_count: usize, damage_count: usize) -> f32 {
    let total_targets = heal_count + damage_count;
    match total_targets {
        0..=1 => 1.0,      // No bonus for minimal impact
        2..=3 => 1.1,      // 10% bonus for good positioning
        4..=5 => 1.25,     // 25% bonus for excellent positioning
        6.. => 1.4,        // 40% bonus for perfect positioning
    }
}
```

### Area Coverage

- **Core Radius**: 2-tile radius creates 5x5 grid (25 total tiles)
- **Center Position**: Cardinal occupies center tile, cannot self-heal
- **Obstacle Penetration**: Holy energy penetrates walls, cover, and line-of-sight blockers
- **Elevation Independence**: Works across different height levels within range
- **Precise Boundaries**: Uses true geometric distance, not Manhattan distance

## Player Psychology & Engagement

### Risk/Reward Psychology

Holy Nova leverages **loss aversion** by requiring Cardinals to risk personal safety for maximum healing impact. The
fear of losing position safety feels twice as significant as the potential healing gains, creating memorable high-stakes
moments.

### Positioning Mastery

The ability creates **competence satisfaction** through spatial skill development. Players must learn battlefield
reading, timing, and positioning - skills that transfer to tactical understanding beyond just this ability.

### Social Coordination

Holy Nova demands **team interdependence** - allies must coordinate around Cardinal positioning, creating natural
communication and strategy formation. This builds **relatedness** through shared tactical decisions.

### Flow State Engineering

The 0.8-second channel creates perfect **challenge-skill balance**: long enough to create tension and allow counterplay,
short enough to maintain action flow. The timing rewards precise battlefield reading while punishing hesitation.

## Advanced Tactical Applications

### Positioning Strategies

- **Frontline Integration**: Advanced Cardinals position near melee combat for maximum ally coverage
- **Chokepoint Control**: Use nova threat to control enemy movement through narrow passages
- **Team Fight Initiation**: Coordinate nova timing with team engagements for massive health swings
- **Defensive Clustering**: Group damaged allies near Cardinal for efficient group healing

### Counterplay Opportunities

- **Interrupt Targeting**: Enemies can focus fire during 0.8-second channel window
- **Area Denial**: Enemy positioning can force Cardinals into suboptimal nova placement
- **Mobility Counters**: Fast enemies can escape area during channel time
- **Resource Pressure**: High mana cost creates windows of vulnerability

### Mastery Indicators

- **Target Efficiency**: Expert players consistently hit 4+ targets per nova
- **Risk Calculation**: Masters position aggressively but time retreats perfectly
- **Battlefield Reading**: Advanced players predict enemy movement for optimal placement
- **Resource Management**: Experts time novas for maximum battle impact per mana spent

## Upgrade Paths

### Tier 1: Radiant Expansion

- **Range Increase**: 2-tile radius → 2.5-tile radius (7x7 grid coverage)
- **Cast Speed**: 0.8 seconds → 0.6 seconds channel time
- **Efficiency Scaling**: Target bonuses increased by 5% across all thresholds
- **Strategic Value**: Larger area control with faster execution for safer positioning

### Tier 2: Divine Resonance

- **Healing Over Time**: Allies gain 15 HP/second for 6 seconds after nova
- **Damage Over Time**: Enemies take 8 damage/second for 6 seconds after nova
- **Resonance Chain**: Targets hit by nova extend effects to adjacent allies/enemies
- **Enhanced Impact**: Transforms burst ability into sustained battlefield control

### Tier 3: Celestial Convergence

- **Sacred Ground**: Nova area becomes blessed terrain for 15 seconds
- **Ally Buffs**: Blessed ground provides +10% damage and +15% healing received
- **Enemy Debuffs**: Blessed ground applies -10% damage dealt to enemies
- **Area Persistence**: Creates long-term tactical value and battlefield shaping

## Balance Considerations

### Power Level Scaling

- **Base Power**: 120 healing / 80 damage provides meaningful but not overwhelming impact
- **Mana Efficiency**: 40 MP cost requires careful resource management (3-4 casts per full bar)
- **Cooldown Pacing**: 9-second cooldown prevents spam while allowing 2-3 uses per battle
- **Risk Premium**: Vulnerability during channel balances high potential impact

### Comparative Analysis

- **vs Single Heal**: Nova trades guaranteed safety for potential multi-target impact
- **vs Damage Abilities**: Lower damage than pure offensive spells, balanced by healing component
- **vs Support Abilities**: Higher risk than passive support, rewarded with higher potential impact
- **vs Team Resources**: High mana cost forces strategic timing rather than constant use

### Scaling Mechanisms

- **Level Scaling**: +8 healing and +5 damage per Cardinal level
- **Equipment Scaling**: Divine focus items provide +15% efficiency bonus
- **Mastery Scaling**: Target count bonuses reward positioning skill development
- **Team Scaling**: Effectiveness scales with team coordination and communication

## Implementation Guidelines

### Technical Specifications

```rust
// Core damage/healing calculation
fn apply_holy_nova_effects(targets: &HolyNovaTargeting, efficiency: f32) {
    for ally in &targets.affected_allies {
        let heal_amount = 120.0 * efficiency;
        apply_healing(*ally, heal_amount);
        trigger_heal_effect(*ally, HealType::Divine);
    }

    for enemy in &targets.affected_enemies {
        let damage_amount = 80.0 * efficiency;
        apply_damage(*enemy, damage_amount, DamageType::Holy);
        trigger_damage_effect(*enemy, DamageType::Holy);
    }
}

// Efficiency calculation based on positioning mastery
fn calculate_positioning_mastery(heal_count: usize, damage_count: usize, risk_level: f32) -> f32 {
    let base_bonus = calculate_efficiency_bonus(heal_count, damage_count);
    let risk_bonus = risk_level * 0.2; // Up to 20% bonus for high-risk positioning
    (base_bonus + risk_bonus).min(1.6) // Cap at 60% total bonus
}
```

### Visual Effect Guidelines

- **Nova Core**: Brilliant white-gold explosion expanding from Cardinal position
- **Ally Effects**: Warm golden light with upward-floating healing particles
- **Enemy Effects**: Searing white light with downward-striking damage effects
- **Area Indicators**: Clear visual boundaries for 2-tile radius during channel
- **Performance**: Optimized particle systems for up to 8 simultaneous target effects

### Audio Design Philosophy

- **Channel Buildup**: Rising orchestral swell with divine choir harmonics
- **Nova Explosion**: Thunderous divine resonance with crystalline bell overlay
- **Ally Healing**: Gentle chimes and warm harmonic resolution
- **Enemy Damage**: Sharp, piercing tones with minor chord dissonance
- **Spatial Audio**: Omnidirectional source with distance-appropriate falloff

### Animation Timing

- **Pre-cast (0.2s)**: Cardinal raises hands, energy begins gathering
- **Channel (0.8s)**: Energy sphere grows, reaching maximum intensity
- **Explosion (0.1s)**: Instantaneous burst affecting all targets
- **Resolution (0.3s)**: Individual target effects play out
- **Cooldown (9.0s)**: Cardinal cannot cast nova again

## Community & Ecosystem Impact

### Prosocial Design Elements

- **Interdependence Creation**: Teams must coordinate positioning around Cardinal
- **Shared Success**: Effective novas create visible positive outcomes for multiple allies
- **Communication Catalyst**: Ability requires callouts and coordination
- **Mastery Sharing**: Expert positioning techniques become community knowledge

### Anti-Toxic Design

- **No Griefing Potential**: Cannot negatively affect allies (healing is always beneficial)
- **Clear Counterplay**: Enemies have obvious interruption and avoidance options
- **Skill Expression**: Success depends on player skill, not RNG or pay-to-win mechanics
- **Positive Sum**: Creates more healing than damage, promoting engagement over avoidance

### Long-term Engagement

- **Mastery Curve**: High skill ceiling through positioning optimization
- **Meta Evolution**: Strategies evolve as community learns optimal formations
- **Social Recognition**: Exceptional nova usage becomes recognized achievement
- **Replay Value**: Each battle presents unique positioning challenges

## Visual & Audio Design

### Lighting Design Philosophy

Holy Nova employs **explosive divine radiance** that contrasts sharply with the previous heal's gentle glow. The effect
emphasizes **power and impact** through brilliant white-gold illumination that momentarily dominates the battlefield.

**Technical Implementation:**

- **Nova Core**: Ultra-bright point light (Intensity: 25) expanding to 8-unit radius over 0.1 seconds
- **Area Illumination**: Omnidirectional burst affecting all surfaces within 2-tile range
- **Dynamic Shadows**: Sharp shadow casting during explosion for dramatic contrast
- **Color Palette**: Pure white core (RGB: 1.0, 1.0, 1.0) transitioning to warm gold edges (RGB: 1.0, 0.8, 0.4)
- **PBR Materials**: Maximum emission values (8.0+) with rapid falloff for explosive feel

### Channel Phase Lighting

**Gathering Energy:**

- **Cardinal Aura**: Growing omnidirectional light (0.5→6.0 intensity over 0.8 seconds)
- **Energy Accumulation**: Expanding light sphere with increasing brightness
- **Color Progression**: Soft gold building to brilliant white core
- **Environmental Response**: Nearby surfaces increasingly illuminated
- **Performance**: Single expanding point light with animated intensity curve

**Visual Effects:**

- **Energy Manifestation**: 200+ brilliant particles spiraling inward toward Cardinal
- **Sacred Symbols**: Rotating divine geometry orbiting at 3-unit radius
- **Ground Effects**: Runic circles appearing beneath Cardinal's feet
- **Anticipation Building**: Progressive brightening of Cardinal's entire model

### Nova Explosion Phase

**Detonation Lighting:**

- **Core Burst**: Instantaneous maximum intensity explosion (Intensity: 25, Duration: 0.1s)
- **Radial Expansion**: Light wave expanding outward at 40 units/second
- **Area Saturation**: All surfaces within range receive full illumination
- **Shadow Casting**: Sharp, dramatic shadows cast outward from nova center
- **Performance**: Multiple expanding lights with synchronized timing

**Target-Specific Effects:**

- **Healed Allies**: Warm golden illumination with gentle upward light rays
- **Damaged Enemies**: Harsh white light with downward-striking effects
- **Dual Rendering**: Simultaneous healing/damage lighting on all targets
- **Performance Optimization**: Shared light calculations for similar effects

**Visual Effects:**

- **Nova Explosion**: 500+ particles expanding in perfect sphere
- **Target Impacts**: Distinct particle systems for healing vs damage
- **Environmental Interaction**: Light interacts with dust, fog, and atmospheric effects
- **Screen Effects**: Brief screen-space brightening for explosive impact

### Audio Design Architecture

**Channel Phase Audio:**

- **Divine Buildup**: Orchestral swell with pipe organ foundation (C major scale ascending)
- **Choir Layer**: Ethereal voices building in volume and harmony complexity
- **Energy Resonance**: Subtle electrical/magical humming with increasing frequency
- **Spatial Design**: Omnidirectional source at Cardinal with 15-unit range
- **Performance**: Layered streaming audio with dynamic mixing

**Explosion Audio:**

- **Thunder Crash**: Massive orchestral hit with timpani and cymbals (fortissimo)
- **Divine Resonance**: Crystalline bell overlay with harmonic overtones
- **Frequency Range**: Full spectrum from 40Hz bass to 8kHz treble
- **Echo Environment**: 2-second reverb decay simulating divine space acoustics
- **Performance**: High-quality samples with convolution reverb

**Target-Specific Audio:**

- **Healing Sounds**: Gentle chimes in major key (C-E-G progression)
- **Damage Sounds**: Sharp, discordant tones in minor key (C-Eb-G progression)
- **Layered Playback**: Up to 8 simultaneous target audio sources
- **Distance Attenuation**: Individual target sounds with appropriate falloff

Prompt:
Divine AoE heal one‑shot with a soft rising shimmer and crystalline chime impact. Airy breath-like swell (0.6–0.8 s
build) into a gentle bright ‘glint’ burst (200–300 ms tail). High‑pass at 200–300 Hz, emphasize soothing upper mids (1–3
kHz) with sparkling air (8–12 kHz), smooth attack (no harsh transients), very light short room reverb (0.2–0.3 s, 8–12%
wet). Non‑melodic, neutral pitch center, no musical progression, no pads.

### Performance Optimization for Mass Combat

**Lighting Optimization:**

- **Nova Light LOD**:
    - **High (0-20 units)**: Full dynamic lighting with shadows
    - **Medium (20-50 units)**: Point lights only, no area illumination
    - **Low (50+ units)**: Emissive materials only, no dynamic lighting
- **Effect Batching**: Multiple simultaneous novas use shared lighting calculations
- **Culling System**: Off-screen nova effects use simplified rendering

**Visual Effects Optimization:**

- **Particle Scaling**: Particle count scales with distance and performance settings
- **Shader LOD**: Background nova effects use simplified shaders
- **Effect Pooling**: Pre-allocated pools for 10 concurrent nova effects
- **Memory Management**: Texture atlases for all nova visual components

**Audio Optimization:**

- **Voice Management**: Maximum 2 concurrent nova audio sources
- **Distance Culling**: Nova audio disabled beyond 30-unit range
- **Dynamic Compression**: Audio levels automatically adjusted for multiple novas
- **Memory Optimization**: Shared buffers for explosion and resonance sounds

### Accessibility Considerations

**Visual Accessibility:**

- **High Contrast Mode**: Alternative blue-white nova for improved visibility
- **Reduced Motion**: Option to minimize particle effects while maintaining core visuals
- **Colorblind Support**: Distinct brightness patterns for healing vs damage effects
- **Photosensitive Safety**: Configurable intensity limits for sensitive players

**Audio Accessibility:**

- **Visual Audio Cues**: Screen border flashing for explosion timing
- **Frequency Alternatives**: Lower frequency options for hearing-impaired players
- **Haptic Feedback**: Controller vibration patterns for nova phases
- **Subtitle Support**: Text descriptions of all divine audio components

### Cardinal Class Visual Evolution

Holy Nova establishes the Cardinal's **explosive divine power** identity:

- **Primary Colors**: Brilliant white core (RGB: 1.0, 1.0, 1.0) with warm gold accents
- **Power Expression**: High-intensity, area-dominating effects emphasizing impact
- **Material Evolution**: Maximum emission surfaces with sharp contrast ratios
- **Lighting Character**: Explosive, attention-commanding illumination
- **Animation Style**: Powerful, decisive movements with earth-shaking divine forces

This visual language differentiates from the gentle, targeted healing approach, positioning Holy Nova as the Cardinal's
signature battlefield-changing ability.

## Behavioral Economics Integration

### Scarcity and Timing

- **Long Cooldown**: 9-second timer creates scarcity, making each cast precious
- **Mana Investment**: 40 MP represents significant resource commitment
- **Optimal Timing Windows**: Players must identify perfect moments for maximum impact

### Loss Aversion Application

- **Positioning Risk**: Fear of losing safe positioning feels stronger than potential healing gains
- **Interruption Vulnerability**: 0.8-second channel creates anxiety around losing the cast
- **Opportunity Cost**: Missing optimal nova timing feels worse than successful moderate usage

### Social Proof Integration

- **Visible Impact**: Dramatic effects create social recognition for skilled usage
- **Team Coordination**: Success depends on team positioning, creating shared investment
- **Mastery Display**: Expert nova usage becomes community status symbol

### Ethical Motivation Enhancement

- **Intrinsic Mastery**: Success depends on skill development, not external rewards
- **Team Benefit**: Ability primarily helps others, fostering prosocial behavior
- **Clear Fairness**: Transparent mechanics with obvious counterplay options

## Conclusion

Holy Nova transforms the Cardinal from passive healer to active battlefield tactician, creating a signature ability that
rewards positioning mastery while demanding team coordination. The dual healing/damage nature creates unique tactical
scenarios impossible with traditional single-purpose abilities.

The design leverages proven psychological principles to create intrinsic motivation for skill development while
fostering positive team interactions. Through careful balance of risk and reward, Holy Nova becomes a high-mastery
ability that defines expert Cardinal play without creating oppressive gameplay patterns.

This ability exemplifies **prosocial game design** by creating interdependence, rewarding cooperation, and providing
clear counterplay options. The result is an ability that enhances community formation while maintaining competitive
integrity and individual skill expression.

## ECS Implementation in This Repo (Single-Use Components + Composition)

This repository implements gameplay using small, single-purpose components that are composed to form behavior. Rather
than large, stateful ability structs, we create short-lived entities with marker components and data-only components,
then drive them with focused systems. This mirrors how projectiles are implemented in src\main.rs.

Key patterns from src\main.rs to follow:

- Marker components for categorization:
    - Projectile (marker used to drive movement/despawn logic)
    - Character, Boss, Active (used for queries and selection)
- Data-only components for specific responsibilities:
    - TimeToLive(elapsed, total) — per-entity lifetime tracking
    - Origin(Vec3) and Target(Vec3) — movement endpoints for lerp
    - Transform — world position; systems write to translation
- Systems do one job:
    - move_projectiles: advances TTL, lerps Transform from Origin to Target, despawns at end
    - autoshot_ability: periodically spawns fully-formed projectile entities (composition at spawn)

Example of entity composition (from main.rs):

- A projectile is spawned with the full set of components it needs to exist and be simulated:
    - (Projectile, Transform, Origin, Target, TimeToLive, Mesh3d, MeshMaterial3d)
- The movement system only reads what it needs (Transform, TimeToLive, Origin, Target) and knows nothing about who
  spawned it.

Why this matters for abilities like Holy Nova:

- Abilities become either:
    1) Ephemeral entities with a short TimeToLive that perform area checks and effects while they exist; or
    2) Stateless “events” that cause immediate spawns/effects, with any visuals handled by short-lived entities.
- Systems remain small, composable, and cache-friendly; change detection and filters like With<T>/Without<T> keep
  queries efficient.

---

## How to Implement Holy Nova Using This Pattern

Goal: Centered AoE that heals allies and damages enemies within a 2-tile radius. We’ll use short-lived entities and
single-use components so that systems stay decoupled.

1. Define minimal single-purpose components

- HolyNovaPulse — marker for the transient nova entity
- Radius(f32) — world-units radius; if you prefer tile units, keep a TileRadius(u8) and convert once
- HealAmount(f32), DamageAmount(f32)
- TimeToLive(f32 elapsed, f32 total) — e.g., 0.0, 0.1 for a brief pulse lifetime
- Origin(Vec3) — center where the pulse applies (Cardinal’s position at cast time)
- Optional visuals (Mesh3d, MeshMaterial3d) or light burst, following the projectile’s composition pattern

2. Spawn the nova as a composed entity at resolution time

- When the cast/channel completes, spawn a single nova entity:
    - (HolyNovaPulse, Origin(caster_pos), Radius(world_radius), HealAmount(120.0), DamageAmount(80.0), TimeToLive(0.0,
      0.1), …visuals)
- This mirrors projectile spawning: one entity with everything it needs.

3. Drive it with two simple systems

- holy_nova_apply_system:
    - Query nova pulse entities and all relevant targets (e.g., allies With<Character>, enemies With<Boss> or a more
      general Allegiance component when available)
    - For each target, compute distance from Origin and apply effects if within Radius
    - Healing and damage application can be events or direct component/resource mutations, depending on your health
      model
- holy_nova_lifecycle_system:
    - Increment TimeToLive with Time.delta
    - Despawn the nova when elapsed >= total
    - Optionally animate visual intensity during its brief life

4. Use timers for cast and cooldown (see autoshot_ability)

- Use a Local<Timer> or component Timer on the caster for channeling and cooldown tracking, similar to
  autoshot_ability’s periodic spawning pattern.
- For channeling: attach a Channel(Timer) component to the caster when they start casting; a system advances it and on
  completion spawns the HolyNovaPulse entity.
- For cooldown: attach or update a Cooldown(Timer) component; systems prevent re-cast until it finishes.

5. Coordinate visuals the same way as projectiles

- Compose visuals directly on the pulse entity (Mesh3d + MeshMaterial3d) or spawn child entities for particles/lights.
- Keep visuals separate from gameplay logic by querying only what’s needed; e.g., the apply system never needs to read
  MeshMaterial3d.

6. Example pseudo-code snippets (mirroring main.rs style)

- Spawning the nova on cast completion:

```rust
commands.spawn((
HolyNovaPulse,
Origin(caster_pos),
Radius(2.0 * TILE_SIZE),
HealAmount(120.0),
DamageAmount(80.0),
TimeToLive(0.0, 0.1),
// Optional visuals
Mesh3d(nova_mesh),
MeshMaterial3d(mats.gold.clone()),
));
```

- Applying effects (single-responsibility system):

```rust
fn holy_nova_apply_system(
    mut pulses: Query<(&Origin, &Radius), With<HolyNovaPulse>>,
    mut allies: Query<(Entity, &GlobalTransform), With<Ally>>, // or With<Character> if that’s your ally marker
    mut enemies: Query<(Entity, &GlobalTransform), With<Enemy>>, // or With<Boss> in current prototype
) {
    for (origin, radius) in pulses.iter_mut() {
        let center = origin.0;
        let r2 = radius.0 * radius.0;
        for (ally_e, gt) in allies.iter_mut() {
            if gt.translation().distance_squared(center) <= r2 {
                // apply_heal(ally_e, 120.0);
            }
        }
        for (enemy_e, gt) in enemies.iter_mut() {
            if gt.translation().distance_squared(center) <= r2 {
                // apply_damage(enemy_e, 80.0);
            }
        }
    }
}
```

- Lifecycle/despawn (pattern identical to move_projectiles):

```rust
fn holy_nova_lifecycle_system(
    mut commands: Commands,
    time: Res<Time>,
    mut q: Query<(Entity, &mut TimeToLive), With<HolyNovaPulse>>,
) {
    for (e, mut ttl) in q.iter_mut() {
        ttl.0 += time.delta_secs();
        if ttl.0 >= ttl.1 { commands.entity(e).despawn(); }
    }
}
```

7. Performance and ECS hygiene

- Use With<>/Without<> filters and Added/Changed detection where appropriate
- Keep systems under ~50 lines and focused on one responsibility
- Prefer squared-distance checks to avoid sqrt where possible
- Compose entities with all required components at spawn to avoid archetype churn

8. Extending to upgrades (Radiant Expansion, Resonance, Convergence)

- Add single-use components describing modifiers (e.g., DurationBonus, RadiusBonus, ApplyHot{ amount, secs }, ApplyDot{
  amount, secs }) and let systems interpret them during the pulse.
- Keep upgrades declarative: data components, not branches inside systems.

Summary: Holy Nova in this codebase should be implemented as a short-lived, composed entity (like projectiles) with
single-purpose components and two tiny systems: one to apply effects and one to manage lifetime. Timers on the caster
manage channeling and cooldown, mirroring autoshot_ability’s spawning cadence. This aligns the ability with the
repository’s existing ECS style and keeps gameplay, visuals, and lifecycles cleanly separated.
