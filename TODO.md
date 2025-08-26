# LudoLingua TODO List

## 🔍 **Deep Analysis Summary**
**Current App State:**
- ✅ Working translation app for RPG Maker MV/MZ + Wolf RPG
- ✅ Multi-AI provider support (Ollama, OpenAI, etc.)
- ✅ SQLite database with glossary system
- ❌ **Critical Issues:** No progress preservation, memory limitations for large projects

**What We're Changing:**
1. **Database Architecture**: `glossaries/` → `db/` (shared state + glossaries + translation modules)
2. **Progress Preservation**: Store extracted texts in DB instead of memory
3. **Engine Modification**: All engines store texts directly in database during extraction
4. **UI Enhancement**: Progress bars, resume dialogs, statistics tracking
5. **Migration System**: Add 0003 for translation progress tables
6. **Documentation Updates**: Update BACKEND_STRUCTURE.md and FRONTEND_STRUCTURE.md with DRY/SOLID principles

**Impact:** Users can now resume translations after app crashes, handle larger projects, and track progress in real-time.

**Project Isolation Benefits:**
- ✅ Each project has its own translation session (no confusion between projects)
- ✅ `.ludolingua.json` files with unique IDs (robust even if folders move)
- ✅ Clean database separation by project ID (not fragile paths)
- ✅ Multiple projects can be worked on independently
- ✅ Easy project cleanup (delete session = remove all project data)
- ✅ Project-specific progress tracking and metadata storage

## 🏗️ **Directory Structure - Progress Preservation System**

### **📁 New Database Architecture**
```
src-tauri/src/
├── db/                          # 🆕 NEW: Database layer
│   ├── mod.rs                  # Module exports
│   ├── state.rs                # Database connection & setup
│   ├── translation/            # Progress tracking
│   │   ├── mod.rs
│   │   ├── models.rs           # TranslationSession, TranslationUnit, ProgressMetadata
│   │   └── repo.rs             # CRUD operations for progress
│   └── glossaries/             # Terms management (moved from glossaries/)
│       ├── mod.rs
│       ├── models.rs           # GlossaryTerm, etc.
│       └── repo.rs             # CRUD operations for terms
├── engines/                    # Modified for database storage
│   ├── rpg_maker_mv/
│   │   ├── mod.rs
│   │   ├── engine.rs           # ← MODIFIED: Store texts in DB
│   │   └── files/              # RPG Maker MV specific files
│   │       ├── actors.rs
│   │       ├── classes.rs
│   │       └── ...
│   ├── rpg_maker_mz/
│   │   ├── mod.rs
│   │   ├── engine.rs           # ← MODIFIED: Store texts in DB
│   │   └── ...                 # RPG Maker MZ specific files
│   └── wolf_rpg/
│       ├── mod.rs
│       ├── engine.rs           # ← MODIFIED: Store texts in DB
│       └── files/              # Wolf RPG specific files
│           ├── database.rs
│           └── ...
├── commands/                   # Add progress commands
│   ├── mod.rs
│   ├── handler.rs              # ← MODIFIED: Add progress commands
│   ├── translation.rs          # ← MODIFIED: Integrate progress
│   └── ...
├── glossaries/                 # ← DELETE: Move to db/glossaries/
├── core/                       # Update for database integration
│   ├── mod.rs                  # ← MODIFIED: Add db module exports
│   ├── engine.rs               # ← MODIFIED: Update Engine trait for database
│   └── ...
└── lib.rs                      # Update module exports
```

### **📁 Project File Structure**
```
MyGame/
├── data/                      # Game data files
├── js/                        # JavaScript files
├── index.html                 # Main game file
└── .ludolingua.json           # 🆕 Project metadata
    {
      "project_id": "uuid-12345678-90ab-cdef-1234-567890abcdef",
      "project_name": "My Awesome Game",
      "engine_type": "RPGMakerMV",
      "created_at": 1704067200,
      "ludolingua_version": "1.0.0"
    }

# SQLite Database Location (OS App Data Directory)
# 📁 Linux: ~/.local/share/ludolingua/ludolingua.db
# 📁 macOS: ~/Library/Application Support/ludolingua/ludolingua.db
# 📁 Windows: %APPDATA%\ludolingua\ludolingua.db
#
# Current: Glossary terms (via src/glossaries/)
# Future: Translation sessions & progress (after migration)
```

