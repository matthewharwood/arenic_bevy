use bevy::prelude::*;
use bevy::gltf::GltfMesh;
use bevy::pbr::StandardMaterial;
use bevy::render::mesh::Mesh;

/// Plugin for debugging and fixing tile materials
pub struct MaterialDebuggerPlugin;

impl Plugin for MaterialDebuggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_debugger)
            .add_systems(Update, (
                debug_loaded_materials.run_if(resource_exists::<DebuggerState>),
                fix_pink_inset_material.run_if(resource_exists::<DebuggerState>),
                continuously_monitor_pink_materials.run_if(resource_exists::<DebuggerState>),
            ).chain());
    }
}

#[derive(Resource)]
struct DebuggerState {
    tile_scene_handle: Handle<Scene>,
    has_inspected: bool,
    has_fixed: bool,
}

/// Marker component for tiles that have been fixed
#[derive(Component)]
struct MaterialFixed;

fn setup_debugger(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Store the tile scene handle for inspection
    let tile_scene_handle = asset_server.load("tile.glb#Scene0");
    
    commands.insert_resource(DebuggerState {
        tile_scene_handle: tile_scene_handle.clone(),
        has_inspected: false,
        has_fixed: false,
    });
    
    info!("Material Debugger: Initialized. Monitoring tile.glb materials...");
}

fn debug_loaded_materials(
    mut debugger_state: ResMut<DebuggerState>,
    scenes: Res<Assets<Scene>>,
    _gltf_meshes: Res<Assets<GltfMesh>>,
    _meshes: Res<Assets<Mesh>>,
    materials: Res<Assets<StandardMaterial>>,
    _asset_server: Res<AssetServer>,
    query: Query<(Entity, &SceneRoot), Without<MaterialFixed>>,
) {
    if debugger_state.has_inspected {
        return;
    }
    
    // Check if the scene is loaded
    if let Some(_scene) = scenes.get(&debugger_state.tile_scene_handle) {
        info!("Material Debugger: Scene loaded successfully!");
        debugger_state.has_inspected = true;
        
        // Count entities using this scene
        let tile_count = query.iter().filter(|(_, scene_root)| {
            scene_root.0 == debugger_state.tile_scene_handle
        }).count();
        
        info!("Material Debugger: Found {} tile instances in the world", tile_count);
    }
    
    // Inspect all loaded materials
    let mut material_count = 0;
    let mut emissive_materials = Vec::new();
    let mut non_emissive_materials = Vec::new();
    
    for (handle_id, material) in materials.iter() {
        material_count += 1;
        
        let material_info = MaterialInfo {
            base_color: material.base_color,
            emissive: material.emissive,
            metallic: material.metallic,
            roughness: material.perceptual_roughness,
            unlit: material.unlit,
        };
        
        // Check if this is likely the pink inset material
        let is_pink = is_pink_color(material.base_color) || is_pink_linear(material.emissive);
        let is_emissive = material.emissive != LinearRgba::BLACK || material.unlit;
        
        if is_emissive || is_pink {
            emissive_materials.push((handle_id, material_info, is_pink));
            info!(
                "Material Debugger: Found potential inset material (Handle: {:?})",
                handle_id
            );
            info!("  - Base Color: {:?}", material.base_color);
            info!("  - Emissive: {:?}", material.emissive);
            info!("  - Unlit: {}", material.unlit);
            info!("  - Is Pink: {}", is_pink);
        } else {
            non_emissive_materials.push((handle_id, material_info));
        }
    }
    
    info!("Material Debugger: Total materials loaded: {}", material_count);
    info!("Material Debugger: Emissive/Pink materials: {}", emissive_materials.len());
    info!("Material Debugger: Non-emissive materials: {}", non_emissive_materials.len());
    
    // Detailed analysis
    if emissive_materials.is_empty() {
        warn!("Material Debugger: NO EMISSIVE MATERIALS FOUND!");
        warn!("This suggests the pink inset material may not be properly exported from Blender.");
        warn!("Please check:");
        warn!("  1. The inset face has the emission material assigned in Blender");
        warn!("  2. The emission shader is connected directly to Material Output");
        warn!("  3. The glTF export settings include materials");
    } else {
        for (i, (_, info, is_pink)) in emissive_materials.iter().enumerate() {
            info!("Emissive Material #{}", i + 1);
            info!("  - Base Color: {:?}", info.base_color);
            info!("  - Emissive: {:?}", info.emissive);
            info!("  - Unlit: {}", info.unlit);
            if !is_pink {
                warn!("  - WARNING: This emissive material is NOT pink!");
            }
        }
    }
}

