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
    *   **Backend (cargo):** `async-trait`, `reqwest`, ~~`tauri-plugin-store`~~ (removed in Phase 4 migration).
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

### ✅ Phase 4: Unified JSON File Storage (Milestone 4) - Settings + Glossary Migration

**Goal:** Implement unified JSON file storage for settings (priority) and glossary (future improvement) to ensure data consistency, portability, and simplified architecture for Actors.json translations.

*   **Dependencies Removed:**
    *   ~~**Backend (cargo):** `sqlx` (with `runtime-tokio`, `sqlite`, and `macros` features).~~
    *   ~~**Backend (cargo):** `tauri-plugin-store`.~~
    *   ~~**Frontend (npm):** `@tauri-apps/plugin-store`.~~
    *   ~~**Tooling:** `sqlx-cli` (via `cargo install sqlx-cli`).~~
*   **Tasks & Files Created/Modified:**
    *   [x] **Backend Architecture Enhancement:**
        *   [x] Create `src/utils/` directory for application-wide utilities.
        *   [x] Create `src/utils/prompts/` for prompt-specific utilities.
        *   [x] Create `src/utils/prompts/builder.rs` to extract shared prompt building logic from `ollama.rs`.
        *   [x] Refactor `src/llm/services/ollama.rs` to use shared prompt builder.
        *   [x] Update `src/lib.rs` to include utils module.
        *   **Benefits:** Reusable prompt building across providers, cleaner separation of concerns, better testability.
    *   [x] **Unified Storage System (MIGRATED FROM DATABASE + TAURI STORE TO CUSTOM JSON):**
        *   [x] ~~Used `sqlx-cli` to create the `migrations/` folder and the first migration file.~~ **REMOVED**
        *   [x] ~~Create `src/db/mod.rs` and `src/db/connection.rs`.~~ **REMOVED**
        *   [x] ~~Create `src/db/glossary.rs` with all the CRUD functions.~~ **REPLACED WITH JSON STORAGE**
        *   [x] ~~Used Tauri Store plugin for settings persistence.~~ **REPLACED WITH CUSTOM JSON STORAGE**
        *   [x] Create `src/models/glossary.rs`.
        *   [x] Create `src/commands/glossary.rs` with the logic to be called by the frontend.
        *   [x] Update `src/commands/handler.rs`.
        *   [x] Update `src/lib.rs` to register glossary commands in invoke_handler.
        *   [x] Modify the `translate_text` logic to inject glossary terms into the prompt for Actors.json data.
                    *   [x] **NEW: Unified JSON File Storage Implementation:**
                *   [x] Create `storages/` directory in app root (auto-created on first run)
                *   [x] Create `src/storage/` directory for file-based storage modules
                *   [ ] **Phase 1: Core Storage Infrastructure (PRIORITY)**
                    *   [ ] Implement `src/storage/mod.rs` with common file operations and directory initialization
                    *   [ ] Add atomic file write operations for data safety (temp file + rename pattern)
                    *   [ ] Implement auto-creation of `storages/` folder on app startup
                    *   [ ] Add cross-platform path handling and error handling
                *   [ ] **Phase 2: Settings Storage Implementation (PRIORITY)**
                    *   [ ] Implement `src/storage/settings.rs` with JSON file operations (replaces Tauri Store plugin)
                    *   [ ] Replace database models with simpler JSON-serializable structs
                    *   [ ] Update settings commands to use file storage instead of Tauri Store plugin
                    *   [ ] Test settings persistence end-to-end
                    *   [ ] **Note:** Settings are essential for core translation functionality
                *   [ ] **Phase 3: Frontend Integration (PRIORITY)**
                    *   [ ] Update `stores/settings.ts` to remove Store plugin calls
                    *   [ ] Test frontend compatibility with new storage system
                    *   [ ] Ensure translation workflow works with new settings storage
                *   [ ] **Phase 4: Glossary Storage Implementation (FUTURE IMPROVEMENT)**
                    *   [ ] Implement `src/storage/glossary.rs` with JSON file operations (replaces database)
                    *   [ ] Update glossary commands to use file storage instead of database
                    *   [ ] Test glossary CRUD operations end-to-end
                    *   [ ] **Note:** Glossary is nice-to-have, not essential for core functionality
                *   [x] Remove database dependencies and migrations code
                *   [x] Remove `tauri-plugin-store` from `lib.rs` and dependencies
                *   [ ] **Phase 5: Testing & Validation**
                    *   [ ] Test file creation and atomic writes
                    *   [ ] Test cross-platform compatibility (Windows/Mac/Linux)
                    *   [ ] Test error handling (corrupted files, missing directories)
                    *   [ ] Implement backup and recovery for settings data (glossary can be added later)
    *   [x] **Frontend:**
        *   [x] Create `types/glossary.ts`.
        *   [x] Create a new `stores/glossary.ts` to manage glossary state.
        *   [x] Build the UI in `pages/glossary.vue` with basic add/remove functionality.
        *   [x] ~~Create `components/glossary/GlossaryTable.vue` for term display.~~ **SIMPLIFIED TO BASIC UI**
        *   [x] ~~Create `components/glossary/GlossaryForm.vue` for add/edit functionality.~~ **INTEGRATED INTO PAGE**
                    *   [x] **NEW: Frontend Store Updates for Unified Storage:**
                *   [x] Update `stores/settings.ts` to use custom JSON storage commands instead of Tauri Store plugin
                *   [x] Update `stores/glossary.ts` to use custom JSON storage commands (if needed)
                *   [x] Remove `@tauri-apps/plugin-store` imports from frontend
                *   [x] Test frontend compatibility with new backend storage commands
                *   [ ] **Priority Update:** Focus on settings storage first, glossary as future improvement

