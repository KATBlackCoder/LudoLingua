# LudoLingua Development Roadmap

This document outlines the planned development phases for the LudoLingua application. Each phase represents a major milestone and a concrete set of files to be created or modified, based on our architecture documents.

## Development Strategy

**Approach:** Complete all phases (1-5) using only Actors.json parsing to build a fully functional translation application. After the core application is complete and tested, we'll add support for additional RPG Maker MV file types and engines.

This approach ensures we have a complete, working translation workflow before expanding to other file types, allowing us to:
- Validate the entire application architecture
- Test the complete user workflow
- Identify and fix any core issues
- Deliver a functional product sooner

---

### ✅ Phase 1: Project Setup & Core Foundation (Milestone 1)

**Goal:** Establish a solid, working project structure with all necessary dependencies and a basic UI shell.

*   **Dependencies to Install:**
    *   **Frontend (npm):** `@nuxt/ui`, `pinia`.
    *   **Backend (cargo):** `tokio`, `anyhow`, `serde`, `tauri-plugin-log`, `tauri-plugin-window-state`.
*   **Tasks & Files to Create/Modify:**
    *   [x] Initialize a new Nuxt.js project and add Tauri.
    *   [x] **Frontend:**
        *   Configure `nuxt.config.ts` (disable SSR, add modules).
        *   Create `layouts/default.vue` with a basic structure (`<NuxtPage />`).
        *   Create placeholder pages: `pages/index.vue`, `pages/glossary.vue`, `pages/settings.vue`.
        *   Create `stores/ui.ts` for managing global UI state.
        *   Create `assets/css/main.css` for global styles.
    *   [x] **Backend:**
        *   Set up `src/main.rs` and `src/lib.rs` with the Tauri builder.
        *   Create `src/core/mod.rs` and `src/core/error.rs`.
        *   Create an empty command structure: `src/commands/mod.rs` and `src/commands/handler.rs`.

---

### ✅ Phase 2: File Handling & Data Parsing (Milestone 2) - Actors.json Focus

**Goal:** Implement the ability to open an RPG Maker project, read Actors.json file, and display the extracted text in the UI.

*   **Dependencies to Install:**
    *   **Backend (cargo):** `tauri-plugin-dialog`.
*   **Tasks & Files to Create/Modify:**
    *   [x] **Backend:**
        *   Create `src/core/engine.rs` to define the `Engine` trait.
        *   Create `src/models/mod.rs`, `src/models/engine.rs`, `src/models/translation.rs`.
        *   Create the engine factory: `src/engines/mod.rs` and `src/engines/factory.rs`.
        *   Implement RPG Maker MV engine (e.g., in `src/engines/rpg_maker_mv/engine.rs`).
        *   Implement Actors.json parsing logic (`src/engines/rpg_maker_mv/files/mod.rs`, `.../files/actors.rs`).
        *   Create `src/commands/engine.rs` with `load_project` logic.
        *   Update `src/commands/handler.rs` to expose the `load_project` command.
    *   [x] **Frontend:**
        *   Create `stores/project.ts` to hold the project state.
        *   Create `types/engine.ts` and `types/translation.ts`.
        *   Implement a "Load Project" button in `pages/index.vue` that calls the backend command.
        *   Create editor components `components/editor/TranslationTable.vue` and `components/editor/FileExplorer.vue` to display the data from the store.

---

### ✅ Phase 3: Translation Core & UI (Milestone 3) - Actors.json Focus

**Goal:** Integrate the AI translation functionality and build the primary editing interface for Actors.json data.

*   **Dependencies Installed:**
    *   **Frontend (npm):** Zod (included in Nuxt UI).
    *   **Backend (cargo):** `async-trait`, `reqwest`, `tauri-plugin-store`.
*   **Tasks & Files Created/Modified:**
    *   [x] **Backend:**
        *   Created `src-tauri/prompts/` directory with professional templates (character.txt, description.txt, dialogue.txt).
        *   Created `src/llm/mod.rs`, `src/llm/trait_def.rs`, `src/llm/factory.rs`, and `src/llm/ollama.rs`.
        *   Implemented complete LLM abstraction with trait-based design and Ollama provider.
        *   Created `src/commands/translation.rs` with single and batch translation logic for Actors.json.
        *   Created `src/commands/settings.rs` with persistent LLM configuration management.
        *   Updated `src/commands/handler.rs` to expose all new translation and settings commands.
        *   Embedded prompt templates in binary using `include_str!()` for better architecture.
        *   Added context-aware prompt selection based on text type.
    *   [x] **Frontend:**
        *   Created `types/llm.ts` with comprehensive interfaces for LLM configuration and translation.
        *   Created `stores/settings.ts` to manage LLM settings with reactive state management.
        *   Built complete settings UI in `pages/settings.vue` with tabbed navigation.
        *   Created `components/settings/LlmSettingsForm.vue` with connection testing and validation.
        *   Created `components/settings/TranslationSettingsForm.vue` for language configuration.
        *   Enhanced `stores/project.ts` with translation functions for single and batch operations.
        *   Updated `components/editor/TranslationTable.vue` with translate buttons, status indicators, and proper UI feedback.
        *   Added settings validation and error handling throughout the translation workflow.

