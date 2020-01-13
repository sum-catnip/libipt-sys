use cmake::Config;

fn main() {
    let dst = Config::new("libipt")
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("BUILD_SHARED_LIBS", "OFF")
        .build();

    #[cfg(windows)]
    println!("cargo:rustc-link-lib=static=libipt");
    #[cfg(not(windows))]
    println!("cargo:rustc-link-lib=static=ipt");

    println!("cargo:rustc-link-search=native={}",
             dst.join("lib").to_str().unwrap());

    std::fs::copy(dst.join("include/intel-pt.h"), "intel-pt.h")
        .expect("intel-pt.h not found. build probably failed");
}