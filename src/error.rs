use std::fmt;

use apng::errors::APNGError;

#[derive(Debug)]
pub enum FfbeError {
    IoError(std::io::Error),
    ImageError(image::ImageError),
    ApngError(APNGError),
    ParseError(String),
    FileNotFound(String),
    InvalidInput(String),
    NotImplemented(String),
    MissingValue(String),
}

impl fmt::Display for FfbeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FfbeError::IoError(err) => write!(f, "IO error: {}", err),
            FfbeError::ImageError(err) => write!(f, "Image error: {}", err),
            FfbeError::ApngError(err) => write!(f, "APNG error: {}", err),
            FfbeError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            FfbeError::FileNotFound(path) => write!(f, "File not found: {}", path),
            FfbeError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            FfbeError::NotImplemented(msg) => write!(f, "Not implemented: {}", msg),
            FfbeError::MissingValue(msg) => write!(f, "Missing value: {}", msg),
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

impl From<APNGError> for FfbeError {
    fn from(err: APNGError) -> Self {
        FfbeError::ParseError(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, FfbeError>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::io;

    #[test]
    fn test_ffbe_error_display() {
        let io_err = FfbeError::IoError(io::Error::new(io::ErrorKind::NotFound, "file not found"));
        assert!(io_err.to_string().contains("IO error"));
        assert!(io_err.to_string().contains("file not found"));

        let parse_err = FfbeError::ParseError("invalid format".to_string());
        assert_eq!(parse_err.to_string(), "Parse error: invalid format");

        let file_err = FfbeError::FileNotFound("/path/to/file".to_string());
        assert_eq!(file_err.to_string(), "File not found: /path/to/file");

        let input_err = FfbeError::InvalidInput("bad input".to_string());
        assert_eq!(input_err.to_string(), "Invalid input: bad input");

        let not_impl_err = FfbeError::NotImplemented("feature X".to_string());
        assert_eq!(not_impl_err.to_string(), "Not implemented: feature X");

        let missing_err = FfbeError::MissingValue("field Y".to_string());
        assert_eq!(missing_err.to_string(), "Missing value: field Y");
    }

    #[test]
    fn test_from_io_error() {
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "access denied");
        let ffbe_err: FfbeError = io_err.into();

        match ffbe_err {
            FfbeError::IoError(_) => {} // Expected
            _ => panic!("Expected IoError variant"),
        }
    }

    #[test]
    fn test_from_image_error() {
        let img_err = image::ImageError::Limits(image::error::LimitError::from_kind(
            image::error::LimitErrorKind::DimensionError,
        ));
        let ffbe_err: FfbeError = img_err.into();

        match ffbe_err {
            FfbeError::ImageError(_) => {} // Expected
            _ => panic!("Expected ImageError variant"),
        }
    }

    #[test]
    fn test_error_trait() {
        let err = FfbeError::ParseError("test error".to_string());

        // Test that it implements std::error::Error
        let _: &dyn std::error::Error = &err;

        // Test source (should be None for our custom errors)
        assert!(err.source().is_none());
    }

    #[test]
    fn test_debug_trait() {
        let err = FfbeError::ParseError("debug test".to_string());
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("ParseError"));
        assert!(debug_str.contains("debug test"));
    }
}
