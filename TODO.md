# Translation Database Storage Implementation

## Phase 1: Database Architecture Refactoring ✅ FULLY COMPLETED
### New Database Module Structure ✅ COMPLETED
- [x] Create `src-tauri/src/db/` root directory
- [x] Move `glossaries/state.rs` to `src-tauri/src/db/state.rs` (shared database state)
- [x] Create `src-tauri/src/db/glossary/` folder for glossary operations
- [x] Create `src-tauri/src/db/translation/` folder for translation operations
- [x] Update all imports to use new module structure

### Backend Database Schema ✅ COMPLETED
- [x] Create `0003_create_text_units.sql` migration file
- [x] Define `text_units` table schema with all necessary fields
- [x] Add unique constraint on `(project_path, file_path, field_type, source_text)`
- [x] Add indexes for performance (status, project_path, updated_at)

### Project Manifest System (.ludolingua.json) ✅ COMPLETED
- [x] Create `.ludolingua.json` manifest file in project root on first project load
- [x] Include project metadata (engine type, paths, file structure)
- [x] Include engine detection criteria and export roots
- [x] Use manifest for project identification on subsequent loads
- [x] Remove dependency on `export_translated_subset_via_factory` function

### Translation Repository Pattern ✅ COMPLETED
- [x] Create `src-tauri/src/db/translation/mod.rs` module file
- [x] Create `src-tauri/src/db/translation/model.rs` with `TextUnitRecord` struct
- [x] Create `src-tauri/src/db/translation/repo.rs` with CRUD operations
- [x] Implement `find_units()`, `upsert_unit()`, `bulk_upsert()`, `delete_unit()`
- [x] Add filtering by status, project, file path, and manifest-based identification
- [x] Add `find_units_by_project_and_status()` for loading existing translations
- [x] Add `mark_units_as_translating()` for concurrency control

### State Management Integration ✅ COMPLETED
- [x] Update `src-tauri/src/db/state.rs` for shared database connection
- [x] Add `TranslationState` wrapper around shared `DbState`
- [x] Update command handlers in `src-tauri/src/commands/handler.rs`
- [x] Add database state to main app builder
- [x] Update glossary.rs to use new db module structure
- [x] Update lib.rs to manage both DbState and GlossaryState
- [x] ✅ **Glossary functionality verified working correctly**

## Phase 2: Core Translation Commands
### New Command Handlers
- [ ] Create `load_project_translations` command using manifest for project identification
- [ ] Create `save_translation_state` command with manifest-based project tracking
- [ ] Create `get_translation_history` command
- [ ] Create `create_project_manifest` command for initial project setup
- [x] **Modify `translate_text_unit` to save results to database immediately after translation**
- [x] **Refactor `translate_text_unit` to use new db module structure**
- [x] **Remove dead code from `translate_text_unit` and related functions**
- [ ] Update `extract_text` to merge with existing database translations
- [ ] Update `load_project` to check/create `.ludolingua.json` manifest
- [ ] Add `load_existing_translations` command to load previously translated units

### Translation State Operations
- [ ] Implement database connection pooling and caching in shared state
- [ ] Add error handling for database operations
- [ ] Add transaction support for bulk operations
- [ ] Implement optimistic locking for concurrent access
- [ ] Add manifest-based project validation and identification

## Phase 3: Frontend Integration
### Engine Store Updates
- [ ] Add `loadProjectTranslations()` method using manifest for project identification
- [ ] Modify `updateTextUnit()` to save to database immediately
- [ ] Add `saveTranslationState()` for periodic auto-save
- [ ] Add `createProjectManifest()` for initial project setup
- [ ] Add translation persistence status tracking
- [ ] Remove dependency on export functions

### Translation Store Updates
- [ ] Update `translateTextUnit()` to use new database commands
- [ ] Add `batchSaveTranslations()` for bulk database operations
- [ ] Add error handling for database failures
- [ ] Implement translation recovery on app restart
- [ ] Add manifest-based project validation

### UI Component Updates
- [ ] Add "saving..." indicators during translations
- [ ] Show translation persistence status in project info
- [ ] Add "Load Previous Session" option if `.ludolingua.json` exists
- [ ] Update progress tracking for saved vs unsaved work
- [ ] Add project manifest status indicator

## Phase 4: Migration & Cleanup
### Data Migration
- [ ] Create migration script for existing in-memory translations
- [ ] Add backward compatibility for projects with/without `.ludolingua.json`
- [ ] Test data integrity during migration with manifest system
- [ ] Add rollback mechanisms for failed migrations

### Architecture Migration
- [ ] Migrate glossary operations to `db/glossary/` structure
- [ ] Update all import statements to use new module paths
- [ ] Refactor state management to use shared database state
- [ ] Update command handlers to use new module structure
- [ ] Test all database operations with new architecture

### Dead Code Removal
- [ ] Remove `export_translated_subset_via_factory` function from factory.rs
- [ ] Remove unused token estimation functions
- [ ] Clean up duplicate translation retry logic
- [ ] Remove commented-out code in stores
- [ ] Remove deprecated export functions
- [ ] Clean up unused imports and functions
- [ ] Remove old glossary module structure

### Performance Optimization
- [ ] Add database query optimization for manifest-based queries
- [ ] Implement lazy loading for large projects
- [ ] Add database connection pooling
- [ ] Optimize bulk operations with manifest validation
- [ ] Add caching for frequently accessed manifest data

## Phase 5: Testing & Documentation


### Testing
- [ ] Add unit tests for translation repository with manifest support
- [ ] Add integration tests for database operations with new module structure
- [ ] Test manifest creation and project identification scenarios
- [ ] Test concurrent access scenarios with shared database state
- [ ] Test data recovery scenarios with `.ludolingua.json` files
- [ ] Test backward compatibility for projects without manifests

### Documentation Updates
- [ ] Update ARCHITECTURE.md with new database module structure
- [ ] Update BACKEND_STRUCTURE.md with db/ folder organization
- [ ] Update FRONTEND_STRUCTURE.md with store changes and manifest support
- [ ] Add database migration documentation and manifest system
- [ ] Document `.ludolingua.json` manifest file format and usage
- [ ] Update CHANGELOG.md with new features and architecture changes
