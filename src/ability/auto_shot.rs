use crate::ability::{Origin, Projectile, Target, TimeToLive};
use crate::arena::TILE_SIZE;
use crate::character::{Boss, Character};
use crate::materials::Materials;
use crate::selectors::Active;
use bevy::asset::{Assets, AssetServer, Handle};
use bevy::audio::{AudioPlayer, AudioSource};
use bevy::pbr::MeshMaterial3d;
use bevy::prelude::{
    Commands, Component, Entity, GlobalTransform, Local, Mesh, Mesh3d, Query, Res, ResMut, Sphere,
    Time, Timer, TimerMode, Transform, With,
};

#[derive(Component, Debug)]
pub struct AutoShot {
    pub(crate) distance: f32,
}

impl AutoShot {
    pub fn new(dist: f32) -> Self {
        Self {
            distance: (TILE_SIZE * dist).round(),
        }
    }
}

/// System to move projectiles using lerp with single-purpose components
pub fn move_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut TimeToLive, &Origin, &Target), With<Projectile>>,
) {
    for (entity, mut transform, mut ttl, origin, target) in query.iter_mut() {
        // Update elapsed time
        ttl.0 += time.delta_secs();

        // Calculate lerp progress (0.0 to 1.0)
        let progress = (ttl.0 / ttl.1).clamp(0.0, 1.0);

        // Lerp between origin and target
        transform.translation = origin.0.lerp(target.0, progress);

        // Despawn when lifetime expires
        if progress >= 1.0 {
            commands.entity(entity).despawn();
        }
    }
}
pub fn tile_dist(pos1: (f32, f32), pos2: (f32, f32)) -> f32 {
    let dx = (pos1.0 - pos2.0).abs();
    let dy = (pos1.1 - pos2.1).abs();
    dx.max(dy).round()
}

/// System that handles autoshot ability - spawns projectiles as independent entities
pub fn auto_shot_ability(
    mut commands: Commands,
    mats: Res<Materials>,
    mut meshes: ResMut<Assets<Mesh>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut timer: Local<Timer>,
    mut sound_resource: Local<Option<Handle<AudioSource>>>,
    character_query: Query<(&GlobalTransform, &AutoShot), (With<Character>, With<AutoShot>)>,
    boss_query: Query<&GlobalTransform, (With<Boss>, With<Active>)>,
) {
    // Initialize timer and load sound on first run
    if timer.duration().as_secs_f32() == 0.0 {
        *timer = Timer::from_seconds(1.0, TimerMode::Repeating);
        
        // Load the autoshot sound effect once
        *sound_resource = Some(asset_server.load("abilities/autoshot.mp3"));
    }

    timer.tick(time.delta());

    // Only proceed if timer finished
    if !timer.just_finished() {
        return;
    }

    // Iterate over all characters with AutoShot
    for (character_transform, autoshot) in character_query.iter() {
        let character_pos = character_transform.translation();

        // Check distance to all bosses
        for boss_transform in boss_query.iter() {
            let boss_pos = boss_transform.translation();

            let x1 = character_pos.x;
            let y1 = character_pos.y;
            let x2 = boss_pos.x;
            let y2 = boss_pos.y;

            if tile_dist((x1, y1), (x2, y2)) <= autoshot.distance {
                let distance = character_pos.distance(boss_pos);
                let travel_time = distance / TILE_SIZE; // 1 tile per second

                let projectile_radius = 2.5;
                let projectile_mesh = meshes.add(Sphere::new(projectile_radius));

                // Spawn projectile
                commands.spawn((
                    Projectile,
                    Transform::from_translation(character_pos),
                    Origin(character_pos),
                    Target(boss_pos),
                    TimeToLive(0.0, travel_time),
                    Mesh3d(projectile_mesh),
                    MeshMaterial3d(mats.black.clone()),
                ));
                
                // Play the autoshot sound effect
                if let Some(sound_handle) = &*sound_resource {
                    commands.spawn((
                        AudioPlayer::new(sound_handle.clone()),
                    ));
                }
            }
        }
    }
}
