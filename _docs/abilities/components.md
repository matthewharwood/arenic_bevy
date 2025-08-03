# Ability Components Architecture

A comprehensive component-based architecture for Arenic's deterministic ability system, designed for recording/replay compatibility and mass character performance.

## Architecture Overview

Every ability in Arenic is implemented as a Bevy entity with:
1. **Unique Marker Component** - Identifies the specific ability type
2. **Shared Functional Components** - Compose ability behavior from reusable parts
3. **Recording Components** - Enable deterministic timeline recording and replay
4. **Visual/Audio Components** - Manage effects independent of core logic

This design ensures perfect determinism for the 2-minute recording system while scaling efficiently for 320 characters across 8 arenas.

## Core Design Principles

### 1. Components First
- Use Components for entity state, not Resources
- Resources only for truly global singletons (Time, Input, AssetServer)
- Prefer `Query<&Component>` over `Res<GlobalState>`

### 2. Single Responsibility Systems
- One system = one job
- Name systems by what they do
- Keep systems under 50 lines
- Chain systems with explicit ordering

### 3. Deterministic Recording
- All random elements use seeded RNG
- Component state drives behavior, not wall clock time
- Frame-perfect reproducibility across replays
- Events communicate state changes, not direct mutation

### 4. Shallow Components
- No nested properties or complex structures
- Prefer multiple simple components over single complex ones
- Easy to serialize for timeline storage
- Cache-friendly memory layout

## Ability Marker Components

Each ability has a unique zero-sized marker component for identification and queries.

### Hunter Abilities
```rust
#[derive(Component)]
pub struct AutoShotAbility;

#[derive(Component)]
pub struct PoisonShotAbility;

#[derive(Component)]
pub struct TrapAbility;

#[derive(Component)]
pub struct SniperAbility;
```

### Cardinal Abilities
```rust
#[derive(Component)]
pub struct HealAbility;

#[derive(Component)]
pub struct BarrierAbility;

#[derive(Component)]
pub struct BeamAbility;

#[derive(Component)]
pub struct ResurrectAbility;
```

### Forager Abilities
```rust
#[derive(Component)]
pub struct DigAbility;

#[derive(Component)]
pub struct BorderAbility;

#[derive(Component)]
pub struct BoulderAbility;

#[derive(Component)]
pub struct MushroomAbility;
```

### Thief Abilities
```rust
#[derive(Component)]
pub struct ShadowStepAbility;

#[derive(Component)]
pub struct BackstabAbility;

#[derive(Component)]
pub struct PickpocketAbility;

#[derive(Component)]
pub struct SmokeScreenAbility;
```

### Warrior Abilities
```rust
#[derive(Component)]
pub struct BlockAbility;

#[derive(Component)]
pub struct BashAbility;

#[derive(Component)]
pub struct TauntAbility;

#[derive(Component)]
pub struct BulwarkAbility;
```

### Merchant Abilities
```rust
#[derive(Component)]
pub struct DiceAbility;

#[derive(Component)]
pub struct CoinTossAbility;

#[derive(Component)]
pub struct FortuneAbility;

#[derive(Component)]
pub struct VaultAbility;
```

### Alchemist Abilities
```rust
#[derive(Component)]
pub struct IronskinDraftAbility;

#[derive(Component)]
pub struct AcidFlaskAbility;

#[derive(Component)]
pub struct TransmuteAbility;

#[derive(Component)]
pub struct SiphonAbility;
```

### Bard Abilities
```rust
#[derive(Component)]
pub struct CleanseAbility;

#[derive(Component)]
pub struct DanceAbility;

#[derive(Component)]
pub struct HelixAbility;

#[derive(Component)]
pub struct MimicAbility;
```

## Core Functional Components

### Casting and Timing
```rust
#[derive(Component)]
pub struct InstantCast;

#[derive(Component)]
pub struct CastTime(pub Timer);

#[derive(Component)]
pub struct Cooldown(pub Timer);

#[derive(Component)]
pub struct Duration(pub Timer);

#[derive(Component)]
pub struct PeriodicEffect {
    pub interval: Timer,
    pub effect_type: String,
    pub remaining_ticks: Option<u32>,
}

#[derive(Component)]
pub struct ChannelTime {
    pub duration: Timer,
    pub can_interrupt: bool,
}
```

