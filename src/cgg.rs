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

    let msg_prefix = "failed to parse";
    let msg_suffix = "should be a numerical value";

    let parts: Vec<PartData> = params
        .chunks(chunk_size)
        .enumerate()
        .filter_map(|(index, chunk)| match chunk {
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
            ] => Some(PartData {
                anchor,
                x_pos: x_pos
                    .parse()
                    .expect(&format!("{msg_prefix} x_pos: [{x_pos}] {msg_suffix}")),
                y_pos: y_pos
                    .parse()
                    .expect(&format!("{msg_prefix} y_pos: [{y_pos}] {msg_suffix}")),
                next_type: next_type.parse().expect(&format!(
                    "{msg_prefix} next_type: [{next_type}] {msg_suffix}"
                )),
                blend_mode: blend_mode.parse().expect(&format!(
                    "{msg_prefix} blend_mode: [{blend_mode}] {msg_suffix}"
                )),
                opacity: opacity
                    .parse()
                    .expect(&format!("{msg_prefix} opacity: [{opacity}] {msg_suffix}")),
                rotate: rotate
                    .parse()
                    .expect(&format!("{msg_prefix} rotate: [{rotate}] {msg_suffix}")),
                img_x: img_x
                    .parse()
                    .expect(&format!("{msg_prefix} img_x: [{img_x}] {msg_suffix}")),
                img_y: img_y
                    .parse()
                    .expect(&format!("{msg_prefix} img_y: [{img_y}] {msg_suffix}")),
                img_width: img_width.parse().expect(&format!(
                    "{msg_prefix} img_width: [{img_width}] {msg_suffix}"
                )),
                img_height: img_height.parse().expect(&format!(
                    "{msg_prefix} img_height: [{img_height}] {msg_suffix}"
                )),
                page_id: page_id
                    .parse()
                    .expect(&format!("{msg_prefix} page_id: [{page_id}] {msg_suffix}")),
                index,
                flip_x: *next_type == "1" || *next_type == "3",
                flip_y: *next_type == "2" || *next_type == "3",
                line_index: row,
            }),
            _ => None,
        })
        .rev()
        .collect();

    Some(parts)
}
