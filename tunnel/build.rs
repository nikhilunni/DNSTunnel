use walkdir::WalkDir;

fn main() {
    // Tell Cargo to rerun the build script if any file in the c_code directory changes
    for entry in WalkDir::new("src/c") {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }

    // Compile the C code
    cc::Build::new()
        .files(WalkDir::new("src/c")
            .into_iter()
            .filter_map(|e| {
                let path = e.unwrap().into_path();
                if path.extension().and_then(|ext| ext.to_str()) == Some("c") {
                    println!("Caching path: {:?}", path);
                    Some(path)
                } else {
                    None
                }
            }))
        .compile("tunnel");
    //
    // // Generate bindings
    // let bindings = bindgen::Builder::default()
    //     .header("src/c/c_extern.h")
    //     .generate()
    //     .expect("Unable to generate bindings");
    //
    // // Write the bindings to the $OUT_DIR/bindings.rs file.
    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    // bindings
    //     .write_to_file(out_path.join("bindings.rs"))
    //     .expect("Couldn't write bindings!");
}