**Migration Benefits:**
*   ✅ **Eliminates app restart issues** - No more database WAL file conflicts with hot reload
*   ✅ **Unifies data location** - Settings in predictable `storages/` folder (glossary can be added later)
*   ✅ **Simplifies architecture** - No database connection pooling, migrations, or plugin dependencies
*   ✅ **Improves development experience** - No database initialization race conditions or plugin conflicts
*   ✅ **Enables easier debugging** - Human-readable JSON files instead of binary database or system app data
*   ✅ **Provides portable data** - Entire `storages/` folder can be backed up, versioned, or moved with app
*   ✅ **Ensures cross-platform consistency** - Same data location on Windows/Mac/Linux
*   ✅ **Reduces dependencies** - Removes sqlx, tauri-plugin-store, and related complexity
*   ✅ **Clean compilation** - All dependencies successfully removed and project compiles without errors
*   ✅ **Focused development** - Prioritize essential settings storage over optional glossary features

**New Storage Structure:**
```
LudoLingua/
├── storages/                    # Auto-created on first run
│   ├── settings.json           # App settings (replaces Tauri Store) - PRIORITY
│   ├── glossary.json           # Glossary terms (replaces SQLite database) - FUTURE
│   └── backups/                # Future: automatic backups
├── src-tauri/
├── pages/
└── ...
```

---



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

### ✅ Phase 5: Additional File Types Support (Milestone 5) - RPG Maker MV Expansion - PARTIALLY COMPLETED

**Goal:** Expand translation support beyond Actors.json to include other RPG Maker MV data files and event-based content, building on the proven Actors.json workflow.

### **✅ Completed: Common Helper Functions Architecture**
*   [x] **Created `src/engines/common.rs`** with reusable helper functions for all engine implementations
*   [x] **Implemented `extract_file_type_text`** for generic file extraction across different engines
*   [x] **Implemented `inject_file_type_translations`** for generic translation injection
*   [x] **Updated RPG Maker MV engine** to use common helper functions instead of private implementations
*   [x] **Benefits achieved:**
    *   [x] Eliminated code duplication between engine implementations
    *   [x] Consistent error handling and logging across all file types
    *   [x] Easy to add new file types (just 3 lines of code per file type)
    *   [x] Reusable across different RPG Maker versions (MV, MZ, etc.)
    *   [x] Type-safe generic functions with proper error handling
    *   [x] Reduced code complexity and improved maintainability

### **✅ Completed: Advanced Common Functions Architecture**
*   [x] **Created `src/engines/rpg_maker_mv/files/common.rs`** with higher-level abstraction functions
*   [x] **Implemented `extract_text_from_file_with_objects`** for generic file parsing and object iteration
*   [x] **Implemented `inject_translations_into_file_with_objects`** for generic translation injection with object updates
*   [x] **Refactored all file modules** (`actors.rs`, `items.rs`, `skills.rs`, `weapons.rs`, `armors.rs`, `classes.rs`) to use new common functions
*   [x] **Benefits achieved:**
    *   [x] ~70% reduction in code duplication across file modules
    *   [x] Consistent file I/O, JSON parsing, and error handling patterns
    *   [x] File-specific modules now focus only on their unique data structures and field logic
    *   [x] Improved maintainability and readability of all file processing code
    *   [x] Eliminated boilerplate code for file existence checks, JSON parsing, and iteration

