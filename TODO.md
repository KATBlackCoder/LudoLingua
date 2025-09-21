# LudoLingua Development TODO

## 🎉 **Phase 4: Cleanup & Modernization - COMPLETED**

All Phase 4 objectives have been successfully achieved! The application now has a modern, consistent, and fully functional foundation.

---

### ✅ **Phase 4.1: File Renaming for Consistency** - COMPLETED
- [x] **Directory Rename**: `app/components/translation/` → `app/components/translator/`
- [x] **Page Rename**: `app/pages/translation.vue` → `app/pages/translator.vue`
- [x] **Navigation Updates**: Updated all route references from `/translation` to `/translator`
- [x] **Import Updates**: Fixed all import statements across components
- [x] **Clear Naming**: Translator (workflow) vs Translations (management)

### ✅ **Phase 4.2: Nuxt 4.1 Upgrade** - COMPLETED
- [x] Update `package.json` dependencies to Nuxt 4.1.1
- [x] Run `pnpm install` and test build process
- [x] Update `nuxt.config.ts` for new features
- [x] Test all existing functionality
- [x] Verify enhanced chunk stability and performance improvements

### ✅ **Phase 4.3: Nuxt UI v4 Migration** - COMPLETED
- [x] Update `package.json`: `"@nuxt/ui": "4.0.0-alpha.1"`
- [x] Review and implement breaking changes from migration guide
- [x] Update component usage across all pages and components
- [x] Test glossary functionality (reference implementation)
- [x] Update custom styling and theming
- [x] Verify TypeScript compatibility
- [x] Migrate all icons from Heroicons to Lucide for consistency

### ✅ **Phase 4.4: Rebuild Translation Management** - COMPLETED
- [x] **Backend**: Created `src-tauri/src/commands/translations.rs` (CRUD commands)
- [x] **Backend**: Enhanced `src-tauri/src/db/translation/repo.rs` (database operations)
- [x] **Backend**: Added project management commands in `src-tauri/src/commands/engine.rs`
- [x] **Backend**: Fixed database schema constraints for legitimate duplicate text storage
- [x] **Backend**: Registered all commands in `src-tauri/src/commands/handler.rs`
- [x] **Frontend**: Created `app/stores/translations.ts` (state management)
- [x] **Frontend**: Created `app/composables/useTranslations.ts` (UI logic)
- [x] **Frontend**: Built `TranslationTable.vue` with full functionality
- [x] **Frontend**: Built `TranslationForm.vue` for editing
- [x] **Frontend**: Created `app/pages/translations.vue` page
- [x] **Frontend**: Added navigation integration to `AppHeader.vue`
- [x] **Advanced Features**: Fixed table width, implemented actions column, native dialogs
- [x] **Advanced Features**: Row selection, bulk operations, project management
- [x] **Database**: Fixed critical `UNIQUE constraint failed` error
- [x] **Database**: Created migration `0004_fix_text_units_uniqueness.sql`

### ✅ **Phase 4.5: Color Scheme Migration** - COMPLETED
- [x] **Updated app.config.ts**: Defined semantic color palette (primary: blue, secondary: purple, etc.)
- [x] **Updated nuxt.config.ts**: Configured Nuxt UI to use custom colors
- [x] **Updated Components**: Migrated all hardcoded colors to semantic colors across:
  - [x] `app/components/about/referral/` (AboutNutaku.vue, AboutRunPod.vue, AboutFreeBitcoin.vue)
  - [x] `app/components/about/AboutGlossary.vue`
  - [x] `app/pages/translations.vue` (stats cards)
  - [x] `app/components/translator/TranslationProcess.vue`
  - [x] `app/components/translations/TranslationTable.vue`
  - [x] `app/components/glossaries/GlossaryTable.vue`
- [x] **Verified**: All other components already using semantic colors
- [x] **Result**: Consistent, maintainable color scheme across entire application

---

## 🚀 **Phase 5: Enhanced Features & Polish**

