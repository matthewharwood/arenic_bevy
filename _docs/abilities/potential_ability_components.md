# Potential Ability Components for Arenic

This document catalogs all potential ability components that could exist within the game, organized by category. Each component is designed for rapid prototyping and maximum composability across the 8 character classes and their abilities.

## Core Combat Stats

### Damage & Healing
```rust
#[derive(Component)]
struct Damage(f32);
// Primary damage output
// Usage: commands.spawn((Damage(25.0), ProjectileTarget(pos)));

#[derive(Component)]
struct DamageOverTime {
    damage_per_tick: f32,
    duration: Timer,
    tick_interval: Timer,
}
// DOT effects like poison, acid, bleed
// Usage: entity.insert(DamageOverTime { damage_per_tick: 5.0, duration: Timer::from_seconds(10.0, TimerMode::Once), tick_interval: Timer::from_seconds(1.0, TimerMode::Repeating) });

#[derive(Component)]
struct Healing(f32);
// Direct healing amount
// Usage: commands.spawn((Healing(50.0), AreaOfEffect(2.0)));

#[derive(Component)]
struct HealOverTime {
    heal_per_tick: f32,
    duration: Timer,
    tick_interval: Timer,
}
// HOT effects like regeneration
// Usage: entity.insert(HealOverTime { heal_per_tick: 10.0, duration: Timer::from_seconds(15.0, TimerMode::Once), tick_interval: Timer::from_seconds(2.0, TimerMode::Repeating) });

#[derive(Component)]
struct DamageReduction(f32);
// Reduces incoming damage by percentage (0.0-1.0)
// Usage: entity.insert(DamageReduction(0.25)); // 25% damage reduction

#[derive(Component)]
struct DamageReflection(f32);
// Reflects percentage of damage back to attacker
// Usage: entity.insert(DamageReflection(0.5)); // 50% reflection

#[derive(Component)]
struct TrueDamage(f32);
// Damage that bypasses all defenses
// Usage: commands.spawn((TrueDamage(15.0), ProjectileTarget(boss_pos)));
```

### Attack Mechanics
```rust
#[derive(Component)]
struct AttackSpeed(f32);
// Attacks per second
// Usage: entity.insert(AttackSpeed(1.5));

#[derive(Component)]
struct CritChance(f32);
// Critical hit chance (0.0-1.0)
// Usage: entity.insert(CritChance(0.15));

#[derive(Component)]
struct CritMultiplier(f32);
// Damage multiplier for critical hits
// Usage: entity.insert(CritMultiplier(2.5));

#[derive(Component)]
struct AttackRange(f32);
// Attack range in tiles
// Usage: entity.insert(AttackRange(4.0));

#[derive(Component)]
struct PierceCount(u32);
// Number of enemies projectile can pierce through
// Usage: commands.spawn((PierceCount(3), Damage(20.0)));

#[derive(Component)]
struct Multishot(u32);
// Number of additional projectiles to fire
// Usage: entity.insert(Multishot(2)); // Fires 3 total projectiles

#[derive(Component)]
struct ChainTarget {
    max_chains: u32,
    current_chains: u32,
    chain_damage_falloff: f32,
}
// Chain lightning/abilities that jump between enemies
// Usage: projectile.insert(ChainTarget { max_chains: 4, current_chains: 0, chain_damage_falloff: 0.8 });
```

## Projectile Systems

### Projectile Behavior
```rust
#[derive(Component)]
struct ProjectileSpeed(f32);
// Movement speed in pixels per second
// Usage: projectile.insert(ProjectileSpeed(150.0));

#[derive(Component)]
struct ProjectileTarget(Vec3);
// Target position for projectile
// Usage: projectile.insert(ProjectileTarget(enemy_position));

#[derive(Component)]
struct HomingProjectile {
    turn_rate: f32,
    max_lifetime: Timer,
}
// Projectile that follows moving targets
// Usage: projectile.insert(HomingProjectile { turn_rate: 3.0, max_lifetime: Timer::from_seconds(5.0, TimerMode::Once) });

#[derive(Component)]
struct BouncingProjectile {
    bounces_remaining: u32,
    bounce_range: f32,
}
// Projectile that bounces between enemies
// Usage: projectile.insert(BouncingProjectile { bounces_remaining: 3, bounce_range: 2.0 });

#[derive(Component)]
struct ProjectileLifetime(Timer);
// Projectile despawns after timer expires
// Usage: projectile.insert(ProjectileLifetime(Timer::from_seconds(3.0, TimerMode::Once)));

#[derive(Component)]
struct ProjectileGravity(f32);
// Gravity effect on projectile arc
// Usage: projectile.insert(ProjectileGravity(9.8));

#[derive(Component)]
struct ExplodeOnImpact {
    explosion_radius: f32,
    explosion_damage: f32,
}
// Projectile explodes when hitting target
// Usage: projectile.insert(ExplodeOnImpact { explosion_radius: 2.0, explosion_damage: 30.0 });
```

