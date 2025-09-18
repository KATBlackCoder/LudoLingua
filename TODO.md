# LudoLingua Development TODO

## 🎉 **Phase 4: Cleanup & Modernization - COMPLETED**

All Phase 4 objectives have been successfully achieved! The application now has a modern, consistent, and fully functional foundation.

---

### ✅ **Phase 4.1: File Renaming for Consistency** - COMPLETED

**Goal:** Eliminate confusion between translation workflow vs translation management

- [x] **Directory Rename**: `app/components/translation/` → `app/components/translator/`
- [x] **Page Rename**: `app/pages/translation.vue` → `app/pages/translator.vue`
- [x] **Navigation Updates**: Updated all route references from `/translation` to `/translator`
- [x] **Import Updates**: Fixed all import statements across components
- [x] **Clear Naming**: Translator (workflow) vs Translations (management)

**Result:** Clean, consistent naming convention with no confusion.

---

### ✅ **Phase 4.2: Nuxt 4.1 Upgrade** - COMPLETED

**Goal:** Upgrade from Nuxt 4.0 to 4.1 with latest features

- [x] Update `package.json` dependencies to Nuxt 4.1.1
- [x] Run `pnpm install` and test build process
- [x] Update `nuxt.config.ts` for new features
- [x] Test all existing functionality
- [x] Verify enhanced chunk stability and performance improvements

**Result:** Modern Nuxt 4.1.1 with enhanced performance and stability.

---

### ✅ **Phase 4.3: Nuxt UI v4 Migration** - COMPLETED

**Goal:** Migrate from Nuxt UI v3 to v4

- [x] Update `package.json`: `"@nuxt/ui": "4.0.0-alpha.1"`
- [x] Review and implement breaking changes from migration guide
- [x] Update component usage across all pages and components
- [x] Test glossary functionality (reference implementation)
- [x] Update custom styling and theming
- [x] Verify TypeScript compatibility
- [x] Migrate all icons from Heroicons to Lucide for consistency

**Result:** Latest Nuxt UI v4 with improved components and unified design system.

---

### ✅ **Phase 4.4: Rebuild Translation Management** - COMPLETED

**Goal:** Create simple, working translation management using proven glossary patterns

**✅ Backend Implementation (Complete):**
1. ✅ Created `src-tauri/src/commands/translations.rs` (CRUD commands)
2. ✅ Enhanced `src-tauri/src/db/translation/repo.rs` (database operations)
3. ✅ Added project management commands in `src-tauri/src/commands/engine.rs`
4. ✅ Fixed database schema constraints for legitimate duplicate text storage
5. ✅ Registered all commands in `src-tauri/src/commands/handler.rs`

**✅ Frontend Implementation (Complete):**
6. ✅ Created `app/stores/translations.ts` (state management)
7. ✅ Created `app/composables/useTranslations.ts` (UI logic)
8. ✅ Created `app/components/translations/` directory
9. ✅ Built `TranslationTable.vue` with full functionality
10. ✅ Built `TranslationForm.vue` for editing
11. ✅ Created `app/pages/translations.vue` page
12. ✅ Added navigation integration to `AppHeader.vue`

**✅ Advanced Features (Complete):**
13. ✅ Fixed table width using `UContainer max-w-full`
14. ✅ Implemented working actions column with edit/delete buttons
15. ✅ Fixed native Tauri dialog confirmations for reliable operations
16. ✅ Updated all icons from Heroicons to Lucide for consistency
17. ✅ Implemented row selection with checkboxes and bulk operations
18. ✅ Added project selection dropdown and delete project functionality
19. ✅ Enhanced about page with comprehensive translation management information
20. ✅ Fixed navigation flow (translations page routes to home, not translator)
21. ✅ Added conditional translator link visibility (only when project loaded)

**✅ Database & Infrastructure (Complete):**
22. ✅ Fixed critical `UNIQUE constraint failed` error affecting text storage
23. ✅ Created migration `0004_fix_text_units_uniqueness.sql` to remove restrictive constraints
24. ✅ Updated repository code to allow legitimate duplicate text storage
25. ✅ Verified all project loading and text extraction works correctly

**Result:** Complete, production-ready translation management system with advanced features!

