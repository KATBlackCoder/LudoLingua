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

# ✅ Completed: Phase 4 - Unified JSON File Storage (Settings + Glossary)

## ✅ Completed: Working Copy Utility Simplification

**Decision:** Simplified working copy functionality to use Tauri plugins directly from frontend
- [x] **Removed complex backend working copy utility** - All functionality now handled by Tauri plugins
- [x] **Updated `src-tauri/src/utils/working_copy.rs`** - Simplified to placeholder with plugin references
- [x] **Benefits achieved:**
    - [x] Better user experience with native system dialogs
    - [x] Simplified backend architecture (less custom code)
    - [x] Cross-platform consistency using Tauri File System and Dialog plugins
    - [x] User choice for working copy locations (Desktop, Documents, etc.)
    - [x] Familiar interface on each platform (Windows Explorer, macOS Finder, Linux file managers)

## Unified Storage System Implementation (TAURI STORE PLUGIN → CUSTOM JSON STORAGE)
- [x] **Backend Architecture Enhancement:**
    - [x] Created LLM trait abstraction and shared prompt builder
    - [x] Implemented prompt template system with glossary integration
    - [x] Created utility modules for reusable functionality
    - [x] Enhanced architecture for better separation of concerns

- [x] **Unified Storage Migration (Database + Tauri Store → Custom JSON Storage):**
    - [x] ~~Used SQLite database with sqlx for glossary~~ **REPLACED WITH JSON FILE STORAGE**
    - [x] ~~Used Tauri Store plugin for settings~~ **REPLACED WITH CUSTOM JSON STORAGE**
    - [x] ~~Created database migrations and connection pooling~~ **SIMPLIFIED TO FILE I/O**
    - [x] ~~Implemented SQL-based CRUD operations~~ **REPLACED WITH JSON OPERATIONS**
    - [x] **NEW: Unified JSON File Storage Implementation:**
        - [x] Create `storages/` directory in app root (auto-created on first run)
        - [x] Create `src/storage/` directory for file-based storage modules
        - [x] **DECISION: Use Tauri Store Plugin for Settings (COMPLETED)**
            - [x] Removed custom JSON file storage implementation
            - [x] Kept Tauri Store plugin for settings persistence
            - [x] Removed glossary storage (not needed for core functionality)
            - [x] Simplified architecture by using built-in plugin
            - [x] **Benefits:** Atomic writes, cross-platform, automatic persistence, type safety
            - [x] **Updated `stores/settings.ts`** to use Tauri Store plugin properly
            - [x] **Re-added dependencies:** `tauri-plugin-store` and `@tauri-apps/plugin-store`
            - [x] **Re-registered plugin** in `src-tauri/src/lib.rs`
            - [x] **Re-added permissions** in `src-tauri/capabilities/default.json`
            - [x] **Clean compilation** - both backend and frontend compile successfully
        - [ ] **Phase 5: Testing & Validation**
            - [ ] Test file creation and atomic writes
            - [ ] Test cross-platform compatibility (Windows/Mac/Linux)
            - [ ] Test error handling (corrupted files, missing directories)
            - [ ] Implement backup and recovery for both settings and glossary data
        - [x] Remove database dependencies and migrations
        - [x] Remove `tauri-plugin-store` dependency

- [x] **Backend Commands & Prompt Integration:**
    - [x] Created glossary management commands
    - [x] Integrated glossary terms into translation prompts
    - [x] Added language-specific term lookup functionality
    - [x] Implemented fallback to basic vocabulary when glossary is empty

- [x] **Frontend Implementation:**
    - [x] Created TypeScript interfaces for glossary terms
    - [x] Built Pinia store for glossary state management
    - [x] Implemented basic glossary UI with add/remove functionality
    - [x] Added language integration for automatic term categorization
    - [x] Created modal-based form for adding new terms
    - [x] **NEW: Frontend Store Updates for Unified Storage:**
        - [x] Update `stores/settings.ts` to use custom JSON storage commands instead of Tauri Store plugin
        - [x] Update `stores/glossary.ts` to use custom JSON storage commands (if needed)
        - [x] Remove `@tauri-apps/plugin-store` imports from frontend
        - [x] Test frontend compatibility with new backend storage commands

- [x] **Features Completed:**
    - [x] Language-specific term management (source_language → target_language)
    - [x] User customization of character names and game terms
    - [x] Integration with existing translation workflow
    - [x] Fallback system using curated vocabulary from `basic.txt`

