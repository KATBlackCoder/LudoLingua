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
- [x] Streamlined base prompt (`erobasic.txt`)
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

## Current Phase: Major Platform Upgrade & Polish

### 🎯 Phase 6: Comprehensive Platform Upgrade
**Goal:** Modernize the entire application with latest technologies and enhanced UI/UX

#### **6.1: Nuxt 4 Migration**
- [ ] **Framework Upgrade:**
  - [ ] Migrate from Nuxt 3 to Nuxt 4
  - [ ] Update all dependencies to latest versions
  - [ ] Test compatibility with Tauri integration
  - [ ] Update TypeScript configurations
  - [ ] Migrate to new Nuxt 4 APIs and conventions
  - [ ] Update build configuration for optimal performance

#### **6.2: PrimeVue UI Migration**
- [ ] **UI Framework Replacement:**
  - [ ] Replace Nuxt UI with [PrimeVue](https://primevue.org/) for enhanced data table capabilities
  - [ ] Install and configure PrimeVue with proper theming
  - [ ] Migrate all existing components to PrimeVue equivalents
  - [ ] Implement PrimeVue DataTable for advanced translation table features
  - [ ] Add PrimeVue components: DataTable, MultiSelect, Dropdown, ProgressBar, etc.
  - [ ] Implement responsive design with PrimeVue's mobile-first approach
  - [ ] Add PrimeVue themes and dark mode support

#### **6.3: Enhanced Data Table Features**
- [ ] **Advanced Translation Table:**
  - [ ] Implement PrimeVue DataTable with sorting, filtering, and pagination
  - [ ] Add column-specific filters (file type, status, prompt type)
  - [ ] Implement row selection and bulk operations
  - [ ] Add inline editing capabilities for manual translation corrections
  - [ ] Implement export functionality (CSV, JSON)
  - [ ] Add translation progress visualization with ProgressBar
  - [ ] Implement virtual scrolling for large datasets (2000+ text units)
  - [ ] Add search functionality across all columns
  - [ ] Implement column resizing and reordering

#### **6.4: Frontend Polish & Refinement**
- [ ] **UI/UX Enhancements:**
  - [ ] Redesign main dashboard with modern card-based layout
  - [ ] Implement advanced project statistics with charts and metrics
  - [ ] Add file type filtering and grouping in sidebar
  - [ ] Implement drag-and-drop file upload for project loading
  - [ ] Add keyboard shortcuts for common operations
  - [ ] Implement toast notifications for better user feedback
  - [ ] Add loading states and skeleton screens
  - [ ] Implement responsive design for tablet and mobile devices
  - [ ] Add accessibility features (ARIA labels, keyboard navigation)

#### **6.5: Backend Polish & Refinement**
- [ ] **Performance Optimizations:**
  - [ ] Implement connection pooling for LLM API calls
  - [ ] Add caching layer for frequently accessed data
  - [ ] Optimize file parsing for large projects
  - [ ] Implement background processing for long-running operations
  - [ ] Add comprehensive error handling and recovery
  - [ ] Implement retry mechanisms for failed API calls
  - [ ] Add performance monitoring and logging

- [ ] **Architecture Improvements:**
  - [ ] Refactor command handlers for better error handling
  - [ ] Implement proper async/await patterns throughout
  - [ ] Add input validation and sanitization
  - [ ] Implement proper resource cleanup
  - [ ] Add comprehensive logging and debugging tools
  - [ ] Optimize memory usage for large projects

#### **6.6: Translation Workflow Enhancements**
- [ ] **Advanced Translation Features:**
  - [ ] Implement batch translation with progress tracking
  - [ ] Add translation memory for consistent terminology
  - [ ] Implement undo/redo functionality
  - [ ] Add translation validation and quality checks
  - [ ] Implement save/load translation sessions
  - [ ] Add translation export/import capabilities
  - [ ] Implement translation history and versioning

### Phase 7: Large Scale Translation Support
**Goal:** Optimize translation workflow for large projects (2000+ text units)
- [ ] **Progress Management System:**
  - Real-time progress tracking and display
  - Progress bar component with current item/total
  - Translation status per file type
  - Pause/resume/cancel functionality
- [ ] **Batch Processing Options:**
  - File-by-file translation (System.json only, Items.json only, etc.)
  - Selective translation by file type
  - Background processing to keep UI responsive
  - Progress persistence in database
  - File-level progress tracking and status display
  - Natural breakpoints for error recovery and user control
- [ ] **Performance Optimizations:**
  - Memory management for large text unit sets
  - Efficient UI updates during translation
  - Error recovery and checkpoint system
  - Translation time estimation
  - HTTP connection pooling for LLM API calls
  - Batch API calls to reduce network overhead
  - File-level connection management (one connection per file type)
  - Hybrid approach: file-level connections + batch API calls

## Future Phase: Release Preparation

### Phase 8: Release Preparation
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
- **UI:** PrimeVue + Tailwind CSS
- **State:** Pinia stores
- **Validation:** Zod schemas

### Architecture
- **Engine System:** Trait-based with factory pattern
- **File Processing:** Common helper functions for consistency
- **Prompt System:** Context-aware template selection
- **Translation:** One-by-one processing with error handling
- **Progress Management:** Real-time tracking and batch processing
- **Network Optimization:** File-level connection management + batch API calls 