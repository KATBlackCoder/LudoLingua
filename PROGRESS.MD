# LudoLingua Development Progress

This document tracks the completed milestones of the LudoLingua application, based on the `ROADMAP.md`.

---

### ✅ Phase 1: Project Setup & Core Foundation (Milestone 1)

**Goal:** Establish a solid, working project structure with all necessary dependencies and a basic UI shell.

*   **Dependencies Installed:**
    *   **Frontend (npm):** `@nuxt/ui`, `pinia`, `@tauri-apps/api`.
    *   **Backend (cargo):** `tokio`, `anyhow`, `serde`, `tauri-plugin-log`, `tauri-plugin-window-state`.
*   **Tasks & Files Created/Modified:**
    *   [x] Initialized a Nuxt.js project and added Tauri.
    *   [x] **Frontend:**
        *   Configured `nuxt.config.ts` (disabled SSR, added modules, css, head).
        *   Created `layouts/default.vue` with a basic structure and navigation.
        *   Created placeholder pages: `pages/index.vue`, `pages/glossary.vue`, `pages/settings.vue`.
        *   Created `stores/ui.ts` for managing global UI state.
        *   Created `assets/css/main.css` for global styles.
        *   Wrapped application in `UApp` component for proper Nuxt UI configuration.
        *   Updated layout structure to follow Nuxt best practices with proper slot usage.
    *   [x] **Backend:**
        *   Set up `src/main.rs` and `src/lib.rs` with the Tauri builder.
        *   Created `src/core/mod.rs` and `src/core/error.rs`.
        *   Created `src/commands/mod.rs` and `src/commands/handler.rs`.
        *   Registered the command handler in `lib.rs`.
        *   Added a sample `hello` command to test frontend-backend communication.
---

### ✅ Phase 2: File Handling & Data Parsing (Milestone 2) - Completed

**Goal:** Implement the ability to open an RPG Maker project, read its data files, and display the extracted text in the UI.

*   **Tasks & Files Created/Modified:**
    *   **Backend:**
        *   [x] Created `src/core/engine.rs` to define the core `Engine` trait and `ProjectInfo` struct.
        *   [x] Created `src/models/mod.rs` to declare the data model modules.
        *   [x] Created `src/models/engine.rs` with the `GameDataFile` struct.
        *   [x] Created `src/models/translation.rs` with `TextUnit`, `TextContext`, and `TranslationStatus` structs/enums.
        *   [x] Created `src/engines/mod.rs` and `src/engines/factory.rs` for engine type detection and instantiation.
        *   [x] Implemented `src/engines/rpg_maker_mv/engine.rs` with the `RpgMakerMvEngine` struct and `Engine` trait implementation.
        *   [x] Added project detection criteria and metadata extraction from package.json.
        *   [x] Created `src/commands/engine.rs` with `load_project` and `extract_text` commands.
        *   [x] Updated command handler to register the new commands.
        *   [x] Improved engine detection with robust criteria matching for different project structures.
        *   [x] Enhanced error handling for projects without package.json.
        *   [x] Added detailed logging for better diagnostics during project loading.
        *   [x] Fixed architecture to follow the pattern where only handler.rs uses #[tauri::command].
        *   [x] Added `extract_game_data_files` command to retrieve structured game data files.
        *   [x] Updated `Engine` trait to include `as_any` method for downcasting to specific engine implementations.
        *   [x] Fixed Actors.json parsing to match the actual file structure.
        *   [x] Added extraction of note field from actors as translatable text.
        *   [x] Enhanced Actor struct with default values and flexible field handling to accommodate varying JSON structures.
        *   [x] Added proper field name mapping using serde rename attributes for camelCase JSON fields.
        *   [x] Added missing fields (equips, traits) and additional derive traits to the Actor struct.
    *   **Frontend:**
        *   [x] Created `types/engine.ts` with TypeScript interfaces matching the Rust structs.
        *   [x] Created `types/translation.ts` with TypeScript interfaces matching the Rust structs.
        *   [x] Created `stores/project.ts` with a basic Pinia setup store.
        *   [x] Implemented "Load Project" functionality in the UI.
        *   [x] Updated the project store to call the backend commands.
        *   [x] Added project info display to the main page.
        *   [x] Fixed project loading workflow to handle different project structures.
        *   [x] Created `components/editor/TranslationTable.vue` to display extracted text units.
        *   [x] Updated project store to use the new `extract_game_data_files` command.
        *   [x] Implemented file browser alongside translation table for better organization.
        *   [x] **UI Refactoring:**
            *   [x] Refactored layout into component-based architecture with `AppHeader` and `AppFooter` components.
            *   [x] Enhanced UI with Nuxt UI components (UContainer, UButtonGroup, UCard, etc.).
            *   [x] Added dark mode toggle in the footer.
            *   [x] Created About page to showcase application information.
            *   [x] Improved navigation with icons and active state indicators.
            *   [x] Fixed TypeScript errors in the TranslationTable component.
            *   [x] Created `components/editor/ProjectLoader.vue` to encapsulate project loading functionality.
            *   [x] Created `components/editor/FileExplorer.vue` to handle file selection and display.
            *   [x] Refactored `pages/index.vue` to use the new components for better organization.
            *   [x] Added container component for better layout consistency.
            *   [x] Improved conditional rendering to hide test functionality when a project is loaded.
            *   [x] Removed the `hello` command and related functionality.
            *   [x] Simplified TranslationTable component for better maintainability.
            *   [x] Created `components/layout/ProjectInfoAlert.vue` to display project information globally.
            *   [x] Enhanced user experience by showing loaded project information consistently across all pages.
    *   **Text Extraction:**
        *   [x] Created `src/engines/rpg_maker_mv/files/mod.rs` and `src/engines/rpg_maker_mv/files/actors.rs`.
        *   [x] Implemented parsing for RPG Maker MV's Actors.json file.
        *   [x] Added extraction of actor names, nicknames, and profiles as translatable text.
        *   [x] Updated the engine implementation to use the actors parser.
        *   [x] Fixed Actors.json parsing to handle the array structure correctly.
