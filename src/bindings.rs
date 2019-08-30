#![allow(dead_code, clippy::all)]

pub use self::root::*;

include!(concat!(env!("OUT_DIR"), "/tflite_types.rs"));
