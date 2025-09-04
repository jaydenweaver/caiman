use crate::asset::*;
use std::path::Path;

pub trait AssetLoader: Send + Sync + 'static {
    fn load(path: &Path) -> Result<Self::AssetType, String>;
}

pub struct TextureLoader;

impl AssetLoader for TextureLoader {
    fn load(path: &Path) -> Result<Texture, AssetError> {
        let img = image::open(path).map_err(|_| AssetError::NotFound)?; // load file
        let rgba = img.to_rgba8(); // parse file to image
        Ok(Texture { width: rgba.width(), height: rgba.height(), data: rgba.into_raw() }) // create Texture object
    }
}

pub struct SoundLoader;

impl AssetLoader for SoundLoader {
    fn load(path: &Path) -> Result<Self::AssetType, String> {
        Ok(format!("loaded sound from {:?}", path))
    }
}
