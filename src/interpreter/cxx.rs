pub use self::interpreter::*;

#[cxx::bridge]
mod interpreter {
    // #[namespace = "tflite"]
    // extern "C++" {
    //     pub type FlatBufferModel;
    // }
    unsafe extern "C++" {
        include!("tflite/cxx/include/interpreter.h");
        pub type FlatBufferModel = tflite::FlatBufferModel;
        pub fn flatbuffer_from_buffer(buffer: &[u8]) -> UniquePtr<FlatBufferModel>;
    }
}