---

---

### ✅ Phase 3: Translation Core & UI (Milestone 3) - Completed

**Goal:** Integrate the AI translation functionality and build the primary editing interface for Actors.json data.

*   **Dependencies Installed:**
    *   **Backend (cargo):** `async-trait`, `reqwest`, `tauri-plugin-store`.
    *   **Frontend:** (none specific; using Nuxt UI built-ins).
*   **Tasks & Files Created/Modified:**
    *   **Backend LLM System:**
        *   [x] Created complete LLM abstraction layer with `src/llm/trait_def.rs` defining LLM provider traits.
        *   [x] Implemented Ollama provider in `src/llm/ollama.rs` with professional prompt engineering.
        *   [x] Created LLM factory in `src/llm/factory.rs` for provider instantiation.
        *   [x] Created `src/commands/translation.rs` with single and batch translation commands.
        *   [x] Created `src/commands/settings.rs` with persistent configuration management using Tauri store.
        *   [x] Updated command handler to register all new translation and settings commands.
        *   [x] **Architecture Improvement:** Moved prompt templates from frontend to `src-tauri/prompts/` directory.
        *   [x] Embedded prompt templates in binary using `include_str!()` for better distribution and security.
        *   [x] Added context-aware prompt selection (character names, descriptions, dialogue).
    *   **Frontend Translation System:**
        *   [x] Created comprehensive `types/llm.ts` with TypeScript interfaces matching backend structs.
        *   [x] Built `stores/settings.ts` with reactive state management for LLM configuration.
        *   [x] Created complete settings page with tabbed navigation in `pages/settings.vue`.
        *   [x] Built `components/settings/LlmSettingsForm.vue` with connection testing and validation.
        *   [x] Created `components/settings/TranslationSettingsForm.vue` for language preferences.
        *   [x] Enhanced `stores/project.ts` with translation functions for both single and batch operations.
        *   [x] Updated `components/editor/TranslationTable.vue` with translation buttons, status indicators, and proper error handling.
        *   [x] Added settings validation modal to prevent translation without proper configuration.
    *   **Translation Workflow:**
        *   [x] Implemented complete translation workflow from UI to LLM and back.
        *   [x] Added proper status tracking (NotTranslated, Translated, Error) with visual indicators.
        *   [x] Integrated glossary term support (ready for Phase 4).
        *   [x] Added connection testing and configuration validation.
        *   [x] Implemented persistent settings that survive app restarts.

---

### ✅ LLM Architecture Refactoring (Best-of-Breed Approach) - Completed

**Goal:** Modernize the LLM system using dedicated service crates for optimal performance and maintainability.

*   **Backend LLM System Refactoring:**
    *   [x] **Migrated to ollama-rs crate:** Replaced custom HTTP client with dedicated `ollama-rs` crate.
    *   [x] **Created service architecture:** Built `src-tauri/src/llm/services/ollama.rs` service.
    *   [x] **Ollama-only:** Removed `LlmProvider` trait and `LlmFactory`; simplified `LlmConfig`.
    *   [x] **Commands rewired:** Direct calls to `OllamaService`; added `get_ollama_models`.
    *   [x] **Improved prompt system:** Enhanced prompt template loading with dev FS reads and prod embedded resources; vocabulary filtered by prompt type.

*   **Frontend System Updates:**
    *   [x] **Updated type definitions:** Enhanced `types/llm.ts` with new `Language` interface and updated `LlmConfig`.
    *   [x] **Refactored project store:** Updated `stores/project.ts` to use `Language` objects and removed batch translation.
    *   [x] **Updated settings store:** Enhanced `stores/settings.ts` with proper `Language` object initialization.
    *   [x] **Fixed form components:** Updated `TranslationSettingsForm.vue` to use `UFormField` and handle `Language` objects.
    *   [x] **Enhanced UI components:** Updated `TranslationTable.vue` with correct status handling and removed batch translation.
    *   [x] **Fixed form validation:** Updated `LlmSettingsForm.vue` schema to match new backend interface.

*   **Translation Workflow Improvements:**
    *   [x] **One-by-one processing:** Removed batch translation in favor of individual text unit processing.
    *   [x] **Context-aware prompts:** Maintained automatic prompt template selection based on `PromptType`.
    *   [x] **Enhanced status tracking:** Updated status enum handling for `MachineTranslated`, `HumanReviewed`, etc.
    *   [x] **Improved error handling:** Added proper error propagation and user-friendly error messages.
    *   [x] **Native language support:** Added native language names for better UX in language selection.

### ✅ LLM Architecture Enhancement (Pure Data Structures & Type Safety) - Completed
### ✅ Phase 7 Backend Polish (Shared State, Retry/Timeout) - Completed

Goal: Stabilize and optimize per-unit translation calls with connection reuse.

*   Completed:
    *   Added `src-tauri/src/llm/state.rs` with `LlmState` (lazy `OllamaService` + `Semaphore`).
    *   Registered managed state in `src-tauri/src/lib.rs`.
    *   Updated `#[tauri::command] translate_text_unit` to accept `State<LlmState>` and forward it to command implementation.
    *   Refactored command layer to build prompts and call a new `translate_with_retry` (90s timeout + simple backoff retries).
    *   Simplified service with `generate(prompt)`; kept `translate` as a wrapper for compatibility.
    *   Verified HTTP keep-alive reuse in logs (single connection start, continuous completions).
    *   Applied Ollama ModelOptions: Temperature and max_tokens (num_predict) are now passed from settings to generation requests.


**Goal:** Refactor LLM system to use pure data structures and enum-based type safety for better maintainability and reliability.

