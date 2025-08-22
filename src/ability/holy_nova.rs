// Standard library and external crates
use bevy::audio::{AudioPlayer, PlaybackSettings};
use bevy::pbr::MeshMaterial3d;
use bevy::prelude::*;

// Local crate modules
use crate::ability::{AbilityType, Duration, ElapsedTime, EndRadius, StartRadius};
use crate::arena::{Arena, ArenaEntities, CurrentArena};
use crate::audio::Audio;
use crate::character::Character;
use crate::materials::Materials;
use crate::recording::{Recording, RecordingMode, RecordingState};
use crate::selectors::Active;
use crate::timeline::{
    DraftTimeline, EventType, GlobalTimelinePause, TimelineClock, TimelineEvent,
};

#[derive(Component, Debug)]
pub struct HolyNova;

#[derive(Component, Debug)]
pub struct HolyNovaVfx;

impl HolyNovaVfx {
    pub fn new() -> Self {
        Self
    }
}

/// Spawns a holy nova VFX sphere at the active character when the user presses '1'.
pub fn holy_nova_ability(
    mut commands: Commands,
    mats: Res<Materials>,
    audio: Res<Audio>,
    mut meshes: ResMut<Assets<Mesh>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    character_q: Query<
        (Entity, Option<&Recording>),
        (With<Character>, With<Active>, With<HolyNova>),
    >,
    // Recording system integration
    recording_state: Res<RecordingState>,
    mut draft_timeline: ResMut<DraftTimeline>,
    arena_q: Query<(&Arena, &TimelineClock)>,
    current_arena: Res<CurrentArena>,
    arena_entities_res: Res<ArenaEntities>,
    global_pause: Res<GlobalTimelinePause>,
) {
    // Trigger on '1' key (both main row and numpad)
    let pressed = keyboard_input.just_pressed(KeyCode::Digit1)
        || keyboard_input.just_pressed(KeyCode::Numpad1);

    if !pressed {
        return;
    }

    // Check if there are any valid characters with HolyNova before executing
    if character_q.is_empty() {
        return;
    }

    // Handle recording for characters that are recording
    for (_character_entity, recording_marker) in character_q.iter() {
        if let Some(_) = recording_marker {
            if recording_state.mode == RecordingMode::Recording && !global_pause.is_paused {
                let current_arena_entity = arena_entities_res.get(current_arena.0);
                if let Ok((_, clock)) = arena_q.get(current_arena_entity) {
                    let timestamp = clock.current();

                    let event = TimelineEvent {
                        timestamp,
                        event_type: EventType::Ability(AbilityType::HolyNova, None),
                    };

                    if let Err(e) = draft_timeline.add_event(event) {
                        bevy::log::warn!("Failed to record ability event: {:?}", e);
                    } else {
                        bevy::log::trace!("Recorded HolyNova ability at {}", timestamp);
                    }
                }
            }
        }
    }

    // Play the holy nova sound effect with automatic cleanup
    commands.spawn((
        AudioPlayer::new(audio.holy_nova.clone()),
        PlaybackSettings::DESPAWN,
    ));

    // Spawn a VFX sphere as a child of each active character
    for (character_entity, _) in character_q.iter() {
        let vfx_mesh = meshes.add(Sphere::new(0.0625)); // unit sphere, scale controls radius
        commands.entity(character_entity).with_child((
            HolyNovaVfx::new(),
            ElapsedTime(0.0),
            Duration(0.225), // seconds
            StartRadius(4.0),
            EndRadius(32.0),
            Transform::from_scale(Vec3::splat(0.125 * 4.0)), // start radius
            Mesh3d(vfx_mesh),
            MeshMaterial3d(mats.yellow.clone()),
        ));
    }
}

/// Updates scaling over time with ease-in and despawns when finished.
pub fn update_holy_nova_vfx(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<
        (
            Entity,
            &mut Transform,
            &mut ElapsedTime,
            &Duration,
            &StartRadius,
            &EndRadius,
        ),
        With<HolyNovaVfx>,
    >,
) {
    for (entity, mut transform, mut elapsed, duration, start_radius, end_radius) in query.iter_mut()
    {
        elapsed.0 += time.delta_secs();
        let t = (elapsed.0 / duration.0).clamp(0.0, 1.0);

        // Use Bevy's official cubic ease-in function
        let easing_curve = EasingCurve::new(0.0, 1.0, EaseFunction::ExponentialOut);
        let eased = easing_curve.sample(t).unwrap_or(0.0);

        let radius = start_radius.0 + (end_radius.0 - start_radius.0) * eased;
        transform.scale = Vec3::splat(radius);

        if elapsed.0 >= duration.0 {
            commands.entity(entity).despawn();
        }
    }
}
