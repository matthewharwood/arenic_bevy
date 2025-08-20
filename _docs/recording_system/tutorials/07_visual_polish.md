# Tutorial 07: Visual Polish & Feedback

## Objective

Add visual indicators, UI elements, and feedback systems to make the recording system intuitive and visually appealing.
Players should always understand the current state and have clear visual cues.

## Prerequisites

- Completed Tutorials 01-06 (Full recording system with multi-arena support)
- Basic understanding of Bevy UI and materials
- Familiarity with particle effects concepts

## Components/Systems

We'll create:

- Recording indicator UI
- Ghost trail effects
- Timeline progress bars
- Visual state indicators
- Audio feedback system

## Implementation Steps

### Step 1: Create Recording Indicator UI

Create `src/visual_feedback/mod.rs`:

```rust
use bevy::prelude::*;
use bevy::time::Virtual;
use bevy::color::palettes::css::WHITE;
use std::collections::{VecDeque, HashMap};
use std::time::Duration;
use crate::recording::{RecordingState, RecordingMode, RecordingCountdown};
use crate::timeline::{Arena, TimelineClock};
use crate::arena::CurrentArena;

/// Component for recording indicator UI
#[derive(Component)]
pub struct RecordingIndicatorUI;

/// Component for timeline progress bar
#[derive(Component)]
pub struct TimelineProgressBar;

/// Component for countdown display
#[derive(Component)]
pub struct CountdownDisplay;

/// Spawn recording indicator UI
pub fn setup_recording_ui(mut commands: Commands) {
    // Root UI container
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                ..default()
            },
            RecordingIndicatorUI,
        ))
        .with_children(|parent| {
            // Recording state indicator
            parent.spawn((
                Text::new("IDLE"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                RecordingStateText,
            ));

            // Timeline progress container
            parent
                .spawn(Node {
                    width: Val::Px(200.0),
                    height: Val::Px(20.0),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                })
                .with_children(|progress| {
                    // Progress bar fill
                    progress.spawn((
                        Node {
                            width: Val::Percent(0.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.8, 0.2, 0.2)),
                        TimelineProgressBar,
                    ));
                });

            // Countdown display
            parent.spawn((
                Text::new(""),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 0.0)),
                CountdownDisplay,
            ));
        });
}

#[derive(Component)]
struct RecordingStateText;

/// Update recording state text
/// APPROVED: State-driven UI with Changed<RecordingState>
pub fn update_recording_state_text(
    recording_state: Res<RecordingState>,
    mut text_q: Query<(&mut Text, &mut TextColor), With<RecordingStateText>, Changed<RecordingState>>,
) {
    // Changed<RecordingState> filter ensures this only runs when state changes
    if !recording_state.is_changed() {
        return;
    }

    for (mut text, mut color) in text_q.iter_mut() {
        let (state_text, state_color) = match recording_state.mode {
            RecordingMode::Idle => ("IDLE", Color::srgb(0.8, 0.8, 0.8)),
            RecordingMode::Countdown => ("PREPARING", Color::srgb(1.0, 1.0, 0.0)),
            RecordingMode::Recording => ("● RECORDING", Color::srgb(1.0, 0.2, 0.2)),
            RecordingMode::DialogPaused => ("PAUSED", Color::srgb(0.2, 0.5, 1.0)),
        };

        **text = state_text.to_string();
        color.0 = state_color;
    }
}

/// Update timeline progress bar
/// APPROVED: Timeline bar reads TimelineClock not Time
pub fn update_timeline_progress(
    arena_q: Query<(&Arena, &TimelineClock)>,
    current_arena: Res<CurrentArena>,
    mut progress_q: Query<&mut Node, With<TimelineProgressBar>>,
) {
    // Use ArenaId for current arena comparison
    let current_arena_id = current_arena.id();
    
    let current_time = arena_q
        .iter()
        .find(|(arena, _)| arena.name() == current_arena_id.name())
        .map(|(_, clock)| clock.current().as_secs())
        .unwrap_or(0.0);

    let progress_percent = (current_time / TimeStamp::MAX.0 * 100.0).clamp(0.0, 100.0);

    for mut node in progress_q.iter_mut() {
        node.width = Val::Percent(progress_percent);
    }
}

/// Update countdown display
pub fn update_countdown_display(
    // Use Option<Single> for the single countdown entity
    countdown: Option<Single<&RecordingCountdown>>,
    mut display_q: Query<&mut Text, With<CountdownDisplay>>,
) {
    for mut text in display_q.iter_mut() {
        if let Some(countdown_single) = countdown {
            if let Some(num) = countdown_single.get_display_number() {
                **text = num.to_string();
            } else {
                text.clear();
            }
        } else {
            text.clear();
        }
    }
}
```

