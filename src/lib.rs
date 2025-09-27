pub mod cgg;
pub mod cgs;
pub mod character_db;
pub mod constants;
pub mod discovery;
pub mod error;
pub mod imageops;
pub mod metadata;
pub mod validation;

use std::str::FromStr;

pub use error::{FfbeError, Result};

// Coordinate naming convention:
// - atlas_x/y: Source image coordinates for cropping
// - canvas_x/y: Canvas positioning coordinates
// - frame_offset_x/y: Animation frame positioning

pub type Frames = Vec<cgg::FrameParts>;

#[derive(Debug, Default, Clone)]
pub struct Unit {
    pub id: u32,
    pub frames: Frames,
    pub top_left: Option<imageops::Point>,
    pub bottom_right: Option<imageops::Point>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub x_offset: Option<i32>,
    pub y_offset: Option<i32>,
}

#[derive(Clone)]
pub enum UnitType {
    Character,
    Monster,
}

impl UnitType {
    pub fn file_str(&self) -> &str {
        match self {
            Self::Character => "unit",
            Self::Monster => "monster",
        }
    }
}

impl FromStr for UnitType {
    type Err = FfbeError;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "char" | "character" => Ok(Self::Character),
            "monster" => Ok(Self::Monster),
            _ => Err(FfbeError::ParseError(
                "Failed to parse UnitType".to_string(),
            )),
        }
    }
}
