use bevy::prelude::*;

// Only needed if we compile to WebAssembly
#[cfg(target_arch = "wasm32")]
use web_sys::Storage;

// Only used on native desktop builds (not wasm)
#[cfg(not(target_arch = "wasm32"))]
use std::collections::HashMap;
#[cfg(not(target_arch = "wasm32"))]
use std::fs;
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;

/// A resource that can read/write strings to a persistent store.
/// - **Web** (wasm32): localStorage
/// - **Desktop** (non-wasm): JSON file in user's home directory
#[derive(Resource)]
pub struct LocalStorage;

impl LocalStorage {
    pub fn new() -> Self {
        Self
    }

    /// Store a key-value string.
    /// On **WASM**, this goes to localStorage.
    /// On **Desktop**, this goes to our JSON file.
    pub fn save_string(&self, key: &str, value: &str) {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(storage) = get_local_storage() {
                let _ = storage.set_item(key, value);
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut map = self.load_map();
            map.insert(key.to_string(), value.to_string());
            self.save_map(&map);
        }
    }

    /// Retrieve a string value by key.
    /// On **WASM**, it reads from localStorage.
    /// On **Desktop**, it reads from our JSON file.
    pub fn load_string(&self, key: &str) -> Option<String> {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(storage) = get_local_storage() {
                return storage.get_item(key).ok().flatten();
            }
            return None;
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let map = self.load_map();
            return map.get(key).cloned();
        }
    }
}

// ------------------------
// WASM-ONLY HELPER
// ------------------------
#[cfg(target_arch = "wasm32")]
fn get_local_storage() -> Option<Storage> {
    let window = web_sys::window()?;
    window.local_storage().ok().flatten()
}

// ------------------------
// DESKTOP-ONLY HELPERS
// ------------------------
#[cfg(not(target_arch = "wasm32"))]
impl LocalStorage {
    // Loads our entire key-value store from a JSON file on disk
    fn load_map(&self) -> HashMap<String, String> {
        let path = get_data_path();
        if let Ok(contents) = fs::read_to_string(&path) {
            // If the JSON is valid, parse it; otherwise start empty
            serde_json::from_str(&contents).unwrap_or_default()
        } else {
            HashMap::new()
        }
    }

    // Saves our entire map to a JSON file
    fn save_map(&self, map: &HashMap<String, String>) {
        let path = get_data_path();
        if let Ok(json) = serde_json::to_string_pretty(map) {
            let _ = fs::write(path, json);
        }
    }
}

/// Determine where we store our JSON data on desktop.
/// On Unix-like (macOS, Linux): `~/.my_app_storage.json`
/// On Windows: `%USERPROFILE%\my_app_storage.json`
/// Fallback: current directory if we can't find a home dir
#[cfg(not(target_arch = "wasm32"))]
fn get_data_path() -> PathBuf {
    if cfg!(windows) {
        if let Ok(userprofile) = std::env::var("USERPROFILE") {
            return PathBuf::from(userprofile).join("my_app_storage.json");
        }
    } else {
        if let Ok(home) = std::env::var("HOME") {
            return PathBuf::from(home).join(".my_app_storage.json");
        }
    }
    // Fallback if no env var found
    PathBuf::from("my_app_storage.json")
}

// ------------------------
// PLUGIN
// ------------------------
pub struct LocalStoragePlugin;

impl Plugin for LocalStoragePlugin {
    fn build(&self, app: &mut App) {
        // Insert the resource on all platforms
        app.insert_resource(LocalStorage::new());
    }
}