use ffbetool::cgg;
use std::io::BufRead;

fn main() -> std::io::Result<()> {
    let input_path = "input-assets/holy-dragoon-kain";
    let unit_id = 204002917;

    println!("ffbetool on {unit_id} cgg-file:[{input_path}]");
    match cgg::read_file(unit_id, input_path) {
        Ok(reader) => {
            for (idx, line_result) in reader.lines().enumerate() {
                match line_result {
                    Ok(line) => {
                        let parts = cgg::process(&line, idx);
                        println!("parts: {parts:?}");
                    }
                    Err(err) => eprintln!("failed to read line {idx}: {err}"),
                }
            }
            Ok(())
        }
        Err(err) => {
            eprintln!("failed to process cgg file: {err}");
            Err(err)
        }
    }
}
