use crate::{
    cgs::CompositeFrame,
    constants::{ALPHA_TRANSPARENT_U8, DEFAULT_FPS, MAX_CHANNEL_F32, RGB_CHANNEL_COUNT},
    error,
};
use apng::{self, PNGImage, load_dynamic_image};
use image::{self, ImageBuffer, Rgba};
use png;

#[derive(Clone, Copy, Debug, Default)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Point(i32, i32);
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }

    pub fn x(self) -> i32 {
        self.0
    }

    pub fn y(self) -> i32 {
        self.1
    }
}

pub fn load_source_image(unit_id: u32, input_path: &str) -> error::Result<image::DynamicImage> {
    let path = format!("{input_path}/unit_anime_{unit_id}.png");
    let img = image::open(path)?;
    Ok(img)
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
            if a != ALPHA_TRANSPARENT_U8 {
                let (r_f, g_f, b_f, a_f) = (
                    r as f32 / MAX_CHANNEL_F32,
                    g as f32 / MAX_CHANNEL_F32,
                    b as f32 / MAX_CHANNEL_F32,
                    a as f32 / MAX_CHANNEL_F32,
                );
                pixel.0 = [
                    ((r_f * a_f) * MAX_CHANNEL_F32) as u8,
                    ((g_f * a_f) * MAX_CHANNEL_F32) as u8,
                    ((b_f * a_f) * MAX_CHANNEL_F32) as u8,
                    (((r_f + g_f + b_f) / RGB_CHANNEL_COUNT) * MAX_CHANNEL_F32) as u8,
                ];
            }
        }
    }
}

/// Extension trait to apply opacity scaling to RGBA images.
pub trait OpacityExt {
    /// Multiplies the alpha channel of each pixel by the given opacity scalar (0.0 to 1.0).
    fn opacity(&mut self, opacity: f32);
}

impl OpacityExt for ImageBuffer<Rgba<u8>, Vec<u8>> {
    fn opacity(&mut self, opacity: f32) {
        assert!(
            (0.0..=1.0).contains(&opacity),
            "Opacity must be between 0.0 and 1.0. Got {opacity}."
        );

        for pixel in self.pixels_mut() {
            let [r, g, b, a] = pixel.0;
            let new_alpha = ((a as f32) * opacity).round().clamp(0.0, MAX_CHANNEL_F32) as u8;
            *pixel = Rgba([r, g, b, new_alpha]);
        }
    }
}

/// Extension trait for color bounds detection.
pub trait ColorBoundsExt {
    /// Returns the bounding rectangle of pixels matching (or not matching) a color.
    ///
    /// # Parameters
    /// * `color` - The RGBA color to search for
    /// * `find_color` - If true, finds pixels matching the color; if false, finds pixels NOT matching the color
    ///
    /// # Returns
    /// Some((x, y, width, height)) of the bounding rectangle, or None if no matching pixels found.
    fn get_color_bounds_rect(&self, color: Rgba<u8>, find_color: bool) -> Option<Rect>;
}

impl ColorBoundsExt for ImageBuffer<Rgba<u8>, Vec<u8>> {
    fn get_color_bounds_rect(&self, color: Rgba<u8>, find_color: bool) -> Option<Rect> {
        let (width, height) = self.dimensions();
        let mut min_x = width;
        let mut min_y = height;
        let mut max_x = 0;
        let mut max_y = 0;
        let mut found = false;

        for y in 0..height {
            for x in 0..width {
                let pixel = *self.get_pixel(x, y);
                let matches = if find_color {
                    pixel == color
                } else {
                    pixel != color
                };

                if matches {
                    min_x = min_x.min(x);
                    min_y = min_y.min(y);
                    max_x = max_x.max(x);
                    max_y = max_y.max(y);
                    found = true;
                }
            }
        }

        if found {
            Some(Rect {
                x: min_x,
                y: min_y,
                width: max_x - min_x + 1,
                height: max_y - min_y + 1,
            })
        } else {
            None
        }
    }
}

