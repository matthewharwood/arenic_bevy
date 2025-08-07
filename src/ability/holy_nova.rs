use crate::character::Character;
use crate::materials::Materials;
use crate::selectors::Active;
use bevy::pbr::MeshMaterial3d;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct HolyNova;

#[derive(Component)]
pub struct HolyNovaVfx {
    pub elapsed: f32,
    pub duration: f32,
    pub start_radius: f32,
    pub end_radius: f32,
}

impl HolyNovaVfx {
    pub fn new() -> Self {
        Self {
            elapsed: 0.0,
            duration: 0.3, // seconds
            start_radius: 4.0,
            end_radius: 32.0,
        }
    }
}

/// Spawns a holy nova VFX sphere at the active character when the user presses '1'.
pub fn holy_nova_ability(
    mut commands: Commands,
    mats: Res<Materials>,
    mut meshes: ResMut<Assets<Mesh>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    character_q: Query<Entity, (With<Character>, With<Active>, With<HolyNova>)>,
) {
    // Trigger on '1' key (both main row and numpad)
    let pressed = keyboard_input.just_pressed(KeyCode::Digit1)
        || keyboard_input.just_pressed(KeyCode::Numpad1);

    if !pressed {
        return;
    }

    // Spawn a VFX sphere as a child of each active character
    for character_entity in character_q.iter() {
        let vfx_mesh = meshes.add(Sphere::new(1.0)); // unit sphere, scale controls radius
        commands.entity(character_entity).with_child((
            HolyNovaVfx::new(),
            Transform::from_scale(Vec3::splat(4.0)), // start radius
            Mesh3d(vfx_mesh),
            MeshMaterial3d(mats.yellow.clone()),
        ));
    }
}

/// Updates scaling over time with ease-in and despawns when finished.
pub fn update_holy_nova_vfx(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut HolyNovaVfx)>,
) {
    for (entity, mut transform, mut vfx) in query.iter_mut() {
        vfx.elapsed += time.delta_secs();
        let t = (vfx.elapsed / vfx.duration).clamp(0.0, 1.0);

        // Use Bevy's official cubic ease-in function
        let easing_curve = EasingCurve::new(0.0, 1.0, EaseFunction::CubicIn);
        let eased = easing_curve.sample(t).unwrap_or(0.0);

        let radius = vfx.start_radius + (vfx.end_radius - vfx.start_radius) * eased;
        transform.scale = Vec3::splat(radius);

        if vfx.elapsed >= vfx.duration {
            commands.entity(entity).despawn();
        }
    }
}
