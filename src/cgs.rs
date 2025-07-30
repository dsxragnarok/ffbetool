use std::fs::File;
use std::io::{self, BufReader};

use crate::cgg;

#[derive(Debug, Default)]
pub struct PartData {
    pub anchor: i32,
    pub x_pos: i32,
    pub y_pos: i32,
    pub next_type: i32,
    pub blend_mode: i32,
    pub opacity: i32,
    pub rotate: i32,
    pub img_x: i32,
    pub img_y: i32,
    pub img_width: u32,
    pub img_height: u32,
    pub page_id: u32,
    pub index: usize,
    pub flip_x: bool,
    pub flip_y: bool,
    pub line_index: usize,
    pub frame_index: usize,
    pub x: i32,
    pub y: i32,
    pub delay: u32,
}

impl From<cgg::PartData> for PartData {
    fn from(value: cgg::PartData) -> Self {
        value.into()
    }
}

/// frame_index, x, y, delay
#[derive(Debug)]
pub struct CgsMeta(pub usize, pub u32, pub u32, pub i32);

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