### ✅ **Phase 5.0: Code Architecture Refinement** - COMPLETED
- [x] **Unified Text Processing Pipeline**: Created `utils/text/` module with universal text handling
- [x] **Engine Trait Enhancement**: Automatic text processing via core trait defaults
- [x] **Engine Migration**: Eliminated 49+ text processing calls across all engines
- [x] **LLM Output Cleaning**: Enhanced smart cleaning with context-aware artifact removal
- [x] **Comprehensive Testing**: Unit tests, integration tests, performance validation

### ✅ **Phase 5.1: Native Notifications Integration** - COMPLETED
- [x] **Tauri Notification Plugin**: Installed and configured notification system
- [x] **Core Notification Logic**: Created `useNotifications.ts` composable
- [x] **Project Loading Notifications**: Automatic notifications for successful project loads
- [x] **Translation Completion Notifications**: Batch translation completion alerts
- [x] **Cross-Platform Testing**: Verified compatibility across platforms

### ✅ **Recent Enhancement: Field Type Column Added**
- [x] **TranslationTable.vue Enhancement**: Added field type column to display detailed location information
  - Added `field_type` column to table showing exact file location (e.g., `name:www/data/MapInfos.json:2`)
  - Enhanced search functionality to include field type in search queries
  - Updated search placeholder to reflect new searchable content
  - Improved user experience for identifying translation sources and locations

### **✅ Phase 5.0: Code Architecture Refinement - COMPLETED**
**Goal:** Create unified, automatic text processing utility for all current and future engines
**Status:** 100% Complete - All Objectives Achieved

#### **✅ Phase 5.0.1: Create Unified Text Processing Pipeline - COMPLETED**
- [x] **Create `utils/text/` Module Structure**:
  - [x] `utils/text/llm_output.rs` - **Smart LLM output cleaning and extraction**
    - [x] `clean_output()` - Smart LLM response cleaning (context-aware)
    - [x] Remove thinking blocks, input/output tags (always safe)
    - [x] Smart artifact removal: only removes clear LLM commentary, not game content
    - [x] Preserves legitimate "Note:", "Explanation:" that are part of game text
    - [x] Quality validation: ensures cleaning doesn't remove all content
  - [x] `utils/text/formatting.rs` - **Universal text handler for all engines**
    - [x] Single `TextFormatter` struct with no engine specification
    - [x] `prepare_for_translation()` - **UNIFIED METHOD**: Handles ALL placeholders in one pass
      - [x] RPG Maker codes → placeholders
      - [x] Wolf RPG codes → placeholders  
      - [x] Whitespace patterns → placeholders
      - [x] All engines use same unified processing pipeline
    - [x] `restore_after_translation()` - **UNIFIED METHOD**: Restores ALL placeholders in one pass
      - [x] All placeholders → original codes
      - [x] No separate whitespace or engine-specific methods
      - [x] Single restore pipeline for all engines
    - [x] **Complete Unified Placeholder System** - ALL placeholders handled together:
      - [x] **All 36 Placeholder Types**: `[COLOR_n]`, `[NAME_n]`, `[NEWLINE_n]`, `[VARIABLE_n]`, `[variable_n]`, `[SWITCH_n]`, `[ITEM_n]`, `[WEAPON_n]`, `[ARMOR_n]`, `[ACTOR_n]`, `[GOLD]`, `[CURRENCY]`, `[ARG_n]`, `[NUM_PREFIX_n]`, `[CTRL_DOT]`, `[CTRL_WAIT]`, `[CTRL_INSTANT]`, `[CTRL_INPUT]`, `[CONDITIONAL_vn>n]`, `[WOLF_END]`, `[ICON_n]`, `[FONT_n]`, `[AT_n]`, `[SLOT_n]`, `[CSELF_n]`, `[RUBY_START]`, `[CARRIAGE_RETURN]`, `[NEWLINE]`, `[FWSPC_n]`, `[SPC_n]`, `[TAB_n]`
      - [x] **No Separate Methods**: All placeholders processed in same replace/restore pipeline
      - [x] **Engine Agnostic**: Same processing for RPG Maker, Wolf RPG, and future engines
  - [x] `utils/text/validation.rs` - **Universal validation: common logic for all engines**
    - [x] `ContentValidator` struct - Unified interface for all engines
    - [x] `validate_text()` - Universal validation logic (empty, length, problematic chars, etc.)
    - [x] `get_initial_status()` - Determine status: Pending (needs translation) or Ignored (already in target language)
    - [x] `get_warnings()` - Optional warnings for text that passed validation
    - [x] Translation direction logic: mark text as Ignored if already in target language script
    - [x] Simple and universal: same validation rules for all engines
  - [x] `utils/text/pipeline.rs` - **Unified `TextProcessor` struct with complete processing pipeline**
  - [x] `utils/text/mod.rs` - Public API exports and module organization

