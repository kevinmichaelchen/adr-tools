use std::path::{Path, PathBuf};
use glob::glob;

/// Ingests ADR markdown files into SurrealDB
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
    // TODO figure out how to panic
    // if !path.is_dir() {
    //     panic("path must be directory");
    // }

    // Convert path into full glob
    let pattern = path.to_str().unwrap().to_owned() + "/**/*.md";

    // Get list of paths for all markdown files
    let paths = get_paths(pattern.as_str());

    // Convert each markdown file to an ADR, and insert into DB
    dbg!(paths);
}

fn get_paths(pattern: &str) -> Vec<PathBuf> {
    glob(pattern).unwrap().filter_map(Result::ok).collect()
}