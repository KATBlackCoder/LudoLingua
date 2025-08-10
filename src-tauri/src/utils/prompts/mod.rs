//! Prompt utilities module.
//!
//! Houses helpers for building and loading prompt templates used by the
//! LLM service. Dev builds read from the filesystem; production builds
//! embed templates at compile time for robustness.

pub mod builder;
