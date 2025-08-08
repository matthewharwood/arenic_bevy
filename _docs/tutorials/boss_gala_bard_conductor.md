# Boss Prototype Tutorial: Gala — The Bard (Stage Conductor)

Goals

- Simple to implement today; fun in under 30 minutes.
- Follows our Bevy 0.16 migration rules: no bundles, typed components, clear systems.
- Uses Components for per‑arena state (not Resources).
- Clear separation: data (const patterns) → systems (spawn, telegraph, damage) → visuals (hazard quads).

What we’ll build

- A stationary Bard boss (“StageConductor”) that runs a repeating 4‑beat pattern.
- Each beat activates a set of hazardous lanes (rows or columns). Telegraph first, then apply damage.
- Visuals are translucent quads on tiles; damage is a logged hit event for now.

Easiest Fun Simplifications (proactive tweaks)

- One phase only; no adds, no immunities.
- Lanes only (row or column bands) — fast to read, fast to dodge.
- Printed damage events first; upgrade to Health later.
- Spawn only in the active arena so you can iterate quickly.

File/Module Plan

- `src/boss/mod.rs`: Boss types, data, and systems glue.
- `src/boss/bard.rs`: Bard‑specific data and systems.
- Minimal edits in `src/main.rs` to register systems.

Design Data (consts first)

- Beat length and telegraph windows are constants for predictable rhythm.
- Lane patterns are small const tables you can remix live.

Implementation Steps

1) Create boss module scaffolding

Add a new module with a simple boss marker and a Bard module.

```rust
// src/boss/mod.rs
use bevy::prelude::*;

pub mod bard;

#[derive(Component, Debug)]
pub struct StageConductor; // Marker for the Bard boss

#[derive(Component, Debug)]
pub struct BardRhythm {
    pub beat: u32,      // current beat index
    pub elapsed: f32,   // seconds into current beat
}

impl Default for BardRhythm {
    fn default() -> Self { Self { beat: 0, elapsed: 0.0 } }
}

pub struct BossPlugin;
impl Plugin for BossPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            bard::run_bard_rhythm,
            bard::resolve_bard_damage,
            bard::cleanup_hazards,
        ));
    }
}
```

2) Add Bard logic with const beat pattern

```rust
// src/boss/bard.rs
use bevy::prelude::*;
use crate::arena::{GRID_HEIGHT, GRID_WIDTH, TILE_SIZE, get_local_tile_space};
use crate::character::Character;
use crate::selectors::Active;
use crate::materials::Materials;
use crate::boss::{StageConductor, BardRhythm};

// Core knobs — tweak these live
const BEAT_DURATION: f32 = 0.80;        // seconds per beat
const TELEGRAPH_LEAD: f32 = 0.40;       // seconds before damage when tiles glow
const HAZARD_ACTIVE: f32 = 0.30;        // seconds hazard stays “hot”

// 4-beat repeating lane pattern
// Rows: even, odd, middle; Columns: middle (example mix)
#[derive(Clone, Copy)]
enum LaneKind { RowsEven, RowsOdd, RowMid, ColMid }

const PATTERN: [LaneKind; 4] = [
    LaneKind::RowsEven,
    LaneKind::RowsOdd,
    LaneKind::RowMid,
    LaneKind::ColMid,
];

#[derive(Component)]
pub struct HazardTile {
    pub row: u32,
    pub col: u32,
    pub hot_at: f32,     // world time when damage applies
    pub cool_at: f32,    // world time when damage window ends
}

#[derive(Event, Debug)]
pub struct HazardHit {
    pub entity: Entity,
    pub row: u32,
    pub col: u32
}

pub fn run_bard_rhythm(
    mut commands: Commands,
    time: Res<Time>,
    mats: Res<Materials>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut q_bard: Query<(Entity, &mut BardRhythm), (With<StageConductor>, With<Active>)>,
    parent_q: Query<&Parent>,
) {
    for (bard_entity, mut rhythm) in q_bard.iter_mut() {
        rhythm.elapsed += time.delta_secs();
        if rhythm.elapsed < BEAT_DURATION { continue; }

        rhythm.elapsed = 0.0;
        rhythm.beat = (rhythm.beat + 1) % PATTERN.len() as u32;
        let lane = PATTERN[rhythm.beat as usize];

        // Spawn telegraphs and schedule hazards as children of the arena parent
        if let Ok(parent) = parent_q.get(bard_entity) {
            let now = time.elapsed_secs();
            let hot_at = now + TELEGRAPH_LEAD;
            let cool_at = hot_at + HAZARD_ACTIVE;

            match lane {
                LaneKind::RowsEven => spawn_rows(&mut commands, parent.get(), &mats, &mut meshes, |r| r % 2 == 0, hot_at, cool_at),
                LaneKind::RowsOdd => spawn_rows(&mut commands, parent.get(), &mats, &mut meshes, |r| r % 2 == 1, hot_at, cool_at),
                LaneKind::RowMid => spawn_specific_rows(&mut commands, parent.get(), &mats, &mut meshes, &[GRID_HEIGHT / 2], hot_at, cool_at),
                LaneKind::ColMid => spawn_specific_cols(&mut commands, parent.get(), &mats, &mut meshes, &[GRID_WIDTH / 2], hot_at, cool_at),
            }
        }
    }
}

fn spawn_rows<F: Fn(u32) -> bool>(
    commands: &mut Commands,
    arena: Entity,
    mats: &Res<Materials>,
    meshes: &mut ResMut<Assets<Mesh>>,
    pred: F,
    hot_at: f32,
    cool_at: f32,
) {
    for row in 0..GRID_HEIGHT {
        if !pred(row) { continue; }
        for col in 0..GRID_WIDTH {
            spawn_hazard(commands, arena, mats, meshes, row, col, hot_at, cool_at);
        }
    }
}

fn spawn_specific_rows(
    commands: &mut Commands,
    arena: Entity,
    mats: &Res<Materials>,
    meshes: &mut ResMut<Assets<Mesh>>,
    rows: &[u32],
    hot_at: f32,
    cool_at: f32,
) {
    for &row in rows {
        for col in 0..GRID_WIDTH {
            spawn_hazard(commands, arena, mats, meshes, row, col, hot_at, cool_at);
        }
    }
}

fn spawn_specific_cols(
    commands: &mut Commands,
    arena: Entity,
    mats: &Res<Materials>,
    meshes: &mut ResMut<Assets<Mesh>>,
    cols: &[u32],
    hot_at: f32,
    cool_at: f32,
) {
    for &col in cols {
        for row in 0..GRID_HEIGHT {
            spawn_hazard(commands, arena, mats, meshes, row, col, hot_at, cool_at);
        }
    }
}

fn spawn_hazard(
    commands: &mut Commands,
    arena: Entity,
    mats: &Res<Materials>,
    meshes: &mut ResMut<Assets<Mesh>>,
    row: u32,
    col: u32,
    hot_at: f32,
    cool_at: f32,
) {
    // Thin quad per tile (or use Cuboid::from_size if preferred)
    let mesh = meshes.add(Plane3d::default().mesh().size(TILE_SIZE, TILE_SIZE));
    let pos = get_local_tile_space(row, col) + Vec3::Z * 0.5; // slight Z offset
    commands.entity(arena).with_child((
        HazardTile { row, col, hot_at, cool_at },
        Transform::from_translation(pos),
        Mesh3d(mesh),
        MeshMaterial3d(mats.red.clone()),  // red telegraph; swap to yellow for lead if desired
    ));
}

pub fn resolve_bard_damage(
    time: Res<Time>,
    mut writer: EventWriter<HazardHit>,
    hazards: Query<&HazardTile>,
    chars: Query<(Entity, &GlobalTransform), With<Character>>,
) {
    let now = time.elapsed_secs();
    for hazard in &hazards {
        if !(now >= hazard.hot_at && now <= hazard.cool_at) { continue; }
        // Very cheap overlap: compare grid cell by rounding character local to tile indices
        for (entity, xf) in &chars {
            let p = xf.translation();
            let row = ((p.y / -TILE_SIZE).round() as i32).max(0) as u32;
            let col = ((p.x / TILE_SIZE).round() as i32).max(0) as u32;
            if row == hazard.row && col == hazard.col {
                writer.write(HazardHit { entity, row, col });
            }
        }
    }
}

pub fn cleanup_hazards(
    time: Res<Time>,
    mut commands: Commands,
    q: Query<(Entity, &HazardTile)>,
) {
    let now = time.elapsed_secs();
    for (e, h) in &q {
        if now > h.cool_at { commands.entity(e).despawn(); }
    }
}
```

