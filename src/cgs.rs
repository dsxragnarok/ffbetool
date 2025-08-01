use std::fs::File;
use std::io::{self, BufReader};

use image::{ImageBuffer, Rgba};

use crate::cgg;
use crate::imageops::Rect;

#[derive(Clone)]
pub struct Frame {
    pub frame_idx: usize,
    pub parts: cgg::FrameParts,
    pub x: i32,
    pub y: i32,
    pub delay: u32,
}

pub struct CompositeFrame {
    pub frame_idx: usize,
    pub image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub rect: Rect,
    pub delay: u32,
}

impl Frame {
    pub fn composite(
        self,
        image: ImageBuffer<Rgba<u8>, Vec<u8>>,
        rect: Rect,
    ) -> CompositeFrame {
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

pub fn process(text: &str, row: usize) -> Option<CgsMeta> {
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
                .expect("frame_index should be numerical value"),
            x.parse().expect("x should be numerical value"),
            y.parse().expect("y should be numerical value"),
            delay.parse().expect("delay should be numerical value"),
        )),
        _ => None,
    }
}