### **📁 Database Schema Files**
```
src-tauri/migrations/
├── 0001_create_glossary_terms.sql    # Existing
├── 0002_glossary_unique_index.sql    # Existing
└── 0003_translation_progress.sql     # 🆕 NEW
```

### **📁 Frontend Structure** (Minor Updates)
```
app/
├── stores/
│   ├── project.ts              # ← MODIFIED: Add progress tracking
│   └── translate.ts            # ← MODIFIED: Integrate with backend progress
└── components/
    ├── editor/
    │   └── ProjectStats.vue    # ← MODIFIED: Show progress stats
    └── translation/
        ├── TranslationEditor.vue
        └── TranslationResult.vue
```

## 🚀 Immediate Tasks (Next Sprint) - Organized by Finishable Steps

### Progress Preservation System - PHASE 1 START

#### **Step 1: Architecture Foundation** ✅ (Day 1)
- [ ] **Architecture Documentation Update**: Update BACKEND_STRUCTURE.md for new `db/` structure with DRY/SOLID principles
- [ ] **Database Structure Creation**: Create `db/` directory structure (`db/mod.rs`, `db/state.rs`, `db/glossaries/`, `db/translation/`)

#### **Step 2: Database Schema** ✅ (Day 1-2)
- [ ] **Project ID System**: Create `.ludolingua.json` files with unique project IDs in project roots
- [ ] **SQL Migration 0003**: Create translation_sessions, translation_units, translation_progress tables (use project_id instead of project_path)
- [ ] **Project Detection**: Implement logic to read `.ludolingua.json` and detect existing sessions
- [ ] **Update Existing Glossary**: Move glossary_terms to use new `db/glossaries/` structure

#### **Step 3: Data Models** ✅ (Day 2)

**📋 Current `models/` Analysis:**
- **translation.rs** (4.1KB, 115 lines): Contains `TextUnit`, `TranslationStatus`, `PromptType`
- **engine.rs** (3.3KB, 100 lines): Contains `EngineInfo`, `EngineType`, `EngineCriteria`
- **provider.rs** (1.5KB, 46 lines): Contains `ModelInfo`, `LlmConfig`, `TokenPricing`
- **language.rs** (817B, 21 lines): Contains `Language` model
- **mod.rs** (73B, 5 lines): Module exports

**🔄 Required New Models (Keep in `src/models/`):**
- **TranslationSession**: Project-level metadata with UUID tracking
- **TranslationProgress**: Progress tracking with completion statistics
- **GlossaryTerm**: Move from `src/glossaries/model.rs` to `src/models/glossary.rs`

**🎯 Clean Architecture Approach:**
```rust
// Keep in src/models/translation.rs (NOT in db/)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationSession {
    pub project_id: String,           // Primary key (UUID from .ludolingua.json)
    pub project_path: String,         // For display purposes
    pub engine_type: EngineType,      // RPGMakerMV, MZ, WolfRPG
    pub source_language: Language,
    pub target_language: Language,
    pub created_at: i64,              // Unix timestamp
    pub last_modified: i64,           // Unix timestamp
    pub total_texts: i32,             // Total text units to translate
    pub completed_texts: i32,         // Completed translations
    pub status: TranslationStatus,    // 'active', 'completed', 'paused'
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationProgress {
    pub project_id: String,           // Primary key
    pub last_processed_file: Option<String>,
    pub last_processed_key: Option<String>,
    pub completed_count: i32,
    pub error_count: i32,
    pub last_save_at: i64,
}

// Existing TextUnit gets database fields when needed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextUnit {
    // ... existing fields ...
    pub db_id: Option<i64>,           // Database primary key (when stored)
    pub project_id: Option<String>,   // Foreign key to sessions (when stored)
    pub file_path: Option<String>,    // Relative path in project (when stored)
    pub created_at: Option<i64>,      // Unix timestamp (when stored)
    pub updated_at: Option<i64>,      // Unix timestamp (when stored)
}
```

**🏗️ Architecture Benefits:**
- **Models remain reusable** across UI, commands, engines, and database
- **Database layer imports** models from `src/models/` (not defines its own)
- **Clean separation** between data structures and persistence logic
- **Single source of truth** for data models

