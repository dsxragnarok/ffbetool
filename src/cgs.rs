use image::{
    DynamicImage, ImageBuffer, Rgba, RgbaImage,
    imageops::{self, overlay},
};
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufReader};

use crate::cgg;
use crate::constants::{CANVAS_SIZE, HALF_CANVAS};
use crate::imageops::{BlendExt, ColorBoundsExt, OpacityExt, Rect};

#[derive(Clone)]
pub struct Frame {
    pub frame_idx: usize,
    pub parts: cgg::FrameParts,
    pub x: i32,
    pub y: i32,
    pub delay: u32,
}

#[derive(Clone)]
pub struct CompositeFrame {
    pub frame_idx: usize,
    pub image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub rect: Rect,
    pub delay: u32,
}

impl Frame {
    pub fn composite(self, image: ImageBuffer<Rgba<u8>, Vec<u8>>, rect: Rect) -> CompositeFrame {
        let Frame {
            frame_idx, delay, ..
        } = self;
        CompositeFrame {
            frame_idx,
            image,
            rect,
            delay,
        }
    }
}

/// frame_index, x, y, delay
#[derive(Debug)]
pub struct CgsMeta(pub usize, pub i32, pub i32, pub u32);

pub fn read_file(unit_id: u32, anim_name: &str, input_path: &str) -> io::Result<BufReader<File>> {
    let file_path = format!("{input_path}/unit_{anim_name}_cgs_{unit_id}.csv");
    println!("[cgs] processing `cgs` file [{file_path}]");

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    Ok(reader)
}

