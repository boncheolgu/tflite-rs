#[cfg(feature = "generate_model_apis")]
#[macro_use]
extern crate bart_derive;

use std::env;
use std::env::VarError;
use std::path::{Path, PathBuf};
#[cfg(feature = "build")]
use std::time::Instant;

fn manifest_dir() -> PathBuf {
    PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
}

fn submodules() -> PathBuf {
    manifest_dir().join("submodules")
}

#[cfg(feature = "build")]
fn prepare_tensorflow_source() -> PathBuf {
    println!("Moving tflite source");
    let start = Instant::now();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let tf_src_dir = out_dir.join("tensorflow/tensorflow");
    let submodules = submodules();

    let mut copy_dir = fs_extra::dir::CopyOptions::new();
    copy_dir.overwrite = true;
    copy_dir.buffer_size = 65536;

    if !tf_src_dir.exists() {
        fs_extra::dir::copy(submodules.join("tensorflow"), &out_dir, &copy_dir)
            .expect("Unable to copy tensorflow");
    }

    let download_dir = tf_src_dir.join("lite/tools/make/downloads");
    if !download_dir.exists() {
        fs_extra::dir::copy(
            submodules.join("downloads"),
            download_dir.parent().unwrap(),
            &copy_dir,
        )
        .expect("Unable to copy download dir");
    }

    println!("Moving source took {:?}", start.elapsed());

    tf_src_dir
}

fn binary_changing_features() -> String {
    let mut features = String::new();
    if cfg!(feature = "debug_tflite") {
        features.push_str("-debug");
    }
    if cfg!(feature = "no_micro") {
        features.push_str("-no_micro");
    }
    features
}

fn prepare_tensorflow_library() {
    let arch = env::var("CARGO_CFG_TARGET_ARCH").expect("Unable to get TARGET_ARCH");

    #[cfg(feature = "build")]
    {
        let tflite = prepare_tensorflow_source();
        let out_dir = env::var("OUT_DIR").unwrap();
        // append tf_lib_name with features that can change how it is built
        // so a cached version that doesn't match expectations isn't used
        let binary_changing_features = binary_changing_features();
        let tf_lib_name =
            Path::new(&out_dir).join(format!("libtensorflow-lite{}.a", binary_changing_features,));
        let os = env::var("CARGO_CFG_TARGET_OS").expect("Unable to get TARGET_OS");
        if !tf_lib_name.exists() {
            println!("Building tflite");
            let start = Instant::now();
            let mut make = std::process::Command::new("make");
            if let Ok(prefix) = env::var("TARGET_TOOLCHAIN_PREFIX") {
                make.arg(format!("TARGET_TOOLCHAIN_PREFIX={}", prefix));
            };
            // Use cargo's cross-compilation information while building tensorflow
            // Now that tensorflow has an aarch64_makefile.inc use theirs
            let target = if &arch == "aarch64" { &arch } else { &os };

            #[cfg(feature = "debug_tflite")]
            {
                println!("Feature debug_tflite enabled. Changing optimization to 0");
                let makefile = tflite.join("lite/tools/make/Makefile");
                let makefile_contents =
                    std::fs::read_to_string(&makefile).expect("Unable to read Makefile");
                let replaced = makefile_contents.replace("-O3", "-Og -g").replace("-DNDEBUG", "");
                std::fs::write(&makefile, &replaced).expect("Unable to write Makefile");
                if !replaced.contains("-Og") {
                    panic!("Unable to change optimization settings");
                }
            }

            let make_dir = tflite.parent().unwrap();

            make.arg("-j")
                // allow parallelism to be overridden
                .arg(
                    env::var("TFLITE_RS_MAKE_PARALLELISM").unwrap_or_else(|_| {
                        env::var("NUM_JOBS").unwrap_or_else(|_| "1".to_string())
                    }),
                )
                .arg("BUILD_WITH_NNAPI=false")
                .arg("-f")
                .arg("tensorflow/lite/tools/make/Makefile");

            for (make_var, default) in &[
                ("TARGET", Some(target.as_str())),
                ("TARGET_ARCH", Some(arch.as_str())),
                ("TARGET_TOOLCHAIN_PREFIX", None),
                ("EXTRA_CFLAGS", None),
            ] {
                let env_var = format!("TFLITE_RS_MAKE_{}", make_var);
                println!("cargo:rerun-if-env-changed={}", env_var);

                match env::var(&env_var) {
                    Ok(result) => {
                        make.arg(format!("{}={}", make_var, result));
                    }
                    Err(VarError::NotPresent) => {
                        // Try and set some reasonable default values
                        if let Some(result) = default {
                            make.arg(format!("{}={}", make_var, result));
                        }
                    }
                    Err(VarError::NotUnicode(_)) => {
                        panic!("Provided a non-unicode value for {}", env_var)
                    }
                }
            }

            if cfg!(feature = "no_micro") {
                println!("Building lib but no micro");
                make.arg("lib");
            } else {
                make.arg("micro");
            }
            make.current_dir(make_dir);
            eprintln!("make command = {:?} in dir  {:?}", make, make_dir);
            if !make.status().expect("failed to run make command").success() {
                panic!("Failed to build tensorflow");
            }

            // find library
            let library = std::fs::read_dir(tflite.join("lite/tools/make/gen"))
                .expect("Make gen file should exist")
                .filter_map(|de| Some(de.ok()?.path().join("lib/libtensorflow-lite.a")))
                .find(|p| p.exists())
                .expect("Unable to find libtensorflow-lite.a");
            std::fs::copy(&library, &tf_lib_name).unwrap_or_else(|_| {
                panic!("Unable to copy libtensorflow-lite.a to {}", tf_lib_name.display())
            });

            println!("Building tflite from source took {:?}", start.elapsed());
        }
        println!("cargo:rustc-link-search=native={}", out_dir);
        println!("cargo:rustc-link-lib=static=tensorflow-lite{}", binary_changing_features);
    }
    #[cfg(not(feature = "build"))]
    {
        let arch_var = format!("TFLITE_{}_LIB_DIR", arch.replace("-", "_").to_uppercase());
        let all_var = "TFLITE_LIB_DIR".to_string();
        let lib_dir = env::var(&arch_var).or(env::var(&all_var)).unwrap_or_else(|_| {
            panic!(
                "[feature = build] not set and environment variables {} and {} are not set",
                arch_var, all_var
            )
        });
        println!("cargo:rustc-link-search=native={}", lib_dir);
        let static_dynamic = if Path::new(&lib_dir).join("libtensorflow-lite.a").exists() {
            "static"
        } else {
            "dylib"
        };
        println!("cargo:rustc-link-lib={}=tensorflow-lite", static_dynamic);
        println!("cargo:rerun-if-changed={}", lib_dir);
    }
    println!("cargo:rustc-link-lib=dylib=pthread");
    println!("cargo:rustc-link-lib=dylib=dl");
}

