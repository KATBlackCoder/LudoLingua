# LudoLingua Development Roadmap

This document outlines the planned development phases for the LudoLingua application. Each phase represents a major milestone and a concrete set of files to be created or modified, based on our architecture documents.

---

### Phase 1: Project Setup & Core Foundation (Milestone 1)

**Goal:** Establish a solid, working project structure with all necessary dependencies and a basic UI shell.

*   **Dependencies to Install:**
    *   **Frontend (npm):** `@nuxt/ui`, `pinia`.
    *   **Backend (cargo):** `tokio`, `anyhow`, `serde`, `tauri-plugin-log`, `tauri-plugin-window-state`.
*   **Tasks & Files to Create/Modify:**
    *   [ ] Initialize a new Nuxt.js project and add Tauri.
    *   [ ] **Frontend:**
        *   Configure `nuxt.config.ts` (disable SSR, add modules).
        *   Create `layouts/default.vue` with a basic structure (`<NuxtPage />`).
        *   Create placeholder pages: `pages/index.vue`, `pages/glossary.vue`, `pages/settings.vue`.
        *   Create `stores/ui.ts` for managing global UI state.
        *   Create `assets/css/main.css` for global styles.
    *   [ ] **Backend:**
        *   Set up `src/main.rs` and `src/lib.rs` with the Tauri builder.
        *   Create `src/core/mod.rs` and `src/core/error.rs`.
        *   Create an empty command structure: `src/commands/mod.rs` and `src/commands/handler.rs`.

---

### Phase 2: File Handling & Data Parsing (Milestone 2)

**Goal:** Implement the ability to open an RPG Maker project, read its data files, and display the extracted text in the UI.

*   **Dependencies to Install:**
    *   **Backend (cargo):** `tauri-plugin-dialog`.
*   **Tasks & Files to Create/Modify:**
    *   [ ] **Backend:**
        *   Create `src/core/engine.rs` to define the `Engine` trait.
        *   Create `src/models/mod.rs`, `src/models/engine.rs`, `src/models/translation.rs`.
        *   Create the engine factory: `src/engines/mod.rs` and `src/engines/factory.rs`.
        *   Implement a first engine (e.g., in `src/engines/rpg_maker_mv/engine.rs`).
        *   Implement file parsing logic (e.g., `src/engines/rpg_maker_mv/files/mod.rs`, `.../files/actors.rs`).
        *   Create `src/commands/engine.rs` with `load_project` logic.
        *   Update `src/commands/handler.rs` to expose the `load_project` command.
    *   [ ] **Frontend:**
        *   Create `stores/project.ts` to hold the project state.
        *   Create `types/engine.ts` and `types/translation.ts`.
        *   Implement a "Load Project" button in `pages/index.vue` that calls the backend command.
        *   Create an editor component `components/editor/TranslationTable.vue` to display the data from the store.

---

### Phase 3: Translation Core & UI (Milestone 3)

**Goal:** Integrate the AI translation functionality and build the primary editing interface.

*   **Dependencies to Install:**
    *   **Frontend (npm):** `zod`.
    *   **Backend (cargo):** `llm`, `tauri-plugin-store`.
*   **Tasks & Files to Create/Modify:**
    *   [ ] **Backend:**
        *   Create `prompts/` directory with example templates (e.g., `prompts/dialogue.txt`).
        *   Create `src/llm/mod.rs` and `src/llm/trait.rs`.
        *   Create the LLM factory: `src/llm/factory.rs`.
        *   Implement a first provider: `src/llm/ollama.rs`.
        *   Create `src/commands/translation.rs` with `translate_text` logic.
        *   Create `src/commands/settings.rs` with logic to save/load settings.
        *   Update `src/commands/handler.rs` to expose new commands.
    *   [ ] **Frontend:**
        *   Create `types/settings.ts`.
        *   Create `stores/settings.ts` to manage and persist settings.
        *   Build the UI in `pages/settings.vue` using components like `components/settings/LlmSettingsForm.vue`.
        *   Add a "Translate" button to the UI that calls the store action.

---

### Phase 4: Glossary / Termbase (Milestone 4)

**Goal:** Implement the glossary feature to ensure translation consistency.

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
        *   Modify the `translate_text` logic to inject glossary terms into the prompt.
    *   [ ] **Frontend:**
        *   Create `types/glossary.ts`.
        *   Create a new `stores/glossary.ts` to manage glossary state.
        *   Build the UI in `pages/glossary.vue` using components like `components/glossary/GlossaryTable.vue`.

---

### Phase 5: Project Saving & Final Touches (Milestone 5)

**Goal:** Allow the user to save their translated work back to the game files.

*   **Dependencies to Install:**
    *   **Backend (cargo):** `tauri-plugin-opener`.
*   **Tasks & Files to Create/Modify:**
    *   [ ] **Backend:**
        *   Implement the `save_project` method in the engine implementation (e.g., in `src/engines/rpg_maker_mv/engine.rs`).
        *   Add `save_project` logic to `src/commands/engine.rs`.
        *   Update `src/commands/handler.rs`.
    *   [ ] **Frontend:**
        *   Implement a "Save Project" button in the UI.
        *   Add a utility button to open the project folder.

---

### Phase 6: Polishing, Testing & Distribution (Release)

**Goal:** Prepare the application for a public release.

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