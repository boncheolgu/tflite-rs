use std::io::Error as IoError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] IoError),
    #[error("`{0}`")]
    InternalError(String),
}

impl Error {
    pub fn internal_error<T: Into<String>>(s: T) -> Self {
        Self::InternalError(s.into())
    }
}
