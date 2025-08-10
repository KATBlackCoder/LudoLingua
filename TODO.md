# TODO — Critical issues to fix first

1) Translate store: fix invoke payloads
   - File: `app/stores/translate.ts`
   - Change:
     - `config: providerStore.currentProviderConfig.value`
     - `engineInfo: engineStore.projectInfo.value`
     - Guard `engineStore.projectInfo?.value` before invoking

2) Add text processing to translation command
   - File: `src-tauri/src/commands/translation.rs`
   - Import from `crate::utils::text_processing`:
     - `replace_formatting_codes_for_translation`
     - `restore_formatting_codes_after_translation`
     - `is_technical_content`
   - Flow:
     - If `is_technical_content(source)` → return original unit (or mark Skipped)
     - Else pre-process → build prompt → LLM → post-process → set `translated_text`

3) Configurable LLM concurrency
   - File: `src-tauri/src/lib.rs`
   - Read `LLM_CONCURRENCY` env var, default 1, pass to `LlmState::new(...)`

4) Dynamic model discovery
   - Files:
     - `src-tauri/src/llm/services/ollama.rs` → add `list_local_models()`
     - `src-tauri/src/commands/provider.rs` → try dynamic, fallback to JSON

5) Pre-flight provider connectivity in UI
   - File: `app/stores/translate.ts`
   - Call `await providerStore.ensureConnected()` before translation

6) Optional: unifying language/provider settings
   - Files: `app/stores/language.ts`, `app/stores/settings.ts`
   - Make `settings.ts` source-of-truth; `language.ts` derives/reflects