*   **Backend Data Structure Refactoring:**
    *   [x] **Pure data structures:** Removed all `impl` methods from `models/provider.rs` for cleaner separation of concerns.
    *   [x] **Provider enum:** Created type-safe `Provider` enum replacing string-based provider selection.
    *   [x] **ModelInfo struct:** Added `ModelInfo` struct with display_name and model_name fields for better model management.
    *   [x] **OllamaModel enum:** Implemented comprehensive `OllamaModel` enum with 10 popular models and conversion methods.
    *   [x] **Enhanced OllamaProvider:** Added model discovery, validation, and display name methods.
    *   [x] **Factory pattern updates:** Updated `LlmFactory` to use pattern matching on `Provider` enum instead of string matching.
    *   [x] **Command handler fixes:** Updated translation commands to work with new enum-based provider system.

*   **Frontend Type Safety Updates:**
    *   [x] **OllamaConfig:** Replaced `ProviderConfig` with `OllamaConfig` (`app/types/provider.ts`).
    *   [x] **Settings/Provider stores:** Updated to Ollama-only; removed provider selector.
    *   [x] **Invokes:** Switched to `get_ollama_models`.

*   **Enhanced Prompt System:**
    *   [x] **Dedicated prompt templates:** Created specialized prompt templates for `item.txt`, `skill.txt`, and `other.txt`.
    *   [x] **Context-specific translation:** Each prompt type now has tailored instructions for better translation quality.
    *   [x] **Updated prompt selection:** Modified `OllamaProvider` to use dedicated templates instead of fallback to description.
    *   [x] **Professional guidelines:** Added specific guidelines for items, skills, and general game text translation.

*   **Model Management System:**
    *   [x] **Model discovery:** Models loaded from `src-tauri/models/ollama.json` via backend service.
    *   [x] **Display name support:** UI uses `display_name`, backend uses `model_name`.
    *   [x] **Language Catalog:** Added `src-tauri/models/language.json` (id, label, native_name, dir, enabled); `get_languages` returns enabled languages; frontend store loads list and applies RTL via `dir`.

---

### ✅ Store Architecture Refactoring (Domain-Driven Design) - Completed

**Goal:** Refactor the store architecture to follow proper separation of concerns and domain-driven design principles.

*   **Store Refactoring:**
    *   [x] **Created `stores/translate.ts`:** Dedicated store for translation process management with batch operations, progress tracking, and error handling.
    *   [x] **Renamed `stores/project.ts` to `stores/engine.ts`:** Focused on engine/project management, file operations, and text unit organization.
    *   [x] **Refined `stores/settings.ts`:** Focused purely on configuration management, connection testing, and validation.
    *   [x] **Enhanced `stores/language.ts`:** Centralized language management with proper source/target language handling.

*   **Architecture Improvements:**
    *   [x] **Clear separation of concerns:** Each store now has a focused, single responsibility.
    *   [x] **Enhanced maintainability:** Changes to one domain don't affect others.
    *   [x] **Improved testability:** Isolated stores are easier to test and debug.
    *   [x] **Better reusability:** Translation logic can be reused across different contexts.
    *   [x] **Scalable design:** Easy to add new features without bloating existing stores.

*   **Translation Store Features:**
    *   [x] **Individual translation:** Single text unit translation with proper error handling.
    *   [x] **Batch translation:** Process multiple text units with progress tracking and cancellation.
    *   [x] **Translation history:** Track previous translations with undo functionality.
    *   [x] **Failed translation tracking:** Manage and retry failed translations.
    *   [x] **Progress indicators:** Real-time progress updates with percentage completion.
    *   [x] **Toast notifications:** User-friendly feedback for translation operations.

*   **Engine Store Features:**
    *   [x] **Project management:** Loading, refreshing, and managing project state.
    *   [x] **File operations:** Current file selection and file-specific text unit access.
    *   [x] **Text unit management:** CRUD operations and search functionality.
    *   [x] **Project statistics:** Translation progress and completion metrics.
    *   [x] **Error handling:** Comprehensive error management with user feedback.

*   **Settings Store Features:**
    *   [x] **Configuration management:** LLM provider settings with validation.
    *   [x] **Connection testing:** Real-time connection status with timestamps.
    *   [x] **Configuration validation:** Comprehensive validation with error reporting.
    *   [x] **Settings persistence:** Automatic save/load with proper error handling.

*   **Component Updates:**
    *   [x] **Updated all imports:** Changed from `useProjectStore` to `useEngineStore` throughout the application.
    *   [x] **Enhanced component integration:** Direct store integration for better performance.
    *   [x] **Improved error handling:** Better error propagation and user feedback.
    *   [x] **Maintained functionality:** All existing features continue to work with improved architecture.

---

### ✅ Method Naming Refactor (Phase 5 Preparation) - Completed

**Goal:** Refactor method naming to accurately reflect functionality and prepare for backup-based save system.

*   **Naming Refactor Completed:**
    *   [x] **Backend trait updates:** Renamed `save_text_units` to `inject_text_units` in Engine trait (`src/core/engine.rs`).
    *   [x] **Engine implementation:** Updated RPG Maker MV engine implementation in `src/engines/rpg_maker_mv/engine.rs`.
    *   [x] **Command layer:** Updated command functions in `src/commands/engine.rs` and handler in `src/commands/handler.rs`.
    *   [x] **Tauri integration:** Updated command registration in `src/lib.rs` to use new command name.
    *   [x] **Frontend integration:** Updated engine store in `stores/engine.ts` to call new backend command.
    *   [x] **Documentation enhancement:** Updated all logging messages and documentation to reflect injection semantics.
    *   [x] **Architecture preparation:** Created clean foundation for upcoming backup-based save system implementation.

*   **Benefits Achieved:**
    *   [x] **Accurate naming:** Method names now clearly indicate "injection" vs "saving" operations.
    *   [x] **Clean foundation:** Prepared codebase for Phase 5 backup system without naming conflicts.
    *   [x] **Improved clarity:** Enhanced developer understanding of what each method actually does.
    *   [x] **Consistent terminology:** Unified naming across frontend and backend components.

---

### ✅ Phase 3.6: JSON Model Configuration System - Completed

**Goal:** Replace hardcoded model definitions with external JSON configuration files for better maintainability and user customization.

