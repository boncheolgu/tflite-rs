extern crate bindgen;
extern crate cpp_build;

use std::env;

fn build_inline_cpp() {
    println!(
        "cargo:rustc-link-search=native={}/external/tensorflow/tensorflow/contrib/lite/tools/make/gen/linux_x86_64/lib", env::var("CARGO_MANIFEST_DIR").unwrap()

    );
    println!("cargo:rustc-link-lib=static=tensorflow-lite");
    println!("cargo:rustc-link-lib=dylib=pthread");
    println!("cargo:rustc-link-lib=dylib=dl");

    cpp_build::Config::new()
        .include("external/tensorflow")
        .include(
            "external/tensorflow/tensorflow/contrib/lite/tools/make/downloads/flatbuffers/include",
        ).flag("-fPIC")
        .flag("-std=c++11")
        .flag("-Wno-sign-compare")
        .define("GEMMLOWP_ALLOW_SLOW_SCALAR_FALLBACK", None)
        .debug(true)
        .opt_level(2)
        .build("src/lib.rs");
}

fn main() {
    build_inline_cpp();
}