---

## 🚀 **What's Next?**

Phase 4 is **100% complete**! The application now has:
- ✅ **Consistent naming** (translator vs translations)
- ✅ **Modern framework** (Nuxt 4.1.1 + UI v4)
- ✅ **Full translation management** (CRUD interface with advanced features)
- ✅ **Fixed database constraints** (proper text storage for RPG games)
- ✅ **Complete icon consistency** (all Lucide icons)
- ✅ **Enhanced about page** (comprehensive feature documentation)
- ✅ **Improved navigation** (conditional links and proper routing)

---

## 🎯 **Ready for Phase 5: Enhanced Features & Polish**

The solid foundation is now complete. Ready to begin **Phase 5** development with:

### **🔧 Phase 5.0: Code Architecture Refinement** (Preparation)
**Goal:** Create unified, automatic text processing pipeline for all engines

#### **📁 Phase 5.0.1: Create Unified Text Processing Pipeline** (Week 1)
- [ ] **Create `utils/text/` Module Structure**:
  - [ ] `utils/text/llm_output.rs` - Move `clean_model_output` from `translator.rs`
  - [ ] `utils/text/formatting.rs` - Move RPG Maker + Wolf RPG code functions from `text_processing.rs` and `wolf_rpg/files/regex.rs`
  - [ ] `utils/text/whitespace.rs` - Move whitespace functions from `text_processing.rs`
  - [ ] `utils/text/validation.rs` - Move `is_technical_content` from `text_processing.rs`
  - [ ] `utils/text/pipeline.rs` - **Unified `TextProcessor` struct with complete processing pipeline**
  - [ ] `utils/text/mod.rs` - Public API exports and module organization

- [ ] **Implement Complete Processing Pipeline**:
  - [ ] `TextProcessor::prepare_for_translation()` - Filter → Format → Encode pipeline
  - [ ] `TextProcessor::clean_llm_output()` - **Enhanced LLM response cleaning and extraction**:
    - [ ] Remove `<think>...</think>`, `<thinking>...</thinking>` blocks (current + new patterns)
    - [ ] Remove `<<<INPUT_START>>>` / `<<<INPUT_END>>>` tag artifacts directly from translation text
    - [ ] Remove trailing artifacts: "Note:", "P.S.:", "Explanation:" suffixes
    - [ ] Support multiple structured tag patterns for different LLM providers
    - [ ] Final cleanup with whitespace normalization and quality validation
  - [ ] `TextProcessor::restore_for_injection()` - Decode → Restore pipeline
  - [ ] Add configuration options for engine-specific processing
  - [ ] **Unified text processing approach**:
    - [ ] All engines use same text processing pipeline (engine-agnostic)
    - [ ] Formatting codes removed during translation preparation
    - [ ] Formatting codes restored during injection (same process for all engines)
    - [ ] No engine-specific logic needed - unified `formatting.rs` handles everything

#### **🏗️ Phase 5.0.2: Engine Trait Enhancement** (Week 1-2)
- [ ] **Simplified Engine Trait Architecture**:
  - [ ] Keep existing `TextUnit` structure (no `RawTextUnit` needed)
  - [ ] Add automatic text processing to existing `extract_text_units()` method
  - [ ] Add automatic text processing to existing `inject_text_units()` method
  - [ ] Engine trait provides default implementations with unified pipeline

- [ ] **Automatic Text Processing Integration**:
  - [ ] Engine trait provides default implementations with unified pipeline
  - [ ] Engines focus on file I/O, text processing happens automatically
  - [ ] All engines (current and future) get consistent text processing for free
  - [ ] Zero manual text processing calls required in engine implementations

