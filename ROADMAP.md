# LudoLingua Development Roadmap

## Development Strategy
**Approach:** Build a complete translation workflow with RPG Maker MV support, then expand to other engines.

## Development Phases

### ✅ Phase 1: Project Setup & Core Foundation
**Goal:** Establish solid project structure with basic UI
- [x] Nuxt.js + Tauri setup
- [x] Basic pages and components
- [x] Core backend structure

### ✅ Phase 2: File Handling & Data Parsing  
**Goal:** Load RPG Maker projects and parse Actors.json
- [x] Engine trait system
- [x] RPG Maker MV engine implementation
- [x] Actors.json parsing and display

### ✅ Phase 3: Translation Core & UI
**Goal:** AI translation functionality with complete UI
- [x] LLM integration (Ollama)
- [x] Prompt system with context-aware selection
- [x] Translation workflow and settings management

### ✅ Phase 4: Storage & Architecture
**Goal:** Robust data persistence and architecture improvements
- [x] Tauri Store plugin for settings
- [x] Common helper functions architecture
- [x] ~70% code reduction through abstraction

### ✅ Phase 5: Additional File Types Support
**Goal:** Expand beyond Actors.json to comprehensive RPG Maker MV support
- [x] **6 Core Data Files Completed:**
  - Actors.json ✅
  - Items.json ✅  
  - Skills.json ✅
  - Weapons.json ✅
  - Armors.json ✅
  - Classes.json ✅

### ✅ Phase 5.5: Prompt System Optimization
**Goal:** Improve translation quality and user experience
- [x] Streamlined base prompt (`basic.txt`)
- [x] Unified equipment prompts (`equipment.txt`)
- [x] Specific translation mappings for consistency
- [x] Enhanced output formatting
- [x] UI/UX improvements (Load New Project, Reset, etc.)

### ✅ Phase 5.6: System.json Support
**Goal:** Complete System.json file support with proper prompt handling
- [x] System.json parsing and translation extraction
- [x] Complex nested structure handling (terms object)
- [x] Proper prompt type selection (System instead of Other)
- [x] Placeholder preservation in system messages
- [x] Support for 500+ text units from System.json

### ✅ Phase 5.7: Additional RPG Maker MV File Types
**Goal:** Complete comprehensive RPG Maker MV support
- [x] **System Files (High Impact, Easy):**
  - [x] **System.json** - Game title, UI elements, currency, elements
  - [x] **States.json** - Status effect names and descriptions  
  - [x] **Enemies.json** - Enemy names and descriptions
- [x] **Event Files (High Impact, Complex):**
  - [x] **CommonEvents.json** - Common events with dialogue
  - [x] **Troops.json** - Troop names and event commands
- [x] **Map Files (Complex but High Value):**
  - [x] **MapXXX.json** - Individual map files with event dialogue

### ✅ Phase 5.8: Smart Content Filtering & Quality Enhancement
**Goal:** Implement intelligent filtering to reduce translation noise and improve quality
- [x] **Universal Content Filtering:**
  - Extend `is_technical_content()` filtering to all file types
  - Apply smart filtering to field names, descriptions, and notes
  - Filter out technical IDs, placeholders, and non-translatable content
  - Consistent filtering across all RPG Maker MV file types
- [x] **Advanced Filtering Rules:**
  - Single character names (like "ｘ") - likely placeholders
  - Numeric-only names (like "9") - likely technical IDs
  - Short technical abbreviations (HP, MP, ATK, etc.)
  - File paths and asset references
  - JavaScript code and expressions
  - Developer markers and comments
- [x] **Quality Improvements:**
  - Reduce translation noise by filtering technical content
  - Focus translation efforts on actual player-facing text
  - Improve translation consistency by removing edge cases
  - Better separation of translatable vs. technical content

## Current Phase: Frontend & Backend Polish

### ✅ Phase 6: Framework Migration - COMPLETED
**Goal:** Migrate to latest technologies and package manager

#### **6.1: Nuxt 4 + pnpm Migration ✅ COMPLETED**
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
  - [x] **Directory Structure Migration:** Automatically migrated to new Nuxt 4 `app/` directory structure
  - [x] **Pinia Integration:** Resolved Pinia initialization issues with manual plugin setup

### 🎯 Phase 7: Frontend & Backend Polish
**Goal:** Optimize and refine both frontend and backend for better performance and user experience

- Execution order:
  - [x] Backend first: Ollama-only refactor (remove provider abstraction, rename command to `get_ollama_models`, robust prompt template packaging)
  - [x] Frontend: settings UI shows connection status and last-tested, auto-tests on mount
  - [ ] Then: batch groundwork — moved to Phase 8.3
  - Deferral: Backend rate limiting, translation cache, and auto-backup moved to Phase 8.0; frontend batch groundwork moved to Phase 8.3

 #### **7.1: Backend Polish & Refinement**
  - [x] Simplify LLM to Ollama-only: remove provider abstraction in code (delete trait/factory) and call Ollama service directly
  - [x] Remove `get_available_providers` command and all references
  - [x] Rename command to `get_ollama_models` (no provider arg) and update frontend calls
  - [x] Package prompt templates robustly (embed in prod, FS in dev)
  - [ ] Integrate placeholder pre/post-processing via engines (`engines/.../files/common.rs`) using `text_processing.rs`; preserve bracket tokens end-to-end
  - [ ] Compact prompts (basic + per-type) and add hard delimiters in input to reduce leakage
  - [x] Add `models/language.json` (id, label, native_name, dir, enabled); expose `get_languages()` and use in FE store
  - [x] **Performance Optimizations:**
  - [x] Use a single shared Ollama client instance (reuse underlying HTTP client) via managed `LlmState`
  - [x] Add timeout + retry policy in command layer (`translate_with_retry`)
  - [ ] Add caching layer for frequently accessed data
  - [ ] Optimize file parsing for large projects
  - [ ] Add comprehensive error handling and recovery
  - [ ] Implement retry mechanisms for failed API calls
  - [ ] Add performance monitoring and logging
  - [ ] Batch groundwork: group by `PromptType` and translate sequentially per group; throttle (1–3 rps)