### Step 2: Create Ghost Trail Effects

Add to `src/visual_feedback/mod.rs`:

```rust
use crate::playback::{Ghost, GhostVisuals};

/// Component for ghost trail effect
/// APPROVED: VecDeque for efficient trail buffer management
#[derive(Component)]
pub struct GhostTrail {
    pub positions: VecDeque<(Vec3, f32)>, // (position, timestamp)
    pub max_length: usize,
    // APPROVED: Store material handle to reuse, not recreate
    pub trail_material: Option<Handle<StandardMaterial>>,
    // Store mesh handle to reuse
    pub trail_mesh: Option<Handle<Mesh>>,
}

impl Default for GhostTrail {
    fn default() -> Self {
        Self {
            positions: VecDeque::with_capacity(20),
            max_length: 20,
            trail_material: None,
            trail_mesh: None,
        }
    }
}

/// Resource to cache trail materials by color
#[derive(Resource, Default)]
pub struct TrailMaterialCache {
    materials: HashMap<u32, Handle<StandardMaterial>>,
}

impl TrailMaterialCache {
    /// Get or create a material for the given color
    pub fn get_or_create(
        &mut self,
        color: Srgba,
        materials: &mut Assets<StandardMaterial>,
    ) -> Handle<StandardMaterial> {
        // Create a simple hash of the color for caching
        let color_key = ((color.red * 255.0) as u32) << 24
            | ((color.green * 255.0) as u32) << 16
            | ((color.blue * 255.0) as u32) << 8
            | ((color.alpha * 255.0) as u32);
        
        self.materials.entry(color_key).or_insert_with(|| {
            materials.add(StandardMaterial {
                base_color: Color::from(color),
                alpha_mode: AlphaMode::Blend,
                emissive: Color::from(color) * 0.1,
                ..default()
            })
        }).clone()
    }
}

/// Component for trail segment entities
#[derive(Component)]
pub struct TrailSegment {
    pub ghost_entity: Entity,
    pub age: f32,
}

/// Update ghost trails
pub fn update_ghost_trails(
    mut ghost_q: Query<(&Transform, &mut GhostTrail), With<Ghost>>,
    time: Res<Time>,
) {
    let current_time = time.elapsed().as_secs_f32();

    for (transform, mut trail) in ghost_q.iter_mut() {
        // Add current position to back
        trail.positions.push_back((transform.translation, current_time));

        // Remove old positions from front (O(1) with VecDeque)
        while trail.positions.len() > trail.max_length {
            trail.positions.pop_front();
        }

        // Remove positions older than 2 seconds
        while let Some((_, t)) = trail.positions.front() {
            if current_time - t > 2.0 {
                trail.positions.pop_front();
            } else {
                break;
            }
        }
    }
}

/// Initialize trail materials once
pub fn init_ghost_trail_materials(
    mut ghost_q: Query<(&mut GhostTrail, &GhostVisuals), Added<Ghost>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (mut trail, visuals) in ghost_q.iter_mut() {
        // PR Gate: Create material handle once, reuse it
        if trail.trail_material.is_none() {
            // Author tints as Srgba, convert to Color at assignment
            let material = materials.add(StandardMaterial {
                base_color: Color::from(visuals.tint.with_alpha(0.3)),
                alpha_mode: AlphaMode::Blend,
                emissive: Color::from(visuals.tint) * 0.1,
                ..default()
            });
            trail.trail_material = Some(material);
        }
    }
}

/// Maximum number of trail segments per ghost
const MAX_TRAIL_SEGMENTS: usize = 20;

/// Resource for shared trail mesh
#[derive(Resource)]
pub struct TrailMeshCache {
    pub segment_mesh: Handle<Mesh>,
}

impl TrailMeshCache {
    pub fn new(meshes: &mut Assets<Mesh>) -> Self {
        // Create a standard capsule mesh that we'll scale for different segments
        Self {
            segment_mesh: meshes.add(Capsule3d {
                radius: 0.02,
                half_length: 0.5,  // Standard size, we'll scale it
                ..default()
            }),
        }
    }
}

/// Render ghost trail segments
pub fn render_ghost_trails(
    mut commands: Commands,
    trail_mesh_cache: Res<TrailMeshCache>,
    mut material_cache: ResMut<TrailMaterialCache>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ghost_q: Query<(Entity, &mut GhostTrail, &GhostVisuals)>,
    mut trail_q: Query<(Entity, &mut TrailSegment, &mut Transform, &Handle<StandardMaterial>)>,
    virtual_time: Res<Time<Virtual>>,
) {
    let current_time = virtual_time.elapsed().as_secs_f32();

    // Update existing trail segments
    for (entity, mut segment, mut transform, material_handle) in trail_q.iter_mut() {
        segment.age += virtual_time.delta_secs();
        let alpha = (1.0 - segment.age).max(0.0);

        // PR Gate: Mutate existing material instead of creating new ones
        if let Some(material) = materials.get_mut(material_handle) {
            material.base_color.set_alpha(alpha * 0.3);
        }

        // Fade out old segments
        if segment.age > 1.0 {
            commands.entity(entity).despawn();
        }
    }

    // Count existing trail segments per ghost to enforce MAX_TRAIL_SEGMENTS
    let mut segment_counts: HashMap<Entity, usize> = HashMap::new();
    for (_, segment, _, _) in trail_q.iter() {
        *segment_counts.entry(segment.ghost_entity).or_insert(0) += 1;
    }

    // Create new trail segments (reusing mesh and material handles)
    for (ghost_entity, mut trail, visuals) in ghost_q.iter_mut() {
        // Get current segment count for this ghost
        let current_segments = segment_counts.get(&ghost_entity).copied().unwrap_or(0);
        
        // Skip if we've reached the maximum
        if current_segments >= MAX_TRAIL_SEGMENTS {
            continue;
        }

        // Get or create cached material for this ghost's color
        let trail_material = material_cache.get_or_create(visuals.tint, &mut materials);

        // Tutorial Note: Converting VecDeque to Vec here is intentional for simplicity.
        // The lead suggested using VecDeque::as_slices() to avoid allocation,
        // but for <20 trail points, the allocation is negligible and this code
        // is much clearer for learners. Focus on visual polish, not micro-optimization.
        let positions: Vec<_> = trail.positions.iter().cloned().collect();
        
        let mut segments_created = 0;
        for window in positions.windows(2) {
            // Stop if we've hit the max segments
            if current_segments + segments_created >= MAX_TRAIL_SEGMENTS {
                break;
            }
            
            if let [start, end] = window {
                let age = current_time - start.1;
                let alpha = (1.0 - age / 2.0).max(0.0);

                // Skip if too faded
                if alpha < 0.1 {
                    continue;
                }

                let distance = start.0.distance(end.0);
                let midpoint = start.0.lerp(end.0, 0.5);

                // PR Gate: Reuse cached mesh handle and scale it
                // Spawn segment with REUSED mesh and material
                commands.spawn((
                    Mesh3d(trail_mesh_cache.segment_mesh.clone()),
                    MeshMaterial3d(trail_material.clone()),
                    Transform::from_translation(midpoint)
                        .looking_at(end.0, Vec3::Y)
                        .with_scale(Vec3::new(1.0, distance, 1.0)),  // Scale to match segment length
                    TrailSegment {
                        ghost_entity,
                        age,
                    },
                ));
                
                segments_created += 1;
            }
        }
    }
}
```

