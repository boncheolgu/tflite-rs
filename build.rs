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
        )
        .flag("-fPIC")
        .flag("-std=c++11")
        .flag("-Wno-sign-compare")
        .define("GEMMLOWP_ALLOW_SLOW_SCALAR_FALLBACK", None)
        .debug(true)
        .opt_level(2)
        .build("src/lib.rs");
}

// This generates "tflite_types.rs" containing structs and enums which are inter-operable with Glow.
fn import_tflite_types() {
    use bindgen::*;

    let bindings = bindgen::Builder::default()
        .whitelist_recursively(false)
        .prepend_enum_name(false)
        .with_codegen_config(CodegenConfig::TYPES)
        .layout_tests(false)
        .enable_cxx_namespaces()
        .derive_default(true)
        .whitelist_type("tflite::FlatBufferModel")
        .opaque_type("tflite::FlatBufferModel")
        .whitelist_type("tflite::InterpreterBuilder")
        .opaque_type("tflite::InterpreterBuilder")
        .whitelist_type("tflite::Interpreter")
        .opaque_type("tflite::Interpreter")
        .whitelist_type("tflite::ops::builtin::BuiltinOpResolver")
        .opaque_type("tflite::ops::builtin::BuiltinOpResolver")
        .whitelist_type("tflite::OpResolver")
        .opaque_type("tflite::OpResolver")
        .blacklist_type("tflite::Interpreter_TfLiteDelegatePtr")
        .blacklist_type("tflite::Interpreter_State")
        .default_enum_style(bindgen::EnumVariation::Rust)
        .header("csrc/tflite_wrapper.hpp")
        .clang_arg("-Iexternal/tensorflow")
        .clang_arg("-Iexternal/tensorflow/tensorflow/contrib/lite/tools/make/downloads/flatbuffers/include")
        .clang_arg("-DGEMMLOWP_ALLOW_SLOW_SCALAR_FALLBACK")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++11");

    let bindings = bindings.generate().expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/tflite_types.rs file.
    let out_path = std::path::PathBuf::from(env::var("OUT_DIR").unwrap()).join("tflite_types.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}

fn main() {
    import_tflite_types();
    build_inline_cpp();
}
