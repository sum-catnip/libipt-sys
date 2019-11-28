use cmake;
use bindgen;
use std::env;
use std::path::PathBuf;

fn main() {
    let dst = cmake::build("libipt");
    println!("cargo:rustc-link-lib=libipt");
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").to_str().unwrap()
    );

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::builder()
        // The input header we would like to generate
        // bindings for.
        .header(dst.join("include/intel-pt.h").to_str().unwrap())
        .whitelist_function("pt_.*")
        .whitelist_type("pt_.*")
        .whitelist_var("pt_.*")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    // println!("cargo:rustc-link-lib=static=libipt");
}