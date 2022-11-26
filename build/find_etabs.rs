use std::path::PathBuf;

pub fn find_etabs_dir() -> Option<PathBuf> {
    // compatible versions of ETABS with the current version of the API
    let versions = ["18", "19", "20"];

    // get the program files directory
    let program_files = std::env::var_os("ProgramFiles").unwrap();

    // loop through the compatible versions
    for version in &versions {
        // create the path to the ETABS directory
        let mut etabs_dir = PathBuf::from(&program_files);
        etabs_dir.push("Computers and Structures");
        etabs_dir.push(format!("ETABS {}", version));
        etabs_dir.push("ETABSv1.tlb");

        if etabs_dir.exists() {
            println!(
                "cargo:warning= Found ETABS installation directory: {:?}",
                etabs_dir
            );
            return Some(etabs_dir);
        }
    }

    println!("cargo:warning= Could not find ETABS installation directory");
    None
}
