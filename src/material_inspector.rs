use bevy::prelude::*;
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};

/// Plugin for runtime material inspection
pub struct MaterialInspectorPlugin;

impl Plugin for MaterialInspectorPlugin {
    fn build(&self, app: &mut App) {
        // Add wireframe plugin for better mesh visualization
        if !app.is_plugin_added::<WireframePlugin>() {
            app.add_plugins(WireframePlugin);
        }
        
        app.insert_resource(InspectorSettings::default())
            .add_systems(Update, (
                toggle_inspector,
                update_material_display,
                cycle_through_materials,
            ));
    }
}

#[derive(Resource)]
struct InspectorSettings {
    enabled: bool,
    show_wireframe: bool,
    highlight_pink: bool,
    current_material_index: usize,
}

impl Default for InspectorSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            show_wireframe: false,
            highlight_pink: true,
            current_material_index: 0,
        }
    }
}

fn toggle_inspector(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<InspectorSettings>,
    mut wireframe_config: ResMut<WireframeConfig>,
) {
    // Press I to toggle inspector
    if keyboard.just_pressed(KeyCode::KeyI) {
        settings.enabled = !settings.enabled;
        info!("Material Inspector: {}", if settings.enabled { "ENABLED" } else { "DISABLED" });
        
        if settings.enabled {
            info!("Controls:");
            info!("  I - Toggle inspector");
            info!("  W - Toggle wireframe");
            info!("  P - Toggle pink highlight");
            info!("  N/M - Cycle through materials");
        }
    }
    
    // Press W to toggle wireframe
    if keyboard.just_pressed(KeyCode::KeyW) && settings.enabled {
        settings.show_wireframe = !settings.show_wireframe;
        wireframe_config.global = settings.show_wireframe;
        info!("Wireframe: {}", if settings.show_wireframe { "ON" } else { "OFF" });
    }
    
    // Press P to toggle pink highlighting
    if keyboard.just_pressed(KeyCode::KeyP) && settings.enabled {
        settings.highlight_pink = !settings.highlight_pink;
        info!("Pink Highlight: {}", if settings.highlight_pink { "ON" } else { "OFF" });
    }
}

fn update_material_display(
    settings: Res<InspectorSettings>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    if !settings.enabled || !settings.highlight_pink {
        return;
    }
    
    // Pulse pink materials to make them more visible
    let pulse = (time.elapsed_secs() * 2.0).sin() * 0.5 + 0.5;
    
    for (_, material) in materials.iter_mut() {
        if is_pink_material(material) {
            // Make pink materials pulse
            let base_emissive = material.emissive;
            material.emissive = base_emissive * pulse;
        }
    }
}

fn cycle_through_materials(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<InspectorSettings>,
    materials: Res<Assets<StandardMaterial>>,
    query: Query<&MeshMaterial3d<StandardMaterial>>,
) {
    if !settings.enabled {
        return;
    }
    
    let material_count = materials.len();
    if material_count == 0 {
        return;
    }
    
    // Press N for next material, M for previous
    if keyboard.just_pressed(KeyCode::KeyN) {
        settings.current_material_index = (settings.current_material_index + 1) % material_count;
        inspect_current_material(&settings, &materials);
    } else if keyboard.just_pressed(KeyCode::KeyM) {
        settings.current_material_index = if settings.current_material_index == 0 {
            material_count - 1
        } else {
            settings.current_material_index - 1
        };
        inspect_current_material(&settings, &materials);
    }
}

fn inspect_current_material(
    settings: &InspectorSettings,
    materials: &Assets<StandardMaterial>,
) {
    if let Some((handle_id, material)) = materials.iter().nth(settings.current_material_index) {
        info!("=== Material {} of {} ===", settings.current_material_index + 1, materials.len());
        info!("Handle: {:?}", handle_id);
        info!("Base Color: {:?}", material.base_color);
        info!("Emissive: {:?}", material.emissive);
        info!("Unlit: {}", material.unlit);
        info!("Metallic: {}", material.metallic);
        info!("Roughness: {}", material.perceptual_roughness);
        info!("Alpha Mode: {:?}", material.alpha_mode);
        
        if is_pink_material(material) {
            info!(">>> This is a PINK material! <<<");
        }
    }
}

fn is_pink_material(material: &StandardMaterial) -> bool {
    is_pink_color(material.base_color) || is_pink_color(material.emissive.to_linear())
}

fn is_pink_color(color: LinearRgba) -> bool {
    color.red > 0.7 && color.blue > 0.7 && color.green < 0.3
}