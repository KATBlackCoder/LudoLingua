# LudoLingua Development TODO

## Current Status
**Phase:** Phase 6 - Framework Migration  
**Last Completed:** MVP with full RPG Maker MV support (12 file types)  
**Current Focus:** Nuxt 4 + pnpm migration

## Next Priorities (Phase 6 - Framework Migration)

### 🎯 Priority 1: Nuxt 4 + pnpm Migration (Foundation)
- [ ] **Framework Upgrade:**
  - [ ] Migrate from Nuxt 3 to Nuxt 4
  - [ ] Switch from npm to pnpm for better performance
  - [ ] Update all dependencies to latest versions
  - [ ] Test compatibility with Tauri integration
  - [ ] Update TypeScript configurations
  - [ ] Migrate to new Nuxt 4 APIs and conventions
  - [ ] Update build configuration for optimal performance
  - [ ] Update package.json with pnpm-specific configurations
  - [ ] Migrate lock files and dependency management
  - [ ] Update CI/CD and build scripts for pnpm
  - [ ] Test package installation and build processes

## Upcoming Phases

### 🎯 Phase 7: Frontend & Backend Polish
**Goal:** Optimize and refine both frontend and backend for better performance and user experience

#### **7.1: Frontend Polish & Refinement**
- [ ] **UI/UX Enhancements:**
  - [ ] Optimize existing Nuxt UI components for better performance
  - [ ] Improve responsive design and mobile compatibility
  - [ ] Add loading states and skeleton screens
  - [ ] Implement better error handling and user feedback
  - [ ] Add keyboard shortcuts for common operations
  - [ ] Enhance accessibility features (ARIA labels, keyboard navigation)
  - [ ] Optimize component rendering and state management
  - [ ] Add comprehensive toast notifications system

#### **7.2: Backend Polish & Refinement**
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

### 🎯 Phase 8: Translation Workflow Enhancement
**Goal:** Implement advanced translation features and large-scale processing capabilities

#### **8.1: Translation Workflow Optimization**
- [ ] **Enhanced Translation Features:**
  - [ ] Implement batch translation with progress tracking
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
