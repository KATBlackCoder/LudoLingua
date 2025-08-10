# Glossary (SQLx) integration — implementation plan

- [x] Backend: add dependency (you will add manually)
  - [ ] `sqlx = { version = "0.8.6", default-features = false, features = ["sqlite", "macros", "runtime-tokio"] }` in `src-tauri/Cargo.toml`
  - [ ] Optional (offline compile-time checks & migrations): install CLI locally, not as a dependency
        `cargo install sqlx-cli --no-default-features --features sqlite`
        Prepare metadata and commit `sqlx-data.json`:
        `set DATABASE_URL=sqlite://ludolingua.db && cargo sqlx prepare`
        Build offline later with `set SQLX_OFFLINE=true && cargo build`

- [x] Backend: glossary module (`src-tauri/src/glossaries/`)
  - [ ] `model.rs`: `GlossaryTerm { id, project_scope?, category, prompt_types?, source_lang, target_lang, input, output, enabled, priority }`
  - [ ] `state.rs`: `GlossaryState` with lazy `SqlitePool` init + schema creation + tiny in-memory cache
  - [ ] `repo.rs`: queries (find_terms with filters/limit), `upsert_term`, `delete_term`, `import_from_file`
  - [ ] `mod.rs`: re-exports, `GlossaryQuery` DTO
  - [ ] Wire state in `src-tauri/src/lib.rs`: `.manage(GlossaryState::new())`

- [x] Schema (SQLite)
  - [ ] Create table `glossary_terms` and index on `(enabled, source_lang, target_lang, category)`
  - [ ] Store DB in app data dir; migrate existing `src-tauri/ludolingua.db` if present
  - [ ] Pool options: enable WAL and set `PRAGMA busy_timeout` in `GlossaryState::ensure_pool()`

- [x] Prompt builder integration (`src-tauri/src/utils/prompts/builder.rs`)
  - [ ] Add `render_glossary_terms(terms) -> String` using same format as `prompts/vocabularies.txt`
  - [ ] Add `build_translation_prompt_with_terms(text_unit, engine_info, terms)`
  - [ ] Combine DB block + file `vocabularies.txt`, then call existing `filter_vocabulary_sections`
  - [ ] Cap injected terms (e.g., top N by `priority`) to protect token budget

- [x] Translation command plumbing
  - [ ] Extend Tauri handler to accept `State<GlossaryState>` for `translate_text_unit`
  - [ ] In `commands/translation.rs`, query terms by `(src_lang, tgt_lang, prompt type → categories, project_scope)` and call `build_translation_prompt_with_terms`
  - [ ] Fallback to current `build_translation_prompt` when no DB terms

- [x] Tauri commands (CRUD/import) — keep `#[tauri::command]` only in `commands/handler.rs`
  - [ ] Create `src-tauri/src/commands/glossary.rs` (pure logic; no tauri macros)
  - [ ] Add module export in `src-tauri/src/commands/mod.rs`: `pub mod glossary;`
  - [ ] Implement wrappers in `src-tauri/src/commands/handler.rs`: `glossary_list_terms(filter)`, `glossary_upsert_term(term)`, `glossary_delete_term(id)`, `glossary_import(project_scope?)`
  - [ ] Register the glossary commands in `src-tauri/src/lib.rs` `generate_handler!` list

- [ ] Frontend (Nuxt UI)
  - [ ] Glossary management view (DataTable: filter by lang/category/prompt-type; add/edit/delete; toggle enabled; priority)
  - [ ] Import from file (`prompts/vocabularies.txt`) via `@tauri-apps/plugin-dialog`

- [ ] Tests
  - [ ] Unit: builder render + `filter_vocabulary_sections` with DB/file combined
  - [ ] Integration: translate path uses DB terms when present; falls back to file otherwise

- [ ] Performance & robustness
  - [ ] Small LRU cache for query results during batch translation
  - [ ] Term limit & deterministic ordering (priority desc, id asc)

- [ ] Cleanup (after verification)
  - [ ] Remove unused crates: `reqwest`, `async-trait`, `chrono` (if unused)
  - [ ] Remove unused frontend deps (e.g., `valibot`) if not referenced

- [ ] Documentation & logs
  - [ ] Update `ARCHITECTURE.md`, `BACKEND_STRUCTURE.md`, `FRONTEND_STRUCTURE.md`
  - [ ] Update `CHANGELOG.md` and `PROGRESS.MD` when delivered (per policy)

Acceptance
- [ ] Translations include glossary terms from DB (with fallback to file) filtered by prompt type
- [ ] CRUD works via UI and persists to SQLite
- [ ] Term cap prevents excessive prompt size; priorities respected