## Area of Effect Systems

### AoE Mechanics
```rust
#[derive(Component)]
struct AreaOfEffect(f32);
// Radius in tiles for area effects
// Usage: ability.insert(AreaOfEffect(2.5));

#[derive(Component)]
struct AreaDamage {
    radius: f32,
    damage: f32,
    falloff: f32, // 0.0 = no falloff, 1.0 = full falloff at edge
}
// Area damage with distance falloff
// Usage: explosion.insert(AreaDamage { radius: 3.0, damage: 50.0, falloff: 0.6 });

#[derive(Component)]
struct PersistentArea {
    duration: Timer,
    tick_interval: Timer,
}
// Area effect that persists over time (acid pools, fire patches)
// Usage: area.insert(PersistentArea { duration: Timer::from_seconds(10.0, TimerMode::Once), tick_interval: Timer::from_seconds(0.5, TimerMode::Repeating) });

#[derive(Component)]
struct ExpandingArea {
    initial_radius: f32,
    final_radius: f32,
    expansion_rate: f32,
}
// Area that grows over time
// Usage: shockwave.insert(ExpandingArea { initial_radius: 0.5, final_radius: 4.0, expansion_rate: 2.0 });

#[derive(Component)]
struct ConicalArea {
    angle: f32, // in radians
    range: f32,
    direction: Vec2,
}
// Cone-shaped area effect
// Usage: flame_breath.insert(ConicalArea { angle: PI/3.0, range: 4.0, direction: Vec2::new(1.0, 0.0) });
```

## Targeting Systems

### Target Selection
```rust
#[derive(Component)]
struct TargetNearest;
// Targets closest enemy
// Usage: ability.insert(TargetNearest);

#[derive(Component)]
struct TargetLowestHealth;
// Targets enemy with lowest current health
// Usage: ability.insert(TargetLowestHealth);

#[derive(Component)]
struct TargetHighestThreat;
// Targets enemy dealing most damage
// Usage: ability.insert(TargetHighestThreat);

#[derive(Component)]
struct TargetRandom;
// Randomly selects valid target
// Usage: ability.insert(TargetRandom);

#[derive(Component)]
struct TargetAllInRange;
// Affects all valid targets in range
// Usage: healing_aura.insert(TargetAllInRange);

#[derive(Component)]
struct TargetPosition(Vec3);
// Targets specific world position
// Usage: ability.insert(TargetPosition(click_position));

#[derive(Component)]
struct TargetFilter {
    include_allies: bool,
    include_enemies: bool,
    include_self: bool,
    min_health_percent: f32,
    max_health_percent: f32,
}
// Filters valid targets by criteria
// Usage: ability.insert(TargetFilter { include_allies: true, include_enemies: false, include_self: false, min_health_percent: 0.0, max_health_percent: 0.5 });
```

## Movement & Position

