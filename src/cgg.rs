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

    if params.len() < 2 {
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
    let chunk_size = if count == 0 { 1 } else { params.len() / count };

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_empty_line() {
        let result = process("", 0).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_process_insufficient_params() {
        let result = process("0,1", 0).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_process_valid_single_part() {
        let line = "0,1,-40,-50,0,1,70,0,704,1232,80,64,0";
        let result = process(line, 0).unwrap().unwrap();

        assert_eq!(result.len(), 1);
        let part = &result[0];
        assert_eq!(part.anchor, 0);
        assert_eq!(part.x_pos, -40);
        assert_eq!(part.y_pos, -50);
        assert_eq!(part.next_type, 0);
        assert_eq!(part.blend_mode, 1);
        assert_eq!(part.opacity, 70);
        assert_eq!(part.rotate, 0);
        assert_eq!(part.img_x, 704);
        assert_eq!(part.img_y, 1232);
        assert_eq!(part.img_width, 80);
        assert_eq!(part.img_height, 64);
        assert_eq!(part.page_id, 0);
        assert_eq!(part.index, 0);
        assert!(!part.flip_x);
        assert!(!part.flip_y);
    }

    #[test]
    fn test_process_multiple_parts() {
        let line = "0,2,-40,-50,0,1,70,0,704,1232,80,64,0,0,-60,0,1,70,0,744,1296,40,64,0";
        let result = process(line, 0).unwrap().unwrap();

        assert_eq!(result.len(), 2);

        // First part (note: parts are reversed)
        let part1 = &result[0];
        assert_eq!(part1.x_pos, 0);
        assert_eq!(part1.y_pos, -60);
        assert_eq!(part1.img_x, 744);
        assert_eq!(part1.img_y, 1296);

        // Second part
        let part2 = &result[1];
        assert_eq!(part2.x_pos, -40);
        assert_eq!(part2.y_pos, -50);
        assert_eq!(part2.img_x, 704);
        assert_eq!(part2.img_y, 1232);
    }

    #[test]
    fn test_flip_flags() {
        // Test next_type = 1 (flip_x only)
        let line = "0,1,10,20,1,0,100,0,0,0,50,50,0";
        let result = process(line, 0).unwrap().unwrap();
        assert!(result[0].flip_x);
        assert!(!result[0].flip_y);

        // Test next_type = 2 (flip_y only)
        let line = "0,1,10,20,2,0,100,0,0,0,50,50,0";
        let result = process(line, 0).unwrap().unwrap();
        assert!(!result[0].flip_x);
        assert!(result[0].flip_y);

        // Test next_type = 3 (both flips)
        let line = "0,1,10,20,3,0,100,0,0,0,50,50,0";
        let result = process(line, 0).unwrap().unwrap();
        assert!(result[0].flip_x);
        assert!(result[0].flip_y);
    }

    #[test]
    fn test_process_invalid_anchor() {
        let line = "invalid,1,10,20,0,1,70,0,704,1232,80,64,0";
        let result = process(line, 0);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid anchor value")
        );
    }

    #[test]
    fn test_process_invalid_count() {
        let line = "0,invalid,10,20,0,1,70,0,704,1232,80,64,0";
        let result = process(line, 0);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid count value")
        );
    }

    #[test]
    fn test_process_invalid_field() {
        let line = "0,1,invalid,20,0,1,70,0,704,1232,80,64,0";
        let result = process(line, 0);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid x_pos value")
        );
    }

    #[test]
    fn test_process_insufficient_chunk_fields() {
        let line = "0,1,10,20,0,1,70,0,704,1232"; // Missing fields
        let result = process(line, 0);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid chunk format")
        );
    }

    #[test]
    fn test_read_file_nonexistent() {
        let result = read_file(99999, "nonexistent_path");
        assert!(result.is_err());
    }

    #[test]
    fn test_read_file_existing() {
        let result = read_file(204000103, "test_data");
        assert!(result.is_ok());
    }
}
