use bevy::prelude::*;
use std::fs::File;
use std::io::{Write, Read};
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Resource)]
pub struct CollisionArray(Vec<Vec<u8>>);

impl CollisionArray {
    pub fn get(&self, row: usize, col: usize) -> Option<&u8> {
        self.0.get(row)?.get(col)
    }
}
impl Default for CollisionArray {
    fn default() -> Self {
        let path = Path::new("./stones/collision.json");
        let collision_array = if path.exists() {
            // Load file
            let mut file = File::open(&path).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            let collision_array: CollisionArray =
                serde_json::from_str(&contents).unwrap();
            collision_array
        } else {
            // Create array and file
            let collision_array = create_collision_array();
            let mut file = File::create(&path).unwrap();
            file.write_all(serde_json::to_string(&collision_array).unwrap().as_bytes()).unwrap();
            collision_array
        };
        collision_array
    }
}
pub fn create_collision_array() -> CollisionArray {
    let map_width = 1000;
    let map_height = 1000;
    let default_tile = 0;
    let mut collision_array: Vec<Vec<u8>> = Vec::new();
    for _ in 0..map_height {
        let row = vec![default_tile; map_width];
        collision_array.push(row);
    }
    CollisionArray(collision_array)
}