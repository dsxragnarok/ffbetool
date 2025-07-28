pub mod cgg;
pub mod cgs;
pub mod image;

pub type Frames = Vec<cgg::FrameParts>;

pub struct Unit {
    pub id: u32,
    pub frames: Frames,
}
