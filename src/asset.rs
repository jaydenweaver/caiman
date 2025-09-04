pub trait Asset: Send + Sync + 'static {}

pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl Asset for Texture {}

pub struct Sound {
    pub sample_rate: u32,
    pub channels: u8,
    pub samples: Vec<i16>, //samples are interleaved per frame
}

impl Asset for Sound {}
