use ffbetool::{cgg, cgs, image::BlendExt };
use std::io::BufRead;

fn main() -> std::result::Result<(), String> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        eprintln!("usage: ffbetool <unit_id> <cgg-file>");
        return Ok(());
    }

    let unit_id: u32 = args[1].parse().expect("unit_id should be numerical value");
    let input_path = &args[2];
    let anim_name = if args.len() < 4 { None } else { Some(&args[3]) };

    println!("ffbetool on {unit_id} cgg-file:[{input_path}]");
    let frames = match cgg::read_file(unit_id, input_path) {
        Ok(reader) => {
            let frames: Vec<cgg::FrameParts> = reader
                .lines()
                .enumerate()
                .filter_map(|(row, line_result)| match line_result {
                    Ok(line) => {
                        let parts = cgg::process(&line, row);
                        parts
                    }
                    Err(err) => {
                        eprintln!("failed to read line {row}: {err}");
                        None
                    }
                })
                .collect();
            frames
        }
        Err(err) => {
            eprintln!("failed to process cgg file: {err}");
            return Err(err.to_string());
        }
    };

    let unit = ffbetool::Unit {
        id: unit_id,
        frames,
    };

    let src_img = ffbetool::image::load_source_image(unit_id, input_path);

    match anim_name {
        Some(anim_name) => match cgs::read_file(unit_id, anim_name, input_path) {
            Ok(reader) => {
                let cgs_frames_meta = reader
                    .lines()
                    .enumerate()
                    .filter_map(|(row, line_result)| match line_result {
                        Ok(line) => {
                            let cgs_meta = cgs::process(&line, row);
                            println!("proc cgs row[{row}] {cgs_meta:?}");
                            cgs_meta
                        }
                        Err(err) => {
                            eprintln!("failed to read cgs line: {err}");
                            None
                        }
                    });

                let frames: Vec<Vec<cgs::PartData>> = cgs_frames_meta.map(|meta| {
                    let cgs::CgsMeta(frame_idx, x, y, delay) = meta;
                    let cgg_frame = unit.frames[frame_idx].clone();
                    cgg_frame.into_iter().map(|part_data| part_data.ingest_cgs_data(x, y, delay)).collect()
                }).collect();
                println!("-- frames --\n {frames:?}");

                let mut target_img = image::RgbaImage::new(2000, 2000);
                for frame in frames {
                    frame.iter().for_each(|part_data| {
                        let cgs::PartData { img_x, img_y, img_width, img_height, .. } = part_data;
                        let mut part_img = image::RgbaImage::new(*img_width, *img_height);

                        let cgs::PartData { x_pos, y_pos, blend_mode, flip_x, flip_y, rotate, opacity, .. } = part_data;

                        if *blend_mode == 1 {
                            part_img.blend();
                        }
                    });
                }
            }
            Err(err) => {
                eprintln!("failed to process cgs file: {err}");
                return Err(err.to_string());
            }
        },
        None => {
            eprint!("`anim_name` not specified -- full directory processing not yet supported");
            return Err(
                "`anim_name` not specified -- full directory processing not yet supported"
                    .to_string(),
            );
        }
    }

    Ok(())
}