### Damage and Effects
```rust
#[derive(Component)]
pub struct Damage(pub f32);

#[derive(Component)]
pub struct Healing(pub f32);

#[derive(Component)]
pub struct DamageReduction(pub f32);

#[derive(Component)]
pub struct DamageReflection(pub f32);

#[derive(Component)]
pub struct CriticalChance(pub f32);

#[derive(Component)]
pub struct CriticalMultiplier(pub f32);

#[derive(Component)]
pub struct DamageOverTime {
    pub damage_per_tick: f32,
    pub tick_interval: Timer,
    pub remaining_ticks: u32,
}
```

### Spatial and Targeting
```rust
#[derive(Component)]
pub struct AttackRange(pub f32);

#[derive(Component)]
pub struct ProjectileSpeed(pub f32);

#[derive(Component)]
pub struct ProjectileTarget(pub Vec2);

#[derive(Component)]
pub struct ProjectileLifetime(pub Timer);

#[derive(Component)]
pub struct AreaOfEffect(pub f32);

#[derive(Component)]
pub struct ConicalArea {
    pub angle: f32,
    pub range: f32,
    pub direction: Vec2,
}

#[derive(Component)]
pub struct GridArea {
    pub width: u32,
    pub height: u32,
}

#[derive(Component)]
pub struct LineArea {
    pub length: f32,
    pub width: f32,
    pub direction: Vec2,
}
```

### Movement and Positioning
```rust
#[derive(Component)]
pub struct MovementSpeed(pub f32);

#[derive(Component)]
pub struct DashDistance(pub f32);

#[derive(Component)]
pub struct DashSpeed(pub f32);

#[derive(Component)]
pub struct Knockback {
    pub force: f32,
    pub direction: Vec2,
}

#[derive(Component)]
pub struct DirectionalInput {
    pub required_direction: Vec2,
    pub tolerance: f32,
}
```

### Input and Interaction
```rust
#[derive(Component)]
pub struct MultiTap {
    pub required_taps: u32,
    pub tap_window: f32,
    pub current_taps: u32,
    pub last_tap_time: f32,
}

#[derive(Component)]
pub struct HoldInput {
    pub minimum_duration: f32,
    pub maximum_duration: f32,
    pub current_duration: f32,
}

#[derive(Component)]
pub struct ChargeInput {
    pub charge_rate: f32,
    pub current_charge: f32,
    pub max_charge: f32,
}
```

### Targeting Behavior
```rust
#[derive(Component)]
pub struct TargetNearest;

#[derive(Component)]
pub struct TargetLowestHealth;

#[derive(Component)]
pub struct TargetHighestThreat;

#[derive(Component)]
pub struct TargetSelf;

#[derive(Component)]
pub struct TargetAlly;

#[derive(Component)]
pub struct TargetEnemy;

#[derive(Component)]
pub struct RequiresLOS; // Line of Sight

#[derive(Component)]
pub struct SmartTarget {
    pub strategy: TargetStrategy,
    pub max_range: f32,
}

#[derive(Component)]
pub enum TargetStrategy {
    LowestHealthPercent,
    ClosestEnemy,
    HighestDamage,
    MostVulnerable,
}
```

### Status Effects and Modifiers
```rust
#[derive(Component)]
pub struct Invulnerability {
    pub duration: Timer,
    pub immunity_type: ImmunityType,
}

#[derive(Component)]
pub enum ImmunityType {
    Physical,
    Magical,
    Environmental,
    All,
}

#[derive(Component)]
pub struct Stealth {
    pub duration: Timer,
    pub break_on_action: bool,
}

#[derive(Component)]
pub struct BuffEffect {
    pub effect_type: BuffType,
    pub magnitude: f32,
    pub duration: Timer,
}

#[derive(Component)]
pub enum BuffType {
    DamageBoost,
    SpeedBoost,
    CriticalChance,
    Healing,
    Shield,
}

#[derive(Component)]
pub struct StackableEffect {
    pub current_stacks: u32,
    pub max_stacks: u32,
    pub stack_value: f32,
    pub decay_timer: Option<Timer>,
}
```

