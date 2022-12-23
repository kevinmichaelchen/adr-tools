use std::fs;
use std::path::Path;

pub fn ingest(path: &Path) {
    // Read all
    let paths = fs::read_dir(path).unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
}