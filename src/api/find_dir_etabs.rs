// find if etabs is installed
use std::path::PathBuf;

// compatible versions of ETABS with the current version of the API
const COMPATIBLE_ETABS_VERSIONS: [&str; 3] = ["18", "19", "20"];

pub fn find_etabs_dir() -> Option<Vec<PathBuf>> {
    let program_files = std::env::var_os("ProgramFiles").unwrap();

    let mut etabs_dirs = Vec::new();

    // verify if ETABS is installed and push to vector
    for version in &COMPATIBLE_ETABS_VERSIONS {
        let mut etabs_dir = PathBuf::from(&program_files);
        etabs_dir.push("Computers and Structures");
        etabs_dir.push(format!("ETABS {}", version));

        if etabs_dir.exists() {
            etabs_dirs.push(etabs_dir);
        }
    }

    if etabs_dirs.is_empty() {
        return None;
    }

    Some(etabs_dirs)
}
