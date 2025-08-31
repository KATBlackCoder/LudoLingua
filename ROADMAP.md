# LudoLingua Development Roadmap

## 🎯 Translation Database Storage Enhancement

**Goal**: Implement persistent database storage for text translations, following the established glossary pattern, to improve user experience and data reliability.

**Estimated Timeline**: 4-6 weeks
**Priority**: High
**Impact**: Major improvement to core functionality

### Phase 1: Database Architecture Refactoring ✅ FULLY COMPLETED (Week 1)
**Objective**: Establish new database module structure and manifest-based project identification

**Key Deliverables**:
- ✅ New `src-tauri/src/db/` module structure with shared state
- ✅ Migration of glossary operations to `db/glossary/` folder
- ✅ Translation operations in `db/translation/` folder with immediate DB saving
- ✅ `.ludolingua.json` manifest system for project identification
- ✅ SQLite migration for `text_units` table with manifest hash support
- ✅ Updated command handlers and state management to use new db module
- ✅ **Glossary functionality verified working correctly**

**Success Criteria** ✅ ALL MET:
- ✅ Clean separation between glossary and translation database operations
- ✅ Manifest system allows reliable project identification
- ✅ Database schema supports all TextUnit fields with project metadata
- ✅ Each translation is immediately saved to database for persistence
- ✅ Command handlers successfully updated to use new db module structure
- ✅ Backward compatibility maintained for existing functionality

### Phase 2: Core Translation Commands ✅ FULLY COMPLETED (Week 2)
**Objective**: Implement manifest-aware translation commands with database storage

**Key Deliverables** ✅ ALL COMPLETED:
- ✅ **Refactored `translate_text_unit` with database integration and dead code removal**
- ✅ **Modified `translate_text_unit` command saves to database immediately with manifest validation**
- ✅ New commands: `load_project_translations`, `save_translation_state`, `create_project_manifest`
- ✅ Updated `extract_text` merges with existing database translations using manifest (via `extract_text_with_merge`)
- ✅ Updated `load_project` creates/checks `.ludolingua.json` manifest
- ✅ Transaction support for bulk operations with manifest-based project tracking
- ✅ Smart merging logic preserves existing translations when re-extracting text
- ✅ Comprehensive error handling for database operations

**Success Criteria** ✅ ALL MET:
- ✅ `translate_text_unit` successfully refactored with database integration
- ✅ Translations persist immediately to database with manifest validation
- ✅ Dead code removed and translation code quality improved
- ✅ Manifest system enables reliable project reloading
- ✅ Batch operations work reliably with proper project context
- ✅ Error handling for database failures and manifest issues
- ✅ Backward compatibility for projects without manifests
- ✅ Single database for all projects at `~/.local/share/ml.ludolingua.blackat/ludolingua.db`

**Completion Status**: ✅ All core translation commands implemented with database persistence. Backend database system is fully operational and ready for frontend integration.

### Phase 3: Frontend Integration 🚀 READY TO START (Week 3)
**Objective**: Update frontend stores and UI to leverage manifest-aware database storage

**Key Deliverables**:
- ✅ Backend database commands ready: `extract_text_with_merge`, `load_project_translations`
- Engine store loads/saves translations with manifest-based project identification
- Translation store uses new database commands with manifest validation
- UI indicators for save status, persistence, and manifest status
- Translation recovery on app restart with project manifest detection
- "Load Previous Session" functionality for projects with `.ludolingua.json`

**Priority Tasks**:
1. **Replace `extract_text` calls with `extract_text_with_merge`** in Vue stores
2. **Update translation workflow** to use database persistence
3. **Add loading indicators** for database operations
4. **Update UI components** to show persistence status

**Success Criteria**:
- Seamless user experience with manifest-based auto-save
- Visual feedback for translation persistence and project status
- No data loss on app crashes with proper manifest handling
- Performance impact minimized with efficient manifest queries

### Phase 4: Migration & Optimization (Week 4)
**Objective**: Migrate to new architecture and optimize performance with manifest system

**Key Deliverables**:
- Data migration script for existing translations with manifest creation
- Architecture migration from old glossary structure to new `db/` modules
- Removal of `export_translated_subset_via_factory` dependency
- Dead code cleanup (unused token estimation, duplicate logic, export functions)
- Performance optimizations (manifest-based query optimization, lazy loading)

**Success Criteria**:
- All existing data preserved during migration with manifest generation
- Clean migration to new `db/` module architecture
- Significant reduction in memory usage with database-backed storage
- Improved performance for large projects with manifest caching
- Codebase simplified and maintainable with clear domain separation

### Phase 5: Testing & Documentation (Week 5-6)
**Objective**: Comprehensive testing and documentation for new architecture

**Key Deliverables**:
- Unit and integration tests for database operations with manifest support
- Tests for backward compatibility and manifest creation scenarios
- Updated architecture documentation reflecting `db/` module structure
- Documentation for `.ludolingua.json` manifest file format and usage
- User-facing documentation for new persistence features
- Performance benchmarks with manifest-based optimizations

