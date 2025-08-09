# LudoLingua Development TODO

## Current Status
**Phase:** Phase 7 - Frontend & Backend Polish  
**Last Completed:** Nuxt 4 + pnpm migration with new directory structure  
**Current Focus:** Frontend & Backend Polish

## Priority Organization

### 🔴 HIGH PRIORITY (Critical for Phase 7 completion)
**These tasks must be completed first to enable Phase 7 functionality**

#### Backend Core (Ollama-Only Refactor)
- [x] **LLM Architecture Simplification (Ollama-Only):**
  - [x] Remove provider abstraction layer (`src-tauri/src/core/provider.rs`, trait usage in factory)
  - [x] Simplify `src-tauri/src/llm/factory.rs` → delete/retire file and replace with direct Ollama helpers
  - [x] Keep `src-tauri/src/llm/services/ollama.rs` (no trait impl), expose `test_connection`, `translate`, `get_ollama_models`
  - [x] Simplify models: in `src-tauri/src/models/provider.rs` remove `Provider` enum; reduce `LlmConfig` to `{ model: ModelInfo, base_url?: string, temperature: f32, max_tokens: u32 }`
  - [x] Update commands to call Ollama service directly (`test_llm_connection`, `get_ollama_models`, `translate_text_unit`)
  - [x] Remove `get_available_providers` command and any frontend usage

#### Backend Polish (Phase 7 - Completed)
- [x] Shared LLM state:
  - [x] Added `src-tauri/src/llm/state.rs` with `LlmState` (lazy `OllamaService` + `Semaphore` limiter)
  - [x] Registered managed state in `src-tauri/src/lib.rs` via `.manage(LlmState::new(1))`
  - [x] Updated `#[tauri::command] translate_text_unit` to accept and forward `State<LlmState>`
- [x] Command-layer orchestration:
  - [x] Build prompts in `commands/translation.rs` using `PromptBuilder`
  - [x] Implement `translate_with_retry` with 90s timeout and backoff retries
- [x] Service simplification:
  - [x] Added low-level `OllamaService::generate(prompt)`; kept `translate` as legacy wrapper
  - [x] Verified HTTP keep-alive reuse (single "starting new connection" then steady completions)

#### Prompt & Placeholder Pipeline
- [x] **Prompt & Placeholder Pipeline:**
  - [x] Apply `replace_formatting_codes_for_translation()` during extraction in engines (`src-tauri/src/engines/rpg_maker_mv/files/common.rs`) and `restore_formatting_codes_after_translation()` during injection
  - [x] Extend regex mappings in `text_processing.rs` (consumed by `common.rs`):
    - [x] Map `%n` → `[ARG_n]`
    - [x] Handle lowercase control codes: `\v[`, `\c[`, `\n[` 
    - [x] Normalize control codes: `\.` → `[CTRL_DOT]`, `\|` → `[CTRL_WAIT]`, `\^` → `[CTRL_INSTANT]`, `\!` → `[CTRL_INPUT]` (and restore)
    - [x] Preserve original case for variables: `\V[n]` → `[VARIABLE_n]` and `\v[n]` → `[variable_n]`; restore to matching case
  - [x] Prompt loading robustness:
    - [x] Dev: read prompt templates from filesystem (`src-tauri/prompts/`)
    - [x] Prod: embed prompt templates with `include_str!` to avoid path issues
  - [x] Vocabulary filtering by `PromptType`:
    - [x] Implement header-based filtering in `PromptBuilder::filter_vocabulary_sections`
    - [x] Document per-type header whitelist and customization in `README.md`

#### Frontend Core (Ollama-Only UI)
- [x] **Frontend: remove provider selector (Ollama-only UI), adjust `OllamaConfig` type and `providerStore` accordingly (drop `provider`, `api_key`, `extra_config`)**
- [x] **Keep `src-tauri/models/ollama.json` for model list; ensure model selector consumes backend list**
- [x] **Settings page: show connection status dot and last test time; auto-test on mount**
- [x] **Housekeeping: remove unused `@pinia/nuxt` dependency; rename frontend type `ProviderConfig` → `OllamaConfig`; update invokes to `get_ollama_models`**

