//! LLM integration module.
//!
//! This module contains the shared state for the LLM layer and concrete
//! service implementations used by the command layer to perform
//! translations and connectivity checks.
//!
//! Modules
//! - `services`: Provider-specific LLM service(s) (currently Ollama-only)
//! - `state`: Shared `LlmState` with a lazily-initialized service instance and a
//!   semaphore-based limiter to protect the backend from overload

pub mod services;
pub mod state;
