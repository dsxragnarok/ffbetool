use std::fmt;

use apng::errors::APNGError;
use reqwest;

pub type Result<T> = std::result::Result<T, FfbeError>;

#[derive(Debug)]
pub enum FfbeError {
    CharacterNotFound(String),
    NoDatabaseFile,
    IoError(std::io::Error),
    ImageError(image::ImageError),
    ApngError(APNGError),
    ParseError(String),
    ReqwestError(String),
    FileNotFound(String),
    InvalidInput(String),
    NotImplemented(String),
    MissingValue(String),
}

impl fmt::Display for FfbeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FfbeError::CharacterNotFound(name) => write!(f, "Character not found in DB: {}", name),
            FfbeError::NoDatabaseFile => write!(f, "No database file found"),
            FfbeError::IoError(err) => write!(f, "IO errorerr: {err}"),
            FfbeError::ImageError(err) => write!(f, "Image errorerr: {err}"),
            FfbeError::ApngError(err) => write!(f, "APNG errorerr: {err}"),
            FfbeError::ReqwestError(err) => write!(f, "Fetch errorerr: {err}"),
            FfbeError::ParseError(msg) => write!(f, "Parse error: {msg}"),
            FfbeError::FileNotFound(path) => write!(f, "File not found: {path}"),
            FfbeError::InvalidInput(msg) => write!(f, "Invalid input: {msg}"),
            FfbeError::NotImplemented(msg) => write!(f, "Not implemented: {msg}"),
            FfbeError::MissingValue(msg) => write!(f, "Missing value: {msg}"),
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

impl From<serde_json::Error> for FfbeError {
    fn from(err: serde_json::Error) -> Self {
        FfbeError::ParseError(err.to_string())
    }
}

impl From<reqwest::Error> for FfbeError {
    fn from(err: reqwest::Error) -> Self {
        FfbeError::ReqwestError(err.to_string())
    }
}
