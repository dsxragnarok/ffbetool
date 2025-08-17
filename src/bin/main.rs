use clap::Parser;
use ffbetool::{
    self, FfbeError,
    cgg::{self},
    cgs::{self, process_frames},
    constants::FRAME_PADDING,
    validation,
};
use image::imageops;
use std::io::BufRead;

#[derive(Parser, Clone)]
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

    // Validate inputs early
    validation::validate_input_args(args.uid, &args.input_dir, args.anim.as_deref())?;
    validation::validate_output_dir(&args.output_dir)?;

    let anim_file_type = determine_animation_file_type(&args);

    // Load and process frame data
    let frames = load_cgg_frames(args.uid, &args.input_dir)?;
    let mut unit = create_unit(args.uid, frames);
    let src_img = ffbetool::imageops::load_source_image(args.uid, &args.input_dir)?;

    // Process animation frames
    let (anim_name, mut composite_frames) = process_animation_frames(&args, &mut unit, &src_img)?;

    // Calculate frame bounds and resize empty frames, then crop frames
    let frame_rect = calculate_frame_rect(&unit)?;
    resize_empty_frames_to_bounds(&mut composite_frames, &frame_rect);
    crop_frames_to_bounds(&mut composite_frames, &frame_rect);

    // Generate outputs
    save_animated_files(&args, &anim_name, &composite_frames, anim_file_type)?;
    let spritesheet = create_spritesheet(&composite_frames, &frame_rect, args.columns);
    save_spritesheet(&args, &anim_name, spritesheet)?;

    Ok(())
}

fn determine_animation_file_type(args: &Args) -> AnimFileType {
    match (args.save_gif, args.save_apng) {
        (true, _) => AnimFileType::Gif,
        (_, true) => AnimFileType::Apng,
        _ => AnimFileType::None,
    }
}

fn load_cgg_frames(unit_id: u32, input_path: &str) -> ffbetool::Result<Vec<cgg::FrameParts>> {
    println!("ffbetool on {unit_id} cgg-file:[{input_path}]");

    let reader = cgg::read_file(unit_id, input_path).map_err(|err| {
        eprintln!("failed to process cgg file: {err}");
        err
    })?;

    let mut frames = Vec::new();
    for (row, line_result) in reader.lines().enumerate() {
        let line = line_result.map_err(|err| {
            eprintln!("Failed to read line {}: {}", row + 1, err);
            err
        })?;

        match cgg::process(&line, row)? {
            Some(frame_parts) => frames.push(frame_parts),
            None => continue, // Skip empty lines
        }
    }

    Ok(frames)
}

fn create_unit(unit_id: u32, frames: Vec<cgg::FrameParts>) -> ffbetool::Unit {
    ffbetool::Unit {
        id: unit_id,
        frames,
        ..Default::default()
    }
}

fn process_animation_frames(
    args: &Args,
    unit: &mut ffbetool::Unit,
    src_img: &image::DynamicImage,
) -> ffbetool::Result<(String, Vec<cgs::CompositeFrame>)> {
    let anim_name = args.anim.as_deref().ok_or_else(|| {
        FfbeError::NotImplemented(
            "`anim_name` not specified -- full directory processing not yet supported".to_string(),
        )
    })?;

    let cgs_frames_meta = load_cgs_metadata(args.uid, anim_name, &args.input_dir)?;
    let frames = create_cgs_frames(cgs_frames_meta, unit);
    let composite_frames = process_frames(&frames, src_img, unit, args.include_empty);

    Ok((anim_name.to_string(), composite_frames))
}

fn load_cgs_metadata(
    unit_id: u32,
    anim_name: &str,
    input_path: &str,
) -> ffbetool::Result<Vec<cgs::CgsMeta>> {
    let reader = cgs::read_file(unit_id, anim_name, input_path)
        .map_err(|err| FfbeError::ParseError(format!("failed to process cgs file: {err}")))?;

    let mut cgs_frames_meta = Vec::new();
    for line_result in reader.lines() {
        let line = line_result.map_err(|err| {
            eprintln!("Failed to read cgs line: {err}");
            err
        })?;

        match cgs::process(&line) {
            Some(Ok(meta)) => cgs_frames_meta.push(meta),
            Some(Err(err)) => {
                eprintln!("Failed to parse cgs line: {err}");
                return Err(err);
            }
            None => continue, // Skip empty lines
        }
    }

    Ok(cgs_frames_meta)
}

