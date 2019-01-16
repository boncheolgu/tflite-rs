#![allow(dead_code)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::all))]

pub(crate) use self::root::tflite::*;
pub(crate) use self::root::*;

include!(concat!(env!("OUT_DIR"), "/tflite_types.rs"));