### Movement Abilities
```rust
#[derive(Component)]
struct MovementSpeed(f32);
// Movement speed modifier
// Usage: entity.insert(MovementSpeed(1.5)); // 50% faster

#[derive(Component)]
struct Teleport {
    range: f32,
    target_position: Option<Vec3>,
}
// Instant movement to position
// Usage: ability.insert(Teleport { range: 5.0, target_position: None });

#[derive(Component)]
struct Dash {
    distance: f32,
    speed: f32,
    direction: Vec2,
    invulnerable_during_dash: bool,
}
// Quick movement in direction
// Usage: ability.insert(Dash { distance: 3.0, speed: 10.0, direction: Vec2::new(1.0, 0.0), invulnerable_during_dash: true });

#[derive(Component)]
struct Knockback {
    force: f32,
    direction: Vec2,
}
// Pushes target away
// Usage: ability.insert(Knockback { force: 2.0, direction: (target_pos - caster_pos).normalize() });

#[derive(Component)]
struct Pull {
    force: f32,
    target_position: Vec3,
}
// Pulls target toward position
// Usage: ability.insert(Pull { force: 1.5, target_position: caster_position });

#[derive(Component)]
struct Immobilize(Timer);
// Prevents movement for duration
// Usage: entity.insert(Immobilize(Timer::from_seconds(3.0, TimerMode::Once)));

#[derive(Component)]
struct Slow {
    movement_reduction: f32, // 0.0-1.0
    duration: Timer,
}
// Reduces movement speed
// Usage: entity.insert(Slow { movement_reduction: 0.6, duration: Timer::from_seconds(5.0, TimerMode::Once) });
```

## Status Effects & Buffs

### Beneficial Effects
```rust
#[derive(Component)]
struct AttackSpeedBonus {
    multiplier: f32,
    duration: Timer,
}
// Temporary attack speed increase
// Usage: entity.insert(AttackSpeedBonus { multiplier: 1.3, duration: Timer::from_seconds(8.0, TimerMode::Once) });

#[derive(Component)]
struct DamageBonus {
    flat_bonus: f32,
    multiplier: f32,
    duration: Timer,
}
// Temporary damage increase
// Usage: entity.insert(DamageBonus { flat_bonus: 10.0, multiplier: 1.2, duration: Timer::from_seconds(10.0, TimerMode::Once) });

#[derive(Component)]
struct Shield {
    amount: f32,
    duration: Timer,
}
// Absorbs damage before health
// Usage: entity.insert(Shield { amount: 100.0, duration: Timer::from_seconds(15.0, TimerMode::Once) });

#[derive(Component)]
struct Invulnerability(Timer);
// Immune to all damage
// Usage: entity.insert(Invulnerability(Timer::from_seconds(2.0, TimerMode::Once)));

#[derive(Component)]
struct Stealth {
    duration: Timer,
    break_on_action: bool,
}
// Invisible to enemies
// Usage: entity.insert(Stealth { duration: Timer::from_seconds(6.0, TimerMode::Once), break_on_action: true });

#[derive(Component)]
struct CritBonus {
    chance_bonus: f32,
    multiplier_bonus: f32,
    duration: Timer,
}
// Temporary critical hit improvement
// Usage: entity.insert(CritBonus { chance_bonus: 0.1, multiplier_bonus: 0.5, duration: Timer::from_seconds(12.0, TimerMode::Once) });
```

### Debuffs & Crowd Control
```rust
#[derive(Component)]
struct Stun(Timer);
// Cannot act or move
// Usage: entity.insert(Stun(Timer::from_seconds(2.0, TimerMode::Once)));

#[derive(Component)]
struct Silence(Timer);
// Cannot use abilities
// Usage: entity.insert(Silence(Timer::from_seconds(4.0, TimerMode::Once)));

#[derive(Component)]
struct Blind {
    accuracy_reduction: f32,
    duration: Timer,
}
// Reduces hit chance
// Usage: entity.insert(Blind { accuracy_reduction: 0.5, duration: Timer::from_seconds(6.0, TimerMode::Once) });

#[derive(Component)]
struct Curse {
    damage_reduction: f32,
    healing_reduction: f32,
    duration: Timer,
}
// Reduces effectiveness
// Usage: entity.insert(Curse { damage_reduction: 0.3, healing_reduction: 0.4, duration: Timer::from_seconds(8.0, TimerMode::Once) });

#[derive(Component)]
struct Fear {
    duration: Timer,
    flee_speed: f32,
}
// Forces movement away from caster
// Usage: entity.insert(Fear { duration: Timer::from_seconds(3.0, TimerMode::Once), flee_speed: 1.5 });

#[derive(Component)]
struct Taunt {
    duration: Timer,
    forced_target: Entity,
}
// Forces target to attack specific entity
// Usage: entity.insert(Taunt { duration: Timer::from_seconds(5.0, TimerMode::Once), forced_target: tank_entity });
```