- [x] **Implement Complete Processing Pipeline**:
  - [x] `TextProcessor::process_for_extraction()` - Filter → **UNIFIED FORMAT** → Encode pipeline
  - [x] `TextProcessor::clean_llm_output()` - **Enhanced LLM response cleaning and extraction**:
    - [x] Move current `clean_model_output` logic from `commands/translator.rs` (lines 322-362)
    - [x] Remove `<think>...</think>`, `<thinking>...</thinking>` blocks (current + new patterns)
    - [x] Remove `<<<INPUT_START>>>` / `<<<INPUT_END>>>` tag artifacts directly from translation text
    - [x] Remove trailing artifacts: "Note:", "P.S.:", "Explanation:" suffixes
    - [x] Support multiple structured tag patterns for different LLM providers
    - [x] Final cleanup with whitespace normalization and quality validation
  - [x] `TextProcessor::process_for_injection()` - **UNIFIED RESTORE** → Decode pipeline
  - [x] **Unified text processing approach**:
    - [x] **Single Unified Method**: `prepare_for_translation()` handles ALL 36 placeholder types
    - [x] **Single Unified Method**: `restore_after_translation()` restores ALL 36 placeholder types
    - [x] **No Separate Methods**: No whitespace-specific or engine-specific processing methods
    - [x] **All Engines**: RPG Maker, Wolf RPG, and future engines use identical processing
    - [x] **Same Pipeline**: All formatting codes, whitespace, and engine codes → unified placeholders
    - [x] **Same Restoration**: All placeholders → original codes via single restore method

#### **✅ Phase 5.0.2: Engine Trait Enhancement - COMPLETED**
- [x] **Simplified Engine Trait Architecture**:
  - [x] **Core trait** (`src-tauri/src/core/engine.rs`): Add default implementations for `extract_text_units()` and `inject_text_units()`
  - [x] **Core trait**: Default implementations automatically call engine-specific raw methods + unified text processing
  - [x] **Each engine**: Remove existing `extract_text_units()` and `inject_text_units()` methods
  - [x] **Each engine**: Add new `extract_raw_text_units()` and `inject_raw_text_units()` methods (file I/O only)
  - [x] **No config needed**: Use unified `TextProcessor` for all engines

  - [x] **Automatic Text Processing Integration**:
    - [x] **Public API**: `engine.extract_text_units()` calls core trait default implementation
    - [x] **Core trait**: Calls `engine.extract_raw_text_units()` → processes through unified `TextProcessor` → returns processed `TextUnit`s
    - [x] **Core trait**: Calls `engine.inject_raw_text_units()` after processing through unified `TextProcessor`
    - [x] **Engine implementations**: Focus only on file I/O, no text processing calls
    - [x] **All engines**: Get identical text processing automatically via unified utility
    - [x] **Zero manual calls**: No text processing calls required in engine implementations
    - [x] **Future-proof**: Any new engine automatically gets text processing capabilities

#### **✅ Phase 5.0.3: Engine Migration & Cleanup - COMPLETED**
- [x] **Migration Order**: MV → MZ → Wolf RPG (start simple, end complex)
- [x] **Core Trait Updates**:
  - [x] Update `src-tauri/src/core/engine.rs` with default implementations
  - [x] Add `RawTextUnit` model for raw file I/O
  - [x] Implement automatic text processing pipeline integration