fn fix_pink_inset_material(
    mut debugger_state: ResMut<DebuggerState>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Children), With<SceneRoot>>,
    mesh_query: Query<&MeshMaterial3d<StandardMaterial>>,
    mut commands: Commands,
) {
    if !debugger_state.has_inspected || debugger_state.has_fixed {
        return;
    }
    
    let mut fixed_count = 0;
    let mut checked_count = 0;
    let mut all_material_info = Vec::new();
    
    // Define the hot pink color from the tutorial (#ff00ff)
    let hot_pink_srgba = Srgba::hex("ff00ff").unwrap();
    let hot_pink_linear = LinearRgba::from(hot_pink_srgba);
    
    for (entity, children) in query.iter() {
        // Check all children for mesh materials
        for child in children.iter() {
            if let Ok(mesh_material) = mesh_query.get(child) {
                checked_count += 1;
                
                if let Some(material) = materials.get_mut(&mesh_material.0) {
                    // Log every material we find for debugging
                    let is_pink = is_pink_color(material.base_color) || is_pink_linear(material.emissive);
                    let material_info = format!(
                        "Material {}: Base={:?}, Emissive={:?}, Unlit={}, IsPink={}",
                        checked_count, material.base_color, material.emissive, material.unlit, is_pink
                    );
                    all_material_info.push(material_info.clone());
                    info!("Material Debugger: {}", material_info);
                    
                    // MORE AGGRESSIVE FIX: If material is pink in ANY way, make it unlit
                    if is_pink {
                        if !material.unlit {
                            info!("Material Debugger: Found pink material with unlit=false! Fixing...");
                            info!("  - Previous: Base={:?}, Emissive={:?}, Unlit={}", 
                                material.base_color, material.emissive, material.unlit);
                        }
                        // Always ensure pink materials are unlit and emissive
                        material.emissive = hot_pink_linear;
                        material.unlit = true;
                        material.base_color = Color::from(hot_pink_srgba);
                        fixed_count += 1;
                        info!("  - Fixed to: Base={:?}, Emissive={:?}, Unlit={}", 
                            material.base_color, material.emissive, material.unlit);
                    } else if material.emissive != LinearRgba::BLACK {
                        // If material has any emissive value but isn't pink, it might be a 
                        // wrongly imported pink material
                        info!("Material Debugger: Found non-pink emissive material. Converting to pink...");
                        material.base_color = Color::from(hot_pink_srgba);
                        material.emissive = hot_pink_linear;
                        material.unlit = true;
                        fixed_count += 1;
                    } else if checked_count > 1 && material.metallic < 0.1 && material.perceptual_roughness > 0.7 {
                        // Fallback: second material with low metallic/high roughness might be the inset
                        info!("Material Debugger: Found potential inset material (material #{}). Applying pink fix...", checked_count);
                        material.base_color = Color::from(hot_pink_srgba);
                        material.emissive = hot_pink_linear;
                        material.unlit = true;
                        fixed_count += 1;
                    }
                }
            }
        }
        
        // Mark this entity as checked
        commands.entity(entity).insert(MaterialFixed);
    }
    
    // Summary report
    info!("Material Debugger: === MATERIAL FIX SUMMARY ===");
    info!("Total materials checked: {}", checked_count);
    info!("Materials fixed: {}", fixed_count);
    for (i, info) in all_material_info.iter().enumerate() {
        info!("  [{}] {}", i + 1, info);
    }
    
    if fixed_count > 0 {
        info!("Material Debugger: Successfully fixed {} materials to be unlit pink emissive!", fixed_count);
    } else if checked_count > 0 {
        warn!("Material Debugger: No pink materials found to fix!");
        warn!("This suggests the glTF file doesn't contain the pink material.");
        warn!("Please verify in Blender that:");
        warn!("  1. The inset faces have a separate material slot");
        warn!("  2. The emission shader is assigned to those faces");
        warn!("  3. The glTF export includes materials");
    }
    
    debugger_state.has_fixed = true;
}

