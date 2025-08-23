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

pub mod manager;
pub mod loader;
pub mod cache;
pub mod error;
pub mod hot_reload;

pub use manager::AssetManager;