// This generates "tflite_types.rs" containing structs and enums which are inter-operable with Glow.
fn import_tflite_types() {
    use bindgen::*;

    let submodules = submodules();
    let submodules_str = submodules.to_string_lossy();
    let bindings = Builder::default()
        .whitelist_recursively(true)
        .prepend_enum_name(false)
        .impl_debug(true)
        .with_codegen_config(CodegenConfig::TYPES)
        .layout_tests(false)
        .enable_cxx_namespaces()
        .derive_default(true)
        .size_t_is_usize(true)
        // for model APIs
        .whitelist_type("tflite::ModelT")
        .whitelist_type(".+OptionsT")
        .blacklist_type(".+_TableType")
        // for interpreter
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
        .whitelist_type("TfLiteTensor")
        .opaque_type("std::string")
        .opaque_type("std::basic_string.*")
        .opaque_type("std::map.*")
        .opaque_type("flatbuffers::NativeTable")
        .blacklist_type("std")
        .blacklist_type("tflite::Interpreter_TfLiteDelegatePtr")
        .blacklist_type("tflite::Interpreter_State")
        .default_enum_style(EnumVariation::Rust { non_exhaustive: false })
        .derive_partialeq(true)
        .derive_eq(true)
        .header("csrc/tflite_wrapper.hpp")
        .clang_arg(format!("-I{}/tensorflow", submodules_str))
        .clang_arg(format!("-I{}/downloads/flatbuffers/include", submodules_str))
        .clang_arg("-DFLATBUFFERS_POLYMORPHIC_NATIVETABLE")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++11")
        // required to get cross compilation for aarch64 to work because of an issue in flatbuffers
        .clang_arg("-fms-extensions");

    let bindings = bindings.generate().expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/tflite_types.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("tflite_types.rs");
    bindings.write_to_file(out_path).expect("Couldn't write bindings!");
}

fn build_inline_cpp() {
    let submodules = submodules();

    cpp_build::Config::new()
        .include(submodules.join("tensorflow"))
        .include(submodules.join("downloads/flatbuffers/include"))
        .flag("-fPIC")
        .flag("-std=c++14")
        .flag("-Wno-sign-compare")
        .define("FLATBUFFERS_POLYMORPHIC_NATIVETABLE", None)
        .debug(true)
        .opt_level(if cfg!(debug_assertions) { 0 } else { 2 })
        .build("src/lib.rs");
}

