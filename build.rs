extern crate bindgen;
extern crate cpp_build;
extern crate curl;
extern crate failure;
extern crate flate2;
extern crate hex;
extern crate reqwest;
extern crate sha2;
extern crate tar;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use failure::Fallible;

const TFLITE_VERSION: &'static str = "1.11.0";

fn is_valid_tf_src<P: AsRef<Path>>(filepath: P) -> bool {
    use sha2::{Digest, Sha256};

    let mut sha256 = Sha256::new();
    sha256.input(&fs::read(filepath).unwrap());
    ::hex::encode(sha256.result())
        == "f49ce3f1d04cee854bc9f74fa9696991140b34a2e2447f35f01391b72c8bfa9f"
}

fn download<P: reqwest::IntoUrl, P2: AsRef<Path>>(source_url: P, target_file: P2) -> Fallible<()> {
    let mut resp = reqwest::get(source_url)?;
    let f = fs::File::create(&target_file)?;
    let mut writer = ::std::io::BufWriter::new(f);
    resp.copy_to(&mut writer)?;
    Ok(())
}

fn extract<P: AsRef<Path>, P2: AsRef<Path>>(archive_path: P, extract_to: P2) {
    use flate2::read::GzDecoder;
    use tar::Archive;

    let file = fs::File::open(archive_path).unwrap();
    let unzipped = GzDecoder::new(file);
    let mut a = Archive::new(unzipped);
    a.unpack(extract_to).unwrap();
}

fn prepare_tensorflow_source() -> PathBuf {
    let tf_src_dir = PathBuf::from(env::var("OUT_DIR").unwrap()).join("tensorflow");
    fs::create_dir(&tf_src_dir).unwrap_or_default();

    let tf_src_name = tf_src_dir.join(format!("v{}.tar.gz", TFLITE_VERSION));
    if !tf_src_name.exists() || !is_valid_tf_src(&tf_src_name) {
        let tf_src_url = format!(
            "https://codeload.github.com/tensorflow/tensorflow/tar.gz/v{}",
            TFLITE_VERSION
        );

        download(&tf_src_url, &tf_src_name).unwrap();

        assert!(
            is_valid_tf_src(&tf_src_name),
            "{} is not valid",
            tf_src_name.to_str().unwrap()
        );
    }

    let tf_src_dir_inner = tf_src_dir.join(format!("tensorflow-{}", TFLITE_VERSION));
    if !tf_src_dir_inner.exists() {
        extract(&tf_src_name, &tf_src_dir);

        Command::new("tensorflow/contrib/lite/tools/make/download_dependencies.sh")
            .current_dir(&tf_src_dir_inner)
            .status()
            .expect("failed to download tflite dependencies.");

        // To compile C files with -fPIC
        if std::env::consts::OS == "linux" {
            fs::copy(
                "data/linux_makefile.inc",
                tf_src_dir_inner
                    .join("tensorflow/contrib/lite/tools/make/targets/linux_makefile.inc"),
            ).unwrap();
        }
    }
    tf_src_dir_inner
}

fn prepare_tensorflow_library<P: AsRef<Path>>(tflite: P) {
    let tf_lib_name = PathBuf::from(env::var("OUT_DIR").unwrap()).join("libtensorflow-lite.a");
    if !tf_lib_name.exists() {
        Command::new("make")
            .arg("-j")
            .arg("-f")
            .arg("tensorflow/contrib/lite/tools/make/Makefile")
            .current_dir(&tflite)
            .status()
            .expect("failed to download tflite dependencies.");

        fs::copy(
            tflite.as_ref().join(format!(
                "tensorflow/contrib/lite/tools/make/gen/{OS}_{ARCH}/lib/libtensorflow-lite.a",
                OS = std::env::consts::OS,
                ARCH = std::env::consts::ARCH,
            )),
            &tf_lib_name,
        ).unwrap();
    }
}

// This generates "tflite_types.rs" containing structs and enums which are inter-operable with Glow.
fn import_tflite_types<P: AsRef<Path>>(tflite: P) {
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
        .clang_arg(format!("-I{}", tflite.as_ref().to_str().unwrap()))
        .clang_arg(format!(
            "-I{}/tensorflow/contrib/lite/tools/make/downloads/flatbuffers/include",
            tflite.as_ref().to_str().unwrap()
        )).clang_arg("-DGEMMLOWP_ALLOW_SLOW_SCALAR_FALLBACK")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++11");

    let bindings = bindings.generate().expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/tflite_types.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("tflite_types.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}

fn build_inline_cpp<P: AsRef<Path>>(tflite: P) {
    println!("cargo:rustc-link-lib=static=tensorflow-lite");
    println!("cargo:rustc-link-lib=dylib=pthread");
    println!("cargo:rustc-link-lib=dylib=dl");

    cpp_build::Config::new()
        .include(tflite.as_ref())
        .include(
            tflite
                .as_ref()
                .join("tensorflow/contrib/lite/tools/make/downloads/flatbuffers/include"),
        ).flag("-fPIC")
        .flag("-std=c++11")
        .flag("-Wno-sign-compare")
        .define("GEMMLOWP_ALLOW_SLOW_SCALAR_FALLBACK", None)
        .debug(true)
        .opt_level(2)
        .build("src/lib.rs");
}

fn main() {
    let tflite_src_dir = prepare_tensorflow_source();
    prepare_tensorflow_library(&tflite_src_dir);
    import_tflite_types(&tflite_src_dir);
    build_inline_cpp(&tflite_src_dir);
}
