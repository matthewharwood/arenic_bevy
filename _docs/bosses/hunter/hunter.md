# The Webweaver - Hunter Boss

## Overview
- **Theme**: Master trapper who transforms the arena into a living web of spatial-temporal hazards
- **Difficulty**: High strategic complexity through predictive trap placement
- **Arena**: 12x12 grid chamber with reactive floor panels
- **Unique Verb**: PREDICT - Creates delayed hazards that activate based on player movement patterns

## Phase Structure (2 minutes total)

### Phase 1: Web Initialization (0:00-0:30)
**Core Mechanic**: Introduction to predictive trap system

**Boss Abilities**:
- **Temporal Snare** (every 8s): Places invisible traps that activate 3 seconds after a player steps near them
- **Web Shot** (every 5s): Fires slow projectiles that leave sticky trails
- **Hunter's Mark** (at 0:15): Marks lowest HP player for increased damage

**Environmental**:
- Floor panels begin glowing to indicate future trap locations
- Web strands appear between walls creating movement corridors

**Counter-play**:
- Learn trap activation timing
- Use movement abilities to trigger traps safely
- Position to avoid web shot angles

### Phase 2: Dimensional Weaving (0:30-1:00)
**Core Mechanic**: Traps exist in multiple time states simultaneously

**Boss Abilities**:
- **Echo Trap** (every 10s): Places traps that trigger in past, present, and future positions
- **Spatial Tether** (at 0:45): Connects two players with a damage-sharing web
- **Predictive Volley** (every 7s): Fires arrows where players will be in 2 seconds

**Environmental**:
- Time shadows appear showing where players were 3 seconds ago
- Trap activation creates chain reactions with nearby traps
- Arena edges become lined with web walls

**Counter-play**:
- Track your past positions to avoid echo traps
- Coordinate movement with tethered partner
- Deliberately change movement patterns to confuse predictive shots

### Phase 3: Web Convergence (1:00-1:30)
**Core Mechanic**: All previous traps begin converging toward center

**Boss Abilities**:
- **Trap Migration** (continuous): All placed traps slowly move toward players
- **Web Prison** (at 1:15): Encases highest threat player in breakable web
- **Temporal Snapshot** (every 12s): Saves current trap layout, replays it 5 seconds later

**Environmental**:
- Arena shrinks as outer rings become covered in webs
- Destroyed traps leave damaging residue for 3 seconds
- Web density increases, slowing movement by 20%

**Counter-play**:
- Clear paths through migrating traps
- Focus fire to break web prisons quickly
- Remember snapshot positions to avoid double damage

### Phase 4: Predictive Overload (1:30-2:00)
**Core Mechanic**: Boss predicts and punishes all movement patterns

**Boss Abilities**:
- **Future Sight** (at 1:30): Shows ghostly images of where traps will appear
- **Inevitability Web** (every 6s): Creates traps at all possible player positions in next 2 seconds
- **Hunter's Finale** (at 1:50): Channels for 10s, if not interrupted, instantly defeats lowest HP player

**Environmental**:
- Entire arena becomes a probability matrix of potential trap locations
- Time acceleration zones appear, speeding up trap activation
- Web strands connect all active traps, creating damage chains

**Counter-play**:
- Use limited safe zones efficiently
- Save interrupt abilities for Hunter's Finale
- Coordinate team positioning to minimize inevitability coverage

## Orthogonal Design Analysis

### Unique Mechanics
- **Predictive Trap System**: No other boss uses future-position targeting
- **Temporal Layering**: Traps existing in multiple time states
- **Spatial Convergence**: Mobile hazards that adapt to player movement

### Taxonomy Mapping
- **Verb**: PREDICT (anticipate future positions)
- **Modifier**: Temporal displacement, spatial convergence
- **Cost**: Cognitive load (tracking time states), spatial restriction

### OC Score: 0.22
- Lowest overlap with: Ironwall (0.14) - attrition vs prediction
- Highest overlap with: Shadowdancer (0.29) - both use deception

### Strategic Niche
The Webweaver creates a unique chess-like experience where players must think several moves ahead while managing an increasingly complex spatial puzzle. Unlike other bosses that react to player actions, the Webweaver anticipates them.

## Component Architecture

```rust
// Predictive Trap Entity
commands.spawn((
    PredictiveTrap,
    FuturePosition(Vec3),
    ActivationDelay(3.0),
    TrapDamage(150.0),
    TemporalLayer(TimeState::Future),
    ElapsedTime(0.0),
    Duration(10.0),
));

// Echo Trap (multiple time states)
commands.spawn((
    EchoTrap,
    PastPosition(past_pos),
    PresentPosition(current_pos),
    FuturePosition(future_pos),
    EchoDelay(1.5),
    TrapDamage(100.0),
));

// Web Tether Connection
commands.spawn((
    WebTether,
    SourceEntity(player1),
    TargetEntity(player2),
    DamageShare(0.5),
    TetherRange(8.0),
    BreakThreshold(300.0),
));

// Trap Migration System
fn migrate_traps_system(
    mut traps: Query<(&mut Transform, &MigrationTarget, &MigrationSpeed), With<PredictiveTrap>>,
    targets: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    for (mut trap_transform, target, speed) in traps.iter_mut() {
        if let Ok(player_transform) = targets.get(target.0) {
            let direction = (player_transform.translation - trap_transform.translation).normalize();
            trap_transform.translation += direction * speed.0 * time.delta_secs();
        }
    }
}
```

### Visual Effects
- Temporal shadows showing past/future positions
- Web strand particles connecting traps
- Predictive ghosting for future trap locations
- Floor panel heat mapping for danger zones