## Issues Resolved by Unified Storage Migration:
- ✅ **No more app restarts** when saving glossary terms (database WAL files were triggering hot reload)
- ✅ **Consistent data location** - both settings and glossary in predictable `storages/` folder
- ✅ **Simplified architecture** - no database connection pooling, migrations, or plugin dependencies
- ✅ **Better development experience** - no database initialization race conditions or plugin conflicts
- ✅ **Easier debugging** - human-readable JSON files instead of binary database or app data locations
- ✅ **Portable data** - entire `storages/` folder can be backed up, versioned, or moved with app
- ✅ **Cross-platform consistency** - same data location on Windows/Mac/Linux
- ✅ **Reduced dependencies** - eliminates `sqlx`, `tauri-plugin-store`, and related complexity
- ✅ **Clean compilation** - all dependencies successfully removed and project compiles without errors

## New Storage Structure:
```
LudoLingua/
├── storages/                    # Auto-created on first run
│   ├── settings.json           # App settings (replaces Tauri Store)
│   ├── glossary.json           # Glossary terms (replaces SQLite database)
│   └── backups/                # Future: automatic backups
├── src-tauri/
├── pages/
└── ...
```

## Implementation Strategy:

### **✅ DECISION: Use Tauri Store Plugin (COMPLETED)**
- **Goal:** Use built-in Tauri Store plugin for settings
- **Benefits:** Atomic writes, cross-platform, automatic persistence, type safety
- **Status:** Removed custom storage implementation, kept plugin

### **✅ Settings Storage Complete (COMPLETED)**
- **Goal:** Use Tauri Store plugin for settings persistence
- **Status:** Fully implemented and working
- **Benefits:** Atomic writes, cross-platform, automatic persistence, type safety

### **Next Priority: Phase 5 - Backup System**
- **Goal:** Implement backup-based translation workflow
- **Files:** Engine backup methods, frontend backup UI
- **Features:** Safe file operations, user choice between backup/original
- **Priority:** Essential for safe translation workflow

### **Phase 5: Testing & Validation**
- **Goal:** Ensure reliability across all scenarios
- **Tests:** File operations, cross-platform, error handling
- **Features:** Backup/recovery, data integrity

## Frontend Enhancements (Optional)
- [ ] Implement responsive design for different screen sizes
- [ ] Add proper loading states and transitions
- [ ] Enhance the TranslationTable with features:
    - [ ] Add row selection functionality
    - [ ] Implement sorting functionality
    - [ ] Add pagination for better performance with large datasets
    - [ ] Add a search/filter feature

## ✅ Phase 4: Glossary / Termbase (Actors.json focus) - COMPLETED
- [x] Implement glossary functionality for Actors.json translations
- [x] Create glossary management UI (basic version)
- [x] Successfully migrated from SQLite database to JSON file storage
- [x] Removed all database dependencies (sqlx, sqlx-cli)
- [x] Removed Tauri Store plugin dependencies
- [x] Updated frontend stores to work with new architecture



## ✅ Phase 5: Additional File Types Support (RPG Maker MV Focus) - PARTIALLY COMPLETED

**Goal:** Expand translation support beyond Actors.json to include other RPG Maker MV data files and event-based content.

### **✅ Completed: Common Helper Functions Architecture**
- [x] **Created `src/engines/common.rs`** with reusable helper functions
- [x] **Implemented `extract_file_type_text`** for generic file extraction across engines
- [x] **Implemented `inject_file_type_translations`** for generic translation injection
- [x] **Updated RPG Maker MV engine** to use common helper functions
- [x] **Benefits achieved:**
    - [x] Eliminated code duplication between engine implementations
    - [x] Consistent error handling and logging across all file types
    - [x] Easy to add new file types (just 3 lines of code per file type)
    - [x] Reusable across different RPG Maker versions (MV, MZ, etc.)
    - [x] Type-safe generic functions with proper error handling

### **✅ Completed: Advanced Common Functions Architecture**
- [x] **Created `src/engines/rpg_maker_mv/files/common.rs`** with higher-level abstraction functions
- [x] **Implemented `extract_text_from_file_with_objects`** for generic file parsing and object iteration
- [x] **Implemented `inject_translations_into_file_with_objects`** for generic translation injection with object updates
- [x] **Refactored all file modules** (`actors.rs`, `items.rs`, `skills.rs`) to use new common functions
- [x] **Benefits achieved:**
    - [x] ~70% reduction in code duplication across file modules
    - [x] Consistent file I/O, JSON parsing, and error handling patterns
    - [x] File-specific modules now focus only on their unique data structures and field logic
    - [x] Improved maintainability and readability of all file processing code
    - [x] Eliminated boilerplate code for file existence checks, JSON parsing, and iteration

