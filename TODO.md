# LudoLingua Development Strategy

## Development Approach
**Strategy:** Complete all phases (1-5) using only Actors.json parsing to build a fully functional translation application. After the core application is complete and tested, we'll add support for additional RPG Maker MV file types.

This approach ensures we have a complete, working translation workflow before expanding to other file types.

---

# Phase 2: File Handling & Data Parsing

## Dependencies
- [x] **Backend (cargo):** `tauri-plugin-dialog`

## 1. Backend: Core Contracts & Models
- [x] **`src/core/engine.rs`**:
    - [x] Define a `ProjectInfo` struct to hold basic project details (name, path, engine type).
    - [x] Define the `Engine` trait with methods like `load_project_info(&self, path: &Path) -> Result<ProjectInfo>` and `extract_text_units(&self, path: &Path) -> Result<Vec<()>>`. (Placeholder `Vec<()>` for now).
- [x] **`src/models/`**:
    - [x] Create `src/models/mod.rs` and declare `engine` and `translation` modules.
    - [x] Create `src/models/engine.rs` and define a `GameDataFile` struct (e.g., with `name`, `path`).
    - [x] Create `src/models/translation.rs` and define a `TextUnit` struct (e.g., with `id`, `source`, `translation`).

## 2. Frontend: Types & Initial Store
- [x] **`types/`**:
    - [x] Create `types/engine.ts` with a `GameDataFile` interface matching the Rust struct.
    - [x] Create `types/translation.ts` with a `TextUnit` interface matching the Rust struct.
- [x] **`stores/project.ts`**:
    - [x] Create the file with a basic Pinia setup store (`useProjectStore`).
    - [x] Add state refs: `projectInfo: Ref<ProjectInfo | null>`, `textUnits: Ref<TextUnit[]>`, `isLoading: Ref<boolean>`.

## 3. Backend: Engine Implementation (RPG Maker MV)
- [x] **Engine Structure**:
    - [x] Create directory: `src/engines/rpg_maker_mv/`.
    - [x] In `src/engines/mod.rs`, declare `factory` and `rpg_maker_mv` modules.
    - [x] Create `src/engines/factory.rs` with a function `get_engine(project_path: &Path) -> Result<Box<dyn Engine>>`. For now, it can just return `RpgMakerMvEngine`.
- [x] **`rpg_maker_mv/` Implementation**:
    - [x] Create `src/engines/rpg_maker_mv/mod.rs` (declaring `engine` and `files` modules).
    - [x] Create `src/engines/rpg_maker_mv/engine.rs` with `RpgMakerMvEngine` struct and `impl Engine for RpgMakerMvEngine`.
    - [x] Implement the `load_project_info` method to read `package.json` and return project details.
    - [x] Defer implementation of `extract_text_units`.

## 4. Backend-Frontend Bridge: Tauri Commands & UI
- [x] **Tauri Plugin Setup**:
    - [x] In `src/lib.rs`, add the `.plugin(tauri_plugin_dialog::init())` to the Tauri builder.
- [x] **Tauri Commands**:
    - [x] Create `src/commands/engine.rs` with a `#[tauri::command] async fn load_project()` function.
        - [x] This function should use the dialog to let the user pick a folder.
        - [x] On selection, it should use the `EngineFactory` to get an engine and call `load_project_info`.
        - [x] It should return the `ProjectInfo` to the frontend.
    - [x] Wire up the new command in `src/commands/mod.rs` and `src/commands/handler.rs`.
- [x] **Frontend UI**:
    - [x] In `pages/index.vue`, add a "Load Project" `<UButton>`.
    - [x] The button's `@click` should call a `projectStore.loadProject()` action.
    - [x] The `loadProject` action in `stores/project.ts` should `invoke` the `load_project` command and store the result in `projectInfo`.
    - [x] Display the loaded project's name on the page (e.g., `{{ projectStore.projectInfo?.name }}`).

