use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let mut dst = cmake::build("rasta-protocol");
    dst.push("lib");

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=dylib=rasta");

    println!("cargo:warning={}", dst.display());
    let mut bindings = bindgen::Builder::default().clang_arg("-Irasta-protocol/src/rasta/headers");
    for header in
        std::fs::read_dir("rasta-protocol/src/rasta/headers").expect("Failed to read directory")
    {
        let header = header.unwrap();
        bindings = bindings.header(header.path().to_string_lossy());
    }

    for header in
        std::fs::read_dir("rasta-protocol/src/sci/headers").expect("Failed to read directory")
    {
        let header = header.unwrap();
        bindings = bindings.header(header.path().to_string_lossy());
    }

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .generate()
        .unwrap()
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