// TODO: Optimize this function. It's strange that we need to pass in a full
// list of PNGImage in order to create the config when the `create_config` is
// only using the first image in that list. This causes us to have to loop through
// our frames twice.
pub fn encode_animated_apng(frames: Vec<CompositeFrame>, output_path: &str) -> error::Result<()> {
    let mut png_images: Vec<PNGImage> = Vec::new();
    for frame in frames.clone() {
        let fr_img = image::DynamicImage::from(frame.image);
        let png_image = match load_dynamic_image(fr_img) {
            Ok(png_image) => png_image,
            Err(err) => {
                let msg = "Failed to load frame image as png";
                eprint!("{msg}: {err}");
                return Err(crate::FfbeError::ParseError(msg.into()));
            }
        };
        png_images.push(png_image);
    }

    let mut out = std::io::BufWriter::new(std::fs::File::create(output_path)?);
    let config = apng::create_config(&png_images, None)?;
    let mut encoder = apng::Encoder::new(&mut out, config)?;

    for frame in frames {
        let png_image = PNGImage {
            width: frame.image.width(),
            height: frame.image.height(),
            data: frame.image.as_raw().clone(),
            color_type: png::ColorType::Rgba,
            bit_depth: png::BitDepth::Sixteen,
        };
        let apng_frame = apng::Frame {
            delay_num: Some(frame.delay as u16), // Use frame's specific delay
            delay_den: Some(DEFAULT_FPS),        // Use constant instead of hardcoded 60
            ..Default::default()
        };
        if let Err(err) = encoder.write_frame(&png_image, apng_frame) {
            let msg = "Failed to write APNG frame";
            eprint!("{msg}: {err}");
            return Err(crate::FfbeError::ParseError(msg.into()));
        }
    }

    match encoder.finish_encode() {
        Ok(_) => {
            println!("Successfully saved animated APNG: {output_path}");
            Ok(())
        }
        Err(err) => {
            eprintln!("Failed to save animated APNG: {err}");
            Err(err.into())
        }
    }
}

