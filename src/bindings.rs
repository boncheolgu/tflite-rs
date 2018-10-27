#![allow(dead_code)]

pub(crate) use self::root::tflite::*;
pub(crate) use self::root::*;

include!(concat!(env!("OUT_DIR"), "/tflite_types.rs"));
