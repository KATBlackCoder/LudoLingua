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

## Current Phase: Additional RPG Maker MV File Types

### 🎯 Priority 1: System Files (High Impact, Easy) - COMPLETED ✅
- [x] **System.json** - Game title, UI elements, currency, elements
- [x] **States.json** - Status effect names and descriptions  
- [x] **Enemies.json** - Enemy names and descriptions

### 🎯 Priority 2: Event Files (High Impact, Complex)
- [x] **CommonEvents.json** - Common events with dialogue
- [ ] **Troops.json** - Troop names and event commands

### 🎯 Priority 3: Map Files (Complex but High Value)
- [ ] **MapXXX.json** - Individual map files with event dialogue

## Next Phase: Large Scale Translation Support

### Phase 6: Translation Performance & Progress Management
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

### ✅ Phase 6.5: Smart Content Filtering & Quality Enhancement
**Goal:** Implement intelligent filtering to reduce translation noise and improve quality
- [x] **Universal Content Filtering:**
  - Extend `is_technical_content()` filtering to all file types
  - Apply smart filtering to field names, descriptions, and notes
  - Filter out technical IDs, placeholders, and non-translatable content
  - Consistent filtering across Actors, Items, Skills, Weapons, Armors, Classes, System, States, Enemies
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
- [x] **Implementation Strategy:**
  - Update `extract_text_units_for_object()` to use filtering
  - Apply filtering to all RPG Maker MV file types
  - Maintain backward compatibility with existing translations
  - Add configuration options for filtering sensitivity

## Future Phase: Polishing & Distribution

### Phase 7: Release Preparation
**Goal:** Prepare for public release with comprehensive RPG Maker MV support
- [ ] **Frontend Enhancements:**
  - File type filtering in TranslationTable
  - Translation progress per file type
  - Enhanced project statistics
- [ ] **Backend Improvements:**
  - Comprehensive error handling
  - Batch translation across file types
  - File type-specific prompt selection
- [ ] **Testing & Polish:**
  - Thorough manual testing
  - Performance optimization
  - User experience refinement

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
- **Framework:** Nuxt.js (SPA mode)
- **UI:** Nuxt UI + Tailwind CSS
- **State:** Pinia stores
- **Validation:** Zod schemas

### Architecture
- **Engine System:** Trait-based with factory pattern
- **File Processing:** Common helper functions for consistency
- **Prompt System:** Context-aware template selection
- **Translation:** One-by-one processing with error handling
- **Progress Management:** Real-time tracking and batch processing (Phase 6)
- **Network Optimization:** File-level connection management + batch API calls (Phase 6) 