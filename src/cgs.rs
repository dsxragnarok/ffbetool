use image::{
    DynamicImage, ImageBuffer, Rgba, RgbaImage,
    imageops::{self, overlay},
};
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufReader};

use crate::cgg;
use crate::imageops::{BlendExt, ColorBoundsExt, OpacityExt, Rect};

// Shared constant for centering offsets
const HALF_CANVAS: i64 = 1000;

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

pub fn process(text: &str) -> Option<CgsMeta> {
    let params = text
        .split(",")
        .take_while(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    if params.len() < 2 {
        return None;
    }

    match params[..] {
        [frame_index, x, y, delay] => Some(CgsMeta(
            frame_index
                .parse()
                .expect("failed to parse `frame_index`: should be numerical value"),
            x.parse()
                .expect("failed to parse `x` should be numerical value"),
            y.parse()
                .expect("failed to parse `y` should be numerical value"),
            delay
                .parse()
                .expect("failed to parse `delay` should be numerical value"),
        )),
        _ => None,
    }
}

/// Process a collection of frames in parallel.
pub fn process_frames(
    frames: &[Frame],
    src_img: &DynamicImage,
    unit: &mut crate::Unit,
) -> Vec<CompositeFrame> {
    let results: Vec<(CompositeFrame, Rect)> = frames
        .par_iter()
        .enumerate()
        .filter_map(|(_frame_num, frame)| {
            let mut target_img = RgbaImage::new(2000, 2000);

            for part in &frame.parts {
                let part_img = process_part(
                    src_img,
                    part.img_x,
                    part.img_y,
                    part.img_width,
                    part.img_height,
                    part.blend_mode as u8,
                    part.flip_x,
                    part.flip_y,
                    part.rotate,
                    part.opacity as u8,
                );

                overlay(
                    &mut target_img,
                    &part_img,
                    HALF_CANVAS + frame.x as i64 + part.x_pos as i64,
                    HALF_CANVAS + frame.y as i64 + part.y_pos as i64,
                );
            }

            target_img
                .get_color_bounds_rect(Rgba([0, 0, 0, 0]), false)
                .map(|rect| (frame.clone().composite(target_img, rect), rect))
        })
        .collect();

    // Update unit bounds after parallel processing
    for (_, rect) in &results {
        match (unit.top_left, unit.bottom_right) {
            (Some(top_left), Some(bottom_right)) => {
                unit.top_left = Some(crate::imageops::Point::new(
                    (top_left.x()).min(rect.x as i32),
                    (top_left.y()).min(rect.y as i32),
                ));

                unit.bottom_right = Some(crate::imageops::Point::new(
                    (bottom_right.x()).max(rect.x as i32 + rect.width as i32),
                    (bottom_right.y()).max(rect.y as i32 + rect.height as i32),
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
    results.into_iter().map(|(cf, _)| cf).collect()
}

/// Processes a single part into a ready-to-overlay image.
fn process_part(
    src_img: &DynamicImage,
    img_x: u32,
    img_y: u32,
    img_width: u32,
    img_height: u32,
    blend_mode: u8,
    flip_x: bool,
    flip_y: bool,
    rotate: i32,
    opacity: u8,
) -> RgbaImage {
    // Zero-copy crop, then convert to owned image
    let mut part_img = imageops::crop_imm(src_img, img_x, img_y, img_width, img_height).to_image();

    if blend_mode == 1 {
        part_img.blend();
    }
    if flip_x {
        part_img = imageops::flip_horizontal(&part_img);
    }
    if flip_y {
        part_img = imageops::flip_vertical(&part_img);
    }
    if rotate != 0 {
        part_img = match rotate.rem_euclid(360) {
            90 => imageops::rotate90(&part_img),
            180 => imageops::rotate180(&part_img),
            270 => imageops::rotate270(&part_img),
            _ => part_img,
        };
    }
    if opacity < 100 {
        part_img.opacity(opacity as f32 / 100.0);
    }

    part_img
}
