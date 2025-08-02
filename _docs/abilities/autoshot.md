# Hunter AutoShot: Building a Projectile System with Upgrades

This tutorial walks through creating a complete projectile system with an upgrade mechanism using Bevy's ECS architecture.

## 1. Define Combat Components

Start by creating individual components that can be shared across different abilities:

```rust
// Core combat stats - composable across all abilities
#[derive(Component)]
struct AttackSpeed(f32);  // Attacks per second

#[derive(Component)]
struct Damage(f32);

#[derive(Component)]
struct CritChance(f32);  // 0.0 to 1.0

#[derive(Component)]
struct AttackRange(f32);  // In tiles

#[derive(Component)]
struct ProjectileSpeed(f32);  // Pixels per second

// Projectile-specific components
#[derive(Component)]
struct ProjectileTarget(Vec3);

#[derive(Component)]
struct IsCritical;

// Timer for attack intervals
#[derive(Component)]
struct HunterAttackTimer(Timer);
```

## 2. Create the Character with Stats

When spawning a Hunter character, compose it with the combat components:

```rust
fn spawn_hunter(mut commands: Commands, /* other resources */) {
    commands.entity(arena_entity).with_children(|parent| {
        parent.spawn((
            // Visual components
            Mesh2d(meshes.add(Circle::new(TILE_SIZE / 2.0))),
            MeshMaterial2d(materials.add(Color::BLACK)),
            Transform::from_xyz(x, y, 1.0),
            
            // Character identification
            CharacterType::Hunter,
            
            // Combat stats
            AttackSpeed(0.2),        // 1 attack per 5 seconds
            Damage(10.0),
            CritChance(0.1),         // 10% crit chance
            AttackRange(3.0),        // 3 tile range
            ProjectileSpeed(100.0),
            
            // Attack timer
            HunterAttackTimer({
                let mut timer = Timer::from_seconds(5.0, TimerMode::Repeating);
                timer.tick(Duration::from_secs_f32(5.0)); // Start ready to fire
                timer
            }),
        ));
    });
}
```

## 3. Implement Target Finding

Create a system to find the nearest enemy within range:

```rust
fn find_nearest_target_in_range(
    hunter_pos: Vec3,
    range_tiles: f32,
    enemy_query: &Query<&Transform, With<Enemy>>,
) -> Option<Vec3> {
    let max_distance = range_tiles * TILE_SIZE;
    
    enemy_query
        .iter()
        .map(|transform| (transform.translation, transform.translation.distance(hunter_pos)))
        .filter(|(_, distance)| *distance <= max_distance)
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(pos, _)| pos)
}
```

## 4. Build the Attack System

Create the main attack system that uses all the components:

```rust
fn hunter_auto_attack_system(
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut hunter_query: Query<(
        &Transform,
        &mut HunterAttackTimer,
        &AttackSpeed,
        &Damage,
        &CritChance,
        &AttackRange,
        &ProjectileSpeed,
    ), With<CharacterType>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    arena_query: Query<Entity, With<Arena>>,
) {
    for (transform, mut timer, attack_speed, damage, crit, range, proj_speed) in &mut hunter_query {
        // Dynamic timer duration based on attack speed
        timer.0.set_duration(Duration::from_secs_f32(1.0 / attack_speed.0));
        timer.0.tick(time.delta());
        
        if !timer.0.just_finished() {
            continue;
        }
        
        // Find target within range
        if let Some(target_pos) = find_nearest_target_in_range(
            transform.translation,
            range.0,
            &enemy_query,
        ) {
            // Roll for critical hit
            let is_critical = rand::random::<f32>() < crit.0;
            
            // Spawn projectile as child of arena
            if let Ok(arena) = arena_query.single() {
                commands.entity(arena).with_children(|parent| {
                    let mut projectile = parent.spawn((
                        // Visual components (all required for rendering!)
                        Mesh2d(meshes.add(Circle::new(4.0))),
                        MeshMaterial2d(materials.add(
                            if is_critical { Color::ORANGE } else { Color::RED }
                        )),
                        Transform::from_xyz(
                            transform.translation.x,
                            transform.translation.y,
                            2.0  // Higher z-order for visibility
                        ),
                        Visibility::default(),
                        InheritedVisibility::default(),
                        ViewVisibility::default(),
                        
                        // Projectile components
                        ProjectileTarget(target_pos),
                        ProjectileSpeed(proj_speed.0),
                        Damage(if is_critical { damage.0 * 2.0 } else { damage.0 }),
                    ));
                    
                    if is_critical {
                        projectile.insert(IsCritical);
                    }
                });
            }
        }
    }
}
```

## 5. Implement Projectile Movement

Create a system to move projectiles toward their targets:

```rust
fn move_projectiles_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &ProjectileTarget, &ProjectileSpeed)>,
) {
    for (entity, mut transform, target, speed) in &mut query {
        // Calculate direction and move
        let direction = (target.0 - transform.translation).normalize_or_zero();
        transform.translation += direction * speed.0 * time.delta_secs();
        
        // Despawn when close to target
        if transform.translation.distance(target.0) < 5.0 {
            commands.entity(entity).despawn();
        }
    }
}
```

## 6. Add Damage System

Handle projectile collisions and damage:

```rust
#[derive(Component)]
struct Health {
    current: f32,
    max: f32,
}

fn projectile_damage_system(
    mut commands: Commands,
    projectile_query: Query<(
        Entity, 
        &Transform, 
        &Damage,
        Option<&IsCritical>,
    ), With<ProjectileTarget>>,
    mut enemy_query: Query<(Entity, &Transform, &mut Health), With<Enemy>>,
) {
    for (proj_entity, proj_transform, damage, is_critical) in &projectile_query {
        for (enemy_entity, enemy_transform, mut health) in &mut enemy_query {
            if proj_transform.translation.distance(enemy_transform.translation) < 8.0 {
                // Apply damage
                health.current -= damage.0;
                
                // Spawn damage number
                spawn_damage_number(
                    &mut commands,
                    enemy_transform.translation,
                    damage.0,
                    is_critical.is_some(),
                );
                
                // Check for death
                if health.current <= 0.0 {
                    commands.entity(enemy_entity).despawn_recursive();
                }
                
                // Despawn projectile
                commands.entity(proj_entity).despawn();
                break;
            }
        }
    }
}
```

## 7. Create Upgrade System

Implement a system to upgrade individual stats:

```rust
#[derive(Resource)]
struct UpgradeLevels {
    damage: u32,
    attack_speed: u32,
    crit_chance: u32,
    range: u32,
    projectile_speed: u32,
}

fn apply_upgrades_system(
    upgrades: Res<UpgradeLevels>,
    mut query: Query<(
        &mut Damage,
        &mut AttackSpeed,
        &mut CritChance,
        &mut AttackRange,
        &mut ProjectileSpeed,
    ), With<CharacterType>>,
) {
    for (mut damage, mut attack_speed, mut crit, mut range, mut proj_speed) in &mut query {
        // Base values + upgrade bonuses
        damage.0 = 10.0 + (upgrades.damage as f32 * 5.0);           // +5 per level
        attack_speed.0 = 0.2 + (upgrades.attack_speed as f32 * 0.1); // +0.1 attacks/sec
        crit.0 = 0.1 + (upgrades.crit_chance as f32 * 0.05);        // +5% per level
        range.0 = 3.0 + (upgrades.range as f32);                    // +1 tile per level
        proj_speed.0 = 100.0 + (upgrades.projectile_speed as f32 * 25.0); // +25 speed
    }
}
```

## 8. Add Visual Feedback

Create floating damage numbers:

```rust
#[derive(Component)]
struct DamageNumber {
    velocity: Vec2,
    lifetime: Timer,
}

fn spawn_damage_number(
    commands: &mut Commands,
    position: Vec3,
    damage: f32,
    is_critical: bool,
) {
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                format!("{:.0}{}", damage, if is_critical { "!" } else { "" }),
                TextStyle {
                    font_size: if is_critical { 24.0 } else { 18.0 },
                    color: if is_critical { Color::ORANGE } else { Color::WHITE },
                    ..default()
                },
            ),
            transform: Transform::from_xyz(position.x, position.y + 20.0, 10.0),
            ..default()
        },
        DamageNumber {
            velocity: Vec2::new(0.0, 50.0),
            lifetime: Timer::from_seconds(1.0, TimerMode::Once),
        },
    ));
}
```

## 9. Register Systems

Add all systems to your app:

```rust
impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UpgradeLevels::default())
            .add_systems(
                Update,
                (
                    hunter_auto_attack_system,
                    move_projectiles_system,
                    projectile_damage_system,
                    apply_upgrades_system,
                    animate_damage_numbers,
                ).chain(),
            );
    }
}
```

## Key Lessons

1. **Component Composition**: Individual components are more flexible than monolithic structs
2. **Parent-Child Hierarchy**: Always spawn projectiles as children of the arena
3. **Visibility Components**: All three visibility components are required for rendering
4. **Dynamic Stats**: Use the upgrade system to modify component values at runtime
5. **Visual Feedback**: Critical hits and damage numbers improve game feel

## Common Pitfalls

- **Missing Visibility**: Projectiles won't render without all visibility components
- **Wrong Parent**: Spawning at root level causes positioning issues
- **Static Timers**: Remember to update timer duration when attack speed changes
- **Z-Fighting**: Use proper z-ordering (projectiles > characters > background)