*   **JSON Model Configuration Completed:**
    *   [x] **Backend Configuration System:**
        *   [x] Created `src-tauri/models/` directory for provider model definitions
        *   [x] Created `src-tauri/models/ollama.json` with comprehensive Ollama model catalog using `ModelInfo` structure
        *   [x] Added template JSON files for future providers (openai.json, anthropic.json)
        *   [x] Updated `src/llm/services/ollama.rs` to load models from JSON instead of hardcoded lists
        *   [x] Added JSON parsing and validation logic with `serde` for model configurations
        *   [x] Implemented graceful fallback to hardcoded defaults if JSON files are missing/corrupt
        *   [x] Updated provider factory to use JSON-based model loading
        *   [x] Added logging for model configuration loading status and error reporting
    *   [x] **Benefits Achieved:**
        *   [x] **Model updates without recompilation:** Users can update model lists without rebuilding the application
        *   [x] **User customization:** Users can modify available models by editing JSON files
        *   [x] **Cleaner separation:** Configuration data is now separate from code logic
        *   [x] **Easier maintenance:** Provider-specific model catalogs are easier to maintain and update
        *   [x] **Enhanced flexibility:** Easy to add new models or modify existing ones without code changes
        *   [x] **Better error handling:** Graceful fallback ensures application works even if JSON files are missing

---

### ✅ Shared Prompt Builder Architecture (Phase 4 Enhancement) - Completed

**Goal:** Create a shared prompt building utility to reduce code duplication and improve maintainability across all LLM providers.

*   **Backend Architecture Enhancement:**
    *   [x] **Created `src/utils/` directory:** Established application-wide utilities structure for shared functionality.
    *   [x] **Implemented `src/utils/prompts/builder.rs`:** Comprehensive prompt building utility with 100+ lines of extracted logic.
    *   [x] **Added comprehensive documentation:** Full RustDoc documentation with examples and parameter descriptions.
    *   [x] **Implemented unit tests:** Complete test coverage for template path resolution, glossary section building, and variable replacement.
    *   [x] **Created fallback system:** Robust fallback prompt generation when template files are unavailable or corrupted.
    *   [x] **Added glossary support:** Built-in support for glossary term injection in prompts for translation consistency.
    *   [x] **Updated application structure:** Modified `src/lib.rs` to include utils module in the application architecture.
    *   [x] **Refactored Ollama provider:** Updated `src/llm/services/ollama.rs` to use shared prompt builder, eliminating code duplication.

*   **Benefits Achieved:**
    *   [x] **Reusability:** Prompt building logic can now be shared across all LLM providers (Ollama, OpenAI, Anthropic, etc.).
    *   [x] **Consistency:** All providers will generate identical prompts using the same logic and templates.
    *   [x] **Maintainability:** Single source of truth for prompt building logic, easier to update and maintain.
    *   [x] **Testability:** Independent unit tests for prompt building functionality, easier to debug and validate.
    *   [x] **Scalability:** Easy to add new prompt types or providers without duplicating logic.
    *   [x] **Code reduction:** Eliminated 100+ lines of duplicate code from provider implementations.

*   **Architecture Improvements:**
    *   [x] **Clean separation of concerns:** Prompt building logic is now separate from provider-specific API calls.
    *   [x] **Better organization:** Utils structure provides clear home for shared functionality.
    *   [x] **Future-ready:** Perfect foundation for Phase 4 glossary integration and future provider additions.
    *   [x] **Type safety:** Maintained full type safety with comprehensive error handling.

---

### ✅ Phase 4: Glossary / Termbase (Milestone 4) - Actors.json Focus - COMPLETED

**Goal:** Implement the glossary feature to ensure translation consistency for Actors.json translations.

*   **Dependencies Installed:**
    *   **Backend (cargo):** `sqlx` (with `runtime-tokio`, `sqlite`, and `macros` features), `chrono`.
    *   **Tooling:** `sqlx-cli` (via `cargo install sqlx-cli`).
*   **Tasks & Files Created/Modified:**
    *   **Backend Architecture Enhancement:**
        *   [x] **Created `src/utils/` directory:** Established application-wide utilities structure for shared functionality.
        *   [x] **Implemented `src/utils/prompts/builder.rs`:** Comprehensive prompt building utility with glossary integration.
        *   [x] **Refactored Ollama provider:** Updated `src/llm/services/ollama.rs` to use shared prompt builder.
        *   [x] **Updated application structure:** Modified `src/lib.rs` to include utils module in the application architecture.
        *   **Benefits:** Reusable prompt building across providers, cleaner separation of concerns, better testability.
    *   **Backend Glossary System:**
        *   [x] **Database setup:** Used `sqlx-cli` to create the `migrations/` folder and the first migration file.
        *   [x] **Database layer:** Created `src/db/mod.rs` and `src/db/connection.rs` with connection pooling.
        *   [x] **Glossary CRUD operations:** Created `src/db/glossary.rs` with all CRUD functions for glossary terms.
        *   [x] **Glossary models:** Created `src/models/glossary.rs` with `GlossaryTerm`, `CreateGlossaryTerm`, and `UpdateGlossaryTerm` structs.
        *   [x] **Database migration:** Created `migrations/001_create_glossary.sql` with proper schema for language-specific terms.
        *   [x] **Tauri commands:** Created `src/commands/glossary.rs` with all glossary management commands.
        *   [x] **Command registration:** Updated `src/commands/handler.rs` to register all glossary commands.
        *   [x] **Application integration:** Updated `src/lib.rs` to register glossary commands in invoke_handler and initialize database.
        *   [x] **Prompt integration:** Modified `src/utils/prompts/builder.rs` to inject glossary terms into translation prompts.
        *   [x] **Fallback system:** Implemented fallback to `basic.txt` vocabulary if database is empty.
        *   [x] **Language-specific lookup:** Added functionality to fetch terms by source and target language pairs.
    *   **Frontend Glossary System:**
        *   [x] **Type definitions:** Created `types/glossary.ts` with TypeScript interfaces matching Rust structs.
        *   [x] **State management:** Created `stores/glossary.ts` for glossary state management with CRUD operations.
        *   [x] **Basic UI:** Built `pages/glossary.vue` with basic "Hello Glossary" interface ready for enhancement.
        *   [x] **Component structure:** Created placeholder components for `components/glossary/GlossaryTable.vue` and `components/glossary/GlossaryForm.vue`.
    *   **Features Implemented:**
        *   [x] **Language-specific term management:** Support for source_language → target_language term pairs.
        *   [x] **User customization:** Framework for user customization of character names and game terms.
        *   [x] **Translation workflow integration:** Glossary terms are automatically injected into translation prompts.
        *   [x] **Fallback system:** Robust fallback to curated vocabulary from `basic.txt` when database is empty.
        *   [x] **Database persistence:** SQLite database with proper migrations and connection pooling.
        *   [x] **Error handling:** Comprehensive error handling throughout the glossary system.