#### **🔄 Phase 5.0.3: Engine Migration & Cleanup** (Week 2)
- [ ] **Migration Order**: MV → MZ → Wolf RPG (start simple, end complex)
- [ ] **Clean Up Existing Engines**:
  - [ ] **Remove text processing calls from direct engine files**:
    - [ ] Remove `replace_formatting_codes_for_translation()` calls from `system.rs` (13 calls)
    - [ ] Remove `restore_formatting_codes_after_translation()` calls from `system.rs` (17 calls)
    - [ ] Remove `is_technical_content()` calls from `system.rs` (6 calls)
    - [ ] Remove text processing calls from `common.rs` (6 calls total)
  - [ ] **Remove Wolf RPG custom text processing**:
    - [ ] Remove `wolf_replace_placeholders_for_translation()` calls from `mps.rs` (2 calls)
    - [ ] Remove `wolf_restore_placeholders_after_translation()` calls from `mps.rs` (1 call)
    - [ ] Remove `is_translatable_wolf_text()` calls from `mps.rs` (1 call)
    - [ ] Remove `wolf_replace_placeholders_for_translation()` calls from `db.rs` (2 calls)
    - [ ] Remove `wolf_restore_placeholders_after_translation()` calls from `db.rs` (2 calls)
    - [ ] Remove `is_translatable_wolf_text()` calls from `db.rs` (2 calls)
  - [ ] **Update common.rs utility functions**:
    - [ ] Remove text processing from `extract_text_units_for_object()` 
    - [ ] Remove text processing from `inject_text_units_for_object()`
    - [ ] Make these functions work with raw text only
  - [ ] **Simplify engine methods**:
    - [ ] Engines focus only on reading/writing files
    - [ ] Text processing happens automatically via trait
    - [ ] Remove all manual text processing calls
  - [ ] Update all engines to use new trait architecture

- [ ] **Database & Migration Strategy**:
  - [ ] **No database changes needed** - same `TextUnitRecord` structure
  - [ ] Same `field_type` format maintained for compatibility
  - [ ] Same export process works with cleaner text processing
  - [ ] Verify existing database records work with new pipeline

- [ ] **Comprehensive Testing & Validation**:
  - [ ] Unit tests for all text processing functions individually
  - [ ] Integration tests for complete text processing pipeline
  - [ ] Verify all engines produce identical results with new architecture
  - [ ] Performance validation - ensure no regression in processing speed
  - [ ] Test text processing consistency across all game engine types

#### **🎯 Benefits Achieved**
- ✅ **Complete Automation**: Engines get text processing automatically, no manual calls
- ✅ **Global Consistency**: Same processing pipeline applied to all engines uniformly  
- ✅ **Future-Proof**: New engines get text processing for free with zero setup
- ✅ **Maintainability**: All text processing logic centralized in one location
- ✅ **Clean Architecture**: Engines focus on file I/O, pipeline handles text concerns
- ✅ **Zero Duplication**: Eliminate repetitive text processing code across engines
- ✅ **Simplified Approach**: No complex enums or new data structures needed
- ✅ **Database Compatibility**: No database changes required, same export process

#### **📊 Current Text Processing Usage Analysis**
**Files currently using `text_processing.rs`:**
- ✅ **`common.rs`**: 6 calls total (extract/inject utilities)
- ✅ **`system.rs`**: 36 calls total (13 extract + 17 inject + 6 validation)
- ✅ **All other engine files**: Use `common.rs` utilities (indirect usage)

**Files using custom Wolf RPG text processing (`regex.rs`):**
- ✅ **`wolf_rpg/files/mps.rs`**: 3 calls total (2 extract + 1 inject)
- ✅ **`wolf_rpg/files/db.rs`**: 4 calls total (2 extract + 2 inject)

**Total calls to eliminate**: ~49 direct text processing calls across all engines