pub fn process(text: &str) -> Option<Result<CgsMeta, crate::FfbeError>> {
    let params = text
        .split(",")
        .take_while(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    if params.len() < 4 {
        return None;
    }

    match params[..] {
        [frame_index, x, y, delay] => {
            let frame_idx = match frame_index.parse() {
                Ok(val) => val,
                Err(_) => {
                    return Some(Err(crate::FfbeError::ParseError(format!(
                        "Invalid frame_index value: '{}'",
                        frame_index
                    ))));
                }
            };

            let x_val = match x.parse() {
                Ok(val) => val,
                Err(_) => {
                    return Some(Err(crate::FfbeError::ParseError(format!(
                        "Invalid x value: '{}'",
                        x
                    ))));
                }
            };

            let y_val = match y.parse() {
                Ok(val) => val,
                Err(_) => {
                    return Some(Err(crate::FfbeError::ParseError(format!(
                        "Invalid y value: '{}'",
                        y
                    ))));
                }
            };

            let delay_val = match delay.parse() {
                Ok(val) => val,
                Err(_) => {
                    return Some(Err(crate::FfbeError::ParseError(format!(
                        "Invalid delay value: '{}'",
                        delay
                    ))));
                }
            };

            Some(Ok(CgsMeta(frame_idx, x_val, y_val, delay_val)))
        }
        _ => None,
    }
}

/// Process a collection of frames in parallel.
pub fn process_frames(
    frames: &[Frame],
    src_img: &DynamicImage,
    unit: &mut crate::Unit,
    include_empty: bool,
) -> Vec<CompositeFrame> {
    let results: Vec<(CompositeFrame, Option<Rect>)> = frames
        .par_iter()
        .enumerate()
        .map(|(_frame_num, frame)| {
            let mut target_img = RgbaImage::new(CANVAS_SIZE, CANVAS_SIZE);
            let frame_offset = (frame.x as i64, frame.y as i64);

            for part in &frame.parts {
                process_and_overlay_part(&mut target_img, src_img, frame_offset, part);
            }

            let bounds_rect = target_img.get_color_bounds_rect(Rgba([0, 0, 0, 0]), false);

            match bounds_rect {
                Some(rect) => (frame.clone().composite(target_img, rect), Some(rect)),
                None => {
                    // Create an empty frame - we'll resize it later to match other frames
                    let empty_rect = Rect {
                        x: 0,
                        y: 0,
                        width: 1,
                        height: 1,
                    };
                    let empty_img = RgbaImage::new(1, 1);
                    (frame.clone().composite(empty_img, empty_rect), None)
                }
            }
        })
        .collect();

    // Update unit bounds after parallel processing (only for non-empty frames)
    for (_, rect_opt) in &results {
        if let Some(rect) = rect_opt {
            merge_bounding_box(unit, rect);
        }
    }

    // Filter frames based on include_empty flag
    results
        .into_iter()
        .filter_map(|(cf, rect_opt)| {
            if rect_opt.is_some() || include_empty {
                Some(cf)
            } else {
                None
            }
        })
        .collect()
}

/// Processes a single part into a ready-to-overlay image.
fn process_part(src_img: &DynamicImage, part: &cgg::PartData) -> RgbaImage {
    let cgg::PartData {
        img_x,
        img_y,
        img_width,
        img_height,
        blend_mode,
        flip_x,
        flip_y,
        rotate,
        opacity,
        ..
    } = part;

    // Zero-copy crop, then convert to owned image
    let mut part_img =
        imageops::crop_imm(src_img, *img_x, *img_y, *img_width, *img_height).to_image();

    if *blend_mode == 1 {
        part_img.blend();
    }
    if *flip_x {
        part_img = imageops::flip_horizontal(&part_img);
    }
    if *flip_y {
        part_img = imageops::flip_vertical(&part_img);
    }
    if *rotate != 0 {
        // Note: the data gives rotation counter-clockwise. The `imageops` crate rotates clockwise.
        //       so we need to match them to the correct angle.
        part_img = match *rotate {
            90 | -270 => imageops::rotate270(&part_img),
            180 | -180 => imageops::rotate180(&part_img),
            270 | -90 => imageops::rotate90(&part_img),
            _ => part_img,
        };
    }
    if *opacity < 100 {
        part_img.opacity(*opacity as f32 / 100.0);
    }

    part_img
}

fn merge_bounding_box(unit: &mut crate::Unit, rect: &Rect) {
    match (unit.top_left, unit.bottom_right) {
        (Some(top_left), Some(bottom_right)) => {
            unit.top_left = Some(crate::imageops::Point::new(
                top_left.x().min(rect.x as i32),
                top_left.y().min(rect.y as i32),
            ));
            unit.bottom_right = Some(crate::imageops::Point::new(
                bottom_right.x().max(rect.x as i32 + rect.width as i32),
                bottom_right.y().max(rect.y as i32 + rect.height as i32),
            ));
        }
        _ => {
            unit.top_left = Some(crate::imageops::Point::new(rect.x as i32, rect.y as i32));
            unit.bottom_right = Some(crate::imageops::Point::new(
                rect.x as i32 + rect.width as i32,
                rect.y as i32 + rect.height as i32,
            ));
        }
    }
}

fn process_and_overlay_part(
    target_img: &mut RgbaImage,
    src_img: &DynamicImage,
    frame_offset: (i64, i64),
    part: &cgg::PartData,
) {
    let part_img = process_part(src_img, part);

    overlay(
        target_img,
        &part_img,
        HALF_CANVAS as i64 + frame_offset.0 + part.x_pos as i64,
        HALF_CANVAS as i64 + frame_offset.1 + part.y_pos as i64,
    );
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_empty_line() {
        let result = process("");
        assert!(result.is_none());
    }

    #[test]
    fn test_process_insufficient_params() {
        let result = process("1,2,3");
        assert!(result.is_none());
    }

    #[test]
    fn test_process_valid_line() {
        let line = "5,10,20,100";
        let result = process(line).unwrap().unwrap();

        assert_eq!(result.0, 5); // frame_index
        assert_eq!(result.1, 10); // x
        assert_eq!(result.2, 20); // y
        assert_eq!(result.3, 100); // delay
    }

    #[test]
    fn test_process_negative_values() {
        let line = "0,-10,-20,50";
        let result = process(line).unwrap().unwrap();

        assert_eq!(result.0, 0);
        assert_eq!(result.1, -10);
        assert_eq!(result.2, -20);
        assert_eq!(result.3, 50);
    }

    #[test]
    fn test_process_invalid_frame_index() {
        let line = "invalid,10,20,100";
        let result = process(line).unwrap();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid frame_index value")
        );
    }

    #[test]
    fn test_process_invalid_x() {
        let line = "5,invalid,20,100";
        let result = process(line).unwrap();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid x value"));
    }

    #[test]
    fn test_process_invalid_y() {
        let line = "5,10,invalid,100";
        let result = process(line).unwrap();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid y value"));
    }

    #[test]
    fn test_process_invalid_delay() {
        let line = "5,10,20,invalid";
        let result = process(line).unwrap();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid delay value")
        );
    }

    #[test]
    fn test_frame_composite() {
        let parts = vec![];
        let frame = Frame {
            frame_idx: 1,
            parts,
            x: 10,
            y: 20,
            delay: 100,
        };

        let image = RgbaImage::new(50, 50);
        let rect = Rect {
            x: 0,
            y: 0,
            width: 50,
            height: 50,
        };

        let composite = frame.composite(image, rect);
        assert_eq!(composite.frame_idx, 1);
        assert_eq!(composite.delay, 100);
        assert_eq!(composite.rect.width, 50);
        assert_eq!(composite.rect.height, 50);
    }

    #[test]
    fn test_read_file_nonexistent() {
        let result = read_file(99999, "nonexistent", "nonexistent_path");
        assert!(result.is_err());
    }

    #[test]
    fn test_read_file_existing() {
        let result = read_file(204000103, "atk", "test_data");
        assert!(result.is_ok());
    }

    #[test]
    fn test_merge_bounding_box_initial() {
        let mut unit = crate::Unit::default();
        let rect = Rect {
            x: 10,
            y: 20,
            width: 100,
            height: 200,
        };

        merge_bounding_box(&mut unit, &rect);

        assert_eq!(unit.top_left.unwrap().x(), 10);
        assert_eq!(unit.top_left.unwrap().y(), 20);
        assert_eq!(unit.bottom_right.unwrap().x(), 110);
        assert_eq!(unit.bottom_right.unwrap().y(), 220);
    }

    #[test]
    fn test_merge_bounding_box_expand() {
        let mut unit = crate::Unit {
            top_left: Some(crate::imageops::Point::new(20, 30)),
            bottom_right: Some(crate::imageops::Point::new(100, 150)),
            ..Default::default()
        };

        let rect = Rect {
            x: 5,
            y: 10,
            width: 200,
            height: 300,
        };
        merge_bounding_box(&mut unit, &rect);

        assert_eq!(unit.top_left.unwrap().x(), 5);
        assert_eq!(unit.top_left.unwrap().y(), 10);
        assert_eq!(unit.bottom_right.unwrap().x(), 205);
        assert_eq!(unit.bottom_right.unwrap().y(), 310);
    }

    #[test]
    fn test_process_frames_empty_handling() {
        use image::DynamicImage;

        // Create a simple test setup
        let frames = vec![Frame {
            frame_idx: 0,
            parts: vec![], // Empty parts - should create empty frame
            x: 0,
            y: 0,
            delay: 100,
        }];

        // Create a minimal test image
        let test_img = DynamicImage::new_rgba8(10, 10);
        let mut unit = crate::Unit::default();

        // Test with include_empty = false (should filter out empty frames)
        let result_no_empty = process_frames(&frames, &test_img, &mut unit, false);
        assert_eq!(result_no_empty.len(), 0);

        // Reset unit for second test
        let mut unit2 = crate::Unit::default();

        // Test with include_empty = true (should include empty frames)
        let result_with_empty = process_frames(&frames, &test_img, &mut unit2, true);
        assert_eq!(result_with_empty.len(), 1);
        // Empty frames start as 1x1 - they get resized later in main.rs
        assert_eq!(result_with_empty[0].image.width(), 1);
        assert_eq!(result_with_empty[0].image.height(), 1);
    }
}
