use bevy::prelude::*;
use serde::{ Serialize, Deserialize };
use std::{ fs::File, io::Read, path::Path };
pub struct SettingsPlugin;
impl Plugin for SettingsPlugin { fn build(&self, app: &mut App) {
    app.insert_resource(Settings::default());
}}
#[derive(Serialize, Deserialize, Resource)] pub struct Settings {
    pub show_fps: bool
} impl Default for Settings {
    fn default() -> Self {
        let path = Path::new("./scrolls/settings.json");
        if !path.exists() { load_default_input_settings(); }
        #[cfg(debug_assertions)] load_default_input_settings();
        let mut contents = String::new();
        File::open(path).unwrap().read_to_string(&mut contents).unwrap();
        let settings: Settings =
            serde_json::from_str(&contents).unwrap();
        settings
    }
}
pub fn load_default_input_settings() {
    let input_path = Path::new("./scrolls/settings.json");
    let default_path = Path::new("./stones/settings_default.json");
    std::fs::copy(default_path, input_path)
        .expect("Failed to copy default settings");
}