fn import_stl_types() {
    use bindgen::*;

    let bindings = Builder::default()
        .enable_cxx_namespaces()
        .whitelist_type("std::string")
        .opaque_type("std::string")
        .whitelist_type("rust::.+")
        .opaque_type("rust::.+")
        .blacklist_type("std")
        .header("csrc/stl_wrapper.hpp")
        .layout_tests(false)
        .derive_partialeq(true)
        .derive_eq(true)
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++14")
        .clang_arg("-fms-extensions")
        .rustfmt_bindings(false)
        .generate()
        .expect("Unable to generate STL bindings");

    // Write the bindings to the $OUT_DIR/tflite_types.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("stl_types.rs");
    bindings.write_to_file(out_path).expect("Couldn't write bindings!");
}

#[cfg(feature = "generate_model_apis")]
fn generate_memory_impl() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;
    let mut file = std::fs::File::create("src/model/stl/memory_impl.rs")?;
    writeln!(
        &mut file,
        r#"
use std::{{fmt, mem}};
use std::ops::{{Deref, DerefMut}};

use crate::model::stl::memory::UniquePtr;
"#
    )?;

    #[derive(BartDisplay)]
    #[template = "data/memory_basic_impl.rs.template"]
    struct MemoryBasicImpl<'a> {
        cpp_type: &'a str,
        rust_type: &'a str,
    }

    let memory_types = vec![
        ("OperatorCodeT", "crate::model::OperatorCodeT"),
        ("TensorT", "crate::model::TensorT"),
        ("OperatorT", "crate::model::OperatorT"),
        ("SubGraphT", "crate::model::SubGraphT"),
        ("BufferT", "crate::model::BufferT"),
        ("QuantizationParametersT", "crate::model::QuantizationParametersT"),
        ("ModelT", "crate::model::ModelT"),
        ("MetadataT", "crate::model::MetadataT"),
        ("TensorMapT", "crate::model::TensorMapT"),
        ("SignatureDefT", "crate::model::SignatureDefT"),
    ];

    for (cpp_type, rust_type) in memory_types {
        writeln!(&mut file, "{}\n", &MemoryBasicImpl { cpp_type, rust_type },)?;
    }
    Ok(())
}

#[cfg(feature = "generate_model_apis")]
fn generate_vector_impl() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;
    let mut file = std::fs::File::create("src/model/stl/vector_impl.rs")?;
    writeln!(
        &mut file,
        r#"
use std::{{fmt, mem, slice}};
use std::ops::{{Deref, DerefMut, Index, IndexMut}};

use libc::size_t;

use super::memory::UniquePtr;
use super::vector::{{VectorOfUniquePtr, VectorErase, VectorExtract, VectorInsert, VectorSlice}};
use crate::model::stl::bindings::root::rust::dummy_vector;

cpp! {{{{
    #include <vector>
}}}}
"#
    )?;

    #[derive(BartDisplay)]
    #[template = "data/vector_primitive_impl.rs.template"]
    #[allow(non_snake_case)]
    struct VectorPrimitiveImpl<'a> {
        cpp_type: &'a str,
        rust_type: &'a str,
        RustType: &'a str,
    }

    let vector_types = vec![
        ("uint8_t", "u8", "U8"),
        ("int32_t", "i32", "I32"),
        ("int64_t", "i64", "I64"),
        ("float", "f32", "F32"),
    ];

    #[allow(non_snake_case)]
    for (cpp_type, rust_type, RustType) in vector_types {
        writeln!(&mut file, "{}\n", &VectorPrimitiveImpl { cpp_type, rust_type, RustType },)?;
    }

    #[derive(BartDisplay)]
    #[template = "data/vector_basic_impl.rs.template"]
    struct VectorBasicImpl<'a> {
        cpp_type: &'a str,
        rust_type: &'a str,
    }

    let vector_types = vec![
        ("std::unique_ptr<OperatorCodeT>", "UniquePtr<crate::model::OperatorCodeT>"),
        ("std::unique_ptr<TensorT>", "UniquePtr<crate::model::TensorT>"),
        ("std::unique_ptr<OperatorT>", "UniquePtr<crate::model::OperatorT>"),
        ("std::unique_ptr<SubGraphT>", "UniquePtr<crate::model::SubGraphT>"),
        ("std::unique_ptr<BufferT>", "UniquePtr<crate::model::BufferT>"),
        ("std::unique_ptr<MetadataT>", "UniquePtr<crate::model::MetadataT>"),
        ("std::unique_ptr<SignatureDefT>", "UniquePtr<crate::model::SignatureDefT>"),
        ("std::unique_ptr<TensorMapT>", "UniquePtr<crate::model::TensorMapT>"),
    ];

    for (cpp_type, rust_type) in vector_types {
        writeln!(&mut file, "{}\n", &VectorBasicImpl { cpp_type, rust_type },)?;
    }
    Ok(())
}

