use std::fs::File;
use std::io::{self, BufReader};

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
}

pub type FrameParts = Vec<PartData>;

pub fn read_file(unit_id: u32, input_path: &str) -> io::Result<BufReader<File>> {
    let file_path = format!("{input_path}/unit_cgg_{unit_id}.csv");
    println!("[cgg] processing `cgg` file [{file_path}]");

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    Ok(reader)
}

pub fn process(text: &str, row: usize) -> Option<FrameParts> {
    let mut params = text
        .split(",")
        .take_while(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    if params.len() <= 2 {
        return None;
    }

    let anchor: i32 = params.remove(0).parse().ok()?;
    let count: usize = params.remove(0).parse().ok()?;
    let chunk_size = params.len() / count;

    if chunk_size == 0 {
        return None;
    }

    let msg = "should be a numerical value";

    let parts: Vec<PartData> = params
        .chunks(chunk_size)
        .enumerate()
        .filter_map(|(index, chunk)| {
            match chunk {

                [x_pos, y_pos, next_type,
                    blend_mode, opacity, rotate,
                    img_x, img_y, img_width,
                    img_height, page_id] => Some(
                    PartData {
                        anchor,
                        x_pos: x_pos.parse().expect(&format!("{x_pos} {msg}")),
                        y_pos: y_pos.parse().expect(&format!("{y_pos} {msg}")),
                        next_type: next_type.parse().expect(&format!("{next_type} {msg}")),
                        blend_mode: blend_mode.parse().expect(&format!("{blend_mode} {msg}")),
                        opacity: opacity.parse().expect(&format!("{opacity} {msg}")),
                        rotate: rotate.parse().expect(&format!("{rotate} {msg}")),
                        img_x: img_x.parse().expect(&format!("{img_x} {msg}")),
                        img_y: img_y.parse().expect(&format!("{img_y} {msg}")),
                        img_width: img_width.parse().expect(&format!("{img_width} {msg}")),
                        img_height: img_height.parse().expect(&format!("{img_height} {msg}")),
                        page_id: page_id.parse().expect(&format!("{page_id} {msg}")),
                        index,
                        flip_x: *next_type == "1" || *next_type == "3",
                        flip_y: *next_type == "2"|| *next_type == "3",
                        line_index: row,
                    }),
                _ => None,
            }
        })
        .rev()
        .collect();

    Some(parts)
}