/// Continuously monitor and fix any pink materials that aren't unlit
fn continuously_monitor_pink_materials(
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    // Only check every second to avoid performance impact
    if time.elapsed_secs() % 1.0 > 0.1 {
        return;
    }
    
    let hot_pink_srgba = Srgba::hex("ff00ff").unwrap();
    let hot_pink_linear = LinearRgba::from(hot_pink_srgba);
    let mut fixed_count = 0;
    
    for (_handle_id, material) in materials.iter_mut() {
        // Check if this is a pink material that isn't properly unlit
        let is_pink = is_pink_color(material.base_color) || is_pink_linear(material.emissive);
        
        if is_pink && !material.unlit {
            // Fix it immediately
            warn!("Material Debugger: CONTINUOUS MONITOR - Found pink material with unlit=false!");
            warn!("  - Current state: Base={:?}, Emissive={:?}, Unlit={}", 
                material.base_color, material.emissive, material.unlit);
            
            material.unlit = true;
            material.emissive = hot_pink_linear;
            material.base_color = Color::from(hot_pink_srgba);
            fixed_count += 1;
            
            warn!("  - Fixed to: Base={:?}, Emissive={:?}, Unlit={}", 
                material.base_color, material.emissive, material.unlit);
        }
    }
    
    if fixed_count > 0 {
        warn!("Material Debugger: CONTINUOUS MONITOR - Fixed {} pink materials that reverted to unlit=false!", fixed_count);
    }
}

/// Helper to determine if a Color is pink/magenta
fn is_pink_color(color: Color) -> bool {
    // Convert to linear for accurate comparison
    let linear = color.to_linear();
    
    // Check multiple conditions for pink detection:
    // 1. Classic pink: high red and blue, low green
    let is_classic_pink = linear.red > 0.7 && linear.blue > 0.7 && linear.green < 0.3;
    
    // 2. Any shade of pink: red and blue dominant over green
    let is_any_pink = linear.red > 0.5 && linear.blue > 0.5 && 
                      linear.green < linear.red * 0.5 && linear.green < linear.blue * 0.5;
    
    // 3. Check if it matches hot pink specifically (#ff00ff)
    let is_hot_pink = (linear.red - 1.0).abs() < 0.1 && 
                      (linear.blue - 1.0).abs() < 0.1 && 
                      linear.green < 0.1;
    
    is_classic_pink || is_any_pink || is_hot_pink
}

/// Helper to determine if a LinearRgba is pink/magenta
fn is_pink_linear(color: LinearRgba) -> bool {
    // Check multiple conditions for pink detection:
    // 1. Classic pink: high red and blue, low green
    let is_classic_pink = color.red > 0.7 && color.blue > 0.7 && color.green < 0.3;
    
    // 2. Any shade of pink: red and blue dominant over green
    let is_any_pink = color.red > 0.5 && color.blue > 0.5 && 
                      color.green < color.red * 0.5 && color.green < color.blue * 0.5;
    
    // 3. Check if it matches hot pink specifically (#ff00ff)
    let is_hot_pink = (color.red - 1.0).abs() < 0.1 && 
                      (color.blue - 1.0).abs() < 0.1 && 
                      color.green < 0.1;
    
    // 4. Any non-black color with red and blue components (very lenient)
    let has_pink_components = color.red > 0.1 && color.blue > 0.1 && 
                             color.green < (color.red + color.blue) * 0.3;
    
    is_classic_pink || is_any_pink || is_hot_pink || has_pink_components
}

#[derive(Debug)]
struct MaterialInfo {
    base_color: Color,
    emissive: LinearRgba,
    metallic: f32,
    roughness: f32,
    unlit: bool,
}