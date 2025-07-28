use ffbetool::cgg;
use std::io::BufRead;

fn main() -> std::io::Result<()> {
    let input_path = "input-assets/holy-dragoon-kain";
    let unit_id = 204002917;

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