### **🔔 Phase 5.1: Native Notifications Integration**
**Goal:** Add native desktop notifications for key user actions using [Tauri Notification Plugin](https://tauri.app/plugin/notification/)

#### **📱 Phase 5.1.1: Tauri Notification Plugin Setup** (Week 1)
- [ ] **Install Tauri Notification Plugin**:
  - [ ] Run `pnpm tauri add notification` in project root
  - [ ] Verify plugin installation in `Cargo.toml` and `lib.rs`
  - [ ] Install frontend package: `pnpm add @tauri-apps/plugin-notification`
  - [ ] ✅ **Permissions already configured** in `capabilities/default.json` (lines 15-16)

#### **🔔 Phase 5.1.2: Notification Implementation** (Week 1)
- [ ] **Project Loading Notifications**:
  - [ ] **Integration Point**: `app/stores/engine.ts` - `loadProject()` function (line 59)
  - [ ] **Trigger**: After successful text extraction (`setTextUnits(extractedUnits)`)
  - [ ] **Data**: Project name, total text units found, engine type
  - [ ] **Notification**: "Project Loaded: {name} ({count} text units, {engine})"

- [ ] **Translation Completion Notifications**:
  - [ ] **Integration Point**: `app/stores/translator.ts` - `startBatchTranslation()` function (line 140)
  - [ ] **Trigger**: After batch translation completes (alongside existing toast)
  - [ ] **Data**: Success count, failed count, total count
  - [ ] **Notification**: "Translation Complete: {success}/{total} units translated{failed > 0 ? ', {failed} failed' : ''}"

#### **⚙️ Phase 5.1.3: User Experience Enhancement** (Week 1)
- [ ] **Notification Settings**:
  - [ ] **Integration Point**: `app/stores/settings.ts` - Add notification preferences
  - [ ] **Settings Page**: `app/pages/settings.vue` - Add notification toggles
  - [ ] **Options**: Enable/disable project load notifications, translation completion notifications
  - [ ] **Storage**: Use existing Tauri store for persistence

- [ ] **Cross-Platform Compatibility**:
  - [ ] Test notifications on Windows, macOS, and Linux
  - [ ] Handle permission requests gracefully (already configured in capabilities)
  - [ ] Add fallback for systems without notification support

#### **🎯 Benefits Achieved**
- ✅ **Better User Experience**: Users get immediate feedback on long-running operations
- ✅ **Multi-tasking Support**: Users can work on other things while translations run
- ✅ **Error Awareness**: Immediate notification of translation failures or issues
- ✅ **Progress Tracking**: Clear indication when operations complete successfully
- ✅ **Professional Feel**: Desktop app behaves like native applications
- ✅ **Simple Integration**: Only 2 notification points, minimal complexity
- ✅ **Existing Infrastructure**: Leverages current toast system and settings store

### **🔧 Advanced Translation Management** (Planned)
- [ ] Multi-criteria filtering system (status + type + project + date)
- [ ] Full-text search across source and translated text
- [ ] Advanced statistics dashboard with visual analytics
- [ ] Export/Import capabilities (JSON, CSV, Excel)
- [ ] Bulk operations and batch processing enhancements

### **🎨 UI/UX Polish & Performance** (Planned)
- [ ] Virtual scrolling for large datasets (10,000+ translations)
- [ ] Keyboard shortcuts for power users
- [ ] Enhanced loading states and skeleton screens
- [ ] Improved error handling and user feedback
- [ ] Mobile-responsive design and touch support

### **🧪 Quality Assurance & Testing** (Planned)
- [ ] Unit tests for components and composables
- [ ] Integration tests for end-to-end workflows
- [ ] Performance testing with large datasets
- [ ] Cross-platform compatibility validation (Windows/macOS/Linux)

---

## 📝 **Development Notes**

### **Architecture Patterns Established:**
- **Naming Convention**: `translator` (workflow) vs `translations` (management)
- **Component Pattern**: Follow `GlossaryTable.vue` and `GlossaryForm.vue` proven patterns
- **State Management**: Pinia stores + composables + components architecture
- **Icon System**: Lucide icons for Nuxt UI v4 consistency
- **Database Design**: Flexible schema allowing legitimate duplicates in RPG game text
- **Text Processing**: Unified `utils/text/` pipeline with automatic Engine trait integration (Phase 5.0 target)
  - **Automatic Processing**: Engines get text processing for free via trait default implementations
  - **Global Consistency**: Same pipeline applied to all engines without manual calls
  - **Future-Proof**: New engines inherit text processing capabilities automatically

### **Quality Standards:**
- ✅ No TypeScript errors or warnings
- ✅ Clean, consistent code structure
- ✅ Proper error handling with native dialogs
- ✅ Responsive design with proper spacing
- ✅ Modern UI patterns and best practices

### **Ready for Next Phase:**
With Phase 4 complete, the application has a solid, modern foundation ready for advanced feature development in Phase 5.

---

**🏆 Status**: Phase 4 complete, ready for Phase 5 development!
