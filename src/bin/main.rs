use ffbetool::cgg;

fn main() {
    let input_path = "input-assets/holy-dragoon-kain";
    let unit_id = 204002917;

    println!("ffbetool on {unit_id} cgg-file:[{input_path}]");
    match cgg::process(unit_id, input_path) {
        Ok(_) => println!("finished processing cgg file"),
        Err(err) => eprintln!("failed to process cgg file: {err}"),
    }
}
