mod find_etabs;

fn main () {

    let etabs_api  = {
        let etabs_api = std::env::var_os("OUT_DIR").unwrap();
        let mut etabs_api = std::path::PathBuf::from(etabs_api);
        etabs_api.push("etabs_api.rs");
        let etabs_api = std::fs::OpenOptions::new().create(true).write(true).truncate(true).open(etabs_api).unwrap();
        std::io::BufWriter::new(etabs_api)
    };

    let etabs_dir = find_etabs::find_etabs_dir().unwrap();

    // TODO: Make path to etabs_api.idl configurable via input from user
    let _ = winapi_tlb_bindgen::build(
        &etabs_dir,
        false,
        etabs_api,
    ).unwrap();

}
