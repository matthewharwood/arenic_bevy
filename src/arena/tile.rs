use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;

use super::TILE_SIZE;

const TILE_DEPTH: f32 = 2.0;
const INSET_SIZE: f32 = 8.0;
const INSET_DEPTH: f32 = 1.0;

/// Build meshes for the arena tile: the gray base and the pink inset.
pub fn build_tile_meshes() -> (Mesh, Mesh) {
    (build_base_mesh(), build_inset_mesh())
}

fn build_base_mesh() -> Mesh {
    let outer = TILE_SIZE / 2.0;
    let inner = INSET_SIZE / 2.0;
    let top = 0.0;
    let bottom = -TILE_DEPTH;

    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();
    let mut i = 0u32;

    let mut add_quad = |verts: [[f32; 3]; 4], normal: [f32; 3]| {
        positions.extend_from_slice(&verts);
        normals.extend_from_slice(&[normal; 4]);
        uvs.extend_from_slice(&[[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
        indices.extend_from_slice(&[i, i + 1, i + 2, i, i + 2, i + 3]);
        i += 4;
    };

    // Top ring
    add_quad(
        [
            [-outer, inner, top],
            [outer, inner, top],
            [outer, outer, top],
            [-outer, outer, top],
        ],
        [0.0, 0.0, 1.0],
    );
    add_quad(
        [
            [-outer, -outer, top],
            [outer, -outer, top],
            [outer, -inner, top],
            [-outer, -inner, top],
        ],
        [0.0, 0.0, 1.0],
    );
    add_quad(
        [
            [-outer, -inner, top],
            [-inner, -inner, top],
            [-inner, inner, top],
            [-outer, inner, top],
        ],
        [0.0, 0.0, 1.0],
    );
    add_quad(
        [
            [inner, -inner, top],
            [outer, -inner, top],
            [outer, inner, top],
            [inner, inner, top],
        ],
        [0.0, 0.0, 1.0],
    );

    // Outer sides
    add_quad(
        [
            [-outer, outer, bottom],
            [-outer, outer, top],
            [outer, outer, top],
            [outer, outer, bottom],
        ],
        [0.0, 1.0, 0.0],
    );
    add_quad(
        [
            [-outer, -outer, top],
            [-outer, -outer, bottom],
            [outer, -outer, bottom],
            [outer, -outer, top],
        ],
        [0.0, -1.0, 0.0],
    );
    add_quad(
        [
            [-outer, -outer, bottom],
            [-outer, -outer, top],
            [-outer, outer, top],
            [-outer, outer, bottom],
        ],
        [-1.0, 0.0, 0.0],
    );
    add_quad(
        [
            [outer, -outer, top],
            [outer, -outer, bottom],
            [outer, outer, bottom],
            [outer, outer, top],
        ],
        [1.0, 0.0, 0.0],
    );

    // Bottom
    add_quad(
        [
            [-outer, -outer, bottom],
            [-outer, outer, bottom],
            [outer, outer, bottom],
            [outer, -outer, bottom],
        ],
        [0.0, 0.0, -1.0],
    );

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_indices(Indices::U32(indices));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh
}

fn build_inset_mesh() -> Mesh {
    let inner = INSET_SIZE / 2.0;
    let top = 0.0;
    let bottom = -INSET_DEPTH;

    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();
    let mut i = 0u32;

    let mut add_quad = |verts: [[f32; 3]; 4], normal: [f32; 3]| {
        positions.extend_from_slice(&verts);
        normals.extend_from_slice(&[normal; 4]);
        uvs.extend_from_slice(&[[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
        indices.extend_from_slice(&[i, i + 1, i + 2, i, i + 2, i + 3]);
        i += 4;
    };

    // Floor
    add_quad(
        [
            [-inner, -inner, bottom],
            [inner, -inner, bottom],
            [inner, inner, bottom],
            [-inner, inner, bottom],
        ],
        [0.0, 0.0, 1.0],
    );

    // Walls
    add_quad(
        [
            [-inner, inner, top],
            [-inner, inner, bottom],
            [inner, inner, bottom],
            [inner, inner, top],
        ],
        [0.0, -1.0, 0.0],
    );
    add_quad(
        [
            [inner, -inner, top],
            [inner, -inner, bottom],
            [-inner, -inner, bottom],
            [-inner, -inner, top],
        ],
        [0.0, 1.0, 0.0],
    );
    add_quad(
        [
            [-inner, -inner, top],
            [-inner, -inner, bottom],
            [-inner, inner, bottom],
            [-inner, inner, top],
        ],
        [1.0, 0.0, 0.0],
    );
    add_quad(
        [
            [inner, inner, top],
            [inner, inner, bottom],
            [inner, -inner, bottom],
            [inner, -inner, top],
        ],
        [-1.0, 0.0, 0.0],
    );

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_indices(Indices::U32(indices));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh
}