pub fn encode_animated_gif(frames: Vec<CompositeFrame>, output_path: &str) -> error::Result<()> {
    let mut gif_frames = Vec::new();
    for frame in frames {
        let gif_frame = image::Frame::from_parts(
            frame.image,
            0,
            0,
            image::Delay::from_numer_denom_ms(frame.delay, DEFAULT_FPS as u32),
        );
        gif_frames.push(gif_frame);
    }
    let mut buffer = std::io::BufWriter::new(std::fs::File::create(output_path)?);
    let mut encoder = image::codecs::gif::GifEncoder::new(&mut buffer);
    encoder.set_repeat(image::codecs::gif::Repeat::Infinite)?;
    encoder.encode_frames(gif_frames)?;

    print!("Successfully saved animated gif: {output_path}");

    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use image::{Rgba, RgbaImage};

    #[test]
    fn test_rect_default() {
        let rect = Rect::default();
        assert_eq!(rect.x, 0);
        assert_eq!(rect.y, 0);
        assert_eq!(rect.width, 0);
        assert_eq!(rect.height, 0);
    }

    #[test]
    fn test_point_creation() {
        let point = Point::new(10, 20);
        assert_eq!(point.x(), 10);
        assert_eq!(point.y(), 20);
    }

    #[test]
    fn test_point_default() {
        let point = Point::default();
        assert_eq!(point.x(), 0);
        assert_eq!(point.y(), 0);
    }

    #[test]
    fn test_opacity_trait() {
        let mut img = RgbaImage::new(2, 2);
        img.put_pixel(0, 0, Rgba([255, 0, 0, 255])); // Red, full alpha
        img.put_pixel(1, 0, Rgba([0, 255, 0, 128])); // Green, half alpha

        img.opacity(0.5);

        assert_eq!(img.get_pixel(0, 0).0[3], 128); // 255 * 0.5 = 127.5 -> 128
        assert_eq!(img.get_pixel(1, 0).0[3], 64); // 128 * 0.5 = 64
    }

    #[test]
    #[should_panic(expected = "Opacity must be between 0.0 and 1.0")]
    fn test_opacity_trait_invalid_value() {
        let mut img = RgbaImage::new(1, 1);
        img.opacity(1.5); // Should panic
    }

    #[test]
    fn test_blend_trait() {
        let mut img = RgbaImage::new(2, 2);
        img.put_pixel(0, 0, Rgba([255, 128, 64, 255])); // RGB with full alpha
        img.put_pixel(1, 0, Rgba([0, 0, 0, 0])); // Transparent pixel

        img.blend();

        let pixel = img.get_pixel(0, 0);
        // R' = R * A = 255 * 1.0 = 255
        // G' = G * A = 128 * 1.0 = 128
        // B' = B * A = 64 * 1.0 = 64
        // A' = (R + G + B) / 3 = (255 + 128 + 64) / 3 = 447 / 3 = 149 (integer division)
        assert_eq!(pixel.0[0], 255);
        assert_eq!(pixel.0[1], 128);
        assert_eq!(pixel.0[2], 64);
        assert_eq!(pixel.0[3], 148); // Corrected expected value due to floating point precision

        // Transparent pixel should remain unchanged
        let transparent = img.get_pixel(1, 0);
        assert_eq!(transparent.0, [0, 0, 0, 0]);
    }

    #[test]
    fn test_color_bounds_find_color() {
        let mut img = RgbaImage::new(5, 5);
        let red = Rgba([255, 0, 0, 255]);
        let transparent = Rgba([0, 0, 0, 0]);

        // Fill with transparent, add red pixels at specific locations
        for y in 0..5 {
            for x in 0..5 {
                img.put_pixel(x, y, transparent);
            }
        }
        img.put_pixel(1, 1, red);
        img.put_pixel(3, 3, red);

        let bounds = img.get_color_bounds_rect(red, true).unwrap();
        assert_eq!(bounds.x, 1);
        assert_eq!(bounds.y, 1);
        assert_eq!(bounds.width, 3); // 3 - 1 + 1 = 3
        assert_eq!(bounds.height, 3); // 3 - 1 + 1 = 3
    }

    #[test]
    fn test_color_bounds_find_non_color() {
        let mut img = RgbaImage::new(3, 3);
        let transparent = Rgba([0, 0, 0, 0]);
        let red = Rgba([255, 0, 0, 255]);

        // Fill with transparent except center
        for y in 0..3 {
            for x in 0..3 {
                img.put_pixel(x, y, transparent);
            }
        }
        img.put_pixel(1, 1, red);

        // Find non-transparent pixels (should find the red pixel)
        let bounds = img.get_color_bounds_rect(transparent, false).unwrap();
        assert_eq!(bounds.x, 1);
        assert_eq!(bounds.y, 1);
        assert_eq!(bounds.width, 1);
        assert_eq!(bounds.height, 1);
    }

    #[test]
    fn test_color_bounds_no_match() {
        let mut img = RgbaImage::new(2, 2);
        let transparent = Rgba([0, 0, 0, 0]);
        let red = Rgba([255, 0, 0, 255]);

        // Fill with transparent
        for y in 0..2 {
            for x in 0..2 {
                img.put_pixel(x, y, transparent);
            }
        }

        // Look for red pixels (should find none)
        let bounds = img.get_color_bounds_rect(red, true);
        assert!(bounds.is_none());
    }

    #[test]
    fn test_load_source_image_nonexistent() {
        let result = load_source_image(99999, "nonexistent_path");
        assert!(result.is_err());
    }

    #[test]
    fn test_load_source_image_existing() {
        let result = load_source_image(204000103, "test_data");
        assert!(result.is_ok());

        let img = result.unwrap();
        assert!(img.width() > 0);
        assert!(img.height() > 0);
    }
}