## Resource Management

### Costs & Resources
```rust
#[derive(Component)]
struct ManaCost(f32);
// Mana required to cast
// Usage: ability.insert(ManaCost(25.0));

#[derive(Component)]
struct HealthCost(f32);
// Health sacrificed to cast
// Usage: ability.insert(HealthCost(15.0));

#[derive(Component)]
struct ItemCost {
    item_type: String,
    quantity: u32,
}
// Items consumed to cast
// Usage: ability.insert(ItemCost { item_type: "rock".to_string(), quantity: 2 });

#[derive(Component)]
struct Cooldown(Timer);
// Time before ability can be used again
// Usage: ability.insert(Cooldown(Timer::from_seconds(8.0, TimerMode::Once)));

#[derive(Component)]
struct GlobalCooldown(Timer);
// Affects all abilities
// Usage: character.insert(GlobalCooldown(Timer::from_seconds(1.5, TimerMode::Once)));

#[derive(Component)]
struct ResourceGeneration {
    mana_per_second: f32,
    health_per_second: f32,
}
// Passive resource regeneration
// Usage: entity.insert(ResourceGeneration { mana_per_second: 5.0, health_per_second: 2.0 });

#[derive(Component)]
struct ResourceDrain {
    mana_per_second: f32,
    health_per_second: f32,
    duration: Timer,
}
// Drains resources over time
// Usage: entity.insert(ResourceDrain { mana_per_second: 10.0, health_per_second: 0.0, duration: Timer::from_seconds(6.0, TimerMode::Once) });
```

## Timing & Duration

### Time-Based Effects
```rust
#[derive(Component)]
struct CastTime(Timer);
// Time required to cast ability
// Usage: ability.insert(CastTime(Timer::from_seconds(2.5, TimerMode::Once)));

#[derive(Component)]
struct ChannelTime {
    total_duration: f32,
    current_time: f32,
    interruptible: bool,
}
// Continuous casting that can be interrupted
// Usage: ability.insert(ChannelTime { total_duration: 5.0, current_time: 0.0, interruptible: true });

#[derive(Component)]
struct DelayedEffect {
    delay: Timer,
    effect_type: String, // Could be enum in actual implementation
}
// Effect triggers after delay
// Usage: ability.insert(DelayedEffect { delay: Timer::from_seconds(3.0, TimerMode::Once), effect_type: "explosion".to_string() });

#[derive(Component)]
struct Duration(Timer);
// How long effect lasts
// Usage: buff.insert(Duration(Timer::from_seconds(10.0, TimerMode::Once)));

#[derive(Component)]
struct PeriodicEffect {
    interval: Timer,
    effect_type: String,
    remaining_ticks: Option<u32>,
}
// Effect that triggers repeatedly
// Usage: dot.insert(PeriodicEffect { interval: Timer::from_seconds(1.0, TimerMode::Repeating), effect_type: "damage".to_string(), remaining_ticks: Some(10) });

#[derive(Component)]
struct StackingDuration {
    base_duration: f32,
    stack_count: u32,
    max_stacks: u32,
    duration_per_stack: f32,
}
// Duration extends with multiple applications
// Usage: poison.insert(StackingDuration { base_duration: 5.0, stack_count: 1, max_stacks: 5, duration_per_stack: 2.0 });
```

## Environmental Interaction

