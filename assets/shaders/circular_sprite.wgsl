#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var texture: texture_2d<f32>;
@group(2) @binding(1) var texture_sampler: sampler;
@group(2) @binding(2) var<uniform> atlas_info: vec4<f32>; // x: cols, y: rows, z: current_index, w: unused

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    // Get UV coordinates (0.0 to 1.0)
    let uv = mesh.uv;
    
    // Calculate distance from center
    let center = vec2<f32>(0.5, 0.5);
    let distance = length(uv - center);
    
    // Create circular mask - discard pixels outside radius 0.5
    if (distance > 0.5) {
        discard;
    }
    
    // Calculate atlas UV coordinates
    let cols = atlas_info.x;
    let rows = atlas_info.y;
    let current_index = atlas_info.z;
    
    // Calculate which cell we're in
    let col = current_index % cols;
    let row = floor(current_index / cols);
    
    // Calculate cell size
    let cell_size = vec2<f32>(1.0 / cols, 1.0 / rows);
    
    // Calculate offset for current frame
    let atlas_offset = vec2<f32>(col * cell_size.x, row * cell_size.y);
    
    // Scale UV to cell size and add offset
    let atlas_uv = uv * cell_size + atlas_offset;
    
    // Sample the texture
    let color = textureSample(texture, texture_sampler, atlas_uv);
    
    return color;
}