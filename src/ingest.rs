use std::fs;
use std::path::{Path, PathBuf};
use glob::glob;

/// Ingests markdown files
///
/// # Arguments
///
/// * `path`:
///
/// returns: ()
///
/// # Examples
///
/// ```
///
/// ```
pub fn ingest(path: &Path) {
    // if !path.is_dir() {
    //     panic("path must be directory");
    // }
    println!("Argument: {}", path.display());
    let pattern = path.to_str().unwrap().to_owned() + "/**/*.md";
    println!("Glob: {}", pattern);
    get_paths(pattern.as_str());
}

fn get_paths(pattern: &str) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = Vec::new();
    for entry in glob(pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                println!("{:?}", path.display());
                out.push(path);
            },
            Err(e) => println!("{:?}", e),
        }
    }
    out
}