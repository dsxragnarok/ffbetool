pub mod cgg;
pub mod cgs;
pub mod imageops;

pub type Frames = Vec<cgg::FrameParts>;

pub struct Unit {
    pub id: u32,
    pub frames: Frames,
}