*   **Architecture Benefits:**
    *   [x] **Translation consistency:** Glossary terms ensure consistent translation of character names and game terms.
    *   [x] **User control:** Users can customize translation vocabulary for their specific projects.
    *   [x] **Performance:** Efficient database queries with connection pooling and proper indexing.
    *   [x] **Scalability:** Easy to extend with additional term types and language pairs.
    *   [x] **Integration:** Seamless integration with existing translation workflow.

---

### ✅ Phase 4: Storage Architecture Decision - COMPLETED

**Goal:** Successfully migrate from SQLite database to simplified storage architecture using Tauri Store plugin for settings.

*   **Dependencies Removed:**
    *   [x] **Backend (cargo):** `sqlx` (with `runtime-tokio`, `sqlite`, and `macros` features)
    *   [x] **Tooling:** `sqlx-cli` (via `cargo install sqlx-cli`)

*   **Dependencies Kept:**
    *   [x] **Backend (cargo):** `tauri-plugin-store` - Kept for settings persistence
    *   [x] **Frontend (npm):** `@tauri-apps/plugin-store` - Kept for settings persistence

*   **Backend Cleanup Completed:**
    *   [x] **Removed database dependencies** from `Cargo.toml`
    *   [x] **Deleted entire `src-tauri/src/db/` directory** (connection.rs, glossary.rs, mod.rs)
    *   [x] **Deleted `src-tauri/migrations/` directory** (SQL migration files)
    *   [x] **Updated `lib.rs`** to remove db module and database initialization
    *   [x] **Re-registered Tauri Store plugin** in `src-tauri/src/lib.rs`
    *   [x] **Re-added store permissions** in `src-tauri/capabilities/default.json`

*   **Frontend Cleanup Completed:**
    *   [x] **Updated `stores/settings.ts`** to properly use Tauri Store plugin
    *   [x] **Tested frontend compilation** and confirmed no import errors

*   **Architecture Benefits Achieved:**
    *   [x] **Eliminated app restart issues** - No more database WAL file conflicts with hot reload
    *   [x] **Simplified architecture** - No database connection pooling, migrations, or complex dependencies
    *   [x] **Improved development experience** - No database initialization race conditions
    *   [x] **Enhanced debugging** - Settings stored in predictable app data location
    *   [x] **Cross-platform consistency** - Tauri Store plugin handles platform-specific storage
    *   [x] **Reduced dependencies** - Eliminates `sqlx` and related database complexity
    *   [x] **Clean compilation** - All dependencies successfully removed and project compiles without errors
    *   [x] **Settings persistence** - Tauri Store plugin provides atomic writes, automatic persistence, and type safety

*   **Decision Rationale:**
    *   [x] **Tauri Store plugin advantages:** Atomic writes, cross-platform compatibility, automatic persistence, type safety
    *   [x] **Simplified development:** No need for custom JSON file storage implementation
    *   [x] **Focus on core functionality:** Settings working properly, glossary can be added later as needed
    *   [x] **Better user experience:** Settings persist automatically with proper error handling

### ✅ Glossary Removal - COMPLETED

**Goal:** Completely remove glossary functionality to simplify the application and focus on core translation features.

*   **Backend Files Removed:**
    *   [x] **Deleted `src-tauri/src/models/glossary.rs`** - Removed all glossary data structures
    *   [x] **Deleted `src-tauri/src/commands/glossary.rs`** - Removed all glossary CRUD commands
    *   [x] **Updated `src-tauri/src/models/mod.rs`** - Removed glossary module declaration
    *   [x] **Updated `src-tauri/src/commands/mod.rs`** - Removed glossary module declaration
    *   [x] **Updated `src-tauri/src/commands/handler.rs`** - Removed all glossary command registrations
    *   [x] **Updated `src-tauri/src/commands/translation.rs`** - Removed glossary parameter from translate_text_unit

*   **Frontend Files Removed:**
    *   [x] **Deleted `stores/glossary.ts`** - Removed glossary state management
    *   [x] **Deleted `types/glossary.ts`** - Removed glossary TypeScript interfaces
    *   [x] **Deleted `pages/glossary.vue`** - Removed glossary page
    *   [x] **Updated `components/layout/AppHeader.vue`** - Removed glossary navigation button

*   **Architecture Benefits Achieved:**
    *   [x] **Simplified codebase** - Removed complex glossary management system
    *   [x] **Reduced complexity** - No more glossary CRUD operations or database integration
    *   [x] **Focused functionality** - Application now focuses on core translation workflow
    *   [x] **Cleaner UI** - Removed glossary navigation and related components
    *   [x] **Faster development** - Less code to maintain and debug
    *   [x] **Better user experience** - Simpler interface without unnecessary features

### ✅ Phase 5: Additional File Types Support - COMPLETED

**Goal:** Expand translation support beyond Actors.json to include other RPG Maker MV data files and event-based content.

**✅ Successfully completed support for 12 RPG Maker MV file types:**
- Actors.json ✅
- Items.json ✅  
- Skills.json ✅
- Weapons.json ✅
- Armors.json ✅
- Classes.json ✅
- System.json ✅
- States.json ✅
- Enemies.json ✅
- CommonEvents.json ✅
- MapInfos.json ✅
- MapXXX.json ✅