#### Planned Architecture Refactoring (Best-of-Breed Approach)

**Goal:** Migrate to a hybrid approach using service-specific crates when available, with graniet/llm as fallback for maximum performance and features.

*   **Planned Dependencies:**
    *   **Backend (cargo):** `ollama-rs` (857 stars, comprehensive Ollama support), `openai` (113 stars, async OpenAI support), `graniet-llm` (fallback for services without dedicated crates).
*   **Architecture Changes:**
    *   **Service-Specific Adapters:** Create `src/llm/services/` directory with adapters using the best available crate for each service.
    *   **Hybrid Approach:** Use ollama-rs for Ollama, openai crate for OpenAI, graniet/llm for Anthropic (fallback).
    *   **Factory Pattern Retention:** Keep the factory pattern to allow users to choose their preferred LLM provider.
    *   **Benefits:** Leverage service-specific optimizations, full feature access, and expert maintenance while maintaining our translation-focused API.

---

### ✅ Phase 3.5: LLM Architecture Refactoring (COMPLETED) - Best-of-Breed Approach

**Goal:** Migrate the current custom LLM implementation to use service-specific crates when available, with optimal performance and features.

*   **Dependencies Installed:**
    *   **Backend (cargo):** `ollama-rs` (for Ollama), `async-trait`.
*   **Tasks & Files Created/Modified:**
    *   [x] **Backend Refactoring:**
        *   [x] Added service-specific dependencies to `Cargo.toml` (ollama-rs, async-trait).
        *   [x] Created `src/llm/services/` directory for service adapters.
        *   [x] Implemented `src/llm/services/ollama.rs` using ollama-rs crate for optimal Ollama support.
        *   [x] Updated `src/llm/factory.rs` to instantiate appropriate service adapters.
        *   [x] Created `LlmProvider` trait with `async_trait` for service-specific implementations.
        *   [x] Removed custom HTTP client code, leveraged ollama-rs optimized implementation.
        *   [x] Updated `src/commands/translation.rs` to work with new provider system.
        *   [x] Enhanced prompt template loading from `prompts/` directory.
    *   [x] **Frontend Updates:**
        *   [x] Updated all components to use new `Language` objects and provider system.
        *   [x] Enhanced translation workflow with proper error handling.
        *   [x] Updated `stores/settings.ts` and related components for new architecture.

### ✅ Phase 3.6: JSON Model Configuration System (Priority Task) - Completed

**Goal:** Replace hardcoded model definitions with external JSON configuration files for better maintainability and user customization.

*   **No New Dependencies Required**
*   **Tasks & Files Created/Modified:**
    *   [x] **Backend Configuration System:**
        *   [x] Create `src-tauri/models/` directory for provider model definitions.
        *   [x] Create `src-tauri/models/ollama.json` with available Ollama models using `ModelInfo` structure.
        *   [x] Create template files: `src-tauri/models/openai.json`, `src-tauri/models/anthropic.json` for future use.
        *   [x] Update `src/llm/services/ollama.rs` to load models from JSON instead of hardcoded `get_available_models()`.
        *   [x] Add JSON parsing and validation logic with `serde` for model configurations.
        *   [x] Implement graceful fallback to hardcoded defaults if JSON files are missing or corrupt.
        *   [x] Update provider factory to use JSON-based model loading.
        *   [x] Add logging for model configuration loading status.
    *   [x] **Benefits Achieved:**
        *   [x] Enable model list updates without recompilation.
        *   [x] Allow user customization of available models.
        *   [x] Cleaner separation between code logic and configuration data.
        *   [x] Easier maintenance of provider-specific model catalogs.

---

### Phase 4: Glossary / Termbase (Milestone 4) - Actors.json Focus

**Goal:** Implement the glossary feature to ensure translation consistency for Actors.json translations.

*   **Dependencies to Install:**
    *   **Backend (cargo):** `sqlx` (with `runtime-tokio`, `sqlite`, and `macros` features).
    *   **Tooling:** `sqlx-cli` (via `cargo install sqlx-cli`).
