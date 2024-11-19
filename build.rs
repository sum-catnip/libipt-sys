use cmake::Config;
use std::env;
use std::path::Path;

const LIBIPT_SOURCE: &str = "libipt";

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    check_submodule(LIBIPT_SOURCE);

    let dst = Config::new("libipt")
        .define("BUILD_SHARED_LIBS", "OFF")
        .build();

    #[cfg(windows)]
    println!("cargo:rustc-link-lib=static=libipt");
    #[cfg(not(windows))]
    println!("cargo:rustc-link-lib=static=ipt");

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").to_string_lossy()
    );

    let bindings = bindgen::Builder::default()
        .header(dst.join("include").join("intel-pt.h").to_string_lossy())
        .allowlist_function("pt_.*")
        .allowlist_type("pt_.*")
        .allowlist_var("pt_.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate libipt bindings");

    bindings
        .write_to_file(Path::new(&out_dir).join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn check_submodule(dir: &str) {
    let path = Path::new(dir);
    if !path.exists() || path.iter().next().is_none() {
        let error = format!("{dir} directory not found or empty");
        println!("cargo:warning={error}");
        println!(
            "cargo:warning=Hint: Please get the submodules with `git submodule update --init --recursive`"
        );
        panic!("{error}");
    }
}