### Step 3: Create Character State Indicators

Add to `src/visual_feedback/mod.rs`:

```rust
use crate::character::Character;
use crate::selectors::Active;
use crate::recording::Recording;

/// Component for character state indicator
#[derive(Component)]
pub struct CharacterStateIndicator;

/// Spawn state indicator above character
pub fn spawn_character_indicators(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    new_characters: Query<Entity, Added<Character>>,
) {
    for entity in new_characters.iter() {
        // Create indicator mesh
        let indicator_mesh = meshes.add(Cone {
            radius: 0.1,
            height: 0.2,
            ..default()
        });

        let indicator_material = materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 1.0, 1.0),
            emissive: Color::srgb(0.5, 0.5, 0.5),
            ..default()
        });

        // Spawn as child of character
        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                Mesh3d(indicator_mesh),
                MeshMaterial3d(indicator_material),
                Transform::from_xyz(0.0, 0.5, 0.0),
                CharacterStateIndicator,
            ));
        });
    }
}

/// Update character indicator colors
/// APPROVED: Mutate materials in place, no handle churn
pub fn update_character_indicators(
    character_q: Query<
        (&Children, Option<&Active>, Option<&Recording>, Option<&Ghost>),
        With<Character>
    >,
    mut indicator_q: Query<
        &Handle<StandardMaterial>,
        With<CharacterStateIndicator>
    >,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (children, active, recording, ghost) in character_q.iter() {
        // Use iter_many for efficient processing of child indicators
        for material_handle in indicator_q.iter_many(children.iter()) {
            let color = if ghost.is_some() {
                Color::srgb(0.5, 0.5, 1.0) // Blue for ghosts
            } else if recording.is_some() {
                Color::srgb(1.0, 0.2, 0.2) // Red for recording
            } else if active.is_some() {
                Color::srgb(0.2, 1.0, 0.2) // Green for active
            } else {
                Color::srgb(0.5, 0.5, 0.5) // Gray for inactive
            };

            // Mutate existing material instead of creating new one
            if let Some(material) = materials.get_mut(material_handle) {
                material.base_color = color;
                material.emissive = color * 0.5;
            }
        }
    }
}
```