### Resource Management
```rust
#[derive(Component)]
pub struct ManaCost(pub f32);

#[derive(Component)]
pub struct ResourceGeneration {
    pub mana_per_second: f32,
    pub health_per_second: f32,
}

#[derive(Component)]
pub struct ResourceCost {
    pub mana: f32,
    pub energy: f32,
    pub special_resource: f32,
}
```

### Environmental Interaction
```rust
#[derive(Component)]
pub struct TerrainModification {
    pub terrain_type: TerrainType,
    pub duration: Option<Timer>,
}

#[derive(Component)]
pub enum TerrainType {
    Normal,
    Dug,
    Barrier,
    Hazardous,
    Mushroom,
}

#[derive(Component)]
pub struct EnvironmentalEffect {
    pub effect_type: EnvironmentType,
    pub radius: f32,
    pub duration: Timer,
}

#[derive(Component)]
pub enum EnvironmentType {
    AcidPool,
    HealingGround,
    SmokeCloud,
    IceField,
    FireZone,
}
```

## Recording and Replay Components

### Timeline Integration
```rust
#[derive(Component)]
pub struct RecordableAction {
    pub action_type: String,
    pub timestamp: f32,
    pub position: Vec3,
    pub parameters: HashMap<String, f32>,
}

#[derive(Component)]
pub struct ReplayableEvent {
    pub event_id: u64,
    pub trigger_time: f32,
    pub event_data: EventData,
}

#[derive(Component)]
pub struct TimelinePosition {
    pub current_time: f32,
    pub loop_duration: f32,
    pub is_recording: bool,
}

#[derive(Component)]
pub struct DeterministicRNG {
    pub seed: u64,
    pub state: u64,
}
```

### State Tracking
```rust
#[derive(Component)]
pub struct AbilityState {
    pub is_active: bool,
    pub charges_remaining: u32,
    pub last_used: f32,
}

#[derive(Component)]
pub struct ComponentHistory {
    pub snapshots: VecDeque<ComponentSnapshot>,
    pub max_history: usize,
}

#[derive(Component)]
pub struct ComponentSnapshot {
    pub timestamp: f32,
    pub component_data: Vec<u8>, // Serialized component state
}
```

## Visual and Audio Components

### Visual Effects
```rust
#[derive(Component)]
pub struct VisualEffect {
    pub effect_type: String,
    pub scale: f32,
    pub color: Color,
    pub duration: Timer,
}

#[derive(Component)]
pub struct ParticleEffect {
    pub particle_count: u32,
    pub emission_rate: f32,
    pub lifetime: f32,
    pub velocity_range: Vec2,
}

#[derive(Component)]
pub struct LightEffect {
    pub intensity: f32,
    pub color: Color,
    pub range: f32,
    pub flicker_pattern: Option<String>,
}
```

### Audio Effects
```rust
#[derive(Component)]
pub struct AudioEffect {
    pub sound_file: String,
    pub volume: f32,
    pub pitch: f32,
}

#[derive(Component)]
pub struct SpatialAudio {
    pub max_distance: f32,
    pub rolloff_factor: f32,
    pub doppler_factor: f32,
}

#[derive(Component)]
pub struct LoopingAudio {
    pub fade_in_duration: f32,
    pub fade_out_duration: f32,
}
```

## Upgrade and Progression Components

### Ability Upgrades
```rust
#[derive(Component)]
pub struct UpgradeLevel(pub u32);

#[derive(Component)]
pub struct UpgradeModifier {
    pub modifier_type: ModifierType,
    pub value: f32,
    pub unlock_level: u32,
}

#[derive(Component)]
pub enum ModifierType {
    DamageMultiplier,
    CooldownReduction,
    RangeIncrease,
    AdditionalEffect,
    NewMechanic,
}

#[derive(Component)]
pub struct ConditionalUpgrade {
    pub condition: UpgradeCondition,
    pub modifier: UpgradeModifier,
}

#[derive(Component)]
pub enum UpgradeCondition {
    TargetHealthBelow(f32),
    StacksAbove(u32),
    EnemiesInRange(u32),
    AllyCount(u32),
}
```