*   **✅ Completed: Common Helper Functions Architecture**
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

*   **✅ Completed: Items.json Support**
    *   [x] **Created `src/engines/rpg_maker_mv/files/items.rs`** with complete Items.json implementation
    *   [x] **Implemented `Item` struct** with translatable fields (name, description, note)
    *   [x] **Added `extract_text` and `inject_translations` functions** following the same pattern as Actors.json
    *   [x] **Updated engine** to include Items.json in file detection alongside Actors.json
    *   [x] **Tested with sample RPG Maker MV project** - Items.json extraction and injection working correctly
    *   [x] **Benefits achieved:**
        *   [x] Successfully extracted item names, descriptions, and notes from Items.json
        *   [x] Proper translation injection back into Items.json files
        *   [x] Consistent ID prefixing (`item_`) for text unit identification
        *   [x] Reused common helper functions for clean, maintainable code

*   **✅ Completed: Skills.json Support**
    *   [x] **Created `src/engines/rpg_maker_mv/files/skills.rs`** with complete Skills.json implementation
    *   [x] **Implemented `Skill` struct** with translatable fields (name, description, message1, message2, note)
    *   [x] **Added `extract_text` and `inject_translations` functions** following the same pattern as Actors.json and Items.json
    *   [x] **Updated engine** to include Skills.json in file detection alongside Actors.json and Items.json
    *   [x] **Tested with sample RPG Maker MV project** - Skills.json extraction and injection working correctly
    *   [x] **Benefits achieved:**
        *   [x] Successfully extracted skill names, descriptions, messages, and notes from Skills.json
        *   [x] Proper translation injection back into Skills.json files
        *   [x] Consistent ID prefixing (`skill_`) for text unit identification
        *   [x] Reused common helper functions for clean, maintainable code

*   **✅ Completed: Advanced Common Functions Architecture**
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

*   **✅ Completed: Weapons.json Support**
    *   [x] **Created `src/engines/rpg_maker_mv/files/weapons.rs`** with complete Weapons.json implementation
    *   [x] **Implemented `Weapon` struct** with translatable fields (name, description, note)
    *   [x] **Added `extract_text` and `inject_translations` functions** following the same pattern as other file modules
    *   [x] **Updated engine** to include Weapons.json in file detection alongside other file types
    *   [x] **Tested with sample RPG Maker MV project** - Weapons.json extraction and injection working correctly
    *   [x] **Benefits achieved:**
        *   [x] Successfully extracted weapon names, descriptions, and notes from Weapons.json
        *   [x] Proper translation injection back into Weapons.json files
        *   [x] Consistent ID prefixing (`weapon_`) for text unit identification
        *   [x] Reused common helper functions for clean, maintainable code

*   **✅ Completed: Armors.json Support**
    *   [x] **Created `src/engines/rpg_maker_mv/files/armors.rs`** with complete Armors.json implementation
    *   [x] **Implemented `Armor` struct** with translatable fields (name, description, note)
    *   [x] **Added `extract_text` and `inject_translations` functions** following the same pattern as other file modules
    *   [x] **Updated engine** to include Armors.json in file detection alongside other file types
    *   [x] **Tested with sample RPG Maker MV project** - Armors.json extraction and injection working correctly
    *   [x] **Benefits achieved:**
        *   [x] Successfully extracted armor names, descriptions, and notes from Armors.json
        *   [x] Proper translation injection back into Armors.json files
        *   [x] Consistent ID prefixing (`armor_`) for text unit identification
        *   [x] Reused common helper functions for clean, maintainable code

*   **✅ Completed: Classes.json Support**
    *   [x] **Created `src/engines/rpg_maker_mv/files/classes.rs`** with complete Classes.json implementation
    *   [x] **Implemented `Class` struct** with translatable fields (name, note)
    *   [x] **Added `extract_text` and `inject_translations` functions** following the same pattern as other file modules
    *   [x] **Updated engine** to include Classes.json in file detection alongside other file types
    *   [x] **Tested with sample RPG Maker MV project** - Classes.json extraction and injection working correctly
    *   [x] **Benefits achieved:**
        *   [x] Successfully extracted class names and notes from Classes.json
        *   [x] Proper translation injection back into Classes.json files
        *   [x] Consistent ID prefixing (`class_`) for text unit identification
        *   [x] Reused common helper functions for clean, maintainable code

*   **✅ Completed: System.json Support**
    *   [x] **Created `src/engines/rpg_maker_mv/files/system.rs`** with complete System.json implementation
    *   [x] **Implemented `System` and `Terms` structs** for complex nested structure handling
    *   [x] **Added custom `extract_text` and `inject_translations` functions** for unique single-object structure
    *   [x] **Updated engine** to include System.json in file detection alongside other file types
    *   [x] **Tested with sample RPG Maker MV project** - System.json extraction and injection working correctly
    *   [x] **Benefits achieved:**
        *   [x] Successfully extracted game title, currency unit, elements, switches, variables, and UI terms/messages
        *   [x] Proper translation injection back into System.json files
        *   [x] Consistent ID prefixing (`system_`) for text unit identification
        *   [x] Handled complex nested structure with terms object containing basic, commands, params, and messages arrays
        *   [x] Fixed System.json message translation to use correct prompt type (System instead of Other)

*   **✅ Completed: States.json Support**
    *   [x] **Created `src/engines/rpg_maker_mv/files/states.rs`** with complete States.json implementation
    *   [x] **Implemented `State` struct** with translatable fields (name, description, message1, message2, message3, message4, note)
    *   [x] **Added `extract_text` and `inject_translations` functions** using common helper functions
    *   [x] **Updated engine** to include States.json in file detection alongside other file types
    *   [x] **Created specialized `state.txt` prompt file** for RPG status effect translation
    *   [x] **Added `PromptType::State` enum variant** for status effect-specific translation guidance
    *   [x] **Tested with sample RPG Maker MV project** - States.json extraction and injection working correctly
    *   [x] **Benefits achieved:**
        *   [x] Successfully extracted status effect names, descriptions, battle messages, and developer notes
        *   [x] Proper translation injection back into States.json files
        *   [x] Consistent ID prefixing (`state_`) for text unit identification
        *   [x] Handled complex array structure with null values and empty states filtering
        *   [x] Specialized prompt guidance for RPG status effect terminology and battle context

