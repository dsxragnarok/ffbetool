use image::{self, ImageBuffer, Rgba};

pub fn load_source_image(unit_id: u32, input_path: &str) -> image::DynamicImage {
    let path = format!("{input_path}/unit_anime_{unit_id}.png");
    let img = image::open(path).unwrap();
    return img;
}

/// Extension trait for applying custom blend operations to RGBA images.
pub trait BlendExt {
    /// Applies a custom blend operation that premultiplies RGB channels by alpha
    /// and sets the alpha channel to the average of the original RGB values.
    ///
    /// This operation modifies each pixel as follows:
    /// - R' = R * A
    /// - G' = G * A
    /// - B' = B * A
    /// - A' = (R + G + B) / 3
    ///
    /// Pixels with zero alpha are left unchanged.
    fn blend(&mut self);
}

impl BlendExt for ImageBuffer<Rgba<u8>, Vec<u8>> {
    fn blend(&mut self) {
        for pixel in self.pixels_mut() {
            let [r, g, b, a] = pixel.0;
            if a != 0 {
                let (r_f, g_f, b_f, a_f) = (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a as f32 / 255.0);
                pixel.0 = [
                    ((r_f * a_f) * 255.0) as u8,
                    ((g_f * a_f) * 255.0) as u8,
                    ((b_f * a_f) * 255.0) as u8,
                    (((r_f + g_f + b_f) / 3.0) * 255.0) as u8,
                ];
            }
        }
    }
}