## 5. File Parsing & Display (The Core Loop)
- [x] **Actors.json Parsing (Current Focus)**:
    - [x] **Note:** The RPG Maker MV project contains a `www/data` directory where all translatable `.json` files are located.
    - [x] Create `src/engines/rpg_maker_mv/files/mod.rs`.
    - [x] Create `src/engines/rpg_maker_mv/files/actors.rs`:
        - [x] Define structs that match the `Actors.json` format using `serde`.
        - [x] Write an `extract_text(json_content: &str) -> Result<Vec<TextUnit>>` function.
        - [x] Focus on extracting actor names, nicknames, profiles, and other translatable text.
    - [x] Update `extract_text_units` in `src/engines/rpg_maker_mv/engine.rs` to only parse `Actors.json` for now.
    - [x] Test the implementation with a sample RPG Maker MV project.
- [x] **Frontend Display**:
    - [x] Create `components/editor/TranslationTable.vue`.
    - [x] Use `<UTable>` inside to display the `textUnits` from the `projectStore`.
    - [x] Update `loadProject` action to also call and store the result of an `extract_text` command.
    - [x] Update `pages/index.vue` to show the `TranslationTable` when `textUnits` are available.
    - [x] Create `components/editor/ProjectLoader.vue` to encapsulate project loading functionality.
    - [x] Create `components/editor/FileExplorer.vue` for file selection and display.
    - [x] Refactor `pages/index.vue` to use the new components for better organization.
    - [x] Simplify the `TranslationTable` component by removing unused functions.
    - [x] Create `components/layout/ProjectInfoAlert.vue` to display project info globally.
    - [x] Update `layouts/default.vue` to include the project info alert below the header.

# ✅ Completed: Phase 3 - Translation Core & UI

## LLM System Implementation (COMPLETED)
- [x] **Backend LLM Architecture:**
    - [x] Created LLM trait abstraction in `src/llm/trait_def.rs`
    - [x] Implemented Ollama provider in `src/llm/ollama.rs`
    - [x] Created LLM factory in `src/llm/factory.rs`
    - [x] Embedded prompt templates in `src-tauri/prompts/`
    - [x] Created translation commands in `src/commands/translation.rs`
    - [x] Created settings commands in `src/commands/settings.rs`
    - [x] Added all translation commands to handler.rs

- [x] **Frontend Translation System:**
    - [x] Created LLM types in `types/llm.ts`
    - [x] Built settings store in `stores/settings.ts`
    - [x] Created LLM settings form in `components/settings/LlmSettingsForm.vue`
    - [x] Created translation settings form in `components/settings/TranslationSettingsForm.vue`
    - [x] Built complete settings page in `pages/settings.vue`
    - [x] Added translation functions to project store
    - [x] Enhanced TranslationTable with translate buttons and status indicators

- [x] **Architecture Improvements:**
    - [x] Moved prompt templates from frontend to backend (`src-tauri/prompts/`)
    - [x] Embedded prompts in binary using `include_str!()` for better distribution
    - [x] Added context-aware prompt selection based on text type
    - [x] Implemented proper error handling and connection testing

## ✅ Completed LLM Architecture Refactoring (Best-of-Breed Approach)
- [x] **Migrated to hybrid service-specific crate approach:**
    - [x] Used dedicated `ollama-rs` crate for Ollama integration (857 stars, feature-rich)
    - [x] Created `src/llm/services/` directory for service adapters
    - [x] Implemented `src/llm/services/ollama.rs` using ollama-rs crate with full API support
    - [x] Created `LlmProvider` trait with `async_trait` for object-safe async methods
    - [x] Updated `LlmConfig` in `models/provider.rs` with proper `Default` implementation
    - [x] Implemented `TranslationLanguages` struct for comprehensive translation requests
    - [x] Created `Language` interface with code, name, and native_name fields
    - [x] Maintained factory pattern in `src/llm/factory.rs` for provider instantiation
    - [x] Removed custom HTTP client code in favor of ollama-rs optimized implementation
    - [x] Updated all frontend components to use new `Language` objects instead of strings
    - [x] Enhanced prompt template loading from `prompts/` directory with fallback handling
    - [x] Updated translation workflow to one-by-one processing (removed batch translation)
    - [x] Fixed all TypeScript interfaces to match new backend architecture