*   **✅ Completed: Enemies.json Support**
    *   [x] **Created `src/engines/rpg_maker_mv/files/enemies.rs`** with complete Enemies.json implementation
    *   [x] **Implemented `Enemy` struct** with translatable fields (name, note)
    *   [x] **Added `extract_text` and `inject_translations` functions** using common helper functions
    *   [x] **Updated engine** to include Enemies.json in file detection alongside other file types
    *   [x] **Tested with sample RPG Maker MV project** - Enemies.json extraction and injection working correctly
    *   [x] **Benefits achieved:**
        *   [x] Successfully extracted enemy names and developer notes from Enemies.json
        *   [x] Proper translation injection back into Enemies.json files
        *   [x] Consistent ID prefixing (`enemy_`) for text unit identification
        *   [x] Handled complex array structure with null values and empty enemies filtering
        *   [x] Used appropriate prompt types (Character for names, Other for notes)

*   **✅ Completed: CommonEvents.json Support**
    *   [x] **Created `src/engines/rpg_maker_mv/files/common_events.rs`** with complete CommonEvents.json implementation
    *   [x] **Implemented `CommonEvent` and `EventCommand` structs** for complex event command structure handling
    *   [x] **Added `extract_text` and `inject_translations` functions** using common helper functions
    *   [x] **Updated engine** to include CommonEvents.json in file detection alongside other file types
    *   [x] **Tested with sample RPG Maker MV project** - CommonEvents.json extraction and injection working correctly
    *   [x] **Benefits achieved:**
        *   [x] Successfully extracted event names, message content (code 401), comments (code 108), and other translatable parameters
        *   [x] Proper translation injection back into CommonEvents.json files
        *   [x] Consistent ID prefixing (`common_event_`) for text unit identification
        *   [x] Handled complex event command structure with nested parameters and command codes
        *   [x] Used appropriate prompt types (Character for names, Dialogue for messages, Other for comments and parameters)
        *   [x] Handled complex array structure with null values and empty events filtering

*   **✅ Completed: MapInfos.json Support**
    *   [x] **Created `src/engines/rpg_maker_mv/files/maps_infos.rs`** with complete MapInfos.json implementation
    *   [x] **Implemented `MapInfo` struct** with translatable fields (name) and metadata fields (id, expanded, order, parent_id, scroll_x, scroll_y)
    *   [x] **Added `extract_text` and `inject_translations` functions** following the same pattern as other file modules
    *   [x] **Updated engine** to include MapInfos.json in file detection alongside other file types
    *   [x] **Tested with sample RPG Maker MV project** - MapInfos.json extraction and injection working correctly
    *   [x] **Benefits achieved:**
        *   [x] Successfully extracted map names from MapInfos.json (map tree/structure file)
        *   [x] Proper translation injection back into MapInfos.json files
        *   [x] Consistent ID prefixing (`map_info_`) for text unit identification
        *   [x] Handled complex array structure with null values and empty map entries filtering
        *   [x] Used appropriate prompt types (Character for map names)
        *   [x] Distinguished between MapInfos.json (map metadata) and MapXXX.json (map content) files

*   **✅ Completed: MapXXX.json Support**
    *   [x] **Created `src/engines/rpg_maker_mv/files/maps.rs`** with complete MapXXX.json implementation
    *   [x] **Implemented dynamic file discovery** to handle Map001.json, Map002.json, etc. automatically
    *   [x] **Implemented comprehensive data structures** for Map, MapEvent, EventPage, and EventCommand
    *   [x] **Added `extract_text` and `inject_translations` functions** following the established architecture pattern
    *   [x] **Updated engine** to include dynamic MapXXX.json discovery and processing
    *   [x] **Benefits achieved:**
        *   [x] Successfully extracts event names, notes, messages (code 401), and comments (code 108) from map events
        *   [x] Proper translation injection back into MapXXX.json files with event command parameter updates
        *   [x] Consistent ID prefixing (`map_event_`) for text unit identification
        *   [x] Handled complex nested structure with events, pages, and commands
        *   [x] Used appropriate prompt types (Character for event names, Dialogue for messages, Other for notes/comments)
        *   [x] Robust error handling with graceful fallback for individual map files
        *   [x] Dynamic file discovery eliminates need for hardcoded map file lists

### ✅ Prompt System Optimization - COMPLETED

**Goal:** Refine and optimize the prompt system for better translation quality and consistency.

*   **✅ Completed: Prompt Architecture Refinement**
    *   [x] **Created `basic.txt`** as streamlined base prompt (removed overwhelming examples)
    *   [x] **Consolidated equipment prompts** (`weapon.txt` + `item.txt` → `equipment.txt`)
    *   [x] **Updated `PromptType` enum** to use `Equipment` instead of separate `Weapon` and `Item` types
    *   [x] **Added specific translation guides** to all prompt files for consistency
    *   [x] **Enhanced output format instructions** to prevent unwanted comments/metadata
    *   [x] **Added field-specific guidelines** for character names, equipment, skills, etc.
    *   [x] **Enhanced erotic content handling** with appropriate examples and terminology
    *   [x] **Organized dialogue examples** by moving dialogue-specific examples from `basic.txt` to `dialogue.txt`
    *   [x] **Implemented flexible skill guidelines** replacing rigid mappings with pattern-based instructions
    *   [x] **Enhanced class translations** with proper RPG distinctions (勇者 → Hero, 戦士 → Warrior)
    *   [x] **Placeholder standardization:** `%n` → `[ARG_n]`, literal newlines in examples → `[NEWLINE_1]`, and `[NEWLINE_n]` added to preserved token list in `basic.txt`