### **Priority 1: Data Files (JSON-based)**
- [x] **Items.json Support:**
    - [x] Create `src/engines/rpg_maker_mv/files/items.rs`
    - [x] Implement `Item` struct with translatable fields (name, description, note)
    - [x] Add `extract_text` and `inject_translations` functions
    - [x] Update engine to include Items.json in file detection
    - [x] Test with sample RPG Maker MV project
- [x] **Skills.json Support:**
    - [x] Create `src/engines/rpg_maker_mv/files/skills.rs`
    - [x] Implement `Skill` struct with translatable fields (name, description, message1, message2, note)
    - [x] Add `extract_text` and `inject_translations` functions
    - [x] Update engine to include Skills.json in file detection
    - [x] Test with sample RPG Maker MV project
- [x] **Weapons.json Support:**
    - [x] Create `src/engines/rpg_maker_mv/files/weapons.rs`
    - [x] Implement `Weapon` struct with translatable fields (name, description, note)
    - [x] Add `extract_text` and `inject_translations` functions
    - [x] Update engine to include Weapons.json in file detection
- [x] **Armors.json Support:**
    - [x] Create `src/engines/rpg_maker_mv/files/armors.rs`
    - [x] Implement `Armor` struct with translatable fields (name, description, note)
    - [x] Add `extract_text` and `inject_translations` functions
    - [x] Update engine to include Armors.json in file detection
- [x] **Classes.json Support:**
    - [x] Create `src/engines/rpg_maker_mv/files/classes.rs`
    - [x] Implement `Class` struct with translatable fields (name, note)
    - [x] Add `extract_text` and `inject_translations` functions
    - [x] Update engine to include Classes.json in file detection

### **Priority 2: Event Files (Map-based)**
- [ ] **MapXXX.json Support:**
    - [ ] Create `src/engines/rpg_maker_mv/files/maps.rs`
    - [ ] Implement `Map` struct with event parsing
    - [ ] Add text extraction from event commands (codes 101, 401, 102, 402, etc.)
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
    - [ ] Add `extract_text` and `inject_translations` functions
    - [ ] Update engine to scan all MapXXX.json files
- [ ] **CommonEvents.json Support:**
    - [ ] Create `src/engines/rpg_maker_mv/files/common_events.rs`
    - [ ] Implement `CommonEvent` struct with translatable fields (name, message)
    - [ ] Add text extraction from event commands
    - [ ] Add `extract_text` and `inject_translations` functions

### **Priority 3: System Files**
- [ ] **System.json Support:**
    - [ ] Create `src/engines/rpg_maker_mv/files/system.rs`
    - [ ] Implement `System` struct with translatable fields (gameTitle, currencyUnit, elements, etc.)
    - [ ] Add `extract_text` and `inject_translations` functions
- [ ] **Troops.json Support:**
    - [ ] Create `src/engines/rpg_maker_mv/files/troops.rs`
    - [ ] Implement `Troop` struct with translatable fields (name, pages with event commands)
    - [ ] Add text extraction from event commands
    - [ ] Add `extract_text` and `inject_translations` functions

### **Frontend Updates**
- [ ] **Enhanced TranslationTable:**
    - [ ] Add file type filtering (Actors, Items, Skills, etc.)
    - [ ] Add file-specific translation status indicators
    - [ ] Implement batch translation per file type
- [ ] **Project Statistics:**
    - [ ] Display translation progress per file type
    - [ ] Show total translatable text units across all files
    - [ ] Add file type breakdown in project info

### **Backend Architecture Updates**
- [ ] **Engine Enhancements:**
    - [ ] Update `extract_game_data_files` to scan all supported file types
    - [ ] Implement file type detection and routing
    - [ ] Add comprehensive error handling for missing files
- [ ] **Translation Workflow:**
    - [ ] Update translation commands to handle multiple file types
    - [ ] Add file type-specific prompt selection
    - [ ] Implement batch translation across file types

## Future Expansion (After Phase 5 Completion)
- [ ] **Additional Engine Support:**
    - [ ] Implement RPG Maker MZ support
    - [ ] Add support for other RPG Maker versions
    - [ ] Create engine detection logic for multiple RPG Maker versions
    - [ ] Add support for other game engines (Wolf RPG, etc.)