#### Language Catalog
- [x] **Language Catalog:**
  - [x] Add `src-tauri/models/language.json` with `{ id, label, native_name, dir, enabled }`
  - [x] Backend: load via `include_str!`, expose `get_languages()` command (return only `enabled`)
  - [x] Frontend: language store loads from `get_languages`, apply `dir` (RTL/LTR) in UI when target is RTL

### 🟡 MEDIUM PRIORITY (Important for Phase 7 quality)
Core stability tasks moved to Phase 8 (see 8.0)

#### Settings Validation & Error Handling (Frontend Display)
- [x] Gate Translate when invalid; show brief inline error
- [x] Keep manual "Test Connection" with spinner; auto-test before translate if stale

### 🟢 LOW PRIORITY (Phase 7 polish and Phase 8 preparation)
**Nice-to-haves; do if time permits**

#### Performance Optimizations
- [ ] Cache model information and project data
- [ ] Implement cache invalidation and cleanup strategies
- [ ] Add cache hit/miss metrics for performance monitoring

- [ ] **File Parsing Optimization:**
  - [ ] Optimize file parsing for large projects (based on current 2653+ text units)
  - [ ] Implement parallel file processing where possible
  - [ ] Add streaming parsing for memory efficiency
  - [ ] Optimize JSON parsing for large RPG Maker files

#### Translation Workflow Architecture
  - [ ] **New Page Structure:**
    - [x] Create `app/pages/translation.vue` as dedicated translation workspace
    - [x] Move translation logic from `index.vue` to `translation.vue`
    - [x] Keep `index.vue` as project loading/overview page
    - [x] Add navigation between project overview and translation workspace

  - [ ] **New Component Organization:**
    - [x] Create `app/components/translation/` folder for translation-specific components
    - [x] Move `TranslationTable.vue` to `app/components/translation/TranslationRaw.vue`
    - [x] Create `app/components/translation/TranslationProcess.vue`
    - [x] Create `app/components/translation/TranslationResult.vue`
    - [x] Create `app/components/translation/TranslationEditor.vue`
    - [x] Update all imports and references

  - [ ] **Composable Architecture:**
    - [ ] Create `useTranslation()` composable as single entry point for all translation functionality
    - [ ] Implement internal composables: `useTranslationWorkflow()`, `useTranslationProgress()`, `useTranslationEditor()`
    - [ ] Add convenience methods for common operations (startTranslation, saveAndContinue, etc.)
    - [ ] Ensure type safety and consistent API across all components

#### 4-Component Translation Workflow
    - [ ] **Component 1: TranslationRaw.vue (Raw Text Display)**
      - [x] Modify existing table to show only source text (remove translated text column)
      - [x] Add individual "Translate" buttons for each row
      - [x] Focus on raw text display and status management
      - [x] Remove bulk operations (moved to translation.vue page)
      - [ ] Integrate with `useTranslation()` composable

    - [ ] **Page: translation.vue (Workflow Management)**
      - [x] Move bulk operations (Translate All, Inject, Reset, Export) to page level
      - [x] Manage overall translation workflow and state
      - [ ] Integrate with `useTranslation()` composable for workflow coordination

    - [ ] **Component 2: TranslationProcess.vue (New - Process Display)**
      - [x] Create new component for real-time translation progress
      - [ ] Show AI model being used and processing status
      - [ ] Display progress bars and timing for each unit
      - [ ] Handle error states and retry options
      - [ ] Integrate with `useTranslation()` composable for progress tracking

    - [ ] **Component 3: TranslationResult.vue (New - Final Results Display)**
      - [x] Create new component to show final translation results
      - [x] Display side-by-side source text and translated text
      - [ ] Add character count and word count indicators
      - [ ] Add RPG Maker code highlighting in source text
      - [x] Add "Edit" button for each row to open TranslationEditor
      - [ ] Show translation status and confidence indicators
      - [ ] Integrate with `useTranslation()` composable for result management

    - [ ] **Component 4: TranslationEditor.vue (New - Human Editing)**
      - [x] Create new component for manual translation editing
      - [x] Implement side-by-side source and translated text display
      - [ ] Add character count and word count indicators
      - [ ] Add RPG Maker code highlighting in source text
      - [x] Implement inline editing with textarea controls
      - [x] Add Save/Cancel options for translations
      - [ ] Integrate with `useTranslation()` composable for editor operations