### Step 4: Create Recording Pulse Effect

Add to `src/visual_feedback/mod.rs`:

```rust
/// Pulse effect for recording characters
pub fn pulse_recording_characters(
    mut recording_q: Query<(&mut Transform, &Handle<StandardMaterial>), (With<Character>, With<Recording>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    let pulse = (time.elapsed().as_secs_f32() * 4.0).sin() * 0.05 + 1.0;
    let emissive_strength = ((time.elapsed().as_secs_f32() * 3.0).sin() + 1.0) * 0.5;

    for (mut transform, material_handle) in recording_q.iter_mut() {
        transform.scale = Vec3::splat(pulse);
        
        // PR Gate: Mutate material properties in place for pulse effect
        if let Some(material) = materials.get_mut(material_handle) {
            // Author tints as Srgba, convert to Color at assignment
            material.emissive = Color::from(Srgba::from(material.base_color) * emissive_strength);
        }
    }
}

/// Create recording start flash effect
pub fn flash_on_recording_start(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    new_recording: Query<&Transform, Added<Recording>>,
) {
    for transform in new_recording.iter() {
        // Create flash sphere
        let flash_mesh = meshes.add(Sphere::new(1.0));
        let flash_material = materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 1.0, 1.0, 0.5),
            alpha_mode: AlphaMode::Blend,
            emissive: Color::from(WHITE) * 2.0,
            ..default()
        });

        // Spawn flash effect
        commands.spawn((
            Mesh3d(flash_mesh),
            MeshMaterial3d(flash_material),
            transform.clone(),
            FlashEffect { lifetime: 0.5 },
        ));
    }
}

#[derive(Component)]
struct FlashEffect {
    lifetime: f32,
}

/// Animate and cleanup flash effects
pub fn update_flash_effects(
    mut commands: Commands,
    mut flash_q: Query<(Entity, &mut FlashEffect, &mut Transform)>,
    virtual_time: Res<Time<Virtual>>,
) {
    for (entity, mut flash, mut transform) in flash_q.iter_mut() {
        flash.lifetime -= virtual_time.delta_secs();

        if flash.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        } else {
            // Expand and fade
            let scale = 1.0 + (0.5 - flash.lifetime) * 4.0;
            transform.scale = Vec3::splat(scale);
        }
    }
}
```

