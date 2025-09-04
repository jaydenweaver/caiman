pub struct AssetHandle<T> {
    id: u64,
    _marker: std::marker::PhantomData<T>, // for type consistency
}

impl<T> AssetHandle<T> {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn id(&self) -> u64 { self.id }
}