## ✅ Phase 3.6: JSON Model Configuration System - Completed

**Goal:** Replace hardcoded model definitions with external JSON configuration files for better maintainability and user customization.

*   **JSON Model Configuration Completed:**
    - [x] **Created `src-tauri/models/` directory** for provider model definitions
    - [x] **Created `src-tauri/models/ollama.json`** with available Ollama models using `ModelInfo` structure
    - [x] **Updated `src/llm/services/ollama.rs`** to load models from JSON instead of hardcoded lists
    - [x] **Added JSON parsing and validation** for model configurations with proper error handling
    - [x] **Implemented graceful fallback** to hardcoded defaults if JSON files are missing/corrupt
    - [x] **Updated provider factory** to use JSON-based model loading
    - [x] **Created template JSON files** for future providers (openai.json, anthropic.json, etc.)
    - [x] **Added logging** for model configuration loading status and error reporting
    - [x] **Enhanced maintainability** by separating configuration data from code logic

# Next Tasks

## Phase 4: Glossary / Termbase (Actors.json focus)
- [ ] Implement glossary functionality for Actors.json translations
- [ ] Create glossary management UI
- [ ] Add term consistency checking

## Frontend Enhancements (Optional)
- [ ] Implement responsive design for different screen sizes
- [ ] Add proper loading states and transitions
- [ ] Enhance the TranslationTable with features:
    - [ ] Add row selection functionality
    - [ ] Implement sorting functionality
    - [ ] Add pagination for better performance with large datasets
    - [ ] Add a search/filter feature

## Phase 4: Glossary / Termbase (Actors.json focus)
- [ ] Implement glossary functionality for Actors.json translations
- [ ] Create glossary management UI

## Phase 5: Backup-Based Project Saving & Final Touches (Actors.json focus)
- [ ] **Backend - Backup System:**
    - [ ] Create selective backup system (only translatable files like Actors.json)
    - [ ] Implement `create_project_backup()` maintaining folder structure
    - [ ] Add `get_backup_path()` for original/backup path resolution
    - [x] Rename `save_text_units` → `inject_text_units` (memory text replacement)
    - [ ] Create `save_files()` with dual modes: backup-only vs direct-to-original
    - [ ] Add backup validation and integrity checks
- [ ] **Frontend - Backup UI:**
    - [ ] Add backup status tracking (exists, working location, save mode)
    - [ ] Implement save mode selection UI with safety warnings
    - [ ] Create backup status indicators and working location display
    - [ ] Add "Save to Backup" and "Apply to Original" buttons
    - [ ] Implement backup management (create, refresh, cleanup)
- [ ] **Workflow Enhancement:**
    - [ ] Modify project loading to auto-create selective backup
    - [ ] Ensure translation work happens on backup copies
    - [ ] Add file conflict detection and resolution
- [ ] Add final polish and testing with backup-based workflow

## Future Expansion (After Core Application is Complete)
- [ ] **Additional File Types Support:**
    - [ ] Implement parsers for other data files (e.g., `Items.json`, `Skills.json`)
    - [ ] Create parsers for event-based files (e.g., `MapXXX.json`, `CommonEvents.json`)
    - [ ] Extract text from commands with specific code values:
        - [ ] `101` (Show Text - Message Window attributes)
        - [ ] `401` (Show Text - Message content lines)
        - [ ] `102` (Show Choices)
        - [ ] `402` (When [Choice] selected - Choice content)
        - [ ] `405` (Show Scrolling Text)
        - [ ] `108` (Comment)
        - [ ] `408` (Comment continuation)
        - [ ] `320` (Change Actor Name)
        - [ ] `324` (Change Actor Nickname)
        - [ ] `355` (Script)
        - [ ] `655` (Script continuation)
- [ ] **Additional Engine Support:**
    - [ ] Implement RPG Maker MZ support
    - [ ] Add support for other RPG Maker versions
