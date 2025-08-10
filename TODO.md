# TODO — Concurrency & Throughput

1) Backend: make concurrency configurable
   - File: `src-tauri/src/lib.rs`
   - Change: read `LLM_CONCURRENCY` (usize) from env and pass to `LlmState::new(...)`, default 1.

2) Backend: do not hold mutex across await
   - Files: `src-tauri/src/llm/state.rs`, `src-tauri/src/commands/translation.rs`
   - Change: store `Arc<OllamaService>` (e.g., inside `Mutex<Option<Arc<_>>>`). In `translate_with_retry`, clone the `Arc` while locked, drop the lock, then `await svc.generate(...)` so multiple requests can progress concurrently.

3) Backend: honor generation options
   - File: `src-tauri/src/llm/services/ollama.rs`
   - Change: apply `temperature` and `max_tokens` from `LlmConfig` to `GenerationRequest` options (adapt to the exact API of `ollama-rs`).

4) Frontend: parallelize batch with small pool
   - File: `app/stores/translate.ts`
   - Change: replace sequential for-loop with a worker-pool (e.g., concurrency = 2 or from settings). Ensure progress, error capture, and cancel still work.

5) Frontend: pre-flight connectivity
   - File: `app/stores/translate.ts`
   - Change: call `await providerStore.ensureConnected()` before starting/each unit and surface a clear error/toast.

6) Optional: dynamic model discovery
   - Files: `src-tauri/src/llm/services/ollama.rs`, `src-tauri/src/commands/provider.rs`
   - Change: query `list_local_models()` first; fallback to JSON. Map to `ModelInfo`.

7) Benchmarks & defaults
   - Decide defaults: `LLM_CONCURRENCY`=2 or 3; UI batch concurrency=2. Document in README.


