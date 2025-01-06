extern crate cbindgen;

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
        .with_language(cbindgen::Language::C)
        .with_crate(crate_dir.clone())
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("bindings_c.h");

    cbindgen::Builder::new()
        .with_language(cbindgen::Language::Cxx)
        .with_crate(crate_dir.clone())
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("bindings_cpp.h");

    cbindgen::Builder::new()
        .with_language(cbindgen::Language::Cython)
        .with_crate(crate_dir.clone())
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("bindings_cython.h");
}
