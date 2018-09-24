extern crate bindgen;

use std::env;

fn main() {
    println!("cargo:rustc-link-lib=OpenSLES");

    if env::var("GENERATE_BINDINGS").is_ok() {
        let android_ndk = env::var("ANDROID_NDK").expect("Couldn't find Android NDK path. Export ANDROID_NDK environment variable.");

        let bindings = bindgen::Builder::default()
            .clang_arg("-target")
            .clang_arg("armv7-linux-androideabi") // see https://github.com/rust-lang-nursery/rust-bindgen/issues/1229
            .clang_arg(format!("--sysroot={}/sysroot", android_ndk))
            .header("./headers/wrapper.h")
            .clang_args(&["-I./headers/"])
            .trust_clang_mangling(false) // otherwise it generates link_name with underscore as prefix and won't find the methods at runtime
            .layout_tests(false) // not useful as we can't execute them on host
            .generate()
            .expect("Unable to generate bindings");

        // output
        bindings
            .write_to_file("./src/bindings.rs")
            .expect("Couldn't write bindings!");
    }
}