#[cfg(feature = "generate_model_apis")]
fn generate_builtin_options_impl() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;
    let mut file = std::fs::File::create("src/model/builtin_options_impl.rs")?;
    writeln!(
        &mut file,
        r#"
use super::{{BuiltinOptions, BuiltinOptionsUnion, NativeTable}};
"#
    )?;

    #[derive(BartDisplay)]
    #[template = "data/builtin_options_impl.rs.template"]
    struct BuiltinOptionsImpl<'a> {
        name: &'a str,
    }

    let option_names = vec![
        "Conv2DOptions",
        "DepthwiseConv2DOptions",
        "ConcatEmbeddingsOptions",
        "LSHProjectionOptions",
        "Pool2DOptions",
        "SVDFOptions",
        "RNNOptions",
        "FullyConnectedOptions",
        "SoftmaxOptions",
        "ConcatenationOptions",
        "AddOptions",
        "L2NormOptions",
        "LocalResponseNormalizationOptions",
        "LSTMOptions",
        "ResizeBilinearOptions",
        "CallOptions",
        "ReshapeOptions",
        "SkipGramOptions",
        "SpaceToDepthOptions",
        "EmbeddingLookupSparseOptions",
        "MulOptions",
        "PadOptions",
        "GatherOptions",
        "BatchToSpaceNDOptions",
        "SpaceToBatchNDOptions",
        "TransposeOptions",
        "ReducerOptions",
        "SubOptions",
        "DivOptions",
        "SqueezeOptions",
        "SequenceRNNOptions",
        "StridedSliceOptions",
        "ExpOptions",
        "TopKV2Options",
        "SplitOptions",
        "LogSoftmaxOptions",
        "CastOptions",
        "DequantizeOptions",
        "MaximumMinimumOptions",
        "ArgMaxOptions",
        "LessOptions",
        "NegOptions",
        "PadV2Options",
        "GreaterOptions",
        "GreaterEqualOptions",
        "LessEqualOptions",
        "SelectOptions",
        "SliceOptions",
        "TransposeConvOptions",
        "SparseToDenseOptions",
        "TileOptions",
        "ExpandDimsOptions",
        "EqualOptions",
        "NotEqualOptions",
        "ShapeOptions",
        "PowOptions",
        "ArgMinOptions",
        "FakeQuantOptions",
        "PackOptions",
        "LogicalOrOptions",
        "OneHotOptions",
        "LogicalAndOptions",
        "LogicalNotOptions",
        "UnpackOptions",
        "FloorDivOptions",
        "SquareOptions",
        "ZerosLikeOptions",
        "FillOptions",
        "BidirectionalSequenceLSTMOptions",
        "BidirectionalSequenceRNNOptions",
        "UnidirectionalSequenceLSTMOptions",
        "FloorModOptions",
        "RangeOptions",
        "ResizeNearestNeighborOptions",
        "LeakyReluOptions",
        "SquaredDifferenceOptions",
        "MirrorPadOptions",
        "AbsOptions",
        "SplitVOptions",
        "UniqueOptions",
        "ReverseV2Options",
        "AddNOptions",
        "GatherNdOptions",
        "CosOptions",
        "WhereOptions",
        "RankOptions",
        "ReverseSequenceOptions",
        "MatrixDiagOptions",
        "QuantizeOptions",
        "MatrixSetDiagOptions",
        "HardSwishOptions",
        "IfOptions",
        "WhileOptions",
        "DepthToSpaceOptions",
    ];

    for name in option_names {
        writeln!(&mut file, "{}\n", &BuiltinOptionsImpl { name },)?;
    }
    Ok(())
}

fn main() {
    import_stl_types();
    #[cfg(feature = "generate_model_apis")]
    {
        generate_memory_impl().unwrap();
        generate_vector_impl().unwrap();
        generate_builtin_options_impl().unwrap();
    }
    import_tflite_types();
    build_inline_cpp();
    if env::var("DOCS_RS").is_err() {
        prepare_tensorflow_library();
    }
}
