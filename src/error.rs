#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    AlsaError {
        message: String,
        sys_error: String,
    },
    #[cfg(not(target_arch = "wasm32"))]
    ReadError(audrey::read::ReadError),
    #[cfg(not(target_arch = "wasm32"))]
    FormatError(audrey::read::FormatError),
    ManyChannelsError,
    NoChannelsError,
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        Error::IOError(error)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<audrey::read::ReadError> for Error {
    fn from(error: audrey::read::ReadError) -> Error {
        Error::ReadError(error)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<audrey::read::FormatError> for Error {
    fn from(error: audrey::read::FormatError) -> Error {
        Error::FormatError(error)
    }
}