*   **✅ Completed: LLM Integration Improvements**
    *   [x] **Updated `PromptBuilder`** to map `PromptType::Equipment` to `equipment.txt`
    *   [x] **Updated file parsers** to use `PromptType::Equipment` for weapons, items, and armor
    *   [x] **Added comprehensive debugging** to `ProjectStats.vue` and `TranslationTable.vue`
    *   [x] **Enhanced settings store** with proper initialization and language configuration display
    *   [x] **Improved provider store** with settings loading on initialization

### ✅ UI/UX Enhancements - COMPLETED

**Goal:** Enhance the user interface and user experience for better workflow and usability.

*   **✅ Completed: Project Management Enhancements**
    *   [x] **Added "Load New Project" button** to main page header for easy project switching
    *   [x] **Implemented project loading workflow** with folder selection dialog
    *   [x] **Enhanced project state management** with proper loading states and error handling
    *   [x] **Improved user feedback** with loading indicators and error messages

*   **✅ Completed: Translation Table Enhancements**
    *   [x] **Added "Reset" button** to TranslationTable for clearing all translations
    *   [x] **Implemented reset functionality** with proper state management and error handling
    *   [x] **Enhanced button states** with improved availability logic (Inject only when translation finished)
    *   [x] **Added translation progress tracking** with better status indicators
    *   [x] **Improved user experience** with clear feedback and intuitive controls

*   **✅ Completed: User Experience Improvements**
    *   [x] **Streamlined workflow** with better button placement and logical flow
    *   [x] **Enhanced error handling** with user-friendly error messages and recovery options
    *   [x] **Improved accessibility** with proper button states and loading indicators
    *   [x] **Better visual feedback** with progress indicators and status updates

### ✅ Phase 6: Framework Migration - COMPLETED

**Goal:** Migrate to latest technologies and package manager for enhanced performance and modern development practices.

*   **✅ Completed: Nuxt 4 + pnpm Migration**
    *   [x] **Framework Upgrade:**
        *   [x] Migrated from Nuxt 3.18.1 to Nuxt 4.0.3
        *   [x] Switched from npm to pnpm for better performance and dependency resolution
        *   [x] Updated all dependencies to latest versions
        *   [x] Tested compatibility with Tauri integration
        *   [x] Updated TypeScript configurations
        *   [x] Migrated to new Nuxt 4 APIs and conventions
        *   [x] Updated build configuration for optimal performance
        *   [x] Updated package.json with pnpm-specific configurations
        *   [x] Migrated lock files and dependency management
        *   [x] Updated CI/CD and build scripts for pnpm
        *   [x] Tested package installation and build processes
    *   [x] **Directory Structure Migration:**
        *   [x] Automatically migrated to new Nuxt 4 `app/` directory structure using official codemod
        *   [x] Moved all frontend files (components, pages, stores, types, etc.) to `app/` directory
        *   [x] Updated file paths and imports to work with new structure
        *   [x] Maintained backward compatibility during migration
    *   [x] **Pinia Integration:**
        *   [x] Resolved Pinia initialization issues with manual plugin setup
        *   [x] Created `app/plugins/pinia.client.ts` for proper Pinia initialization
        *   [x] Fixed module conflicts by removing `@pinia/nuxt` and handling initialization manually
        *   [x] Ensured all stores work correctly with new architecture
    *   [x] **Build and Development:**
        *   [x] Verified successful build process with new structure
        *   [x] Confirmed development server works correctly
        *   [x] Tested all existing functionality with new framework
        *   [x] Resolved all TypeScript and import issues

*   **Benefits Achieved:**
    *   [x] **Better Performance:** pnpm provides faster installs and better dependency resolution
    *   [x] **Modern Structure:** New Nuxt 4 directory structure for better IDE support and performance
    *   [x] **Future-Proof:** Latest Nuxt 4 with all the latest features and improvements
    *   [x] **Tauri Compatible:** All Tauri integration still works perfectly
    *   [x] **Enhanced Development:** Better TypeScript support and development experience

### Next Steps

*   **Phase 7: Frontend & Backend Polish**
    *   [ ] **UI/UX Enhancements:**
        *   [ ] Optimize existing Nuxt UI components for better performance
        *   [ ] Improve responsive design and mobile compatibility
        *   [ ] Add loading states and skeleton screens
        *   [ ] Implement better error handling and user feedback
        *   [ ] Add keyboard shortcuts for common operations
        *   [ ] Enhance accessibility features (ARIA labels, keyboard navigation)
        *   [ ] Optimize component rendering and state management
        *   [ ] Add comprehensive toast notifications system
    *   [ ] **Backend Polish & Refinement:**
        *   [ ] Implement connection pooling for LLM API calls
        *   [ ] Add caching layer for frequently accessed data
        *   [ ] Optimize file parsing for large projects
        *   [ ] Implement background processing for long-running operations
        *   [ ] Add comprehensive error handling and recovery
        *   [ ] Implement retry mechanisms for failed API calls
        *   [ ] Add performance monitoring and logging
        *   [x] Store SQLite glossary DB under OS app data directory using Tauri path resolver
        *   Deferral: Frontend batch groundwork moved to Phase 8.3 (PromptType grouping, FE-only throttle)
    *   Deferral: Backend rate limiting, translation cache, and auto-backup moved to Phase 8.0

*   **Future Expansion: Additional File Types**
    *   [x] **MapXXX.json Support:** ✅ Implemented map event parsing and translation
    *   [x] **CommonEvents.json Support:** ✅ Implemented common events parsing and translation

*   **Frontend Enhancements:**
    *   [ ] Implement responsive design for different screen sizes.
    *   [ ] Add proper loading states and transitions.
    *   [ ] Enhance the TranslationTable with features:
        *   [ ] Add row selection functionality.
        *   [ ] Implement sorting functionality.
        *   [ ] Add pagination for better performance with large datasets.
        *   [ ] Add a search/filter feature.
