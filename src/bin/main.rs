use ffbetool::{
    self,
    cgg::{self, PartData},
    cgs,
    imageops::{BlendExt, ColorBoundsExt, OpacityExt},
    FfbeError,
};
use image::imageops;
use std::io::BufRead;
use clap::Parser;

#[derive(Parser)]
#[command(name = "ffbetool")]
#[command(about = "Tool to assemble Final Fantasy Brave Exvius sprite sheets")]
struct Args {
    /// The unit id
    uid: u32,

    /// The animation name
    #[arg(short = 'a', long = "anim")]
    anim: Option<String>,

    /// The number of columns
    #[arg(short = 'c', long = "columns", default_value = "0")]
    columns: usize,

    /// Include empty frames
    #[arg(short = 'e', long = "empty")]
    include_empty: bool,

    /// Verbose logs
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,

    /// Save json file
    #[arg(short = 'j', long = "json")]
    save_json: bool,

    /// Save animated gif
    #[arg(long = "gif")]
    save_gif: bool,

    /// Save animated png (APNG)
    #[arg(long = "apng")]
    save_apng: bool,

    /// The source input directory
    #[arg(short = 'i', long = "input", default_value = ".")]
    input_dir: String,

    /// The output directory
    #[arg(short = 'o', long = "output", default_value = ".")]
    output_dir: String,
}

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

fn main() -> ffbetool::Result<()> {
    let args = Args::parse();

    let unit_id = args.uid;
    let input_path = &args.input_dir;
    let anim_name = args.anim.as_deref();
    let columns = args.columns;

    let anim_file_type = match (args.save_gif, args.save_apng) {
        (true, _) => AnimFileType::Gif,
        (_, true) => AnimFileType::Apng,
        _ => AnimFileType::None,
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
            return Err(err.into());
        }
    };

    let mut unit = ffbetool::Unit {
        id: unit_id,
        frames,
        ..Default::default()
    };

    let src_img = match ffbetool::imageops::load_source_image(unit_id, input_path) {
        Ok(img) => img,
        Err(err) => {
            eprintln!("failed to load source image: {err}");
            return Err(err.into());
        }
    };

    let mut content = match anim_name {
        Some(anim_name) => {
            match cgs::read_file(unit_id, anim_name, input_path) {
                Ok(reader) => {
                    let cgs_frames_meta = reader.lines().filter_map(
                        |line_result| match line_result {
                            Ok(line) => {
                                let cgs_meta = cgs::process(&line);
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
                    return Err(FfbeError::ParseError(format!("failed to process cgs file: {err}")));
                }
            }
        }
        None => {
            return Err(FfbeError::NotImplemented("`anim_name` not specified -- full directory processing not yet supported".to_string()));
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
            let output_path = format!("{}/{unit_id}-{anim_name}-anim.png", args.output_dir);
            ffbetool::imageops::encode_animated_apng(frames, &output_path);
        },
        AnimFileType::Gif => {
            let (anim_name, frames) = content.clone();
            let output_path = format!("{}/{unit_id}-{anim_name}-anim.gif", args.output_dir);
            ffbetool::imageops::encode_animated_gif(frames, &output_path);
        },
        AnimFileType::None => {}
    }

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

    let output_path = format!("{}/{unit_id}-{anim_name}.png", args.output_dir);
    spritesheet.save(output_path)?;
    Ok(())
}
