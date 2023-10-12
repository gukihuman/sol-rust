use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::{ collections::HashMap, fs::File, io::Read, path::Path };
pub struct InputMapPlugin;
impl Plugin for InputMapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InputSettings::default());
    }
}
pub type InputType = HashMap<String, String>;
#[derive(Deserialize, Serialize, Resource)] pub struct InputSettings {
    interface_keyboard_input: InputType,
    interface_mouse_input: InputType,
    interface_gamepad_input: InputType,
    in_game_keyboard_input: InputType,
    in_game_mouse_input: InputType,
    in_game_gamepad_input: InputType,
}
impl Default for InputSettings {
    fn default() -> Self {
        let path = Path::new("./scrolls/input_settings.json");
        if !path.exists() { load_default_input_settings(); }
        let mut contents = String::new();
        File::open(path).unwrap().read_to_string(&mut contents).unwrap();
        serde_json::from_str(&contents).unwrap()
    }
}
pub fn load_default_input_settings() {
    let input_path = Path::new("./scrolls/input_settings.json");
    let default_path = Path::new("./stones/default_input_settings.json");
    std::fs::copy(default_path, input_path)
        .expect("Failed to copy default settings");
}