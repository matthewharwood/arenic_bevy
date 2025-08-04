# Blender Tile Asset Tutorial

This guide walks you through creating a replacement arena tile for Arenic using Blender and exporting it as a glTF file. The tile matches the game's current sizing and demonstrates Physically Based Rendering (PBR) options, including an unlit hot pink inset.

## Prerequisites
- Blender 4.x
- ~30 minutes
- This repository checked out so you can verify sizes in `src/main.rs`

## 1. Confirm Tile Dimensions
1. Open `src/main.rs` and note the tile constants:
    - `TILE_SIZE: 19.0` world units – tile width and depth.
    - `TILE_HEIGHT: 2.0` world units – tile thickness.
2. Verification: cross‑check the constants in the code.

## 2. Understand Game Camera Orientation
1. In `src/main.rs`, the camera is placed **1000 units above +Z** and looks down at the arena center while using **Y as the up axis**:
   ```rust
   let camera_pos = center + Vec3::new(0.0, 0.0, 1000.0);
   Transform::from_translation(camera_pos).looking_at(center, Vec3::Y);
   ```
2. Bevy and glTF both treat **+Z as forward** and **+Y as up**, so a tile modeled with its top facing +Z in Blender already faces the camera—**no rotation is required**.
3. Verification: enable *Face Orientation* in Blender. The tile's top (pink inset) should be **blue** and point along +Z (upward).

## 3. Prepare Blender Scene
1. Open Blender and delete the default cube.
2. Set units to **Metric** and scale to **1.0** so 1 Blender unit equals 1 game unit.
3. Verification: the scene grid should show 1 unit spacing.

## 4. Model and Orient the Base Tile
1. Add a cube and set its dimensions to **19 × 19 × 2** (X, Y, Z).
2. Enter Edit Mode, select the top face, press **I** to inset by **3 units** to create the inner square.
3. Return to Object Mode and **apply rotation** (`Ctrl+A → Rotation`) to clear any accidental transforms.
4. Go into Object mode, Press A, Press N, Rotate +90deg on X axis.
4. Verification: enable *Face Orientation* overlay—blue should face upward along +Z.

## 5. Assign Materials
### Outer Gray Surface
1. Create a new material using **Principled BSDF**.
2. Set Base Color to a neutral gray (e.g., RGB 0.6, 0.6, 0.6) and leave Metallic and Roughness at defaults for a matte look.
3. Verification: in **Material Preview**, the surface should appear gray.

### Hot Pink Unlit Inset
1. Create a second material.
2. Replace the Principled node with an **Emission** shader and set the color to hot pink (#ff00ff).
3. Connect Emission directly to **Material Output** to enable unlit export.
4. Assign this material to the inset face.
5. Verification: switch to **Rendered** mode and disable lights—the inset remains bright.

## 6. Export to glTF
1. Select the tile and choose **File → Export → glTF 2.0 (.glb)**.
2. Ensure **Apply Modifiers**, **Selected Objects**, and **Apply Transform** are enabled with `+Y Up` orientation.
3. Save the file as `assets/tile.glb` in this repository.
4. Verification: re-import the `.glb` into a new Blender scene—the pink inset should still point up.

## 7. Test in Bevy
1. Add the `.glb` to the `assets/` folder and run a minimal loader:
   ```rust
   commands.spawn(SceneRoot(asset_server.load("tile.glb#Scene0")));
   ```
2. The tile should align with the existing grid when placed at `(0.0, 0.0, 0.0)`.
3. Verification: run the game and confirm the pink inset faces the camera. If you see only a gray side, reopen the `.blend`, rotate the tile so the pink inset points to +Z, apply the rotation, and re-export.

## 8. PBR Lighting Knobs
Bevy’s `StandardMaterial` exposes the following key fields:
- `base_color` – albedo tint.
- `metallic` – 0.0 (dielectric) to 1.0 (metal).
- `perceptual_roughness` – micro-surface roughness.
- `normal_map_texture` – surface detail.
- `occlusion_texture` – ambient occlusion.
- `emissive` / `unlit` – self-illumination or bypass lighting.

Example from Bevy’s PBR demo showing metallic and roughness variation and an unlit material:
```rust
StandardMaterial {
    base_color: Srgba::hex("#ffd891").unwrap().into(),
    metallic: y01,
    perceptual_roughness: x01,
    ..default()
}
// ...
StandardMaterial {
    base_color: Srgba::hex("#ffd891").unwrap().into(),
    unlit: true,
    ..default()
}
```
Try adjusting these values on the gray surface in Blender and re-exporting to see their effect.

## 9. Verification Checklist
- [ ] Code constants match Blender dimensions.
- [ ] Outer material responds to lights; inset is always bright.
- [ ] Exported `tile.glb` re-imports correctly.
- [ ] Tile renders in Bevy at the correct size.

You now have a reusable tile asset with two materials and a basic understanding of PBR controls in Bevy.