#### Backend Commands (Phase 7 polish → Phase 8 groundwork)
- [ ] Add `translate_game_data_file` backend command to translate units file-by-file using shared `LlmState` and retry/timeout logic, and emit per-unit progress (to be consumed by FE progress view)

#### Enhanced Settings Organization
    - [ ] Consolidate settings to a single source of truth
      - [ ] Make `useSettingsStore` the only module that reads/writes Tauri Store (`user_settings` key)
      - [ ] Remove `provider_settings` key and `useProviderStore.loadProviderSettings()`
      - [ ] Consumers use `settingsStore.providerConfig` instead of `providerStore.currentProviderConfig`
      - [ ] Keep `useLanguageStore` for options fetch; stop direct reads from Tauri Store
      - [ ] Optional migration: if `provider_settings` exists, merge into `user_settings` on load
    - [ ] Categorize settings into AI Configuration, Translation Settings, Project Settings, UI Preferences
    - [ ] Implement auto-save for settings changes
    - [ ] Add settings backup/restore functionality
    - [ ] Create settings export/import for sharing configurations

#### Advanced Configuration Options
    - [ ] Add translation quality settings (style preferences, character limits, confidence thresholds)
    - [ ] Implement performance settings (timeouts, retry limits, cache settings)
  - [ ] Add batch processing preferences (Phase 8 feature; no background processing options in Phase 7)

#### User Experience Improvements
    - [ ] Implement progressive disclosure (basic/advanced settings)
    - [ ] Add contextual help with tooltips and explanations
    - [ ] Create expert mode toggle for power users
    - [ ] Add best practice recommendations and example configurations

#### UI/UX Foundation
  - [ ] Optimize existing Nuxt UI components for better performance
  - [ ] Improve responsive design and mobile compatibility
  - [ ] Add loading states and skeleton screens
  - [ ] Implement better error handling and user feedback
  - [ ] Add comprehensive toast notifications system
  - [ ] Optimize component rendering and state management

#### Project Management UX
  - [ ] Add page-level progress indicators for bulk operations (in translation.vue)
  - [ ] Implement drag-and-drop for project loading using Nuxt UI FileUpload component in ProjectLoader.vue

#### Data Integrity
  - [ ] Add checksums for project files to detect corruption
  - [ ] Implement auto-save functionality
  - [ ] Add data migration tools for future updates
  - [ ] Implement proper async/await patterns throughout

#### Architecture Improvements
  - [ ] Refactor command handlers for better error handling
  - [ ] Add input validation and sanitization
  - [ ] Add comprehensive logging and debugging tools
  - [ ] Implement proper resource cleanup

#### Batch Processing Groundwork (Phase 7)
Moved to Phase 8.3

#### Advanced Features (Phase 7.3)
- [ ] **Translation Quality Assurance:**
  - [ ] Implement consistency checking across the entire project
  - [ ] Add terminology validation against the glossary
  - [ ] Create translation quality scoring
  - [ ] Add suggestions for improved translations
  - [ ] Implement context-aware translation (considering surrounding text)

- [ ] **Advanced Translation Editor Features:**
  - [ ] Create a "translation memory" feature showing similar previous translations
  - [ ] Add visual feedback for translation confidence scores
  - [ ] Implement fuzzy matching for translation suggestions
  - [ ] Add AI-powered translation quality assessment

- [ ] **Collaboration Features:**
  - [ ] Add export/import for translation glossaries
  - [ ] Implement translation memory sharing
  - [ ] Create project templates for different RPG Maker versions
  - [ ] Add version control for translation history

- [ ] **AI Enhancement:**
  - [ ] Add style consistency across different text types
  - [ ] Create custom prompt templates for different content types
  - [ ] Add translation confidence scoring
  - [ ] Implement batch translation with progress tracking
  - [ ] Add translation memory for consistent terminology

## Execution Order for Phase 7

