use crate::asset::Asset;
use std::path::Path;

pub trait AssetLoader: Send + Sync + 'static {
    type AssetType: Asset;

    fn load(&self, path: &Path) -> Result<Self::AssetType, String>;
}
