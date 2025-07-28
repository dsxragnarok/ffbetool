use std::fs::File;
use std::io::{self, BufReader};

#[derive(Debug, Default)]
pub struct Data {
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

pub fn read_file(unit_id: u32, input_path: &str) -> io::Result<BufReader<File>> {
    let file_path = format!("{input_path}/unit_cgg_{unit_id}.csv");
    println!("[cgg] processing `cgg` file [{file_path}]");

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    Ok(reader)
}

pub fn process(text: &str, row: usize) -> Option<Vec<Data>> {
    let mut params = text
        .split(",")
        .take_while(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    println!("params:{params:?}");
    if params.len() <= 2 {
        return None;
    }

    let anchor: i32 = params.remove(0).parse().ok()?;
    let count: usize = params.remove(0).parse().ok()?;
    let restlen = params.len();
    let chunklen = params.len() / count;
    println!(
        "[cgg line proc] anchor[{anchor}] count[{count}] restlen[{restlen}] chunklen[{chunklen}]"
    );

    let parts: Vec<Data> = params
        .chunks(chunklen)
        .enumerate()
        .filter_map(|(index, chunk)| {
            match chunk {
            
                [x_pos, y_pos, next_type,
                    blend_mode, opacity, rotate,
                    img_x, img_y, img_width,
                    img_height, page_id] => Some(
                    Data {
                        anchor,
                        x_pos: x_pos.parse().expect("x_pos should have a value"),
                        y_pos: y_pos.parse().expect("y_pos should have a value"),
                        next_type: next_type.parse().expect("next_type should have a value"),
                        blend_mode: blend_mode.parse().expect("blend_mode should have a value"),
                        opacity: opacity.parse().expect("opacity should have a value"),
                        rotate: rotate.parse().expect("rotate should have a value"),
                        img_x: img_x.parse().expect("img_x should have a value"),
                        img_y: img_y.parse().expect("img_y should have a value"),
                        img_width: img_width.parse().expect("img_widht should have a value"),
                        img_height: img_height.parse().expect("img_height should have a value"),
                        page_id: page_id.parse().expect("page_id should have a value"),
                        index,
                        flip_x: *next_type == "1" || *next_type == "3",
                        flip_y: *next_type == "2"|| *next_type == "3",
                        line_index: row,
                    }),
                _ => None,
            }
        })
        .collect();

    Some(parts)
}