- [ ] **TranslationSession Model**: Implement struct in `src/models/translation.rs`
- [ ] **TranslationProgress Model**: Implement struct in `src/models/translation.rs`
- [ ] **Update TextUnit Model**: Add optional database fields to existing struct

#### **Step 4: Repository Pattern** ✅ (Day 2-3)
- [ ] **TranslationProgressRepository**: Implement basic CRUD operations in `db/translation/repo.rs`
- [ ] **Repository Integration**: Connect repository to shared `db/state.rs` connection
- [ ] **Move Glossary Repository**: Migrate existing glossary logic to `db/glossaries/repo.rs` (models stay in src/models/)
- [ ] **Update Prompt Builder**: Fix imports in `utils/prompts/builder.rs` to use new `db/glossaries/` path

#### **Step 5: Engine Integration - RPG Maker MV** ✅ (Day 3)
- [ ] **Extract Text Modification**: Update `rpg_maker_mv/engine.rs` to store texts in database
- [ ] **Database Session Creation**: Create translation session when project loads
- [ ] **Text Storage**: Store extracted texts directly in `translation_units` table

#### **Step 6: Factory Rework** ✅ (Day 4)

**📋 Current `factory.rs` Issues:**
- **No Database Context**: Engines created without database connection
- **In-Memory Export**: `export_translated_subset_via_factory()` works with memory arrays
- **No Session Management**: No project tracking or progress state
- **No Project ID**: Cannot associate operations with specific projects

**🔄 Required Changes:**
- **Database Integration**: Pass `DbState` to engine creation
- **Project ID System**: Generate/read `.ludolingua.json` for project identification
- **Session Management**: Create translation sessions when loading projects
- **Database Export**: Query translation data from database instead of memory
- **Progress Queries**: Add functions to get progress statistics by project

**🎯 New Factory Functions:**
```rust
// OLD: Simple engine creation
pub fn get_engine(project_path: &Path) -> AppResult<Box<dyn Engine>>

// NEW: Engine with database context
pub fn get_engine_with_context(
    project_path: &Path,
    db_state: &DbState
) -> AppResult<(Box<dyn Engine>, String)> // Returns (engine, project_id)

// NEW: Create project session
pub fn create_project_session(
    project_path: &Path,
    db_state: &DbState
) -> AppResult<String> // Returns project_id

// NEW: Database-backed export
pub fn export_translated_subset_via_factory_db(
    project_id: &str,
    db_state: &DbState,
    destination_root: &Path,
) -> AppResult<PathBuf>
```

- [ ] **Progress Factory Pattern**: Adapt `factory.rs` export pattern for progress preservation
- [ ] **Session Management**: Integrate session creation with engine factory
- [ ] **Progress Queries**: Add database queries for progress statistics

#### **Step 7: Backend Commands** ✅ (Day 4-5)

**📋 Current `commands/` Issues:**
- **No Database Context**: Commands work with in-memory data, no persistence
- **Project Path Dependency**: Uses file paths instead of stable project IDs
- **No Session Tracking**: Cannot resume translations across app restarts
- **No Progress Metadata**: No statistics, completion tracking, or error recovery
- **In-Memory Export**: Export works with memory arrays, not database state

**🔄 Required Changes:**
- **Database Integration**: All commands need `DbState` parameter
- **Project ID Migration**: Replace path-based with ID-based operations
- **Session Persistence**: Commands create/load translation sessions automatically
- **Progress Metadata**: Track completion, errors, last position, timestamps
- **Auto-Save Hooks**: Automatic progress saves during translation workflow

**📁 Files Requiring Updates:**

**🔧 `handler.rs` (6.0KB, 195 lines):**
- Add 6 new progress commands
- Modify existing commands to accept `DbState`
- Replace path-based operations with project ID-based operations

**🔧 `translation.rs` (12KB, 277 lines):**
- Modify `translate_text_unit()` to track progress in database
- Add auto-save hooks after each translation
- Update to work with database-stored text units instead of memory

**🔧 `engine.rs` (10KB, 277 lines):**
- Modify `load_project()` to create/check project sessions
- Update `extract_text()` to store in database instead of returning vectors
- Update export functions to query database instead of using memory

**🔧 `glossary.rs` (981B, 26 lines):**
- Move from `src/glossaries/` to `src/db/glossaries/` integration
- Update imports to use new database layer
- Ensure compatibility with new database structure