**Backend first: Ollama-only refactor (remove provider abstraction, rename command to `get_ollama_models`, simplify `LlmConfig`, robust prompt packaging)**
**Frontend next: settings UI to Ollama-only (remove provider selector, keep model selector + connection tester), update invokes**
**Then: batch groundwork — moved to Phase 8.3**
**Deferral:** Rate limiting, cache, and auto-backup moved to Phase 8.0; frontend batch groundwork moved to Phase 8.3

## Completed Phases

### ✅ Phase 6: Framework Migration - COMPLETED

### 🎯 Priority 1: Nuxt 4 + pnpm Migration (Foundation) ✅ COMPLETED
- [x] **Framework Upgrade:**
  - [x] Migrate from Nuxt 3 to Nuxt 4
  - [x] Switch from npm to pnpm for better performance
  - [x] Update all dependencies to latest versions
  - [x] Test compatibility with Tauri integration
  - [x] Update TypeScript configurations
  - [x] Migrate to new Nuxt 4 APIs and conventions
  - [x] Update build configuration for optimal performance
  - [x] Update package.json with pnpm-specific configurations
  - [x] Migrate lock files and dependency management
  - [x] Update CI/CD and build scripts for pnpm
  - [x] Test package installation and build processes

## Upcoming Phases

#### 8.0: Backend Stability Improvements (Deferred from Phase 7)
- [ ] Backend LLM Rate Limiting
  - [ ] Add backend rate limiter for LLM calls (target 1–3 rps) with small jitter
  - [ ] Enforce regardless of UI to prevent overload
- [ ] Backend Translation Cache
  - [ ] In-memory LRU/TTL cache for MT results
    - Key: `source_text|prompt_type|src_lang|tgt_lang|model|PROMPT_VERSION`
    - Caps: entry cap + max bytes; TTL 24h; TTI 1h; skip huge entries
- [ ] Auto-backup before Inject
  - [ ] Timestamped backup per affected file prior to write (backend)
 - [ ] Frontend/Backend command contract normalization
   - [ ] Normalize all Tauri command argument names to snake_case in FE invokes to match Rust params
   - [ ] Add brief contract smoke test (load/extract/translate/inject)

### 🎯 Phase 8: Translation Workflow Enhancement
**Goal:** Implement advanced translation features and large-scale processing capabilities

#### 8.1: Translation Workflow Optimization
- [ ] **Enhanced Translation Features:**
  - [ ] Implement batch translation with progress tracking
  - [ ] Add small client-side concurrency (e.g., 2) with queueing to align with backend `LlmState` limiter
  - [ ] Add translation memory for consistent terminology
  - [ ] Implement undo/redo functionality
  - [ ] Add translation validation and quality checks
  - [ ] Implement save/load translation sessions
  - [ ] Add translation export/import capabilities
  - [ ] Implement translation history and versioning
  - [ ] Optimize prompt system for better translation quality
  - [ ] Add file-level progress tracking and status display
  - [ ] Implement natural breakpoints for error recovery and user control

#### 8.2: Large Scale Translation Support
- [ ] **Progress Management System:**
  - [ ] Real-time progress tracking and display
  - [ ] Progress bar component with current item/total
  - [ ] Translation status per file type
  - [ ] Pause/resume/cancel functionality
  - [ ] Define Tauri event channel and payload schema for per-unit progress (e.g., `translation:progress`)
  - [ ] Throttle server-side emits and debounce FE consumption to reduce UI thrash
- [ ] **Batch Processing Options:**
  - [ ] File-by-file translation (System.json only, Items.json only, etc.)
  - [ ] Selective translation by file type
  - [ ] Background processing to keep UI responsive
  - [ ] Progress persistence in database
  - [ ] File-level progress tracking and status display
  - [ ] Natural breakpoints for error recovery and user control
- [ ] **Performance Optimizations:**
  - [ ] Memory management for large text unit sets
  - [ ] Efficient UI updates during translation
  - [ ] Error recovery and checkpoint system
  - [ ] Translation time estimation
  - [ ] HTTP connection pooling for LLM API calls
  - [ ] Batch API calls to reduce network overhead
  - [ ] File-level connection management (one connection per file type)
  - [ ] Hybrid approach: file-level connections + batch API calls

