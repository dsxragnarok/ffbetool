use ffbetool::{
    cgg::{self, PartData},
    cgs,
    imageops::{BlendExt, ColorBoundsExt, OpacityExt},
};
use image::imageops;
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

    let src_img = ffbetool::imageops::load_source_image(unit_id, input_path);

    match anim_name {
        Some(anim_name) => {
            match cgs::read_file(unit_id, anim_name, input_path) {
                Ok(reader) => {
                    let cgs_frames_meta = reader.lines().enumerate().filter_map(
                        |(row, line_result)| match line_result {
                            Ok(line) => {
                                let cgs_meta = cgs::process(&line, row);
                                cgs_meta
                            }
                            Err(err) => {
                                eprintln!("failed to read cgs line: {err}");
                                None
                            }
                        },
                    );

                    let frames: Vec<cgs::Frame> = cgs_frames_meta
                        .map(|meta| {
                            let cgs::CgsMeta(frame_idx, x, y, delay) = meta;
                            let parts = unit.frames[frame_idx].clone();
                            cgs::Frame {
                                frame_idx,
                                parts,
                                x,
                                y,
                                delay,
                            }
                        })
                        .collect();

                    let frame_data: Vec<_> = frames
                        .into_iter()
                        .enumerate()
                        .map(|(frame_num, frame)| {
                            let mut target_img = image::RgbaImage::new(2000, 2000);
                            frame.parts.iter().for_each(|part_data| {
                                let PartData {
                                    img_x,
                                    img_y,
                                    img_width,
                                    img_height,
                                    ..
                                } = part_data;
                                let mut part_img = src_img
                                    .clone()
                                    .crop(*img_x, *img_y, *img_width, *img_height)
                                    .to_rgba8();

                                let PartData {
                                    x_pos,
                                    y_pos,
                                    blend_mode,
                                    flip_x,
                                    flip_y,
                                    rotate,
                                    opacity,
                                    ..
                                } = part_data;

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
                                    println!(" -- Rotate [{rotate}] -- ");
                                    part_img = match rotate {
                                        90 => imageops::rotate90(&part_img),
                                        180 => imageops::rotate180(&part_img),
                                        270 | -90 => imageops::rotate270(&part_img),
                                        _ => part_img,
                                    };
                                }

                                if *opacity < 100 {
                                    part_img.opacity((*opacity as f32) / 100.0);
                                }

                                imageops::overlay(
                                    &mut target_img,
                                    &part_img,
                                    (2000 / 2) + frame.x as i64 + *x_pos as i64,
                                    (2000 / 2) + frame.y as i64 + *y_pos as i64,
                                );
                            });

                            match target_img.get_color_bounds_rect(image::Rgba([0, 0, 0, 0]), false)
                            {
                                Some(rect) => {
                                    // target_img
                                    //     .save(format!("anim-{anim_name}-{frame_num}.png"))
                                    //     .unwrap();

                                    println!(
                                        "frame[{frame_num}] rect: [{rect:?}] delay[{}]",
                                        frame.delay
                                    );
                                    // TODO: disambiguate all of these coordinates (x_pos, y_pos, img_x, img_y, x and y)
                                    Some(frame.composite(target_img, rect))
                                }
                                None => None,
                            }
                        })
                        .collect();
                }
                Err(err) => {
                    eprintln!("failed to process cgs file: {err}");
                    return Err(err.to_string());
                }
            }
        }
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
