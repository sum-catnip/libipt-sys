use cmake;

fn main() {
    let dst = cmake::build("libipt");
    println!("cargo:rustc-link-lib=libipt");
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").to_str().unwrap()
    );

    std::fs::copy(dst.join("include/intel-pt.h"), "intel-pt.h")
        .expect("intel-pt.h not found. build probably failed");
}