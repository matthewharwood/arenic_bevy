### How to Find Bevy Source Code (Research Tips)

Here's how I locate Bevy source code and how you can do it too:

1. **Bevy Docs Search**: Start at [docs.rs/bevy](https://docs.rs/bevy)
    - Use the search box for types like "TextureAtlasLayout"
    - Click on the type, then look for the `[src]` link next to each method
    - This links directly to GitHub source

2. **GitHub Code Search**:
    - Go to [github.com/bevyengine/bevy](https://github.com/bevyengine/bevy)
    - Press `T` to activate file finder
    - Type filename like "texture_atlas" or use search: `from_grid path:*.rs`
    - Or use GitHub's search bar with: `repo:bevyengine/bevy from_grid`

3. **Local IDE Method** (if you have Bevy as dependency):
   ```bash
   # In your project directory
   cargo doc --open  # Opens local docs with source links
   # Or in VS Code: Ctrl+Click on any Bevy type to go to definition
   ```

4. **Direct GitHub Pattern**: Bevy's crate structure is predictable:
    - `bevy::sprite::*` → `/crates/bevy_sprite/src/`
    - `bevy::asset::*` → `/crates/bevy_asset/src/`
    - `bevy::math::*` → `/crates/bevy_math/src/`

5. **Using grep/ripgrep locally**:
   ```bash
   # Clone Bevy locally for easy searching
   git clone https://github.com/bevyengine/bevy
   cd bevy
   rg "fn from_grid" --type rust
   ```

**Pro tip**: I often keep the Bevy repo cloned locally and use VS Code's workspace search (Ctrl+Shift+F) to find
implementations quickly.
