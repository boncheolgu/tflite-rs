use std::io::Error as IoError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] IoError),
    #[error("`{0}`")]
    InternalError(std::borrow::Cow<'static, str>),
    #[error(transparent)]
    CxxError(#[from] cxx::Exception),
}

impl Error {
    pub fn internal_error<T: Into<std::borrow::Cow<'static, str>>>(s: T) -> Self {
        Self::InternalError(s.into())
    }
}
