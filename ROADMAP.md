## Roadmap

### Q3 2025 — Translation throughput and reliability

- Backend: concurrency & stability
  - Wire `LLM_CONCURRENCY` in `src-tauri/src/lib.rs` to configure `LlmState::new(...)` (default 1). Make it an env knob for dev/prod.
  - Avoid holding the service mutex across network awaits. Store `Arc<OllamaService>` inside `LlmState` (e.g., `RwLock<Option<Arc<...>>>`), clone the `Arc` and drop the lock before `await` in `translate_with_retry()`.
  - Honor `temperature` and `max_tokens` from `LlmConfig` when building `GenerationRequest` in `OllamaService::generate`.
  - Keep timeout/retry (3x, 90s) but make values configurable via env (optional).

- Frontend: parallel batch translation
  - Replace sequential loop with a small concurrency pool (default 2–3) in `app/stores/translate.ts` (`startBatchTranslation`). Persist a "Batch Concurrency" setting.
  - Pre-flight provider health (`ensureConnected()`) before starting the batch, with clear error surfacing.
  - Maintain cancelation responsiveness and accurate progress.

- Provider models
  - Prefer dynamic model discovery via `Ollama::list_local_models()`; fallback to bundled `src-tauri/models/ollama.json` if unavailable.

- QA & Perf
  - Benchmark small/medium/large projects. Document throughput vs. concurrency, choose a sane default for both UI batch concurrency and `LLM_CONCURRENCY`.
  - Validate placeholder and whitespace preservation under parallel load.

### Q4 2025 — UX polish and resilience

- Settings UI: add sliders/inputs for Batch Concurrency, Temperature, Max Tokens with validation and tooltips.
- Add per-unit retry and manual re-translate UX in results list (already partially present; round out edge cases).
- Optional: expose a per-batch backoff policy and a max-failures guard.

Note: Follow the tauri entrypoint pattern (setup in `src-tauri/src/lib.rs`; `src-tauri/src/main.rs` remains minimal).