fn create_cgs_frames(cgs_frames_meta: Vec<cgs::CgsMeta>, unit: &ffbetool::Unit) -> Vec<cgs::Frame> {
    cgs_frames_meta
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
        .collect()
}

fn calculate_frame_rect(unit: &ffbetool::Unit) -> ffbetool::Result<ffbetool::imageops::Rect> {
    let top_left = unit
        .top_left
        .ok_or(FfbeError::MissingValue("top_left".to_string()))?;
    let bottom_right = unit
        .bottom_right
        .ok_or(FfbeError::MissingValue("bottom_right".to_string()))?;

    Ok(ffbetool::imageops::Rect {
        x: top_left.x() as u32,
        y: top_left.y() as u32,
        width: (bottom_right.x() - top_left.x()) as u32 + FRAME_PADDING,
        height: (bottom_right.y() - top_left.y()) as u32 + FRAME_PADDING,
    })
}

fn resize_empty_frames_to_bounds(
    frames: &mut [cgs::CompositeFrame],
    frame_rect: &ffbetool::imageops::Rect,
) {
    for frame in frames.iter_mut() {
        // If this is an empty frame (1x1), resize it to full frame dimensions
        if frame.image.width() == 1 && frame.image.height() == 1 {
            // Create a new transparent image with full frame dimensions
            let mut full_frame = image::RgbaImage::new(frame_rect.width, frame_rect.height);
            // Fill with transparent pixels (this is the default, but being explicit)
            for pixel in full_frame.pixels_mut() {
                *pixel = image::Rgba([0, 0, 0, 0]);
            }
            frame.image = full_frame;
            frame.rect = *frame_rect;
        }
    }
}

fn crop_frames_to_bounds(
    frames: &mut [cgs::CompositeFrame],
    frame_rect: &ffbetool::imageops::Rect,
) {
    frames.iter_mut().for_each(|frame| {
        // Only crop frames that are larger than the target size
        if frame.image.width() > frame_rect.width || frame.image.height() > frame_rect.height {
            frame.image = imageops::crop(
                &mut frame.image,
                frame_rect.x,
                frame_rect.y,
                frame_rect.width,
                frame_rect.height,
            )
            .to_image();
        }
    });
}

fn save_animated_files(
    args: &Args,
    anim_name: &str,
    frames: &[cgs::CompositeFrame],
    anim_file_type: AnimFileType,
) -> ffbetool::Result<()> {
    match anim_file_type {
        AnimFileType::Apng => {
            let output_path = format!("{}/{}-{}-anim.png", args.output_dir, args.uid, anim_name);
            ffbetool::imageops::encode_animated_apng(frames.to_vec(), &output_path)?;
        }
        AnimFileType::Gif => {
            let output_path = format!("{}/{}-{}-anim.gif", args.output_dir, args.uid, anim_name);
            ffbetool::imageops::encode_animated_gif(frames.to_vec(), &output_path)?;
        }
        AnimFileType::None => {}
    }
    Ok(())
}

fn create_spritesheet(
    frames: &[cgs::CompositeFrame],
    frame_rect: &ffbetool::imageops::Rect,
    columns: usize,
) -> image::RgbaImage {
    if columns == 0 || columns >= frames.len() {
        create_single_row_spritesheet(frames, frame_rect)
    } else {
        create_multi_row_spritesheet(frames, frame_rect, columns)
    }
}

fn create_single_row_spritesheet(
    frames: &[cgs::CompositeFrame],
    frame_rect: &ffbetool::imageops::Rect,
) -> image::RgbaImage {
    let mut sheet =
        image::RgbaImage::new(frame_rect.width * (frames.len() as u32), frame_rect.height);

    for (idx, frame) in frames.iter().enumerate() {
        let x = (idx as u32) * frame_rect.width;
        imageops::overlay(&mut sheet, &frame.image, x as i64, 0);
    }

    sheet
}