- [x] **Engine Implementation Updates**:
  - [x] **RPG Maker MV Engine** (`src-tauri/src/engines/rpg_maker_mv/engine.rs`):
    - [x] Remove existing `extract_text_units()` and `inject_text_units()` methods
    - [x] Add `extract_raw_text_units()` method (file I/O only, no text processing)
    - [x] Add `inject_raw_text_units()` method (file I/O only, no text processing)
  - [x] **RPG Maker MZ Engine** (`src-tauri/src/engines/rpg_maker_mz/engine.rs`):
    - [x] Same pattern as MV engine
  - [x] **Wolf RPG Engine** (`src-tauri/src/engines/wolf_rpg/engine.rs`):
    - [x] Same pattern as MV/MZ engines

- [x] **Clean Up Engine File Processing**:
  - [x] **Remove text processing calls from engine file modules**:
    - [x] Remove `replace_formatting_codes_for_translation()` calls from `system.rs` (13 calls)
    - [x] Remove `restore_formatting_codes_after_translation()` calls from `system.rs` (17 calls)
    - [x] Remove `is_technical_content()` calls from `system.rs` (6 calls)
    - [x] Remove text processing calls from `common.rs` (6 calls total)
  - [x] **Remove Wolf RPG custom text processing**:
    - [x] Remove `wolf_replace_placeholders_for_translation()` calls from `mps.rs` (2 calls)
    - [x] Remove `wolf_restore_placeholders_after_translation()` calls from `mps.rs` (1 call)
    - [x] Remove `is_translatable_wolf_text()` calls from `mps.rs` (1 call)
    - [x] Remove `wolf_replace_placeholders_for_translation()` calls from `db.rs` (2 calls)
    - [x] Remove `wolf_restore_placeholders_after_translation()` calls from `db.rs` (2 calls)
    - [x] Remove `is_translatable_wolf_text()` calls from `db.rs` (2 calls)
  - [x] **Update common.rs utility functions**:
    - [x] Remove text processing from `extract_text_units_for_object()` 
    - [x] Remove text processing from `inject_text_units_for_object()`
    - [x] Make these functions work with raw text only

- [x] **Database & Migration Strategy**:
  - [x] **No database changes needed** - same `TextUnitRecord` structure
  - [x] Same `field_type` format maintained for compatibility
  - [x] Same export process works with cleaner text processing
  - [x] Verify existing database records work with new pipeline

