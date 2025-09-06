//! Caiman 🐊
//!
//! A lightweight, engine-agnostic asset manager for Rust applications and games.
//!
//! # Example
//! ```rust
//! use caiman::prelude::*;
//!
//! fn main() {
//!     let mut assets = AssetManager::new();
//!     let handle = assets.load::<Texture>("assets/player.png").unwrap();
//!     if let Some(texture) = assets.get(&handle) {
//!         println!("Loaded texture successfully!");
//!     }
//! }
//! ```

pub mod asset;
pub mod error;
pub mod loader;
pub mod manager;
pub mod handle;

pub use asset::Asset;
pub use loader::AssetLoader;
pub use manager::AssetManager;
pub use handle::AssetHandle;
