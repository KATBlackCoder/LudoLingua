# LudoLingua Development TODO

## Current Status
**Phase:** Phase 6 - Comprehensive Platform Upgrade  
**Last Completed:** MVP with full RPG Maker MV support (12 file types)  
**Current Focus:** Nuxt 4 migration and PrimeVue UI replacement

## Next Priorities (Phase 6 - Platform Upgrade)

### 🎯 Priority 1: Nuxt 4 Migration (Foundation)
- [ ] **Framework Upgrade:**
  - [ ] Migrate from Nuxt 3 to Nuxt 4
  - [ ] Update all dependencies to latest versions
  - [ ] Test compatibility with Tauri integration
  - [ ] Update TypeScript configurations
  - [ ] Migrate to new Nuxt 4 APIs and conventions
  - [ ] Update build configuration for optimal performance

### 🎯 Priority 2: PrimeVue UI Migration (High Impact)
- [ ] **UI Framework Replacement:**
  - [ ] Replace Nuxt UI with PrimeVue for enhanced data table capabilities
  - [ ] Install and configure PrimeVue with proper theming
  - [ ] Migrate all existing components to PrimeVue equivalents
  - [ ] Implement PrimeVue DataTable for advanced translation table features
  - [ ] Add PrimeVue components: DataTable, MultiSelect, Dropdown, ProgressBar, etc.
  - [ ] Implement responsive design with PrimeVue's mobile-first approach
  - [ ] Add PrimeVue themes and dark mode support

### 🎯 Priority 3: Enhanced Data Table Features (Core Feature)
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

### 🎯 Priority 4: Frontend Polish & Refinement (UX)
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

### 🎯 Priority 5: Backend Polish & Refinement (Performance)
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

### 🎯 Priority 6: Translation Workflow Enhancements (Features)
- [ ] **Advanced Translation Features:**
  - [ ] Implement batch translation with progress tracking
  - [ ] Add translation memory for consistent terminology
  - [ ] Implement undo/redo functionality
  - [ ] Add translation validation and quality checks
  - [ ] Implement save/load translation sessions
  - [ ] Add translation export/import capabilities
  - [ ] Implement translation history and versioning

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
- **Frontend:** Nuxt.js + Pinia stores (migrating to Nuxt 4 + PrimeVue)
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
- **Platform Upgrade:** Nuxt 4 + PrimeVue migration for enhanced capabilities
- **Data Table Focus:** Advanced table features for better translation management
- **Performance:** Backend optimizations for large-scale projects
- **User Experience:** Modern UI/UX with professional-grade components

## Future Considerations
- **RPG Maker MZ Support** - After platform upgrade completion
- **Additional Engines** - Wolf RPG, etc.
- **Advanced Features** - Batch translation per file type, filtering UI
- **Distribution** - Cross-platform builds and auto-update system
