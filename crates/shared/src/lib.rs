//! Shared serde types used by both the Bevy WASM frontend and the Rust
//! Vercel serverless API functions.
//!
//! All portfolio content is expressed here as plain data so it can be swapped
//! out (or loaded from a CMS) without touching frontend or API code.
//!
//! Fields that describe *you* are clearly marked with placeholder values and
//! `#[placeholder]`-style doc comments — replace them with your real details.

use serde::{Deserialize, Serialize};

pub mod data;
pub mod types;
