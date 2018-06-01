extern crate bindgen;

use std::env;

fn main() {
    println!("cargo:rustc-link-lib=OpenSLES");

    if env::var("GENERATE_BINDINGS").is_ok() {
        let bindings = bindgen::Builder::default()
            .header("./headers/wrapper.h")
            .clang_args(&["-I./headers/"])
            .trust_clang_mangling(false) // otherwise it generates link_name with underscore as prefix and won't find the methods at runtime
            .generate()
            .expect("Unable to generate bindings");

        // output
        bindings
            .write_to_file("./src/bindings.rs")
            .expect("Couldn't write bindings!");
    }
}

