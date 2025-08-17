use std::fs::File;
use std::io::{self, BufReader};

#[derive(Clone, Debug, Default)]
pub struct PartData {
    pub anchor: i32,
    pub x_pos: i32,
    pub y_pos: i32,
    pub next_type: i32,
    pub blend_mode: i32,
    pub opacity: i32,
    pub rotate: i32,
    pub img_x: u32,
    pub img_y: u32,
    pub img_width: u32,
    pub img_height: u32,
    pub page_id: u32,
    pub index: usize,
    pub flip_x: bool,
    pub flip_y: bool,
    pub line_index: usize,
}

pub type FrameParts = Vec<PartData>;

pub fn read_file(unit_id: u32, input_path: &str) -> io::Result<BufReader<File>> {
    let file_path = format!("{input_path}/unit_cgg_{unit_id}.csv");
    println!("[cgg] processing `cgg` file [{file_path}]");

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    Ok(reader)
}

use crate::error::Result;

pub fn process(text: &str, row: usize) -> Result<Option<FrameParts>> {
    let mut params = text
        .split(",")
        .take_while(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    if params.len() <= 2 {
        return Ok(None);
    }

    let anchor: i32 = params.remove(0).parse().map_err(|_| {
        crate::FfbeError::ParseError(format!(
            "Invalid anchor value at line {}: '{}'",
            row + 1,
            params[0]
        ))
    })?;
    let count: usize = params.remove(0).parse().map_err(|_| {
        crate::FfbeError::ParseError(format!(
            "Invalid count value at line {}: '{}'",
            row + 1,
            params[0]
        ))
    })?;
    let chunk_size = params.len() / count;

    if chunk_size == 0 {
        return Ok(None);
    }

    let mut parts = Vec::new();

    for (index, chunk) in params.chunks(chunk_size).enumerate() {
        match chunk {
            [
                x_pos,
                y_pos,
                next_type,
                blend_mode,
                opacity,
                rotate,
                img_x,
                img_y,
                img_width,
                img_height,
                page_id,
            ] => {
                let part_data = PartData {
                    anchor,
                    x_pos: parse_field(x_pos, "x_pos", row)?,
                    y_pos: parse_field(y_pos, "y_pos", row)?,
                    next_type: parse_field(next_type, "next_type", row)?,
                    blend_mode: parse_field(blend_mode, "blend_mode", row)?,
                    opacity: parse_field(opacity, "opacity", row)?,
                    rotate: parse_field(rotate, "rotate", row)?,
                    img_x: parse_field(img_x, "img_x", row)?,
                    img_y: parse_field(img_y, "img_y", row)?,
                    img_width: parse_field(img_width, "img_width", row)?,
                    img_height: parse_field(img_height, "img_height", row)?,
                    page_id: parse_field(page_id, "page_id", row)?,
                    index,
                    flip_x: *next_type == "1" || *next_type == "3",
                    flip_y: *next_type == "2" || *next_type == "3",
                    line_index: row,
                };
                parts.push(part_data);
            }
            _ => {
                return Err(crate::FfbeError::ParseError(format!(
                    "Invalid chunk format at line {}: expected 11 fields, got {}",
                    row + 1,
                    chunk.len()
                )));
            }
        }
    }

    parts.reverse();
    Ok(Some(parts))
}

fn parse_field<T: std::str::FromStr>(value: &str, field_name: &str, row: usize) -> Result<T> {
    value.parse().map_err(|_| {
        crate::FfbeError::ParseError(format!(
            "Invalid {} value at line {}: '{}'",
            field_name,
            row + 1,
            value
        ))
    })
}