**🔧 `provider.rs` (2.1KB, 49 lines):**
- **No changes needed** - Provider logic remains the same
- Connection testing and model loading unaffected

**🔧 `languages.rs` (490B, 15 lines):**
- **No changes needed** - Static language catalog unaffected

**🔧 `mod.rs` (399B, 14 lines):**
- Update module exports for new database layer
- Add exports for new progress preservation modules

---

**🔧 Core Directory Updates:**

**🔧 `core/engine.rs` (1.2KB, 36 lines) - CRITICAL TRAIT CHANGES:**
- **Update Engine Trait**: Modify trait methods to work with database
- **Add Project ID Parameter**: All methods need `project_id` for database operations
- **Change Return Types**: Methods should store in DB instead of returning data

**Current Engine Trait:**
```rust
pub trait Engine: Send + Sync {
    fn extract_text_units(&self, project_info: &EngineInfo) -> AppResult<Vec<TextUnit>>;
    fn inject_text_units(&self, project_info: &EngineInfo, text_units: &[TextUnit]) -> AppResult<()>;
}
```

**New Engine Trait:**
```rust
#[async_trait]
pub trait Engine: Send + Sync {
    async fn extract_text_units(&self, project_id: &str, db_state: &DbState) -> AppResult<()>;
    async fn inject_text_units(&self, project_id: &str, db_state: &DbState) -> AppResult<()>;
}
```

**🔧 `core/error.rs` (1.3KB, 39 lines):**
- **No changes needed** - Already has `Database(String)` error variant
- Comprehensive error handling already in place

**🔧 `core/provider.rs` (2.0KB, 61 lines):**
- **Minor updates** - May need progress tracking in `LlmService` trait
- Add optional progress callbacks to generation methods

**🔧 `core/mod.rs` (231B, 8 lines):**
- **Update exports** - Add new `db` module to exports
- Ensure new database modules are accessible throughout the application

**🎯 New Commands to Add:**
```rust
// NEW: Project session management
#[tauri::command]
pub async fn create_project_session(
    project_path: String,
    db_state: State<'_, DbState>
) -> Result<String, String> // Returns project_id

#[tauri::command]
pub async fn load_project_with_progress(
    project_path: String,
    source_language: Language,
    target_language: Language,
    db_state: State<'_, DbState>
) -> Result<EngineInfo, String> // Auto-detects/creates session

// NEW: Resume functionality
#[tauri::command]
pub async fn resume_project_translation(
    project_id: String,
    db_state: State<'_, DbState>
) -> Result<TranslationProgress, String>

// NEW: Progress statistics
#[tauri::command]
pub async fn get_project_progress_stats(
    project_id: String,
    db_state: State<'_, DbState>
) -> Result<ProgressStats, String>

// NEW: Auto-save progress
#[tauri::command]
pub async fn save_translation_progress(
    project_id: String,
    progress: ProgressData,
    db_state: State<'_, DbState>
) -> Result<(), String>

// NEW: Database-backed export
#[tauri::command]
pub async fn export_translated_subset_from_db(
    project_id: String,
    destination_root: String,
    db_state: State<'_, DbState>
) -> Result<String, String>
```

**🔧 Existing Commands to Modify:**
```rust
// OLD: Path-based, no persistence
#[tauri::command]
pub async fn extract_text(project_info: EngineInfo) -> Result<Vec<TextUnit>, String>

// NEW: Database storage, session tracking
#[tauri::command]
pub async fn extract_text_with_session(
    project_id: String,
    db_state: State<'_, DbState>
) -> Result<(), String> // Stores in database

// OLD: Single text unit translation
#[tauri::command]
pub async fn translate_text_unit(
    text_unit: TextUnit,
    config: LlmConfig
) -> Result<TranslationResult, String>

// NEW: Translation with progress tracking
#[tauri::command]
pub async fn translate_text_unit_with_progress(
    project_id: String,
    text_unit_id: String,
    config: LlmConfig,
    db_state: State<'_, DbState>
) -> Result<TranslationResult, String>
```