## System Patterns for Recording Integration

### Recording Systems
```rust
// Example system for recording-compatible ability activation
fn ability_activation_system(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    recording: Res<RecordingState>,
    mut abilities: Query<(Entity, &Transform, &mut Cooldown), With<HealAbility>>,
) {
    if input.just_pressed(KeyCode::Digit3) {
        for (entity, transform, mut cooldown) in abilities.iter_mut() {
            if cooldown.0.finished() {
                // Record the action deterministically
                commands.entity(entity).insert(RecordableAction {
                    action_type: "heal_cast".to_string(),
                    timestamp: recording.current_time,
                    position: transform.translation,
                    parameters: HashMap::new(),
                });
                
                cooldown.0.reset();
            }
        }
    }
}

// System for deterministic periodic effects
fn periodic_effect_system(
    mut commands: Commands,
    time: Res<Time>,
    mut effects: Query<(Entity, &mut PeriodicEffect, &Transform)>,
    recording: Res<RecordingState>,
) {
    for (entity, mut periodic, transform) in effects.iter_mut() {
        periodic.interval.tick(time.delta());
        
        if periodic.interval.just_finished() {
            // Execute effect deterministically
            commands.spawn((
                RecordableAction {
                    action_type: periodic.effect_type.clone(),
                    timestamp: recording.current_time,
                    position: transform.translation,
                    parameters: HashMap::new(),
                },
                // Additional effect components...
            ));
            
            // Decrement remaining ticks if limited
            if let Some(ref mut ticks) = periodic.remaining_ticks {
                *ticks = ticks.saturating_sub(1);
                if *ticks == 0 {
                    commands.entity(entity).remove::<PeriodicEffect>();
                }
            }
        }
    }
}
```

### Replay Systems
```rust
// System for replaying recorded actions
fn replay_system(
    mut commands: Commands,
    time: Res<Time>,
    recording: Res<RecordingState>,
    recorded_actions: Query<&RecordableAction>,
) {
    if recording.is_replaying {
        for action in recorded_actions.iter() {
            if (recording.current_time - action.timestamp).abs() < time.delta_seconds() {
                // Execute recorded action exactly
                match action.action_type.as_str() {
                    "heal_cast" => {
                        // Recreate heal effect at recorded position and time
                        commands.spawn((
                            HealEffect {
                                target: /* lookup target */,
                                heal_amount: action.parameters.get("heal_amount").copied().unwrap_or(150.0),
                                channel_progress: 0.0,
                                visual_effect: Entity::PLACEHOLDER,
                                audio_source: Entity::PLACEHOLDER,
                            },
                            Transform::from_translation(action.position),
                        ));
                    },
                    "auto_shot_fire" => {
                        // Recreate projectile with exact parameters
                        // ... 
                    },
                    _ => {}
                }
            }
        }
    }
}
```

## Performance Optimizations for Mass Combat

### Component Storage Optimization
```rust
// Group related components for archetype efficiency
#[derive(Bundle)]
pub struct DamageAbilityBundle {
    pub damage: Damage,
    pub range: AttackRange,
    pub cooldown: Cooldown,
    pub recordable: RecordableAction,
}

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub speed: ProjectileSpeed,
    pub target: ProjectileTarget,
    pub lifetime: ProjectileLifetime,
    pub damage: Damage,
    pub visual: VisualEffect,
}

#[derive(Bundle)]
pub struct AreaEffectBundle {
    pub area: AreaOfEffect,
    pub duration: Duration,
    pub effect: EnvironmentalEffect,
    pub visual: VisualEffect,
}
```

### System Scheduling
```rust
// Optimal system ordering for ability processing
impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                // Input processing first
                ability_input_system,
                // State updates
                cooldown_system,
                duration_system,
                // Effect processing
                damage_system,
                healing_system,
                // Recording
                recording_system,
                // Visual/Audio last
                visual_effect_system,
                audio_effect_system,
            ).chain());
    }
}
```

## Integration with Arenic's Core Systems