- [ ] **Architecture Improvements:**
  - [ ] Refactor command handlers for better error handling
  - [ ] Implement proper async/await patterns throughout
  - [ ] Add input validation and sanitization
  - [ ] Implement proper resource cleanup
  - [ ] Add comprehensive logging and debugging tools
  - [ ] Optimize memory usage for large projects
  
#### **7.2: Frontend Polish & Refinement**
 - [ ] Remove provider selector UI; settings are Ollama-only (keep model selector + connection tester)
 - [ ] Auto-fetch models on settings mount; show connection status dot and last-tested time
 - [ ] Housekeeping: remove unused `@pinia/nuxt` dependency; rename frontend type `ProviderConfig` → `OllamaConfig`; update invokes to `get_ollama_models`
  - [ ] Normalize FE→BE command argument naming to snake_case to match Rust (load_project, extract_text, extract_game_data_files, inject_text_units, translate_text_unit)
  - [ ] Improve prompts: adopt compact basic/per-type prompts; ensure vocabulary block injection; add input delimiters
- [ ] **UI/UX Enhancements:**
  - [ ] Optimize existing Nuxt UI components for better performance
  - [ ] Improve responsive design and mobile compatibility
  - [ ] Add loading states and skeleton screens
  - [ ] Implement better error handling and user feedback
  - [ ] Add keyboard shortcuts for common operations
  - [ ] Enhance accessibility features (ARIA labels, keyboard navigation)
  - [ ] Optimize component rendering and state management
  - [ ] Add comprehensive toast notifications system


### 🎯 Phase 8: Translation Workflow Enhancement
**Goal:** Implement advanced translation features and large-scale processing capabilities

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

#### **8.1: Translation Workflow Optimization**
- [ ] **Enhanced Translation Features:**
  - [ ] Implement batch translation with progress tracking
  - [ ] Add small FE concurrency (e.g., 2) with queueing to align with backend limiter
  - [ ] Add translation memory for consistent terminology
  - [ ] Implement undo/redo functionality
  - [ ] Add translation validation and quality checks
  - [ ] Implement save/load translation sessions
  - [ ] Add translation export/import capabilities
  - [ ] Implement translation history and versioning
  - [ ] Optimize prompt system for better translation quality
  - [ ] Add file-level progress tracking and status display
  - [ ] Implement natural breakpoints for error recovery and user control

#### **8.2: Large Scale Translation Support**
- [ ] **Progress Management System:**
  - [ ] Real-time progress tracking and display
  - [ ] Progress bar component with current item/total
  - [ ] Translation status per file type
  - [ ] Pause/resume/cancel functionality
  - [ ] Define Tauri event(s) for per-unit progress and errors; wire FE listener component (`TranslationProcess.vue`)
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

## Future Phase: Release Preparation

### Phase 9: Release Preparation
**Goal:** Prepare for public release with comprehensive RPG Maker MV support
- [ ] **Testing & Quality Assurance:**
  - [ ] Comprehensive unit and integration testing
  - [ ] Performance testing with large projects
  - [ ] Cross-platform compatibility testing
  - [ ] User acceptance testing
  - [ ] Security audit and vulnerability assessment
- [ ] **Documentation & Support:**
  - [ ] Complete user documentation and tutorials
  - [ ] Developer documentation and API reference
  - [ ] Video tutorials and walkthroughs
  - [ ] FAQ and troubleshooting guide
- [ ] **Distribution & Deployment:**
  - [ ] Automated build and release pipeline
  - [ ] Cross-platform packaging (Windows, macOS, Linux)
  - [ ] Auto-update system implementation
  - [ ] App store distribution (if applicable)

## Future Vision

### Engine Expansion
- [ ] **RPG Maker MZ Support** - Next major engine
- [ ] **Other RPG Maker Versions** - VX, VX Ace, etc.
- [ ] **Alternative Engines** - Wolf RPG, etc.

### Advanced Features
- [ ] **Glossary/Termbase** - User-defined translation mappings
- [ ] **Batch Operations** - Multi-file translation workflows
- [ ] **Translation Memory** - Reuse previous translations
- [ ] **Quality Assurance** - Translation validation tools

### Distribution
- [ ] **Packaging** - Cross-platform builds
- [ ] **Updates** - Auto-update system
- [ ] **Documentation** - User guides and tutorials
- [ ] **Community** - Plugin ecosystem

## Technology Stack

### Backend
- **Core:** Rust + Tauri
- **LLM:** Ollama integration with ollama-rs crate
- **Storage:** Tauri Store plugin
- **Parsing:** Serde for JSON handling

### Frontend  
- **Framework:** Nuxt 4 (SPA mode)
- **UI:** Nuxt UI (optimized for performance)
- **State:** Pinia stores
 - **Validation:** (none; basic form checks in components)

### Architecture
- **Engine System:** Trait-based with factory pattern
- **File Processing:** Common helper functions for consistency
- **Prompt System:** Context-aware template selection
- **Translation:** One-by-one processing with error handling
- **Progress Management:** Real-time tracking and batch processing
- **Network Optimization:** File-level connection management + batch API calls 