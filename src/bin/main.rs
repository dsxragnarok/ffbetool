use clap::Parser;
use ffbetool::{
    self, FfbeError,
    cgg::{self},
    cgs::{self, process_frames},
    validation,
};
use image::imageops;
use std::io::BufRead;

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

    // Validate inputs
    validation::validate_input_args(unit_id, input_path, anim_name)?;
    validation::validate_output_dir(&args.output_dir)?;

    println!("ffbetool on {unit_id} cgg-file:[{input_path}]");
    let frames = match cgg::read_file(unit_id, input_path) {
        Ok(reader) => {
            let mut frames = Vec::new();
            for (row, line_result) in reader.lines().enumerate() {
                match line_result {
                    Ok(line) => match cgg::process(&line, row) {
                        Ok(Some(frame_parts)) => frames.push(frame_parts),
                        Ok(None) => continue, // Skip empty lines
                        Err(err) => {
                            eprintln!("Failed to process cgg line {}: {}", row + 1, err);
                            return Err(err);
                        }
                    },
                    Err(err) => {
                        eprintln!("Failed to read line {}: {}", row + 1, err);
                        return Err(err.into());
                    }
                }
            }
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
            return Err(err);
        }
    };

    let mut content = match anim_name {
        Some(anim_name) => match cgs::read_file(unit_id, anim_name, input_path) {
            Ok(reader) => {
                let mut cgs_frames_meta = Vec::new();
                for line_result in reader.lines() {
                    match line_result {
                        Ok(line) => match cgs::process(&line) {
                            Some(Ok(meta)) => cgs_frames_meta.push(meta),
                            Some(Err(err)) => {
                                eprintln!("Failed to parse cgs line: {err}");
                                return Err(err);
                            }
                            None => continue, // Skip empty lines
                        },
                        Err(err) => {
                            eprintln!("Failed to read cgs line: {err}");
                            return Err(err.into());
                        }
                    }
                }

                let frames: Vec<cgs::Frame> = cgs_frames_meta
                    .into_iter()
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

                let frame_data: Vec<_> = process_frames(&frames, &src_img, &mut unit);
                (anim_name, frame_data)
            }
            Err(err) => {
                return Err(FfbeError::ParseError(format!(
                    "failed to process cgs file: {err}"
                )));
            }
        },
        None => {
            return Err(FfbeError::NotImplemented(
                "`anim_name` not specified -- full directory processing not yet supported"
                    .to_string(),
            ));
        }
    };

    let frame_rect = ffbetool::imageops::Rect {
        x: unit
            .top_left
            .ok_or(FfbeError::MissingValue("top_left".to_string()))?
            .x() as u32,
        y: unit
            .top_left
            .ok_or(FfbeError::MissingValue("top_left".to_string()))?
            .y() as u32,
        width: unit
            .bottom_right
            .ok_or(FfbeError::MissingValue("bottom_right".to_string()))?
            .x() as u32
            - unit
                .top_left
                .ok_or(FfbeError::MissingValue("top_left".to_string()))?
                .x() as u32
            + 10,
        height: unit
            .bottom_right
            .ok_or(FfbeError::MissingValue("bottom_right".to_string()))?
            .y() as u32
            - unit
                .top_left
                .ok_or(FfbeError::MissingValue("top_left".to_string()))?
                .y() as u32
            + 10,
    };

    content.1.iter_mut().for_each(|frame| {
        frame.image = imageops::crop(
            &mut frame.image,
            frame_rect.x,
            frame_rect.y,
            frame_rect.width,
            frame_rect.height,
        )
        .to_image();
    });

    match anim_file_type {
        AnimFileType::Apng => {
            let (anim_name, frames) = content.clone();
            let output_path = format!("{}/{unit_id}-{anim_name}-anim.png", args.output_dir);
            let _ = ffbetool::imageops::encode_animated_apng(frames, &output_path);
        }
        AnimFileType::Gif => {
            let (anim_name, frames) = content.clone();
            let output_path = format!("{}/{unit_id}-{anim_name}-anim.gif", args.output_dir);
            let _ = ffbetool::imageops::encode_animated_gif(frames, &output_path);
        }
        AnimFileType::None => {}
    }

    let (anim_name, frames) = content;
    let spritesheet = if columns == 0 || columns >= frames.len() {
        let mut sheet =
            image::RgbaImage::new(frame_rect.width * (frames.len() as u32), frame_rect.height);

        frames.into_iter().enumerate().for_each(|(idx, frame)| {
            let x = (idx as u32) * frame_rect.width;
            let y = 0;

            imageops::overlay(&mut sheet, &frame.image, x as i64, y as i64);
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

            imageops::overlay(&mut sheet, &frame.image, x as i64, y as i64);
        });

        sheet
    };

    let output_path = format!("{}/{unit_id}-{anim_name}.png", args.output_dir);
    spritesheet.save(output_path)?;
    Ok(())
}
