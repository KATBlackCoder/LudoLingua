use tokio::sync::{Mutex, Semaphore};

use crate::{
    core::error::AppResult,
    core::provider::LlmService,
    llm::factory::create_service,
    models::provider::LlmConfig,
};

/// Shared LLM state managed by Tauri
pub struct LlmState {
    pub service: Mutex<Option<Box<dyn LlmService>>>, // lazily initialized, multi-provider
    pub limiter: Semaphore,                           // simple concurrency/rate cap
}

impl LlmState {
    pub fn new(concurrency: usize) -> Self {
        Self {
            service: Mutex::new(None),
            limiter: Semaphore::new(concurrency.max(1)),
        }
    }

    /// Ensure the internal service is initialized with the provided config.
    /// If the existing service has a different config, it will be rebuilt.
    pub async fn ensure_service(&self, config: &LlmConfig) -> AppResult<()> {
        let mut guard = self.service.lock().await;
        let needs_rebuild = match guard.as_ref() {
            Some(svc) => !svc.config_matches(config),
            None => true,
        };
        if needs_rebuild {
            let new_svc = create_service(config.clone())?;
            *guard = Some(new_svc);
        }
        Ok(())
    }
}
