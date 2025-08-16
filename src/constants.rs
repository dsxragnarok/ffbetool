/// Maximum value for an 8-bit channel (RGBA)
pub const MAX_CHANNEL_U8: u8 = 255;
pub const MAX_CHANNEL_F32: f32 = 255.0;
pub const RGB_CHANNEL_COUNT: f32 = 3.0;

/// Canvas size (replace with actual project dimensions)
pub const CANVAS_SIZE: u32 = 2000;

/// Half canvas values (often used for centering)
pub const HALF_CANVAS: u32 = CANVAS_SIZE / 2;

/// Max and min opaque alpha values (for blending math)
pub const ALPHA_OPAQUE_U8: u8 = 1;
pub const ALPHA_TRANSPARENT_U8: u8 = 0;
pub const ALPHA_OPAQUE_F32: f32 = 1.0;
pub const ALPHA_TRANSPARENT_F32: f32 = 0.0;