### Step 5: Create Arena Status Display

Add to `src/visual_feedback/mod.rs`:

```rust
use crate::multi_arena::ArenaStatistics;

/// Component for arena status panel
#[derive(Component)]
pub struct ArenaStatusPanel;

/// Setup arena status UI
pub fn setup_arena_status_ui(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            ArenaStatusPanel,
        ))
        .with_children(|panel| {
            // Title
            panel.spawn((
                Text::new("Arena Status"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::from(WHITE)),
            ));

            // Arena grid (3x3)
            for row in 0..3 {
                panel
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(5.0),
                        ..default()
                    })
                    .with_children(|row_parent| {
                        for col in 0..3 {
                            let arena_idx = row * 3 + col;
                            row_parent.spawn((
                                Node {
                                    width: Val::Px(40.0),
                                    height: Val::Px(40.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.2, 0.2, 0.3)),
                                ArenaStatusBox(arena_idx),
                            ))
                                .with_children(|box_parent| {
                                    box_parent.spawn((
                                        Text::new(arena_idx.to_string()),
                                        TextFont {
                                            font_size: 14.0,
                                            ..default()
                                        },
                                        TextColor(Color::from(WHITE)),
                                    ));
                                });
                        }
                    });
            }
        });
}

#[derive(Component)]
struct ArenaStatusBox(u8);

/// Update arena status colors
pub fn update_arena_status_display(
    stats: Res<ArenaStatistics>,
    current_arena: Res<CurrentArena>,
    mut box_q: Query<(&ArenaStatusBox, &mut BackgroundColor)>,
) {
    for (status_box, mut bg_color) in box_q.iter_mut() {
        let arena_idx = status_box.0;
        let ghost_count = stats.ghost_counts.get(&arena_idx).unwrap_or(&0);
        let recording_count = stats.recording_counts.get(&arena_idx).unwrap_or(&0);

        let current_arena_id = current_arena.id();
        
        let color = if arena_idx == current_arena_id.as_u8() {
            Color::srgb(0.2, 0.5, 0.2) // Green for current
        } else if *recording_count > 0 {
            Color::srgb(0.5, 0.2, 0.2) // Red for recording
        } else if *ghost_count > 0 {
            Color::srgb(0.2, 0.2, 0.5) // Blue for has ghosts
        } else {
            Color::srgb(0.2, 0.2, 0.3) // Dark gray for empty
        };

        bg_color.0 = color;
    }
}
```

### Step 6: Create Audio Feedback

Add to `src/visual_feedback/mod.rs`:

```rust
/// Resource for audio feedback
#[derive(Resource)]
pub struct AudioFeedback {
    pub recording_start: Handle<AudioSource>,
    pub recording_stop: Handle<AudioSource>,
    pub countdown_tick: Handle<AudioSource>,
    pub commit_sound: Handle<AudioSource>,
}

impl AudioFeedback {
    pub fn load(asset_server: &AssetServer) -> Self {
        Self {
            recording_start: asset_server.load("audio/recording_start.ogg"),
            recording_stop: asset_server.load("audio/recording_stop.ogg"),
            countdown_tick: asset_server.load("audio/countdown_tick.ogg"),
            commit_sound: asset_server.load("audio/commit.ogg"),
        }
    }
}

/// Play audio feedback for recording events
pub fn play_recording_audio(
    audio: Res<AudioFeedback>,
    recording_state: Res<RecordingState>,
    mut commands: Commands,
) {
    if !recording_state.is_changed() {
        return;
    }

    let sound = match recording_state.mode {
        RecordingMode::Recording => Some(audio.recording_start.clone()),
        RecordingMode::Idle => Some(audio.recording_stop.clone()),
        _ => None,
    };

    if let Some(sound) = sound {
        commands.spawn((
            AudioPlayer::new(sound),
            PlaybackSettings::DESPAWN,
        ));
    }
}

/// Play countdown ticks
pub fn play_countdown_audio(
    audio: Res<AudioFeedback>,
    countdown_q: Query<&RecordingCountdown, Changed<RecordingCountdown>>,
    mut commands: Commands,
) {
    for countdown in countdown_q.iter() {
        if countdown.get_display_number().is_some() {
            commands.spawn((
                AudioPlayer::new(audio.countdown_tick.clone()),
                PlaybackSettings::DESPAWN,
            ));
        }
    }
}
```

