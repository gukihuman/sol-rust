use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    // add icon to exe file, requires icon.ico 
    embed_resource::compile("windows.rc");

    // fill start library with "pub mod ..." by all files in code folder
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dest_path = Path::new(&manifest_dir).join("code/start_library.rs");
    let mut f = File::create(&dest_path).unwrap();
    let code_dir = Path::new(&manifest_dir).join("code");
    for entry in code_dir.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_stem() {
                    let name = name.to_str().unwrap();
                    if name != "start" && name != "start_library" {
                        writeln!(f, "pub mod {};", name).unwrap();
                    }
                }
            }
        }
    }
}