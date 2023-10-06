use std::env;
use std::fs::{self, File};
use std::io::{self, BufWriter};
use std::path::Path;
use sha2::{Digest, Sha256};
use ico::{IconDir, IconDirEntry, IconImage, ResourceType};
use serde_json::{json, Value};

// 📜 adapt to many files, currently hashes only icon :)
pub fn startup() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let src_path = Path::new(&manifest_dir).join("whales/icon.png");
    let dst_path = Path::new("./assets/icon.ico");
    let cache_path = Path::new(&manifest_dir).join("whales/cache.json");

    // Compute the current hash of the source file
    let mut hasher = Sha256::new();
    let mut src = File::open(&src_path).expect("Failed to open icon.png");
    let _ = io::copy(&mut src, &mut hasher);
    let current_hash = format!("{:x}", hasher.finalize());

    // Read the previous hash from the cache file
    let cache: Value = serde_json::from_str(&fs::read_to_string(&cache_path).unwrap_or("{}".into())).unwrap();
    if let Some(previous_hash) = cache["hash"].as_str() {
        // If the previous hash equals the current hash, skip the conversion
        if previous_hash == current_hash {
            return;
        }
    }

    // Write the current hash to the cache file
    let new_cache = json!({ "hash": current_hash });
    fs::write(&cache_path, new_cache.to_string()).expect("Failed to write cache file");

    // Run the conversion logic
    let img = image::open(&src_path).expect("Failed to open icon.png");
    let img = img.into_rgba8();
    let icon_img = create_icon_image(img);
    let icon_dir_entry = IconDirEntry::encode(&icon_img).expect("Failed to encode to IconDirEntry");

    let mut icon_dir = IconDir::new(ResourceType::Icon);
    icon_dir.add_entry(icon_dir_entry);

    let file = File::create(&dst_path).expect("Failed to create output file");
    let mut writer = BufWriter::new(file);

    icon_dir.write(&mut writer).expect("Failed to write icon");
    let img = image::open(&src_path).expect("Failed to open icon.png");
    let img = img.into_rgba8();
    let icon_img = create_icon_image(img);
    let icon_dir_entry = IconDirEntry::encode(&icon_img).expect("Failed to encode to IconDirEntry");

    let mut icon_dir = IconDir::new(ResourceType::Icon);
    icon_dir.add_entry(icon_dir_entry);

    let file = File::create(&dst_path).expect("Failed to create output file");
    let mut writer = BufWriter::new(file);

    icon_dir.write(&mut writer).expect("Failed to write icon");
}

fn create_icon_image(image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>) -> IconImage {
    let width = image.width();
    let height = image.height();
    let data = image.into_raw();
    IconImage::from_rgba_data(width, height, data)
}