### Terrain & Objects
```rust
#[derive(Component)]
struct CreateTerrain {
    terrain_type: String,
    size: Vec2,
    duration: Option<Timer>,
}
// Creates temporary terrain
// Usage: ability.insert(CreateTerrain { terrain_type: "ice_wall".to_string(), size: Vec2::new(2.0, 1.0), duration: Some(Timer::from_seconds(15.0, TimerMode::Once)) });

#[derive(Component)]
struct DestroyTerrain {
    radius: f32,
    terrain_types: Vec<String>,
}
// Removes terrain in area
// Usage: ability.insert(DestroyTerrain { radius: 3.0, terrain_types: vec!["wall".to_string(), "barrier".to_string()] });

#[derive(Component)]
struct PlaceObject {
    object_type: String,
    duration: Option<Timer>,
    health: Option<f32>,
}
// Spawns objects like traps, totems
// Usage: ability.insert(PlaceObject { object_type: "healing_totem".to_string(), duration: Some(Timer::from_seconds(30.0, TimerMode::Once)), health: Some(50.0) });

#[derive(Component)]
struct EnvironmentalDamage {
    damage_per_second: f32,
    damage_types: Vec<String>,
}
// Hazardous terrain
// Usage: lava_tile.insert(EnvironmentalDamage { damage_per_second: 20.0, damage_types: vec!["fire".to_string()] });

#[derive(Component)]
struct TerrainModifier {
    movement_speed_multiplier: f32,
    vision_blocked: bool,
    passable: bool,
}
// Changes terrain properties
// Usage: mud.insert(TerrainModifier { movement_speed_multiplier: 0.5, vision_blocked: false, passable: true });
```

## Special Mechanics

### Unique Abilities
```rust
#[derive(Component)]
struct Transform {
    target_type: String,
    duration: Option<Timer>,
    stats_modifier: f32,
}
// Changes entity type temporarily
// Usage: polymorph.insert(Transform { target_type: "sheep".to_string(), duration: Some(Timer::from_seconds(8.0, TimerMode::Once)), stats_modifier: 0.1 });

#[derive(Component)]
struct Resurrection {
    health_restore_percent: f32,
    mana_restore_percent: f32,
    resurrection_sickness_duration: Option<Timer>,
}
// Brings back dead characters
// Usage: revive.insert(Resurrection { health_restore_percent: 1.0, mana_restore_percent: 0.5, resurrection_sickness_duration: Some(Timer::from_seconds(10.0, TimerMode::Once)) });

#[derive(Component)]
struct CopyAbility {
    source_entity: Entity,
    ability_index: u32,
    duration: Option<Timer>,
}
// Mimics another entity's ability
// Usage: mimic.insert(CopyAbility { source_entity: target, ability_index: 0, duration: Some(Timer::from_seconds(15.0, TimerMode::Once)) });

#[derive(Component)]
struct AbilityReflection {
    reflect_chance: f32,
    duration: Timer,
}
// Chance to reflect abilities back at caster
// Usage: mirror.insert(AbilityReflection { reflect_chance: 0.75, duration: Timer::from_seconds(6.0, TimerMode::Once) });

#[derive(Component)]
struct SwapPositions {
    target_entity: Entity,
    swap_stats: bool,
}
// Exchanges positions with target
// Usage: ability.insert(SwapPositions { target_entity: enemy, swap_stats: false });

#[derive(Component)]
struct TimeManipulation {
    time_scale: f32,
    duration: Timer,
    affects_self: bool,
}
// Speeds up or slows down time for targets
// Usage: haste.insert(TimeManipulation { time_scale: 2.0, duration: Timer::from_seconds(5.0, TimerMode::Once), affects_self: true });
```

## Input & Interaction

### Control Types
```rust
#[derive(Component)]
struct InstantCast;
// Ability activates immediately
// Usage: ability.insert(InstantCast);

#[derive(Component)]
struct HoldToCharge {
    min_charge_time: f32,
    max_charge_time: f32,
    charge_multiplier: f32,
}
// Power increases with hold duration
// Usage: ability.insert(HoldToCharge { min_charge_time: 0.5, max_charge_time: 3.0, charge_multiplier: 2.0 });

#[derive(Component)]
struct MultiTap {
    required_taps: u32,
    tap_window: f32,
    current_taps: u32,
    last_tap_time: f32,
}
// Requires multiple quick presses
// Usage: combo.insert(MultiTap { required_taps: 3, tap_window: 0.5, current_taps: 0, last_tap_time: 0.0 });

#[derive(Component)]
struct DirectionalInput {
    required_direction: Vec2,
    tolerance: f32,
}
// Requires specific directional input
// Usage: ability.insert(DirectionalInput { required_direction: Vec2::new(0.0, 1.0), tolerance: 0.2 });

#[derive(Component)]
struct SequenceInput {
    required_sequence: Vec<String>,
    current_sequence: Vec<String>,
    sequence_timeout: f32,
}
// Requires specific input sequence
// Usage: special_move.insert(SequenceInput { required_sequence: vec!["up".to_string(), "down".to_string(), "attack".to_string()], current_sequence: Vec::new(), sequence_timeout: 2.0 });
```