3) Spawn the Bard boss in the active arena

Add a spawn step after arenas are created. For a quick start, attach to arena 1 (our Active arena convention).

```rust
// src/main.rs (snippet) — add near other spawns
use crate::boss::{BossPlugin, StageConductor, BardRhythm};

fn main() {
    App::new()
        // ...
        .add_plugins(DefaultPlugins)
        .add_plugins(BossPlugin)
        // ... existing systems
        .run();
}

fn spawn_bard_in_active_arena(
    mut commands: Commands,
    arenas: Query<Entity, (With<arena::Arena>, With<Active>)>,
) {
    if let Ok(active_arena) = arenas.get_single() {
        commands.entity(active_arena).with_child((
            StageConductor,
            BardRhythm::default(),
            Transform::from_translation(crate::arena::get_local_tile_space(32, 10)),
        ));
    }
}
```

Register `spawn_bard_in_active_arena` in Startup after arena grid setup, similar to the other spawn calls already in
`main.rs`.

4) Visual polish (optional, quick wins)

- Telegraph color split: yellow during lead (before hot_at), red while hot. Easiest is two component variants or swap
  material when time >= hot_at.
- Add a simple “metronome” text UI or a pulsing light as a beat.
- Add a whoosh sound on each beat using `AudioPlayer` + `PlaybackSettings` (Bevy 0.16 pattern).

Extending the Pattern

- Difficulty: shorten `BEAT_DURATION` or increase `HAZARD_ACTIVE` overlap.
- Complexity: add diagonals (tiles where row == col) or checkerboards.
- Boss movement: lerp the Bard one tile per full measure to pressure positioning.
- Damage model: add a `Health` component and subtract on `HazardHit` events.

Why Bard Is Easiest (and correct for prototype)

- No target selection or pathfinding required; deterministic rhythm loop.
- Reads well with our orthographic camera and grid.
- Cleanly showcases movement skill atoms and record‑and‑replay potential.
- Highly extensible: swapping the const pattern instantly creates new “fights”.

Bevy 0.16 Compliance Check

- No bundles; typed components like `Mesh3d`, `MeshMaterial3d`, `AudioPlayer` (if used).
- Component state over Resources for per‑boss rhythm (`BardRhythm`).
- Events (`HazardHit`) for decoupled damage application.
- Color usage via `Color::srgb` and palette handles (already in Materials).

Testing Checklist

- Spawn Bard only in active arena; hazards appear on a beat.
- Character takes “hits” (log lines) when standing in a red tile during hot window.
- WASD movement allows dodging lanes on time.

Next Steps (if desired)

- Add simple `Health` to `Character` with UI text.
- Add safe‑lane telegraph (green) to guide players on learning runs.
- Hook into the recording/replay systems so ghosts must dance too.

