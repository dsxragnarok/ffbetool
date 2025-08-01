use apng::load_dynamic_image;
use image::{self, ImageBuffer, Rgba};

use crate::cgs::CompositeFrame;

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

pub fn load_source_image(unit_id: u32, input_path: &str) -> image::DynamicImage {
    let path = format!("{input_path}/unit_anime_{unit_id}.png");
    let img = image::open(path).unwrap();
    img
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
                let (r_f, g_f, b_f, a_f) = (
                    r as f32 / 255.0,
                    g as f32 / 255.0,
                    b as f32 / 255.0,
                    a as f32 / 255.0,
                );
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
            let new_alpha = ((a as f32) * opacity).round().clamp(0.0, 255.0) as u8;
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


pub fn encode_animated_apng(frames: Vec<Option<CompositeFrame>>, output_path: &str) {
    use apng::PNGImage;
    use image;

    let mut png_images: Vec<PNGImage> = Vec::new();
    for frame in frames {
        if let Some(frame) = frame {
            let fr_img = image::DynamicImage::from(frame.image);
            let png_image = load_dynamic_image(fr_img).expect("to be able to load frame image as png");
            png_images.push(png_image);
        }
    }

    let mut out = std::io::BufWriter::new(std::fs::File::create(output_path).unwrap());
    let config = apng::create_config(&png_images, None).unwrap();
    let mut encoder = apng::Encoder::new(&mut out, config).unwrap();
    let frame = apng::Frame {
        delay_num: Some(1),
        delay_den: Some(60),
        ..Default::default()
    };

    match encoder.encode_all(png_images, Some(&frame)) {
        Ok(_) => println!("Successfully saved animated APNG: {output_path}"),
        Err(e) => println!("Failed to save animated APNG: {e}"),
    }
}

pub fn encode_animated_gif(frames: Vec<Option<CompositeFrame>>, output_path: &str) {
    let mut gif_frames = Vec::new();
    for frame in frames {
        if let Some(frame) = frame {
            let gif_frame = image::Frame::from_parts(
                frame.image,
                0, 0,
                image::Delay::from_numer_denom_ms(frame.delay, 60),
            );
            gif_frames.push(gif_frame);
        }
    }
    let mut buffer = std::io::BufWriter::new(std::fs::File::create(output_path).unwrap());
    let mut encoder = image::codecs::gif::GifEncoder::new(&mut buffer);
    encoder.set_repeat(image::codecs::gif::Repeat::Infinite).unwrap();
    encoder.encode_frames(gif_frames).unwrap();

    print!("Successfully saved animated gif: {output_path}");
}