## Visual & Audio Effects

### Feedback Systems
```rust
#[derive(Component)]
struct VisualEffect {
    effect_type: String,
    scale: f32,
    color: Color,
    duration: Timer,
}
// Visual feedback for abilities
// Usage: explosion.insert(VisualEffect { effect_type: "explosion".to_string(), scale: 2.0, color: Color::RED, duration: Timer::from_seconds(0.5, TimerMode::Once) });

#[derive(Component)]
struct AudioEffect {
    sound_file: String,
    volume: f32,
    pitch: f32,
}
// Audio feedback
// Usage: fireball.insert(AudioEffect { sound_file: "fire_whoosh.ogg".to_string(), volume: 0.8, pitch: 1.0 });

#[derive(Component)]
struct ScreenShake {
    intensity: f32,
    duration: Timer,
}
// Camera shake effect
// Usage: earthquake.insert(ScreenShake { intensity: 5.0, duration: Timer::from_seconds(1.0, TimerMode::Once) });

#[derive(Component)]
struct DamageNumber {
    value: f32,
    is_critical: bool,
    is_healing: bool,
    velocity: Vec2,
    lifetime: Timer,
}
// Floating combat text
// Usage: commands.spawn(DamageNumber { value: 150.0, is_critical: true, is_healing: false, velocity: Vec2::new(0.0, 50.0), lifetime: Timer::from_seconds(2.0, TimerMode::Once) });

#[derive(Component)]
struct Telegraph {
    shape: String, // "circle", "line", "cone", etc.
    size: Vec2,
    warning_duration: Timer,
    danger_color: Color,
}
// Visual warning for incoming attacks
// Usage: boss_attack.insert(Telegraph { shape: "circle".to_string(), size: Vec2::new(3.0, 3.0), warning_duration: Timer::from_seconds(2.0, TimerMode::Once), danger_color: Color::RED });
```

## Recording & Replay Integration

### Timeline Components
```rust
#[derive(Component)]
struct RecordableAction {
    action_type: String,
    timestamp: f32,
    position: Vec3,
    parameters: HashMap<String, f32>,
}
// Action that can be recorded and replayed
// Usage: ability.insert(RecordableAction { action_type: "fireball".to_string(), timestamp: 45.5, position: target_pos, parameters: HashMap::new() });

#[derive(Component)]
struct ReplayMarker {
    original_timestamp: f32,
    loop_count: u32,
}
// Marks replayed actions
// Usage: ghost_action.insert(ReplayMarker { original_timestamp: 45.5, loop_count: 3 });

#[derive(Component)]
struct SyncPoint {
    sync_time: f32,
    sync_group: String,
}
// Synchronization points for coordinated abilities
// Usage: combo_part.insert(SyncPoint { sync_time: 30.0, sync_group: "chain_combo".to_string() });

#[derive(Component)]
struct ConditionalReplay {
    condition_type: String,
    condition_value: f32,
    fallback_action: Option<String>,
}
// Replay behavior depends on game state
// Usage: conditional_heal.insert(ConditionalReplay { condition_type: "health_below".to_string(), condition_value: 0.5, fallback_action: Some("attack".to_string()) });
```

## Marker Components