*   **Tasks & Files to Create/Modify:**
    *   [ ] **Backend:**
        *   Use `sqlx-cli` to create the `migrations/` folder and the first migration file.
        *   Create `src/db/mod.rs` and `src/db/connection.rs`.
        *   Create `src/db/glossary.rs` with all the CRUD functions.
        *   Create `src/models/glossary.rs`.
        *   Create `src/commands/glossary.rs` with the logic to be called by the frontend.
        *   Update `src/commands/handler.rs`.
        *   Modify the `translate_text` logic to inject glossary terms into the prompt for Actors.json data.
    *   [ ] **Frontend:**
        *   Create `types/glossary.ts`.
        *   Create a new `stores/glossary.ts` to manage glossary state.
        *   Build the UI in `pages/glossary.vue` using components like `components/glossary/GlossaryTable.vue`.

---

### Phase 5: Backup-Based Project Saving & Final Touches (Milestone 5) - Actors.json Focus

**Goal:** Implement a backup-based translation workflow where users can safely work with translations and choose how to apply changes.

*   **Dependencies to Install:**
    *   **Backend (cargo):** `tauri-plugin-opener`.
*   **Tasks & Files to Create/Modify:**
    *   [ ] **Backend - Backup System:**
        *   Create backup creation logic that copies only translatable files (Actors.json) maintaining folder structure.
        *   Implement `create_project_backup()` method in engine implementation.
        *   Add `get_backup_path()` utility for path resolution between original and backup locations.
        *   ✅ Rename `save_text_units` to `inject_text_units` (replaces raw text with translated text in memory).
        *   Create new `save_files()` method with two modes: backup-only or direct-to-original.
        *   Update engine trait with new backup-aware methods.
        *   Add backup validation and integrity checks.
    *   [ ] **Frontend - Backup UI:**
        *   Add backup status tracking to engine store (backup exists, working location, save mode).
        *   Implement save mode selection UI (backup vs original destination).
        *   Create backup status indicators and working location display.
        *   Add "Save to Backup" and "Apply to Original" buttons with clear safety warnings.
        *   Implement backup management (create, refresh, cleanup) functionality.
        *   Add utility button to open project and backup folders.
    *   [ ] **Workflow Enhancement:**
        *   Modify project loading to automatically create selective backup of translatable files.
        *   Ensure all translation work happens on backup copies for safety.
        *   Add user choice between safe backup-only saves and direct-to-original application.
        *   Implement file conflict detection and resolution.

---

### Phase 6: Polishing, Testing & Distribution (Release)

**Goal:** Prepare the application for a public release with the complete Actors.json workflow.

*   **Dependencies to Install:**
    *   **Frontend (npm):** `@nuxtjs/i18n`, `vitest`.
    *   **Backend (cargo):** `tauri-plugin-updater`.
*   **Tasks & Files to Create/Modify:**
    *   [ ] **Frontend:**
        *   Create `locales/en.json` (and other languages) and integrate with i18n.
        *   Write unit tests for critical store logic (e.g., `tests/stores/project.spec.ts`).
    *   [ ] **Backend:**
        *   Write unit tests for command logic (e.g., in `src/commands/engine.rs`).
    *   [ ] **Project:**
        *   Conduct thorough manual testing and polish the UX.
        *   Configure the updater in `tauri.conf.json`.
        *   Set the final app icon and metadata, and build for distribution.

---

### Future Expansion (After Core Application Release)

**Goal:** Expand the application to support additional file types and RPG Maker engines.

*   **Additional File Types Support:**
    *   [ ] Implement parsers for other data files (e.g., `Items.json`, `Skills.json`).
    *   [ ] Create parsers for event-based files (e.g., `MapXXX.json`, `CommonEvents.json`).
    *   [ ] Extract text from commands with specific code values:
        *   [ ] `101` (Show Text - Message Window attributes)
        *   [ ] `401` (Show Text - Message content lines)
        *   [ ] `102` (Show Choices)
        *   [ ] `402` (When [Choice] selected - Choice content)
        *   [ ] `405` (Show Scrolling Text)
        *   [ ] `108` (Comment)
        *   [ ] `408` (Comment continuation)
        *   [ ] `320` (Change Actor Name)
        *   [ ] `324` (Change Actor Nickname)
        *   [ ] `355` (Script)
        *   [ ] `655` (Script continuation)
*   **Additional Engine Support:**
    *   [ ] Implement RPG Maker MZ support.
    *   [ ] Add support for other RPG Maker versions.
    *   [ ] Create engine detection logic for multiple RPG Maker versions.
    *   [ ] Add support for other game engines (Wolf RPG, etc.). 