- [ ] **Progress Commands**: Add progress preservation commands to `handler.rs`
- [ ] **Project Detection**: Implement `detect_project_progress()` to check existing sessions
- [ ] **Resume Logic**: Add `resume_project_translation()` with position detection
- [ ] **Auto-save Integration**: Hook into translation workflow for automatic saves
- [ ] **Session Management**: Handle project-specific sessions to avoid confusion
- [ ] **Update translation.rs**: Modify `translate_text_unit()` for database progress tracking
- [ ] **Update engine.rs**: Modify `load_project()` and `extract_text()` for database storage
- [ ] **Update glossary.rs**: Migrate to new `db/glossaries/` structure
- [ ] **Update mod.rs**: Update module exports for database integration
- [ ] **Update core/engine.rs**: Modify Engine trait for database operations (CRITICAL)
- [ ] **Update core/mod.rs**: Add db module exports
- [ ] **Update core/provider.rs**: Add progress tracking to LlmService trait
- [ ] **Update models/mod.rs**: Add new model exports (translation session, progress)
- [ ] **Move Glossary Models**: Move GlossaryTerm and GlossaryQuery from src/glossaries/ to src/models/glossary.rs

#### **Step 8: UI Components** ✅ (Day 5)
- [ ] **Progress Bar Component**: Create progress indicator with completion percentage
- [ ] **Resume Dialog**: Create dialog for resuming from saved progress
- [ ] **Progress Statistics**: Add real-time progress tracking and statistics

#### **Step 9: Integration Testing** ✅ (Day 5)
- [ ] **End-to-End Test**: Load project, extract texts, verify database storage
- [ ] **Progress Test**: Simulate translation, verify auto-save functionality
- [ ] **Resume Test**: Close/restart app, verify resume from saved progress

#### **Step 10: Engine Integration - RPG Maker MZ** ✅ (Day 6 - Only if Step 5 works)
- [ ] **Extract Text Modification**: Update `rpg_maker_mz/engine.rs` to store texts in database
- [ ] **Shared Logic**: Reuse database session creation from MV implementation

#### **Step 11: Engine Integration - Wolf RPG** ✅ (Day 7 - Only if Step 5 works)
- [ ] **Extract Text Modification**: Update `wolf_rpg/engine.rs` to store texts in database
- [ ] **Test All Engines**: Verify all three engines store texts correctly

### Critical Fixes (Before Phase 1 Completion)
- [ ] Fix AI model output cleaning (remove thinking tags, handle empty responses)
- [ ] Optimize prompt templates for better AI compatibility (fix Unicode arrow issues)
- [ ] Add translation batch processing to reduce API calls
- [ ] Implement retry mechanism for failed translations
- [ ] Add translation queue management for better UX
- [ ] Create translation session management

### Build & Distribution
- [ ] Fix AppImage build issues on Linux
- [ ] Test DEB/RPM package generation
- [ ] Add environment variable handling for different platforms
- [ ] Improve error handling in build process

## 🔧 Technical Debt

### Code Quality
- [ ] Remove dead code warnings (translate_with_retry)
- [ ] Add comprehensive error handling
- [ ] Improve logging throughout the application
- [ ] Add unit tests for core functionality

### Performance Optimizations
- [ ] Optimize text processing utilities
- [ ] Improve memory usage for large projects
- [ ] Add caching for frequently used data
- [ ] Optimize database queries

### User Experience
- [ ] Add loading states and progress indicators
- [ ] Improve error messages for users
- [ ] Add keyboard shortcuts
- [ ] Implement dark mode properly

## 📋 Future Enhancements

### AI Integration
- [ ] Add support for more LLM providers
- [ ] Implement model comparison tools
- [ ] Add custom prompt templates
- [ ] Support for multiple languages simultaneously

### Project Management
- [ ] Add project templates
- [ ] Implement project versioning
- [ ] Add collaboration features
- [ ] Create project backup/restore

### Advanced Features
- [ ] Add translation memory
- [ ] Implement quality assurance checks
- [ ] Add glossary management improvements
- [ ] Support for additional game engines

## 🐛 Bug Fixes

### High Priority
- [ ] Fix RunPod connection issues
- [ ] Resolve model not found errors
- [ ] Fix UI state synchronization issues
- [ ] Improve error recovery mechanisms

### Medium Priority
- [ ] Fix file path handling on different platforms
- [ ] Improve text encoding detection
- [ ] Add proper cancellation support
- [ ] Fix memory leaks in long translation sessions

### Low Priority
- [ ] Improve UI responsiveness
- [ ] Add more detailed logging
- [ ] Enhance help documentation
- [ ] Add usage analytics (opt-in)