### Unit Marker Integration
```rust
// Updated ability targeting with unit markers
fn ability_targeting_system(
    hunters: Query<(Entity, &GridPosition, &AttackRange), 
                   (With<Hunter>, With<Active>, Without<Dead>)>,
    cardinals: Query<(Entity, &GridPosition, &AttackRange), 
                     (With<Cardinal>, With<Active>, Without<Dead>)>,
    bosses: Query<(Entity, &GridPosition, &Health), 
                  (With<Boss>, With<BossActive>, Without<Dead>)>,
    heroes: Query<(Entity, &GridPosition, &Health), 
                  (With<Hero>, With<Active>, Without<Dead>)>,
    mut target_events: EventWriter<TargetAcquiredEvent>,
) {
    // Hunter abilities target bosses and enemies
    for (hunter_entity, hunter_pos, range) in hunters.iter() {
        if let Some(target) = find_closest_target(hunter_pos, range, &bosses) {
            target_events.send(TargetAcquiredEvent {
                caster: hunter_entity,
                target: target.0,
                position: target.1,
            });
        }
    }

    // Cardinal abilities target other heroes for healing
    for (cardinal_entity, cardinal_pos, range) in cardinals.iter() {
        if let Some(target) = find_closest_injured_hero(cardinal_pos, range, &heroes) {
            target_events.send(TargetAcquiredEvent {
                caster: cardinal_entity,
                target: target.0,
                position: target.1,
            });
        }
    }
}

// Grid-based targeting with unit markers
fn grid_targeting_system(
    abilities: Query<(&GridPosition, &AttackRange), With<TargetNearest>>,
    enemies: Query<(&GridPosition, &Health), (With<Boss>, With<BossActive>)>,
    mut target_events: EventWriter<TargetAcquiredEvent>,
) {
    for (ability_pos, range) in abilities.iter() {
        let closest_enemy = enemies.iter()
            .filter(|(enemy_pos, _)| {
                let distance = ((enemy_pos.x - ability_pos.x).pow(2) + 
                               (enemy_pos.y - ability_pos.y).pow(2)) as f32;
                distance.sqrt() <= range.0
            })
            .min_by_key(|(enemy_pos, _)| {
                (enemy_pos.x - ability_pos.x).abs() + (enemy_pos.y - ability_pos.y).abs()
            });
            
        // Send targeting event...
    }
}
```

### Timeline Integration
```rust
// Timeline-aware component updates
fn timeline_component_system(
    mut abilities: Query<(&mut Duration, &TimelinePosition)>,
    time: Res<Time>,
) {
    for (mut duration, timeline_pos) in abilities.iter_mut() {
        // Update duration based on timeline position, not wall clock
        let delta = if timeline_pos.is_recording {
            time.delta_seconds()
        } else {
            // Use recorded delta for replay
            timeline_pos.current_time - timeline_pos.loop_duration
        };
        
        duration.0.tick(Duration::from_secs_f32(delta));
    }
}
```

This component-based architecture provides:

1. **Perfect Determinism**: All ability behavior driven by components, not external state
2. **Recording Compatibility**: Every action generates recordable events with timestamps
3. **Mass Performance**: ECS archetype optimization handles 320+ characters efficiently  
4. **Modular Design**: Abilities composed from reusable, single-purpose components
5. **Easy Extension**: New abilities created by combining existing components
6. **Clean Systems**: Each system has single responsibility and clear data dependencies

The architecture scales from simple abilities (few components) to complex abilities (many components) while maintaining consistent patterns and performance characteristics optimal for Arenic's unique recording/replay requirements.

## Integration with Unit Marker System

The ability component architecture seamlessly integrates with Arenic's comprehensive unit marker system for complete game functionality across 8 arenas with 320+ characters.

### Hero Class Integration

Abilities are now tightly coupled with hero class markers for efficient system routing:

```rust
// Class-specific ability queries using unit markers
type HunterAbilities = Query<(Entity, &AbilitySlots), (With<Hunter>, With<Active>)>;
type CardinalAbilities = Query<(Entity, &AbilitySlots), (With<Cardinal>, With<Active>)>;
type WarriorAbilities = Query<(Entity, &AbilitySlots), (With<Warrior>, With<Active>)>;

// Ability activation with hero state filtering
fn hunter_ability_system(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    recording_state: Res<RecordingState>,
    hunters: Query<(Entity, &GridPosition, &AbilitySlots), 
                   (With<Hunter>, With<Player>, With<Recording>)>,
    targets: Query<(Entity, &GridPosition), (With<Boss>, With<BossActive>)>,
) {
    for (hunter_entity, position, abilities) in hunters.iter() {
        if input.just_pressed(KeyCode::Digit1) {
            // Trigger hunter-specific ability with recording integration
            commands.entity(hunter_entity).insert(RecordableAction {
                action_type: "hunter_ability_1".to_string(),
                timestamp: recording_state.current_time,
                position: Vec3::new(position.x as f32, position.y as f32, 0.0),
                parameters: HashMap::new(),
            });
            
            // Spawn class-specific ability effect
            spawn_hunter_ability_1(&mut commands, position, &targets);
        }
    }
}
```

### Boss Targeting Integration

Abilities now properly target bosses and enemies using the boss marker system:

```rust
// Updated targeting for boss encounters
fn ability_boss_targeting_system(
    mut target_events: EventWriter<TargetAcquiredEvent>,
    heroes: Query<(Entity, &GridPosition, &AttackRange), 
                  (With<Hero>, With<Active>, With<TargetNearest>)>,
    bosses: Query<(Entity, &GridPosition, &Health), 
                  (With<Boss>, With<BossActive>, Without<Dead>)>,
    mini_bosses: Query<(Entity, &GridPosition, &Health), 
                       (With<MiniBoss>, With<BossActive>, Without<Dead>)>,
    current_arena: Res<CurrentArena>,
) {
    for (hero_entity, hero_pos, range) in heroes.iter() {
        // Prioritize main bosses, then mini-bosses
        let target = find_closest_boss(hero_pos, range, &bosses, &current_arena)
            .or_else(|| find_closest_mini_boss(hero_pos, range, &mini_bosses, &current_arena));
            
        if let Some((target_entity, target_pos)) = target {
            target_events.send(TargetAcquiredEvent {
                caster: hero_entity,
                target: target_entity,
                position: target_pos,
            });
        }
    }
}
```

### Arena-Scoped Ability Processing

Abilities now respect arena boundaries and only process entities in relevant arenas:

```rust
// Arena-aware ability processing
fn arena_scoped_ability_system(
    current_arena: Res<CurrentArena>,
    abilities: Query<(Entity, &AbilityEffect, &ArenaLocal), With<ActiveAbility>>,
    mut effect_events: EventWriter<AbilityEffectEvent>,
) {
    for (entity, effect, arena_local) in abilities.iter() {
        // Only process abilities in the currently active arena or global effects
        if arena_local.arena_id == current_arena.id || effect.is_global {
            effect_events.send(AbilityEffectEvent {
                ability_entity: entity,
                effect_type: effect.effect_type.clone(),
                arena_id: arena_local.arena_id,
            });
        }
    }
}
```

### Recording Integration with Unit States

Abilities now properly handle different unit states during recording and replay:

```rust
// State-aware ability recording
fn ability_recording_system(
    mut commands: Commands,
    recording_state: Res<RecordingState>,
    player_abilities: Query<(Entity, &AbilityActivation), 
                            (With<Player>, With<Recording>, Added<AbilityActivation>)>,
    ghost_abilities: Query<(Entity, &ReplayAction), 
                           (With<Ghost>, With<Replaying>)>,
) {
    // Record new player actions
    for (entity, activation) in player_abilities.iter() {
        commands.entity(entity).insert(RecordableAction {
            action_type: activation.ability_type.clone(),
            timestamp: recording_state.current_time,
            position: activation.cast_position,
            parameters: activation.parameters.clone(),
        });
    }
    
    // Execute ghost replay actions
    for (entity, replay_action) in ghost_abilities.iter() {
        if should_execute_replay_action(replay_action, &recording_state) {
            execute_ghost_ability(&mut commands, entity, replay_action);
        }
    }
}
```

### Tile Interaction Updates

Abilities now interact with the comprehensive tile marker system:

```rust
// Tile-based ability effects using tile markers
fn ability_tile_interaction_system(
    mut commands: Commands,
    abilities: Query<(Entity, &AreaOfEffect, &GridPosition), With<EnvironmentalAbility>>,
    tiles: Query<(Entity, &GridPosition, &TileType), With<Tile>>,
    interactive_tiles: Query<Entity, (With<InteractiveTile>, With<Targetable>)>,
) {
    for (ability_entity, aoe, ability_pos) in abilities.iter() {
        // Find all tiles within area of effect
        let affected_tiles: Vec<Entity> = tiles.iter()
            .filter(|(_, tile_pos, _)| {
                let distance = ((tile_pos.x - ability_pos.x).pow(2) + 
                               (tile_pos.y - ability_pos.y).pow(2)) as f32;
                distance.sqrt() <= aoe.0
            })
            .map(|(entity, _, _)| entity)
            .collect();
            
        for tile_entity in affected_tiles {
            // Apply ability effects to tiles
            if interactive_tiles.contains(tile_entity) {
                commands.entity(tile_entity).insert(TileEffect {
                    effect_type: TileEffectType::Modified,
                    duration: Timer::from_seconds(10.0, TimerMode::Once),
                    source_ability: ability_entity,
                });
            }
        }
    }
}
```

### Performance Optimization with Markers

The marker system enables significant performance optimizations for ability processing:

```rust
// Optimized ability processing using marker filtering
fn optimized_ability_system(
    // Only process abilities for active, non-dead heroes in current arena
    active_abilities: Query<(Entity, &AbilityComponent), 
                            (With<Hero>, With<Active>, Without<Dead>, With<ArenaLocal>)>,
    current_arena: Res<CurrentArena>,
) {
    // This query automatically filters out ~87.5% of entities (7/8 arenas)
    // Plus additional filtering for dead/inactive heroes
    for (entity, ability) in active_abilities.iter() {
        // Process only the most relevant abilities
        process_ability_efficiently(entity, ability);
    }
}

// Batch processing for similar ability types
fn batched_projectile_system(
    hunter_projectiles: Query<(Entity, &ProjectileComponent), 
                              (With<HunterProjectile>, With<Active>)>,
    cardinal_projectiles: Query<(Entity, &ProjectileComponent), 
                                (With<CardinalProjectile>, With<Active>)>,
) {
    // Process all hunter projectiles together for cache efficiency
    process_projectile_batch(hunter_projectiles.iter());
    
    // Process all cardinal projectiles together
    process_projectile_batch(cardinal_projectiles.iter());
}
```

### Cross-System Communication

Abilities communicate with other game systems through the marker-based event system:

```rust
// Events that integrate with unit markers
#[derive(Event)]
pub struct HeroAbilityEvent {
    pub caster: Entity,
    pub target: Option<Entity>,
    pub ability_type: String,
    pub position: GridPosition,
    pub arena_id: usize,
}

#[derive(Event)]
pub struct BossAbilityEvent {
    pub boss: Entity,
    pub ability_id: u32,
    pub targets: Vec<Entity>,
    pub phase: BossPhase,
    pub arena_id: usize,
}

// System that handles cross-arena ability effects
fn cross_arena_ability_system(
    mut hero_events: EventReader<HeroAbilityEvent>,
    mut boss_events: EventReader<BossAbilityEvent>,
    mut global_effects: EventWriter<GlobalEffectEvent>,
) {
    // Process hero abilities that might affect other arenas
    for event in hero_events.read() {
        if is_global_ability(&event.ability_type) {
            global_effects.send(GlobalEffectEvent {
                source_arena: event.arena_id,
                effect_type: GlobalEffectType::Buff,
                magnitude: 1.0,
                duration: 30.0,
            });
        }
    }
    
    // Process boss abilities that create arena-wide effects
    for event in boss_events.read() {
        if event.ability_id == ARENA_SHAKE_ABILITY {
            // Effect all heroes in the same arena
            apply_arena_wide_effect(event.arena_id, ArenaEffect::Shake);
        }
    }
}
```

This comprehensive integration ensures that the ability system works seamlessly with all unit types, respects arena boundaries, handles 320+ characters efficiently, and maintains perfect determinism for the recording/replay system while supporting all game mechanics described in the RULEBOOK.md.