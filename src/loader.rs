use crate::asset::*;
use std::path::Path;

pub trait AssetLoader: Send + Sync + 'static {
    type AssetType: Asset;

    fn load(&self, path: &Path) -> Result<Self::AssetType, String>;
}

pub struct TextureLoader;

impl AssetLoader for TextureLoader {
    type AssetType = Texture;

    fn load(&self, path: &Path) -> Result<Self::AssetType, String> {
        Ok(format!("loaded texture from {:?}", path))
    }
}

pub struct SoundLoader;

impl AssetLoader for SoundLoader {
    type AssetType = Sound;

    fn load(&self, path: &Path) -> Result<Self::AssetType, String> {
        Ok(format!("loaded sound from {:?}", path))
    }
}
