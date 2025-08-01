use ffbetool::{
    cgg::{self, PartData},
    cgs,
    imageops::{BlendExt, ColorBoundsExt, OpacityExt},
};
use image::imageops;
use std::io::BufRead;

enum AnimFileType {
    Gif,
    Apng,
    None,
}

impl From<&str> for AnimFileType {
    fn from(value: &str) -> Self {
        match value {
            "apng" => AnimFileType::Apng,
            "gif" => AnimFileType::Gif,
            _ => AnimFileType::None,
        }
    }
}

fn main() -> std::result::Result<(), String> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        eprintln!("usage: ffbetool <unit_id> <cgg-file> <anim-name> <columns> <gif|apng>");
        return Ok(());
    }

    let unit_id: u32 = args[1].parse().expect("unit_id should be numerical value");
    let input_path = &args[2];
    let anim_name = if args.len() < 4 { None } else { Some(&args[3]) };
    let columns = if args.len() < 5 {
        0
    } else {
        args[4].parse().expect("columns should be numerical value")
    };
    let anim_file_type: AnimFileType  = if args.len() < 6 {
        AnimFileType::None
    } else {
        AnimFileType::from(args[5].as_str())
    };

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

    let mut unit = ffbetool::Unit {
        id: unit_id,
        frames,
        ..Default::default()
    };

    let src_img = ffbetool::imageops::load_source_image(unit_id, input_path);

    let mut content = match anim_name {
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
                                    println!(" -- Rotate Counter-Clockwise [{rotate}] -- ");
                                    // Note: The values provided by the cgg file for rotation
                                    //       is for counter-clockwise rotation.
                                    //       The `imageops` crate rotates clockwise. So,
                                    //       we need to convert this to clockwise rotation.
                                    //       90 -> 270
                                    //       180 -> 180
                                    //       270 | -90 -> 90
                                    part_img = match rotate {
                                        270 | -90 => imageops::rotate90(&part_img),
                                        180 => imageops::rotate180(&part_img),
                                        90 => imageops::rotate270(&part_img),
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
                                // TODO: deal with `include_empty` - this indicates we should have frames with nothing
                                Some(rect) => {
                                    println!(
                                        "frame[{frame_num}] rect: [{rect:?}] delay[{}]",
                                        frame.delay
                                    );

                                    match (&unit.top_left, &unit.bottom_right) {
                                        (Some(top_left), Some(bottom_right)) => {
                                            let ffbetool::imageops::Rect {
                                                x,
                                                y,
                                                width,
                                                height,
                                            } = rect;

                                            unit.top_left = Some(ffbetool::imageops::Point::new(
                                                (top_left.x()).min(x as i32),
                                                (top_left.y()).min(y as i32),
                                            ));

                                            unit.bottom_right =
                                                Some(ffbetool::imageops::Point::new(
                                                    (bottom_right.x()).max(x as i32 + width as i32),
                                                    (bottom_right.y())
                                                        .max(y as i32 + height as i32),
                                                ));
                                        }
                                        _ => {
                                            let ffbetool::imageops::Rect {
                                                x,
                                                y,
                                                width,
                                                height,
                                            } = rect;
                                            unit.top_left = Some(ffbetool::imageops::Point::new(
                                                x as i32, y as i32,
                                            ));
                                            unit.bottom_right =
                                                Some(ffbetool::imageops::Point::new(
                                                    x as i32 + width as i32,
                                                    y as i32 + height as i32,
                                                ));
                                        }
                                    }

                                    Some(frame.composite(target_img, rect))
                                }
                                None => None,
                            }
                        })
                        .collect();
                    (anim_name, frame_data)
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
    };

    let frame_rect = ffbetool::imageops::Rect {
        x: unit
            .top_left
            .expect("top_left should have a value here")
            .x() as u32,
        y: unit
            .top_left
            .expect("top_left should have a value here")
            .y() as u32,
        width: (unit
            .bottom_right
            .expect("bottom_right should have a value here")
            .x()
            - unit
                .top_left
                .expect("top_left should have a value here")
                .x()) as u32
            + 10,
        height: (unit
            .bottom_right
            .expect("bottom_right should have a value here")
            .y()
            - unit
                .top_left
                .expect("top_left should have a value here")
                .y()) as u32
            + 10,
    };

    content.1 = content.1.into_iter().map(|frame| {
        match frame {
            Some(mut frame) => {
                frame.image = imageops::crop(&mut frame.image, frame_rect.x, frame_rect.y, frame_rect.width, frame_rect.height).to_image();
                Some(frame)
            }
            None => {
                frame
            }
        }
    }).collect();

    match anim_file_type {
        AnimFileType::Apng => {
            let (anim_name, frames) = content.clone();
            let output_path = format!("output/{unit_id}-{anim_name}.apng");
            ffbetool::imageops::encode_animated_apng(frames, &output_path);
        },
        AnimFileType::Gif => {
            let (anim_name, frames) = content.clone();
            let output_path = format!("output/{unit_id}-{anim_name}.gif");
            ffbetool::imageops::encode_animated_gif(frames, &output_path);
        },
        AnimFileType::None => {}
    }
    // if gif {
    //     let (anim_name, frames) = content.clone();
    //     let output_path = format!("output/{unit_id}-{anim_name}.gif");
    //     ffbetool::imageops::encode_animated_gif(frames, &output_path);
    // }

    let (anim_name, frames) = content;
    let spritesheet = if columns == 0 || columns >= frames.len() {
        let mut sheet =
            image::RgbaImage::new(frame_rect.width * (frames.len() as u32), frame_rect.height);

        frames.into_iter().enumerate().for_each(|(idx, frame)| {
            let x = (idx as u32) * frame_rect.width;
            let y = 0;

            if let Some(frame) = frame {
                imageops::overlay(&mut sheet, &frame.image, x as i64, y as i64);
            }
        });

        sheet
    } else {
        let mut sheet = image::RgbaImage::new(
            frame_rect.width * (columns as u32),
            frame_rect.height * ((frames.len() as f32 / columns as f32).ceil() as u32),
        );

        frames.into_iter().enumerate().for_each(|(idx, frame)| {
            let x = ((idx % columns) as u32) * frame_rect.width;
            let y = ((idx / columns) as u32) * frame_rect.height;

            if let Some(frame) = frame {
                imageops::overlay(&mut sheet, &frame.image, x as i64, y as i64);
            }
        });

        sheet
    };

    let output_path = format!("output/{unit_id}-{anim_name}.png");
    spritesheet.save(output_path).unwrap();
    Ok(())
}
