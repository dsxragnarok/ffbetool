use image;

pub fn load_source_image(unit_id: u32, input_path: &str) -> image::DynamicImage {
    let path = format!("{input_path}/unit_anime_{unit_id}.png");
    let img = image::open(path).unwrap();
    return img;
}
