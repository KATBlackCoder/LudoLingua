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
- [x] **Phase 1: Actors.json Parsing (Initial Focus)**:
    - [x] **Note:** The RPG Maker MV project contains a `www/data` directory where all translatable `.json` files are located.
    - [x] Create `src/engines/rpg_maker_mv/files/mod.rs`.
    - [x] Create `src/engines/rpg_maker_mv/files/actors.rs`:
        - [x] Define structs that match the `Actors.json` format using `serde`.
        - [x] Write an `extract_text(json_content: &str) -> Result<Vec<TextUnit>>` function.
        - [x] Focus on extracting actor names, nicknames, profiles, and other translatable text.
    - [x] Update `extract_text_units` in `src/engines/rpg_maker_mv/engine.rs` to only parse `Actors.json` for now.
    - [x] Test the implementation with a sample RPG Maker MV project.
- [ ] **Phase 2: Other Files (After Actors.json is Working)**:
    - [ ] Implement parsers for other data files (e.g., `Items.json`, `Skills.json`).
    - [ ] Create parsers for event-based files (e.g., `MapXXX.json`, `CommonEvents.json`).
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

# Next Tasks

## Frontend Refactoring
- [x] Refactor index page to use Nuxt UI components
- [x] Improve project loading experience with better feedback
- [x] Create reusable components for common UI patterns
- [x] Remove the `hello` test command and related UI
- [x] Simplify TranslationTable to use basic UTable implementation
- [x] Move project information to a global alert component
- [ ] Implement responsive design for different screen sizes
- [ ] Add proper loading states and transitions
- [ ] Enhance the TranslationTable with features:
    - [ ] Add row selection functionality
    - [ ] Implement sorting functionality
    - [ ] Add pagination for better performance with large datasets
    - [ ] Add a search/filter feature

## Phase 2 Completion
- [ ] Implement parsers for additional game data files (Items.json, Skills.json)
- [ ] Create parsers for event-based files (MapXXX.json, CommonEvents.json)

## Phase 3 Preparation
- [ ] Implement the glossary page UI
- [ ] Create settings page with LLM configuration options
- [ ] Prepare for translation functionality integration
