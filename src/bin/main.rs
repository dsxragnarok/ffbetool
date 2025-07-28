use ffbetool::cgg;
use std::io::BufRead;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        eprintln!("usage: ffbetool <unit_id> <cgg-file>");
        return Ok(());
    }

    let unit_id: u32 = args[1].parse().expect("unit_id should be numerical value");
    let input_path = &args[2];

    println!("ffbetool on {unit_id} cgg-file:[{input_path}]");
    let frames = match cgg::read_file(unit_id, input_path) {
        Ok(reader) => {
            let frames: Vec<cgg::FrameParts> = reader.lines().enumerate().filter_map(|(row, line_result)| {
                match line_result {
                    Ok(line) => {
                        let parts = cgg::process(&line, row);
                        parts
                    }
                    Err(err) => {
                        eprintln!("failed to read line {row}: {err}");
                        None
                    },
                }
            }).collect();
            frames
        }
        Err(err) => {
            eprintln!("failed to process cgg file: {err}");
            return Err(err);
        }
    };

    println!("frames: {frames:?}");
    Ok(())
}