**Success Criteria**:
- All tests passing with good coverage including manifest edge cases
- Documentation accurately reflects new database architecture
- Performance benchmarks meet requirements with manifest optimizations
- Clear migration path documented for users
- User feedback incorporated for manifest and persistence features

## 🏗️ Technical Architecture

### Database Schema
```sql
CREATE TABLE text_units (
  id INTEGER PRIMARY KEY,
  project_path TEXT NOT NULL,
  file_path TEXT NOT NULL,
  field_type TEXT NOT NULL,
  source_text TEXT NOT NULL,
  translated_text TEXT,
  status TEXT NOT NULL,
  prompt_type TEXT NOT NULL,
  source_lang TEXT NOT NULL,
  target_lang TEXT NOT NULL,
  manifest_hash TEXT, -- For project identification
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Unique constraint prevents duplicate entries per project/file/field
CREATE UNIQUE INDEX ux_text_units_project_file_field_text
  ON text_units (project_path, file_path, field_type, source_text);
```

### Module Structure
```
src-tauri/src/db/
├── state.rs (shared database state)
├── glossary/
│   ├── mod.rs
│   ├── model.rs
│   └── repo.rs (migrated from src-tauri/src/glossaries/)
└── translation/
    ├── mod.rs
    ├── model.rs (TextUnitRecord)
    ├── repo.rs (CRUD operations)
    └── manifest.rs (project manifest handling)
```

### Project Manifest (.ludolingua.json)
**Location**: Root directory of the game project

```json
{
  "schema_version": 1,
  "project_id": "unique-hash-based-on-path-and-engine",
  "project_path": "/path/to/game/project",
  "engine_type": "RpgMakerMv",
  "engine_version": "1.6.1",
  "source_language": "ja",
  "target_language": "en",
  "detection_criteria": {
    "required_files": ["package.json", "index.html"],
    "required_folders": ["data", "img"],
    "export_data_roots": ["www/data"]
  },
  "created_at": "2024-01-01T00:00:00Z",
  "last_accessed": "2024-01-01T00:00:00Z"
}
```

**Example File Structure**:
```
/path/to/game/project/
├── .ludolingua.json          ← Project manifest
├── package.json              ← Game engine files
├── index.html
├── data/                     ← Game data
├── img/                      ← Game images
└── ...                       ← Other game files
```

## 📊 Benefits & Impact

### ✅ **Current Achievements (Phases 1-2 Completed)**
- **Backend Database System**: ✅ Fully implemented with immediate translation saving
- **Manifest System**: ✅ `.ludolingua.json` files created and validated
- **Smart Merging**: ✅ Preserves existing translations when re-extracting
- **Single Database**: ✅ All projects use `~/.local/share/ml.ludolingua.blackat/ludolingua.db`
- **Backward Compatibility**: ✅ Existing projects work without modification

### User Benefits
- ✅ **No more lost work**: Translations persist between sessions with manifest-based identification
- ✅ **Resume capability**: Continue work from previous sessions with automatic project detection
- ✅ **Better reliability**: Survive app crashes and system restarts with `.ludolingua.json` tracking
- ✅ **Performance**: Better memory management for large projects with database-backed storage
- ✅ **Project portability**: Move projects between machines while maintaining translation progress

### Technical Benefits
- ✅ **Clean architecture**: Domain-driven design with clear separation between glossary and translation concerns
- ✅ **Manifest system**: Robust project identification without relying on fragile export functions
- ✅ **Project-root placement**: `.ludolingua.json` in game project root enables version control and portability
- ✅ **Maintainability**: Modular `db/` structure with shared state management
- ✅ **Scalability**: Database can handle large translation projects with efficient querying
- ✅ **Reliability**: ACID transactions for data integrity and manifest-based validation
- ✅ **Future-proofing**: Extensible manifest format for additional project metadata

## 🔄 Migration Strategy

### Backward Compatibility
- Existing projects continue to work without modification
- Projects without `.ludolingua.json` get manifest created automatically in project root on first load
- Database translations augment, don't replace, existing functionality
- Graceful fallback if database operations fail
- Clear migration path for existing users with automatic manifest generation

### Risk Mitigation
- Database operations wrapped in comprehensive error handling
- Transaction support for atomic operations with manifest validation
- Backup mechanisms for critical data and manifest files
- Incremental rollout with feature flags and rollback capabilities
- Extensive testing for manifest creation and project identification scenarios

## 📈 Success Metrics

### ✅ **Phase 1-2 Achievements**
- **Data Persistence**: ✅ 100% of translations saved to database immediately after translation
- **Project Identification**: ✅ Manifest system implemented with `.ludolingua.json` creation
- **Architecture**: ✅ Clean migration to `db/` module structure completed
- **Backward Compatibility**: ✅ All existing projects work without modification

### 🎯 **Phase 3 Targets**
- **Performance**: No degradation in translation speed with database operations
- **Reliability**: Zero data loss incidents with manifest-based recovery
- **User Experience**: Seamless integration with automatic manifest creation
- **Memory Usage**: 50% reduction for large projects with database-backed storage
