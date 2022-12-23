use std::fs;
use glob::glob;

pub fn ingest(path: &str) {
    // Read all
    for entry in glob(path).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => println!("{:?}", path.display()),
            Err(e) => println!("{:?}", e),
        }
    }
}