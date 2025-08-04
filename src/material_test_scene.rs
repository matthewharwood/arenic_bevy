use bevy::prelude::*;

/// Plugin that creates a test scene to verify material rendering
pub struct MaterialTestScenePlugin;

impl Plugin for MaterialTestScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_material_test_scene);
    }
}

fn spawn_material_test_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("Material Test Scene: Creating reference materials...");
    
    // Create a sphere mesh for testing
    let sphere = meshes.add(Sphere::new(2.0));
    
    // Define hot pink color from the tutorial
    let hot_pink = Srgba::hex("ff00ff").unwrap();
    
    // Test position offset
    let test_offset = Vec3::new(-100.0, 50.0, 5.0);
    
    // 1. Standard pink material (lit)
    let standard_pink = materials.add(StandardMaterial {
        base_color: hot_pink.into(),
        ..default()
    });
    
    commands.spawn((
        Mesh3d(sphere.clone()),
        MeshMaterial3d(standard_pink),
        Transform::from_translation(test_offset + Vec3::new(0.0, 0.0, 0.0)),
    ));
    
    // 2. Emissive pink material (should glow)
    let emissive_pink = materials.add(StandardMaterial {
        base_color: hot_pink.into(),
        emissive: LinearRgba::from(hot_pink),
        emissive_exposure_weight: 1.0,
        ..default()
    });
    
    commands.spawn((
        Mesh3d(sphere.clone()),
        MeshMaterial3d(emissive_pink),
        Transform::from_translation(test_offset + Vec3::new(10.0, 0.0, 0.0)),
    ));
    
    // 3. Unlit pink material (should be constantly bright)
    let unlit_pink = materials.add(StandardMaterial {
        base_color: hot_pink.into(),
        unlit: true,
        ..default()
    });
    
    commands.spawn((
        Mesh3d(sphere.clone()),
        MeshMaterial3d(unlit_pink),
        Transform::from_translation(test_offset + Vec3::new(20.0, 0.0, 0.0)),
    ));
    
    // 4. Emissive + Unlit pink (maximum brightness)
    let emissive_unlit_pink = materials.add(StandardMaterial {
        base_color: hot_pink.into(),
        emissive: LinearRgba::from(hot_pink),
        unlit: true,
        ..default()
    });
    
    commands.spawn((
        Mesh3d(sphere.clone()),
        MeshMaterial3d(emissive_unlit_pink),
        Transform::from_translation(test_offset + Vec3::new(30.0, 0.0, 0.0)),
    ));
    
    // 5. Gray reference material (for comparison)
    let gray = materials.add(StandardMaterial {
        base_color: Srgba::gray(0.6).into(),
        ..default()
    });
    
    commands.spawn((
        Mesh3d(sphere.clone()),
        MeshMaterial3d(gray),
        Transform::from_translation(test_offset + Vec3::new(40.0, 0.0, 0.0)),
    ));
    
    // Add labels for each sphere
    info!("Material Test Scene: Created 5 test spheres:");
    info!("  1. Standard Pink (affected by lighting)");
    info!("  2. Emissive Pink (glows)");
    info!("  3. Unlit Pink (constant brightness)");
    info!("  4. Emissive + Unlit Pink (maximum brightness)");
    info!("  5. Gray Reference (standard material)");
    info!("Look for these spheres to the left of your arena grid.");
}