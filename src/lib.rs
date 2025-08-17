pub mod cgg;
pub mod cgs;
pub mod constants;
pub mod error;
pub mod imageops;
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_default() {
        let unit = Unit::default();
        assert_eq!(unit.id, 0);
        assert!(unit.frames.is_empty());
        assert!(unit.top_left.is_none());
        assert!(unit.bottom_right.is_none());
        assert!(unit.width.is_none());
        assert!(unit.height.is_none());
        assert!(unit.x_offset.is_none());
        assert!(unit.y_offset.is_none());
    }

    #[test]
    fn test_unit_creation() {
        let unit = Unit {
            id: 12345,
            frames: vec![],
            top_left: Some(imageops::Point::new(10, 20)),
            bottom_right: Some(imageops::Point::new(100, 200)),
            width: Some(90),
            height: Some(180),
            x_offset: Some(5),
            y_offset: Some(10),
        };

        assert_eq!(unit.id, 12345);
        assert_eq!(unit.top_left.unwrap().x(), 10);
        assert_eq!(unit.top_left.unwrap().y(), 20);
        assert_eq!(unit.bottom_right.unwrap().x(), 100);
        assert_eq!(unit.bottom_right.unwrap().y(), 200);
        assert_eq!(unit.width, Some(90));
        assert_eq!(unit.height, Some(180));
        assert_eq!(unit.x_offset, Some(5));
        assert_eq!(unit.y_offset, Some(10));
    }

    #[test]
    fn test_unit_clone() {
        let unit1 = Unit {
            id: 999,
            frames: vec![],
            top_left: Some(imageops::Point::new(5, 15)),
            ..Default::default()
        };

        let unit2 = unit1.clone();
        assert_eq!(unit1.id, unit2.id);
        assert_eq!(unit1.top_left.unwrap().x(), unit2.top_left.unwrap().x());
        assert_eq!(unit1.top_left.unwrap().y(), unit2.top_left.unwrap().y());
    }
}
