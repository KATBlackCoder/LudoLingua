//! Tauri command surface for the frontend.
//!
//! This module declares and exports the individual command groups. Keep
//! this file "imports only" to preserve a clean separation between the
//! thin Tauri wrappers in `handler.rs` and the pure business logic in
//! the sibling modules.

pub mod engine;
pub mod glossary;
pub mod handler;
pub mod languages;
pub mod provider;
pub mod translation;
