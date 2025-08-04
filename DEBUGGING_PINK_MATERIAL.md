# Debugging Pink Material Issues in Arenic

This guide helps you debug and fix issues with the pink inset material not showing up correctly in your Bevy game.

## Quick Start

1. **Enable the debugging tools** by adding these lines to your `src/main.rs`:

```rust
mod material_debugger;
mod material_test_scene;
mod material_inspector;

// In your App::new() chain, add:
.add_plugins(material_debugger::MaterialDebuggerPlugin)
.add_plugins(material_test_scene::MaterialTestScenePlugin)
.add_plugins(material_inspector::MaterialInspectorPlugin)
```

2. **Run the game** with `cargo run` and check the console output.

3. **Use the inspector controls** (when the game is running):
   - Press `I` to toggle the material inspector
   - Press `W` to toggle wireframe mode (helps see mesh structure)
   - Press `P` to toggle pink material highlighting
   - Press `N`/`M` to cycle through all loaded materials

## What the Debugging Tools Do

### 1. Material Debugger Plugin
- Automatically inspects all loaded materials when the tile.glb scene loads
- Logs detailed information about each material found
- Attempts to fix pink materials that aren't properly emissive
- Provides warnings if no emissive materials are found

### 2. Material Test Scene Plugin
- Creates 5 test spheres on the left side of your screen showing:
  1. Standard Pink (affected by lighting)
  2. Emissive Pink (glows)
  3. Unlit Pink (constant brightness)
  4. Emissive + Unlit Pink (maximum brightness)
  5. Gray Reference (for comparison)
- These spheres help you see what properly configured pink materials should look like

### 3. Material Inspector Plugin
- Runtime inspection of materials with keyboard controls
- Wireframe mode to see mesh structure
- Material cycling to inspect all loaded materials
- Pink material highlighting with pulsing effect

## Common Issues and Solutions

### Issue 1: No Pink Material Found
**Console shows:** "NO EMISSIVE MATERIALS FOUND!"

**Solution:** The material wasn't exported correctly from Blender.
1. Open your tile.blend file in Blender
2. Select the inset faces in Edit mode
3. In the Shader Editor, ensure:
   - You have an Emission shader node
   - Color is set to hot pink (#FF00FF)
   - Emission is connected directly to Material Output
   - NOT connected through Principled BSDF
4. Re-export as tile.glb with materials included

### Issue 2: Pink Material Exists but Isn't Emissive
**Console shows:** Pink material found but it's not unlit/emissive

**Solution:** The debugger will attempt to fix this automatically. If it doesn't work:
1. Check if multiple materials are being loaded
2. The debugger will make pink materials emissive and unlit
3. Look for "Fixed X materials!" in the console

### Issue 3: Material is Pink but Still Dark
**Console shows:** Material has correct settings but appears dark in game

**Possible causes:**
1. The material is being overridden by Bevy's lighting
2. The emissive value is too low
3. The material alpha mode is interfering

**Solution:** The debugger sets:
- `unlit: true` (ignores all lighting)
- `emissive: hot_pink` (self-illuminating)
- `base_color: hot_pink` (fallback color)

## Manual Material Fix

If the automatic fix doesn't work, you can manually create the correct material:

```rust
// In your setup system, after spawning tiles:
let hot_pink = Srgba::hex("ff00ff").unwrap();
let pink_material = materials.add(StandardMaterial {
    base_color: hot_pink.into(),
    emissive: LinearRgba::from(hot_pink),
    unlit: true,
    ..default()
});

// Then apply this material to the appropriate mesh entities
```

## Verification Steps

1. **Check Test Spheres**: Look at the 5 test spheres on the left. The 3rd and 4th spheres show how your pink inset should look.

2. **Use Inspector**: Press `I` then `N` to cycle through materials. Look for:
   - A material with pink base_color or emissive
   - `unlit: true` or non-zero emissive values

3. **Console Output**: The debugger logs extensive information about:
   - Number of materials found
   - Properties of each material
   - Any fixes applied

## Blender Export Checklist

- [ ] Inset faces have emission material assigned
- [ ] Emission shader connected directly to Material Output
- [ ] Color set to #FF00FF (hot pink)
- [ ] Export settings include materials
- [ ] No modifiers that might affect materials
- [ ] File saved as tile.glb in assets folder

## Still Not Working?

If the pink material still doesn't show:

1. **Verify the mesh structure**: Enable wireframe (press `W` with inspector on) to see if the inset faces exist as separate geometry.

2. **Check face normals**: In Blender, ensure the inset faces point upward (+Z) with blue color in Face Orientation overlay.

3. **Test with a simple cube**: Create a basic cube with emission material in Blender and test if that works.

4. **Share the console output**: The debugging tools provide detailed logs that can help identify the issue.

Remember: The key is that the pink inset must use an Emission shader in Blender, not just a pink Principled BSDF, to appear constantly bright regardless of lighting.