### Immediate roadmap — Critical issues to fix first

- **Fix Tauri invoke payloads (refs/computed)**
  - In `app/stores/translate.ts`, pass plain data to `invoke`:
    - Use `providerStore.currentProviderConfig.value` and `engineStore.projectInfo.value`
    - Guard `engineStore.projectInfo?.value` and throw if null
  - Impact: prevents serialization errors and runtime failures when calling `translate_text_unit`.

- **Integrate RPG Maker formatting-code preservation in translation flow**
  - In `src-tauri/src/commands/translation.rs`:
    - Before building the prompt: `replace_formatting_codes_for_translation`
    - After LLM response: `restore_formatting_codes_after_translation`
    - Optionally short-circuit using `is_technical_content` to skip non-translatable lines
  - Impact: reliably preserves control codes and placeholders regardless of model behavior.

- **Make LLM concurrency configurable**
  - In `src-tauri/src/lib.rs`, read `LLM_CONCURRENCY` env var and pass to `LlmState::new(...)` (default 1)
  - Impact: allows safe parallelism on capable hosts without code changes.

- **Dynamic Ollama model discovery (fallback to JSON)**
  - In `src-tauri/src/llm/services/ollama.rs`, add `list_local_models()` via `ollama-rs`
  - In `src-tauri/src/commands/provider.rs`, prefer runtime discovery and fallback to bundled `models/ollama.json`
  - Impact: up-to-date model list without rebuilding.

- **Pre-flight connectivity checks from UI**
  - In `app/stores/translate.ts`, call `providerStore.ensureConnected()` before `invoke('translate_text_unit', ...)`
  - Impact: fast failure path; fewer hard timeouts.