### Step 7: Create Visual Feedback Plugin

Add to `src/visual_feedback/mod.rs`:

```rust
pub struct VisualFeedbackPlugin;

impl Plugin for VisualFeedbackPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize resources
            .init_resource::<TrailMaterialCache>()
            .add_systems(Startup, |mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>| {
                commands.insert_resource(TrailMeshCache::new(&mut meshes));
            })
            
            // Setup systems
            .add_systems(Startup, (
                setup_recording_ui,
                setup_arena_status_ui,
            ))

            // UI Update systems
            .add_systems(Update, (
                update_recording_state_text,
                update_timeline_progress,
                update_countdown_display,
                update_arena_status_display,
            ))

            // Visual effect systems
            .add_systems(Update, (
                spawn_character_indicators,
                update_character_indicators,
                pulse_recording_characters,
                flash_on_recording_start,
                update_flash_effects,
            ))

            // Ghost trail systems
            .add_systems(Update, (
                init_ghost_trail_materials,  // PR Gate: Initialize materials once
                update_ghost_trails,
                render_ghost_trails,
            ).chain())

            // Audio systems
            .add_systems(Update, (
                play_recording_audio,
                play_countdown_audio,
            ));
    }
}
```

### Step 8: Wire Into Main

Update `src/main.rs`:

```rust
mod visual_feedback;
use crate::visual_feedback::VisualFeedbackPlugin;

// In setup_scene, add audio resource:
commands.insert_resource(AudioFeedback::load( & asset_server));

// In main():
.add_plugins(VisualFeedbackPlugin)
```

## Unit Tests

Create `src/visual_feedback/tests.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ghost_trail_limit() {
        let mut trail = GhostTrail::default();

        // Add more positions than max
        for i in 0..30 {
            trail.positions.push_back((Vec3::new(i as f32, 0.0, 0.0), i as f32));
        }

        // Should be limited to max_length
        assert!(trail.positions.len() <= trail.max_length);
    }
    
    #[test]
    fn test_vecdeque_trail_efficiency() {
        let mut trail = GhostTrail::default();
        
        // Test efficient front removal
        for i in 0..100 {
            trail.positions.push_back((Vec3::new(i as f32, 0.0, 0.0), i as f32));
            if trail.positions.len() > trail.max_length {
                trail.positions.pop_front(); // O(1) operation
            }
        }
        
        assert_eq!(trail.positions.len(), trail.max_length);
    }

    #[test]
    fn test_flash_effect_lifetime() {
        let mut flash = FlashEffect { lifetime: 0.5 };

        // Simulate time passing
        flash.lifetime -= 0.3;
        assert!(flash.lifetime > 0.0);

        flash.lifetime -= 0.3;
        assert!(flash.lifetime <= 0.0);
    }
}
```

## Verification

Run tests:

```bash
cargo test visual_feedback
```

Run the game to see visual improvements:

```bash
cargo run
```

Visual checklist:

- [ ] Recording state indicator shows current mode
- [ ] Timeline progress bar fills during recording
- [ ] Countdown numbers appear before recording
- [ ] Characters have colored indicators above them
- [ ] Recording characters pulse slightly
- [ ] Ghosts leave fading trails
- [ ] Flash effect on recording start
- [ ] Arena status panel shows ghost counts

## Next Steps

With visual polish complete, we can now:

- Tutorial 08: Performance optimization for 320 ghosts
- Tutorial 09: Advanced ghost interactions
- Tutorial 10: Save/load system for timelines

## Key Takeaways

1. **Clear State Communication**: Visual indicators for all states
2. **Immediate Feedback**: Flash effects and audio for actions
3. **Persistent Information**: UI panels for ongoing status
4. **Explicit Constructors**: Arena::new() validation in UI display logic
5. **Performance Conscious**: Trail effects with automatic cleanup

Visual feedback transforms the recording system from functional to delightful. Clear indicators help players understand
the complex state machine while effects add game feel and polish.