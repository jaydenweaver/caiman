//use crate::{Asset, AssetLoader};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

pub struct AssetManager {
    next_id: AtomicU64,
    assets: HashMap<u64, Box<dyn std::any::Any + Send + Sync>>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self::default()
    }
    /*
    pub fn load<T: Asset, L: AssetLoader<T>>(
        &mut self,
        path: &str,
    ) -> Result<AssetHandle<T>, AssetError> {
        let asset = L::load(path)?;
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);

        self.assets.insert(id, Box::new(asset));

        Ok(AssetHandle::new(id))
    }

    pub fn get<T: Asset>(&self, handle: &AssetHandle<T>) -> Option<&T> {
        self.assets
            .get(&handle.id())
            .and_then(|x| x.downcast_ref::<T>())
    } */
}

impl Default for AssetManager {
    fn default() -> Self {
        Self {
            next_id: AtomicU64::new(1),
            assets: HashMap::new(),
        }
    }
}