- [x] **Update Export and Frontend Logic for Ignored Status**:
  - [x] **Backend**: Update `factory.rs` export logic to include `Ignored` status as "translated"
  - [x] **Frontend**: Update `useTranslator.ts` to include `Ignored` status in `translatedItems` computed
  - [x] **Logic**: Text with `Ignored` status should be considered "translated" (already in target language)
  - [x] **Export**: Include ignored text units in export process (they're already "translated")
  - [x] **UI**: Show ignored text as completed/translated in the interface

- [x] **Comprehensive Testing & Validation**:
  - [x] Unit tests for all text processing functions individually
  - [x] Integration tests for complete text processing pipeline
  - [x] Verify all engines produce identical results with new architecture
  - [x] Performance validation - ensure no regression in processing speed
  - [x] Test text processing consistency across all game engine types
  - [x] Test core trait default implementations work correctly
  - [x] Test engine-specific raw methods work with unified pipeline

#### **🎯 Benefits Achieved**
- ✅ **Complete Automation**: Engines get text processing automatically, no manual calls
- ✅ **Global Consistency**: Same processing utility applied to all engines uniformly  
- ✅ **Future-Proof**: New engines get text processing for free with zero setup
- ✅ **Maintainability**: All text processing logic centralized in one location
- ✅ **Clean Architecture**: Engines focus on file I/O, utility handles text concerns
- ✅ **Zero Duplication**: Eliminate repetitive text processing code across engines
- ✅ **Simplified Approach**: No complex enums or new data structures needed
- ✅ **Database Compatibility**: No database changes required, same export process
- ✅ **Engine Agnostic**: Works for all current and future engines without modification

#### **📊 Complete Placeholder Reference for Unified Text Processing**

**🎯 RPG Maker Placeholders (from `text_processing.rs`):**
- `[COLOR_n]` - Color codes: `\C[n]` / `\c[n]` → `[COLOR_n]`
- `[NAME_n]` - Name codes: `\N[n]` → `[NAME_n]`
- `[NEWLINE_n]` - Newline codes: `\n[n]` → `[NEWLINE_n]`
- `[VARIABLE_n]` - Variable codes: `\V[n]` → `[VARIABLE_n]`
- `[variable_n]` - Variable codes: `\v[n]` → `[variable_n]`
- `[SWITCH_n]` - Switch codes: `\S[n]` → `[SWITCH_n]`
- `[ITEM_n]` - Item codes: `\I[n]` → `[ITEM_n]`
- `[WEAPON_n]` - Weapon codes: `\W[n]` → `[WEAPON_n]`
- `[ARMOR_n]` - Armor codes: `\A[n]` → `[ARMOR_n]`
- `[ACTOR_n]` - Actor codes: `\P[n]` → `[ACTOR_n]`
- `[GOLD]` - Gold code: `\G` → `[GOLD]`
- `[CURRENCY]` - Currency code: `\$` → `[CURRENCY]`
- `[ARG_n]` - Parameter placeholders: `%n` / `％n` → `[ARG_n]`
- `[NUM_PREFIX_n]` - Numeric prefixes: `nnn＿` → `[NUM_PREFIX_n]`
- `[CTRL_DOT]` - Control dot: `\.` → `[CTRL_DOT]`
- `[CTRL_WAIT]` - Control wait: `\|` → `[CTRL_WAIT]`
- `[CTRL_INSTANT]` - Control instant: `\^` → `[CTRL_INSTANT]`
- `[CTRL_INPUT]` - Control input: `\!` → `[CTRL_INPUT]`
- `[CONDITIONAL_vn>n]` - Conditional expressions: `en(v[n]>n)` → `[CONDITIONAL_vn>n]`

**🐺 Wolf RPG Placeholders (from `regex.rs`):**
- `[WOLF_END]` - End formatting: `\E` → `[WOLF_END]`
- `[ICON_n]` - Icon codes: `\i[n]` → `[ICON_n]`
- `[FONT_n]` - Font codes: `\f[n]` → `[FONT_n]`
- `[AT_n]` - Event markers: `@n` → `[AT_n]`
- `[SLOT_n]` - Character slots: `\s[n]` → `[SLOT_n]`
- `[CSELF_n]` - Self color codes: `\cself[n]` → `[CSELF_n]`
- `[RUBY_START]` - Ruby text marker: `\r` → `[RUBY_START]`
- `[CARRIAGE_RETURN]` - Carriage return: `\r` → `[CARRIAGE_RETURN]`
- `[NEWLINE]` - Newline character: `\n` → `[NEWLINE]`

**🌐 Universal Whitespace Placeholders (both files):**
- `[FWSPC_n]` - Full-width spaces: `　` (n times) → `[FWSPC_n]`
- `[SPC_n]` - ASCII spaces: ` ` (n times) → `[SPC_n]`
- `[TAB_n]` - Tab characters: `\t` (n times) → `[TAB_n]`

**🔄 Shared Placeholders (used by both engines):**
- `[COLOR_n]` - Color codes (RPG Maker: `\C[n]`, Wolf RPG: `\c[n]`)
- `[ARG_n]` - Parameter placeholders (`%n` / `％n`)
- `[CTRL_DOT]`, `[CTRL_WAIT]`, `[CTRL_INSTANT]`, `[CTRL_INPUT]` - Control codes

**📊 Current Text Processing Usage Analysis**
**Files currently using `text_processing.rs`:**
- ✅ **`common.rs`**: 6 calls total (extract/inject utilities)
- ✅ **`system.rs`**: 36 calls total (13 extract + 17 inject + 6 validation)
- ✅ **All other engine files**: Use `common.rs` utilities (indirect usage)

**Files using custom Wolf RPG text processing (`regex.rs`):**
- ✅ **`wolf_rpg/files/mps.rs`**: 3 calls total (2 extract + 1 inject)
- ✅ **`wolf_rpg/files/db.rs`**: 4 calls total (2 extract + 2 inject)

**Files using LLM output cleaning:**
- ✅ **`commands/translator.rs`**: 1 call to `clean_model_output()` (lines 102, 322-362)

**Total calls to eliminate**: ~49 direct text processing calls across all engines + 1 LLM cleaning call

### **🔔 Phase 5.1: Native Notifications Integration**
**Goal:** Add native desktop notifications for key user actions using [Tauri Notification Plugin](https://tauri.app/plugin/notification/) - simplified implementation with basic `sendNotification` method only

#### **📱 Phase 5.1.1: Tauri Notification Plugin Setup** (Week 1) - ✅ COMPLETED
- [x] **Install Tauri Notification Plugin**:
  - [x] Run `pnpm tauri add notification` in project root
  - [x] Verify plugin installation in `Cargo.toml` and `lib.rs`
  - [x] Install frontend package: `pnpm add @tauri-apps/plugin-notification`
  - [x] ✅ **Permissions already configured** in `capabilities/default.json` (lines 15-16)
- [x] ⚠️ **Note**: `@tauri-apps/api` package needs to be installed: `pnpm add @tauri-apps/api`

#### **🔔 Phase 5.1.2: Core Notification Logic** (Week 1) - ✅ COMPLETED
- [x] **Create Notification Composable**:
  - [x] **File**: `app/composables/useNotifications.ts`
  - [x] **Functions**: `checkAndRequestPermission()`, `notifyProjectLoaded()`, `notifyTranslationComplete()`
  - [x] **API**: Uses only `isPermissionGranted`, `requestPermission`, `sendNotification` from plugin
  - [x] **Permission Management**: Automatic permission checking and requesting

- [x] **Project Loading Notifications**:
  - [x] **Integration Point**: `app/stores/engine.ts` - `loadProject()` function (line 64)
  - [x] **Trigger**: After successful text extraction (`setTextUnits(extractedUnits)`)
  - [x] **Data**: Project name, total text units found, engine type
  - [x] **Notification**: "Project Loaded: {name} ({count} text units, {engine})"

- [x] **Translation Completion Notifications**:
  - [x] **Integration Point**: `app/stores/translator.ts` - `startBatchTranslation()` function (line 146)
  - [x] **Trigger**: After batch translation completes (alongside existing toast)
  - [x] **Data**: Success count, failed count, total count
  - [x] **Notification**: "Translation Complete: {success}/{total} units translated{failed > 0 ? ', {failed} failed' : ''}"

#### **⚙️ Phase 5.1.3: Cross-Platform Testing** (Week 1) - ✅ COMPLETED
- [x] **Cross-Platform Compatibility**:
  - [x] Notification permission handling with graceful fallbacks
  - [x] Error handling for systems without notification support
  - [x] Automatic permission checking and requesting

#### **🎯 Benefits Achieved**
- ✅ **Better User Experience**: Users get immediate feedback on long-running operations
- ✅ **Multi-tasking Support**: Users can work on other things while translations run
- ✅ **Error Awareness**: Immediate notification of translation failures or issues
- ✅ **Progress Tracking**: Clear indication when operations complete successfully
- ✅ **Professional Feel**: Desktop app behaves like native applications
- ✅ **Simple Integration**: Only 2 notification points, minimal complexity
- ✅ **Existing Infrastructure**: Leverages current toast system
- ✅ **Simplified Approach**: Uses only basic `sendNotification` method, no complex features
- ✅ **No Settings Required**: Direct implementation without user preference complexity

---

## 🎯 **Next Priority: GitHub Actions Cross-Platform Build**

### **🔧 Phase 5.2: GitHub Actions Cross-Platform Build Setup** - IN PROGRESS
**Goal:** Set up automated builds for Linux (AppImage) and Windows (EXE) using GitHub Actions
**Note:** Starting with unsigned builds to solve immediate CachyOS compatibility issue

#### **📦 Phase 5.2.1: GitHub Actions Workflow Setup** - PENDING
- [ ] **Create Workflow File**: Create `.github/workflows/build-cross-platform.yml`
- [ ] **Linux Build (AppImage)**: Ubuntu 20.04 runner with `pnpm tauri build --bundles appimage`
- [ ] **Windows Build (EXE)**: Windows runner with `pnpm tauri build` (creates NSIS installer)
- [ ] **Dependencies Setup**: Install Node.js, Rust, and system dependencies for both platforms
- [ ] **Build Configuration**: Configure pnpm and Tauri build process for both targets
- [ ] **Artifact Upload**: Upload AppImage and EXE for download
- [ ] **Technical Verification**: Confirm both bundle formats work with current Tauri configuration

#### **🔧 Phase 5.2.1.1: Technical Implementation Details** - PENDING
- [ ] **Linux AppImage**: Uses `--bundles appimage` flag, creates portable executable (no signing needed)
- [ ] **Windows NSIS**: Uses default `pnpm tauri build`, creates installer (unsigned - will show security warnings)
- [ ] **Current Config**: Verify `tauri.conf.json` with `"targets": "all"` supports both formats
- [ ] **Build Commands**: Test each platform-specific build command locally
- [ ] **Artifact Paths**: Confirm output paths for AppImage and NSIS bundle formats
- [ ] **Unsigned Strategy**: Confirm that unsigned builds work for both platforms (with expected warnings)

#### **🚀 Phase 5.2.2: Release Integration** - PENDING
- [ ] **Automatic Releases**: Create releases on version tags
- [ ] **Cross-Platform Distribution**: Include AppImage and EXE in release assets
- [ ] **Platform Detection**: Ensure compatibility across Linux distributions and Windows versions

#### **🎯 Benefits**
- ✅ **Solves CachyOS Issue**: Ubuntu 18.04 provides compatible glibc for Linux
- ✅ **Windows Support**: Native NSIS installer (EXE) for Windows users (unsigned - shows security warnings)
- ✅ **Free Builds**: GitHub Actions free for public repositories
- ✅ **Automated**: No manual build process needed for either platform
- ✅ **Cross-Platform**: Works on Linux and Windows desktop platforms
- ✅ **Cost-Effective**: No upfront costs for code signing certificates
- ✅ **Official Support**: Both formats officially supported by Tauri distribution system
- ✅ **Proven Approach**: Based on official Tauri documentation and community best practices
- ✅ **Iterative**: Can add macOS and code signing later if needed

---

## 🔮 **Future Enhancements** (Planned)

### **🍎 macOS Support & Code Signing** (Future)
- [ ] **macOS Build Support**: Add macOS runner to GitHub Actions workflow
- [ ] **macOS DMG Generation**: Implement `pnpm tauri build --bundles dmg` for macOS
- [ ] **Windows Code Signing**: Get code signing certificate to eliminate security warnings
- [ ] **macOS Code Signing**: Get Apple Developer account for notarization and professional distribution
- [ ] **GitHub Actions Integration**: Add signing steps to automated build workflow
- [ ] **Professional Distribution**: Eliminate security warnings for better user experience
- [ ] **App Store Distribution**: Enable Microsoft Store and Mac App Store distribution

### **🔧 Advanced Translation Management**
- [ ] Multi-criteria filtering system (status + type + project + date)
- [ ] Full-text search across source and translated text
- [ ] Advanced statistics dashboard with visual analytics
- [ ] Export/Import capabilities (JSON, CSV, Excel)
- [ ] Bulk operations and batch processing enhancements

### **🌍 Advanced Language Detection**
- [ ] Replace script-based detection with actual language detection
- [ ] Integrate language detection library (e.g., `whatlang`, `langdetect`)
- [ ] Support detection of multiple languages (EN, FR, DE, ES, IT, PT, JP, CN, KR)
- [ ] Handle mixed-language text and confidence scores
- [ ] Enhanced translation direction logic

### **🎨 UI/UX Polish & Performance**
- [ ] Virtual scrolling for large datasets (10,000+ translations)
- [ ] Keyboard shortcuts for power users
- [ ] Enhanced loading states and skeleton screens
- [ ] Improved error handling and user feedback
- [ ] Mobile-responsive design and touch support

### **🧪 Quality Assurance & Testing**
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
- **Text Processing**: Unified `utils/text/` utility with automatic Engine trait integration (Phase 5.0 target)
  - **Automatic Processing**: Engines get text processing for free via trait default implementations
  - **Global Consistency**: Same utility applied to all engines without manual calls
  - **Future-Proof**: New engines inherit text processing capabilities automatically
  - **Engine Agnostic**: Works for all current and future engines without modification

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
