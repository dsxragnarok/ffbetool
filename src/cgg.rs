use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn process(unit_id: u32, input_path: &str) -> io::Result<()> {
    let file_path = format!("{input_path}/unit_cgg_{unit_id}.csv");
    println!("[cgg] processing `cgg` file [{file_path}]");

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}
