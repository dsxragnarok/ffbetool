use ffbetool::{cgg, cgs};
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
                let cgs_frames_meta: Vec<cgs::CgsMeta> = reader
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
                    })
                    .collect();
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