fn create_multi_row_spritesheet(
    frames: &[cgs::CompositeFrame],
    frame_rect: &ffbetool::imageops::Rect,
    columns: usize,
) -> image::RgbaImage {
    let rows = (frames.len() as f32 / columns as f32).ceil() as u32;
    let mut sheet = image::RgbaImage::new(
        frame_rect.width * (columns as u32),
        frame_rect.height * rows,
    );

    for (idx, frame) in frames.iter().enumerate() {
        let x = ((idx % columns) as u32) * frame_rect.width;
        let y = ((idx / columns) as u32) * frame_rect.height;
        imageops::overlay(&mut sheet, &frame.image, x as i64, y as i64);
    }

    sheet
}

fn save_spritesheet(
    args: &Args,
    anim_name: &str,
    spritesheet: image::RgbaImage,
) -> ffbetool::Result<()> {
    let output_path = format!("{}/{}-{}.png", args.output_dir, args.uid, anim_name);
    spritesheet.save(output_path)?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_determine_animation_file_type() {
        let args_gif = Args {
            uid: 123,
            anim: Some("test".to_string()),
            columns: 0,
            include_empty: false,
            verbose: false,
            save_json: false,
            save_gif: true,
            save_apng: false,
            input_dir: ".".to_string(),
            output_dir: ".".to_string(),
        };

        let args_apng = Args {
            save_gif: false,
            save_apng: true,
            ..args_gif.clone()
        };

        let args_none = Args {
            save_gif: false,
            save_apng: false,
            ..args_gif.clone()
        };

        assert!(matches!(
            determine_animation_file_type(&args_gif),
            AnimFileType::Gif
        ));
        assert!(matches!(
            determine_animation_file_type(&args_apng),
            AnimFileType::Apng
        ));
        assert!(matches!(
            determine_animation_file_type(&args_none),
            AnimFileType::None
        ));
    }

    #[test]
    fn test_create_unit() {
        let frames = vec![vec![]]; // Empty frame parts
        let unit = create_unit(12345, frames);

        assert_eq!(unit.id, 12345);
        assert_eq!(unit.frames.len(), 1);
        assert!(unit.top_left.is_none());
        assert!(unit.bottom_right.is_none());
    }

    #[test]
    fn test_calculate_frame_rect() {
        let unit = ffbetool::Unit {
            id: 123,
            frames: vec![],
            top_left: Some(ffbetool::imageops::Point::new(10, 20)),
            bottom_right: Some(ffbetool::imageops::Point::new(110, 220)),
            width: None,
            height: None,
            x_offset: None,
            y_offset: None,
        };

        let rect = calculate_frame_rect(&unit).unwrap();
        assert_eq!(rect.x, 10);
        assert_eq!(rect.y, 20);
        assert_eq!(rect.width, 100 + FRAME_PADDING);
        assert_eq!(rect.height, 200 + FRAME_PADDING);
    }

    #[test]
    fn test_calculate_frame_rect_missing_bounds() {
        let unit = ffbetool::Unit::default();
        let result = calculate_frame_rect(&unit);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Missing value: top_left")
        );
    }

    #[test]
    fn test_create_single_row_spritesheet() {
        let frames = vec![
            cgs::CompositeFrame {
                frame_idx: 0,
                image: image::RgbaImage::new(50, 50),
                rect: ffbetool::imageops::Rect {
                    x: 0,
                    y: 0,
                    width: 50,
                    height: 50,
                },
                delay: 100,
            },
            cgs::CompositeFrame {
                frame_idx: 1,
                image: image::RgbaImage::new(50, 50),
                rect: ffbetool::imageops::Rect {
                    x: 0,
                    y: 0,
                    width: 50,
                    height: 50,
                },
                delay: 100,
            },
        ];

        let frame_rect = ffbetool::imageops::Rect {
            x: 0,
            y: 0,
            width: 50,
            height: 50,
        };
        let sheet = create_single_row_spritesheet(&frames, &frame_rect);

        assert_eq!(sheet.width(), 100); // 2 frames * 50 width
        assert_eq!(sheet.height(), 50);
    }

    #[test]
    fn test_create_multi_row_spritesheet() {
        let frames = vec![
            cgs::CompositeFrame {
                frame_idx: 0,
                image: image::RgbaImage::new(50, 50),
                rect: ffbetool::imageops::Rect {
                    x: 0,
                    y: 0,
                    width: 50,
                    height: 50,
                },
                delay: 100,
            },
            cgs::CompositeFrame {
                frame_idx: 1,
                image: image::RgbaImage::new(50, 50),
                rect: ffbetool::imageops::Rect {
                    x: 0,
                    y: 0,
                    width: 50,
                    height: 50,
                },
                delay: 100,
            },
            cgs::CompositeFrame {
                frame_idx: 2,
                image: image::RgbaImage::new(50, 50),
                rect: ffbetool::imageops::Rect {
                    x: 0,
                    y: 0,
                    width: 50,
                    height: 50,
                },
                delay: 100,
            },
        ];

        let frame_rect = ffbetool::imageops::Rect {
            x: 0,
            y: 0,
            width: 50,
            height: 50,
        };
        let sheet = create_multi_row_spritesheet(&frames, &frame_rect, 2);

        assert_eq!(sheet.width(), 100); // 2 columns * 50 width
        assert_eq!(sheet.height(), 100); // 2 rows * 50 height (3 frames, 2 columns = 2 rows)
    }

    #[test]
    fn test_load_cgg_frames_nonexistent() {
        let result = load_cgg_frames(99999, "nonexistent_path");
        assert!(result.is_err());
    }

    #[test]
    fn test_load_cgg_frames_existing() {
        let result = load_cgg_frames(204000103, "test_data");
        assert!(result.is_ok());

        let frames = result.unwrap();
        assert!(!frames.is_empty());
    }

    #[test]
    fn test_save_spritesheet() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path().to_str().unwrap();

        let args = Args {
            uid: 123,
            anim: Some("test".to_string()),
            columns: 0,
            include_empty: false,
            verbose: false,
            save_json: false,
            save_gif: false,
            save_apng: false,
            input_dir: ".".to_string(),
            output_dir: temp_path.to_string(),
        };

        let spritesheet = image::RgbaImage::new(100, 100);
        let result = save_spritesheet(&args, "test", spritesheet);

        assert!(result.is_ok());

        let expected_path = format!("{}/123-test.png", temp_path);
        assert!(std::path::Path::new(&expected_path).exists());
    }
}
#[test]
fn test_resize_empty_frames_to_bounds() {
    let mut frames = vec![
        cgs::CompositeFrame {
            frame_idx: 0,
            image: image::RgbaImage::new(1, 1), // Empty frame (1x1)
            rect: ffbetool::imageops::Rect {
                x: 0,
                y: 0,
                width: 1,
                height: 1,
            },
            delay: 100,
        },
        cgs::CompositeFrame {
            frame_idx: 1,
            image: image::RgbaImage::new(50, 50), // Normal frame
            rect: ffbetool::imageops::Rect {
                x: 0,
                y: 0,
                width: 50,
                height: 50,
            },
            delay: 100,
        },
    ];

    let frame_rect = ffbetool::imageops::Rect {
        x: 0,
        y: 0,
        width: 50,
        height: 50,
    };
    resize_empty_frames_to_bounds(&mut frames, &frame_rect);

    // Empty frame should now be resized to full dimensions
    assert_eq!(frames[0].image.width(), 50);
    assert_eq!(frames[0].image.height(), 50);
    assert_eq!(frames[0].rect.width, 50);
    assert_eq!(frames[0].rect.height, 50);

    // Normal frame should remain unchanged
    assert_eq!(frames[1].image.width(), 50);
    assert_eq!(frames[1].image.height(), 50);
}