*   **Dependencies to Install:**
    *   **Backend (cargo):** No new dependencies required (uses existing serde, anyhow)
    *   **Frontend (npm):** No new dependencies required
*   **Tasks & Files to Create/Modify:**
    *   [x] **Priority 1: Data Files (JSON-based)**
        *   [x] **Items.json Support:**
            *   [x] Create `src/engines/rpg_maker_mv/files/items.rs`
            *   [x] Implement `Item` struct with translatable fields (name, description, note)
            *   [x] Add `extract_text` and `inject_translations` functions following Actors.json pattern
            *   [x] Update engine to include Items.json in file detection
            *   [x] Test with sample RPG Maker MV project
        *   [x] **Skills.json Support:**
            *   [x] Create `src/engines/rpg_maker_mv/files/skills.rs`
            *   [x] Implement `Skill` struct with translatable fields (name, description, message1, message2, note)
            *   [x] Add `extract_text` and `inject_translations` functions
            *   [x] Update engine to include Skills.json in file detection
            *   [x] Test with sample RPG Maker MV project
        *   [x] **Weapons.json Support:**
            *   [x] Create `src/engines/rpg_maker_mv/files/weapons.rs`
            *   [x] Implement `Weapon` struct with translatable fields (name, description, note)
            *   [x] Add `extract_text` and `inject_translations` functions
            *   [x] Update engine to include Weapons.json in file detection
        *   [x] **Armors.json Support:**
            *   [x] Create `src/engines/rpg_maker_mv/files/armors.rs`
            *   [x] Implement `Armor` struct with translatable fields (name, description, note)
            *   [x] Add `extract_text` and `inject_translations` functions
            *   [x] Update engine to include Armors.json in file detection
        *   [x] **Classes.json Support:**
            *   [x] Create `src/engines/rpg_maker_mv/files/classes.rs`
            *   [x] Implement `Class` struct with translatable fields (name, note)
            *   [x] Add `extract_text` and `inject_translations` functions
            *   [x] Update engine to include Classes.json in file detection
    *   [ ] **Priority 2: Event Files (Map-based)**
        *   [ ] **MapXXX.json Support:**
            *   Create `src/engines/rpg_maker_mv/files/maps.rs`
            *   Implement `Map` struct with event parsing
            *   Add text extraction from event commands (codes 101, 401, 102, 402, etc.)
            *   Add `extract_text` and `inject_translations` functions
            *   Update engine to scan all MapXXX.json files
        *   [ ] **CommonEvents.json Support:**
            *   Create `src/engines/rpg_maker_mv/files/common_events.rs`
            *   Implement `CommonEvent` struct with translatable fields (name, message)
            *   Add text extraction from event commands
            *   Add `extract_text` and `inject_translations` functions
    *   [ ] **Priority 3: System Files**
        *   [ ] **System.json Support:**
            *   Create `src/engines/rpg_maker_mv/files/system.rs`
            *   Implement `System` struct with translatable fields (gameTitle, currencyUnit, elements, etc.)
            *   Add `extract_text` and `inject_translations` functions
        *   [ ] **Troops.json Support:**
            *   Create `src/engines/rpg_maker_mv/files/troops.rs`
            *   Implement `Troop` struct with translatable fields (name, pages with event commands)
            *   Add text extraction from event commands
            *   Add `extract_text` and `inject_translations` functions
    *   [ ] **Frontend Updates:**
        *   [ ] **Enhanced TranslationTable:**
            *   Add file type filtering (Actors, Items, Skills, etc.)
            *   Add file-specific translation status indicators
            *   Implement batch translation per file type
        *   [ ] **Project Statistics:**
            *   Display translation progress per file type
            *   Show total translatable text units across all files
            *   Add file type breakdown in project info
    *   [ ] **Backend Architecture Updates:**
        *   [ ] **Engine Enhancements:**
            *   Update `extract_game_data_files` to scan all supported file types
            *   Implement file type detection and routing
            *   Add comprehensive error handling for missing files
        *   [ ] **Translation Workflow:**
            *   Update translation commands to handle multiple file types
            *   Add file type-specific prompt selection
            *   Implement batch translation across file types

---

### Phase 6: Polishing, Testing & Distribution (Release)

**Goal:** Prepare the application for a public release with comprehensive RPG Maker MV translation support.

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

**Goal:** Expand the application to support additional RPG Maker engines and other game engines.

*   **Additional Engine Support:**
    *   [ ] Implement RPG Maker MZ support.
    *   [ ] Add support for other RPG Maker versions.
    *   [ ] Create engine detection logic for multiple RPG Maker versions.
    *   [ ] Add support for other game engines (Wolf RPG, etc.). 