### State Markers
```rust
#[derive(Component)]
struct IsCritical;
// Marks critical hits/effects
// Usage: projectile.insert(IsCritical);

#[derive(Component)]
struct IsHealing;
// Marks healing effects
// Usage: spell.insert(IsHealing);

#[derive(Component)]
struct IsChanneling;
// Currently channeling ability
// Usage: caster.insert(IsChanneling);

#[derive(Component)]
struct IsStealthed;
// Currently invisible
// Usage: character.insert(IsStealthed);

#[derive(Component)]
struct IsShielded;
// Has active shield
// Usage: character.insert(IsShielded);

#[derive(Component)]
struct IsBoss;
// Boss entity marker
// Usage: boss.insert(IsBoss);

#[derive(Component)]
struct IsMinion;
// Summoned creature
// Usage: summon.insert(IsMinion);

#[derive(Component)]
struct IsElite;
// Elite enemy with special properties
// Usage: elite_mob.insert(IsElite);

#[derive(Component)]
struct RequiresLOS; // Line of Sight
// Ability requires clear line of sight
// Usage: snipe.insert(RequiresLOS);

#[derive(Component)]
struct IgnoresCover;
// Ability bypasses obstacles
// Usage: magic_missile.insert(IgnoresCover);

#[derive(Component)]
struct FriendlyFire;
// Can damage allies
// Usage: explosion.insert(FriendlyFire);

#[derive(Component)]
struct AllyOnly;
// Only affects friendly targets
// Usage: heal.insert(AllyOnly);

#[derive(Component)]
struct EnemyOnly;
// Only affects hostile targets
// Usage: curse.insert(EnemyOnly);
```

## Usage Examples

### Combining Components for Complex Abilities

```rust
// Hunter's Explosive Arrow
commands.spawn((
    Damage(40.0),
    ProjectileSpeed(120.0),
    ProjectileTarget(enemy_pos),
    ExplodeOnImpact { explosion_radius: 2.0, explosion_damage: 60.0 },
    CritChance(0.15),
    ManaCost(25.0),
    Cooldown(Timer::from_seconds(6.0, TimerMode::Once)),
    VisualEffect { effect_type: "explosive_arrow".to_string(), scale: 1.0, color: Color::ORANGE, duration: Timer::from_seconds(0.3, TimerMode::Once) },
    AudioEffect { sound_file: "explosion.ogg".to_string(), volume: 0.9, pitch: 1.0 },
));

// Alchemist's Acid Pool
commands.spawn((
    AreaOfEffect(2.5),
    DamageOverTime { damage_per_tick: 15.0, duration: Timer::from_seconds(12.0, TimerMode::Once), tick_interval: Timer::from_seconds(1.0, TimerMode::Repeating) },
    PersistentArea { duration: Timer::from_seconds(12.0, TimerMode::Once), tick_interval: Timer::from_seconds(1.0, TimerMode::Repeating) },
    DamageReduction(0.1), // Reduces enemy armor
    TargetPosition(click_pos),
    CreateTerrain { terrain_type: "acid_pool".to_string(), size: Vec2::new(2.5, 2.5), duration: Some(Timer::from_seconds(12.0, TimerMode::Once)) },
    EnemyOnly,
));

// Warrior's Berserker Rage
commands.spawn((
    AttackSpeedBonus { multiplier: 2.0, duration: Timer::from_seconds(8.0, TimerMode::Once) },
    DamageBonus { flat_bonus: 20.0, multiplier: 1.5, duration: Timer::from_seconds(8.0, TimerMode::Once) },
    DamageReduction(0.3), // Takes 30% less damage
    MovementSpeed(1.3),
    HealthCost(25.0), // Sacrifices health to activate
    Cooldown(Timer::from_seconds(30.0, TimerMode::Once)),
    VisualEffect { effect_type: "berserker_aura".to_string(), scale: 1.5, color: Color::RED, duration: Timer::from_seconds(8.0, TimerMode::Once) },
    IsChanneling, // Prevents other abilities during rage
));

// Thief's Shadow Clone
commands.spawn((
    CopyAbility { source_entity: caster, ability_index: 0, duration: Some(Timer::from_seconds(10.0, TimerMode::Once)) },
    Stealth { duration: Timer::from_seconds(3.0, TimerMode::Once), break_on_action: false },
    PlaceObject { object_type: "shadow_clone".to_string(), duration: Some(Timer::from_seconds(10.0, TimerMode::Once)), health: Some(1.0) },
    SwapPositions { target_entity: clone_entity, swap_stats: false },
    ManaCost(40.0),
    Cooldown(Timer::from_seconds(45.0, TimerMode::Once)),
));
```

This component system enables rapid prototyping by allowing any combination of effects to create unique abilities across all 8 character classes while maintaining consistency and avoiding duplication.