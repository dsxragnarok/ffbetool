use std::fmt;

#[derive(Debug)]
pub enum FfbeError {
    IoError(std::io::Error),
    ImageError(image::ImageError),
    ParseError(String),
    FileNotFound(String),
    InvalidInput(String),
}

impl fmt::Display for FfbeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FfbeError::IoError(err) => write!(f, "IO error: {}", err),
            FfbeError::ImageError(err) => write!(f, "Image error: {}", err),
            FfbeError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            FfbeError::FileNotFound(path) => write!(f, "File not found: {}", path),
            FfbeError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl std::error::Error for FfbeError {}

impl From<std::io::Error> for FfbeError {
    fn from(err: std::io::Error) -> Self {
        FfbeError::IoError(err)
    }
}

impl From<image::ImageError> for FfbeError {
    fn from(err: image::ImageError) -> Self {
        FfbeError::ImageError(err)
    }
}

pub type Result<T> = std::result::Result<T, FfbeError>;