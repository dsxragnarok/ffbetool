pub mod cgg;
pub mod cgs;
pub mod character_db;
pub mod constants;
pub mod discovery;
pub mod error;
pub mod imageops;
pub mod metadata;
pub mod validation;

pub use error::{FfbeError, Result};

// TODO: disambiguate all of these coordinates (x_pos, y_pos, img_x, img_y, x and y)

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