#### 8.3: Batch Processing Groundwork (Frontend UX)
- [ ] Batch processing with `PromptType` groups in FE
  - [ ] Group by `PromptType` for progress/UX
  - [ ] Use appropriate prompt templates per batch type
  - [ ] Optional UI-side throttle until backend rate limiting (8.0)

#### 8.4: Model Catalog & Limits (Metadata-driven UX)
- [ ] Expand `ModelInfo` to include metadata
  - [ ] `description: string`
  - [ ] `context_window_tokens: number` (model max context)
  - [ ] `recommended_max_output_tokens: number`
  - [ ] `params_billion: number` and `quantization: string`
  - [ ] `tags: string[]`, `homepage?: string`
- [ ] Update `src-tauri/models/ollama.json` to include the new fields
- [ ] Backend: parse and return enriched `ModelInfo` in `get_ollama_models`
- [ ] Backend: clamp `max_tokens` to `recommended_max_output_tokens` and warn when exceeded
- [ ] Frontend: update `ModelSelector` to display description, context window, params/quant
- [ ] Frontend: cap the output length control based on model metadata
- [ ] Frontend: show estimated token usage (instructions + source + output budget)

## Completed Achievements (MVP)

### ✅ Phase 1-5: Complete MVP
- [x] **Core Architecture:** Rust + Tauri backend, Nuxt.js frontend
- [x] **RPG Maker MV Support:** All 12 file types implemented
  - [x] Actors.json, Items.json, Skills.json, Weapons.json, Armors.json, Classes.json
  - [x] System.json, States.json, Enemies.json, CommonEvents.json, Troops.json, MapXXX.json
- [x] **AI Translation:** Ollama integration with context-aware prompts
- [x] **Smart Content Filtering:** Technical content filtering for quality
- [x] **Professional UI:** Nuxt UI components with responsive design
- [x] **Settings Management:** Persistent configuration with Tauri Store
- [x] **Translation Workflow:** Complete extract → translate → inject pipeline

## Quick Reference

### Architecture
- **Backend:** Rust + Tauri with trait-based engine system
- **Frontend:** Nuxt.js + Pinia stores (migrating to Nuxt 4 + pnpm)
- **LLM:** Ollama integration with JSON model configs
- **Storage:** Tauri Store plugin for settings

### File Structure
```
src-tauri/src/engines/rpg_maker_mv/files/
├── actors.rs ✅
├── items.rs ✅
├── skills.rs ✅
├── weapons.rs ✅
├── armors.rs ✅
├── classes.rs ✅
├── system.rs ✅
├── states.rs ✅
├── enemies.rs ✅
├── common_events.rs ✅
├── troops.rs ✅
└── maps.rs ✅
```

### Implementation Pattern
Each file follows the same pattern:
1. Define structs matching JSON structure
2. Implement `extract_text()` function using common helpers
3. Implement `inject_translations()` function using common helpers
4. Add to engine file detection
5. Use appropriate prompt types (State, System, Character, etc.)
6. Test with sample project

### Prompt System
- **Base:** `basic.txt` (streamlined core guidelines)
- **Specialized:** `character.txt`, `equipment.txt`, `skill.txt`, `dialogue.txt`, `state.txt`, etc.
- **Vocabulary:** `vocabularies.txt` (game-specific terms)
- **Features:** Direct translation mappings, flexible guidelines, aggressive output formatting

## Key Decisions

### ✅ Completed Optimizations
- **Prompt System:** Consolidated equipment prompts, added specific mappings
- **UI/UX:** Load New Project button, Reset functionality, improved workflow
- **Architecture:** Common helper functions, ~70% code reduction
- **LLM:** Best-of-breed approach with ollama-rs crate
- **Content Filtering:** Smart filtering to reduce translation noise

### 🔄 Current Approach
- **Platform Upgrade:** Nuxt 4 + pnpm migration for enhanced performance
- **Performance Focus:** Backend optimizations and translation workflow improvements
- **User Experience:** Polish existing Nuxt UI components and add advanced features
- **Scalability:** Large-scale translation support with batch processing

## Future Considerations
- **RPG Maker MZ Support** - After platform upgrade completion
- **Additional Engines** - Wolf RPG, etc.
- **Advanced Features** - Batch translation per file type, filtering UI
- **Distribution** - Cross-platform builds and auto-update system
