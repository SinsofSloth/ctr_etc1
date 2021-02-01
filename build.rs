use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    // Link the static library.
    println!("cargo:rustc-link-lib=static=etc1_encoder");
    let root = env!("CARGO_MANIFEST_DIR");

    let files = vec!["etc1_encoder/src/rg_etc1.cpp", "etc1_encoder/src/etc1_encoder.cpp"];
    let result = cc::Build::new()
    .cpp(true)
    .files(files)
    .include("etc1_encoder")
    .try_compile("etc1_encoder");
    match result {
        Ok(_result) => (),
        Err(_error) => println!("cargo:rustc-link-search=native={}/etc1_encoder/lib/release/", root)
    }
}