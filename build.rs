use cmake::Config;
use std::env;
use std::path::Path;
use std::process::Command;

const LIBIPT_GIT: &str = "https://github.com/intel/libipt.git";

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let libipt_source = if cfg!(feature = "libipt_master") {
        println!("cargo:warning=`libipt_master` feature enabled, the master branch of libipt from Github will be used.");
        println!(
            "cargo:warning=The usage of this feature is discouraged and can cause build failures"
        );

        let path = Path::new(&out_dir).join("libipt");
        if !path.exists() {
            assert!(
                Command::new("git")
                    .current_dir(&out_dir)
                    .arg("clone")
                    .arg("--branch")
                    .arg("master")
                    .arg("--single-branch")
                    .arg("--depth")
                    .arg("1")
                    .arg(LIBIPT_GIT)
                    .status()
                    .unwrap()
                    .success(),
                "Failed to clone libipt repository"
            );
        } else {
            assert!(
                Command::new("git")
                    .current_dir(&path)
                    .arg("pull")
                    .status()
                    .unwrap()
                    .success(),
                "Failed to pull libipt repository"
            );
        }
        path
    } else {
        Path::new("libipt").to_path_buf()
    };

    check_submodule(&libipt_source);

    let dst = Config::new(&libipt_source)
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
        .derive_debug(true)
        .impl_debug(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate libipt bindings");

    bindings
        .write_to_file(Path::new(&out_dir).join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn check_submodule<P: AsRef<Path>>(path: P) {
    let path = path.as_ref();
    if !path.exists()
        || !path
            .read_dir()
            .is_ok_and(|mut content| content.next().is_some())
    {
        let error = format!("{} directory not found or empty", path.display());
        println!("cargo:warning={error}");
        println!(
            "cargo:warning=Hint: Please get the submodules with `git submodule update --init --recursive`"
        );
        panic!("{error}");
    }
}
