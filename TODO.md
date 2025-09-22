# LudoLingua Development TODO

## 🎉 **Phase 5: Enhanced Features & Polish - IN PROGRESS**

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

### ✅ **Phase 5.2: GitHub Actions Cross-Platform Build Setup** - COMPLETED
- [x] **GitHub Actions Workflow**: Created `.github/workflows/build-cross-platform.yml`
- [x] **Cross-Platform Build**: Uses official `tauri-apps/tauri-action@v0.5.23`
- [x] **Matrix Strategy**: Builds on Linux (Ubuntu 22.04) and Windows in parallel
- [x] **Automatic Releases**: Creates GitHub releases with all platform artifacts
- [x] **Technical Verification**: Uses official action tested by 10.6k+ repositories
- [x] **Update Artifacts**: Already configured with `includeUpdaterJson: true`

### ✅ **Phase 5.2.1: Provider Configuration Simplification** - COMPLETED
**Goal:** Simplify provider configuration by removing unnecessary user input requirements
**Timeline:** 1 day
**Status:** 100% Complete - All Objectives Achieved

### ✅ **Phase 5.2.3: Bulk Retranslation & Row Selection Enhancement** - COMPLETED
**Goal:** Implement row selection and bulk retranslation functionality with native notifications
**Timeline:** 1 day
**Status:** 100% Complete - All Objectives Achieved

### ✅ **Phase 5.2.4: Text Length Filter Enhancement** - COMPLETED
**Goal:** Add text length filtering using Nuxt UI v4 Slider component for enhanced translation management
**Timeline:** 1 day
**Status:** 100% Complete - All Objectives Achieved

#### **✅ Row Selection Implementation - COMPLETED**
- [x] **TranslationResult.vue Enhancement**:
  - [x] Added row selection using official Nuxt UI v4 table patterns
  - [x] Implemented `v-model:row-selection` with proper table API integration
  - [x] Added select-all checkbox in table header using `table.getIsAllPageRowsSelected()`
  - [x] Added individual row checkboxes using `row.getIsSelected()` and `row.toggleSelected()`
  - [x] Created selection counter showing "X of Y row(s) selected" in table footer
  - [x] Used proper TypeScript types and `useTemplateRef` for table API access
  - [x] Fixed row selection counting issue by using `table.tableApi.getFilteredSelectedRowModel()`
- [x] **Bulk Actions UI**:
  - [x] Added bulk actions bar that appears when rows are selected
  - [x] Shows selection count with info badge
  - [x] "Re-translate Selected" button appears only when 2+ rows selected
  - [x] "Clear Selection" button for easy selection reset
  - [x] Proper loading states and disabled states during bulk operations

#### **✅ Bulk Retranslation Workflow - COMPLETED**
- [x] **Enhanced useTranslator.ts**:
  - [x] Added `startBulkRetranslation()` function for selected rows processing
  - [x] Converts selected rows to ProcessRow format for translation workflow
  - [x] Uses same batch translation system as regular translation process
  - [x] Integrates with existing timer and progress tracking
  - [x] Automatically switches to process view during bulk retranslation
- [x] **TranslationResult.vue Integration**:
  - [x] Modified `onBulkRetranslate()` to emit selected rows to parent
  - [x] Added `retranslate-selected` emit event for parent communication
  - [x] Clears selection after emitting data to parent
- [x] **translator.vue Integration**:
  - [x] Added `handleBulkRetranslation()` function to receive selected rows
  - [x] Connected event handler to `TranslationResult` component
  - [x] Passes selected rows to `startBulkRetranslation()` in composable

#### **✅ Native Notifications Integration - COMPLETED**
- [x] **Single Translation Notifications**:
  - [x] Added native notifications to `TranslationResult.vue` for single retranslations
  - [x] Added native notifications to `TranslationEditor.vue` for editor retranslations
  - [x] Shows "Translation Complete" notification after successful single retranslation
- [x] **Bulk Translation Notifications**:
  - [x] Added native notification after bulk retranslation completion
  - [x] Shows success count, total count, and failed count in notification
  - [x] Example: "Bulk Translation Complete: 45/50 translations completed, 5 failed"
  - [x] Integrated with existing `useNotifications` composable

#### **✅ Auto-Navigation Helpers Control - COMPLETED**
- [x] **Simplified Auto-Navigation Logic**:
  - [x] Added `!isBusy` condition to existing "Regular completion message"
  - [x] Auto-navigation helpers now hide during any translation (regular or bulk)
  - [x] Auto-navigation helpers appear after translation completion
  - [x] Works seamlessly for both regular translation and bulk retranslation
  - [x] Uses existing `isBusy` state instead of complex tracking logic

#### **🎯 Benefits Achieved**
- ✅ **Enhanced User Experience**: Row selection with visual feedback and bulk operations
- ✅ **Consistent Workflow**: Bulk retranslation uses same process view as regular translation
- ✅ **Visual Progress**: Users can see translation progress in real-time during bulk operations
- ✅ **Native Notifications**: Desktop notifications for all translation completion events
- ✅ **Simplified Navigation**: Auto-navigation helpers hide during translation, show when complete
- ✅ **Official UI Patterns**: Uses Nuxt UI v4 official table selection patterns for reliability
- ✅ **Integrated Experience**: Bulk retranslation fits naturally into existing translation workflow
- ✅ **Error Handling**: Proper error tracking and notification for failed bulk translations

#### **✅ Text Length Filter Implementation - COMPLETED**
- [x] **TranslationResult.vue Enhancement**:
  - [x] Added text length range slider using Nuxt UI v4 `USlider` component
  - [x] Implemented dual-thumb slider for min/max character count filtering
  - [x] Added dynamic maximum calculation based on actual text lengths in dataset
  - [x] Created live display of current range (e.g., "25-150 chars")
  - [x] Positioned filter next to search input for easy access
  - [x] Added tooltips showing current values when dragging slider thumbs
- [x] **Smart Filtering Logic**:
  - [x] Filters by both source text length AND translated text length
  - [x] Shows translations where either source or translated text falls within range
  - [x] Works in combination with existing search filter for comprehensive filtering
  - [x] Auto-resets pagination when filters change for better UX
- [x] **Dynamic Range Calculation**:
  - [x] Automatically calculates maximum text length from all translations
  - [x] Updates slider range based on actual data (source and translated text)
  - [x] Minimum of 200 characters, scales up if longer texts exist
  - [x] Auto-adjusts range when data changes to maintain usability
- [x] **Enhanced User Experience**:
  - [x] Step size of 5 characters for precise control
  - [x] Smooth interaction with existing bulk selection functionality
  - [x] Clean layout with "Text Length:" label for clarity
  - [x] Responsive design that fits well in header section

#### **🎯 Benefits Achieved**
- ✅ **Enhanced Translation Management**: Users can filter by text length for specific use cases
- ✅ **Quality Control**: Filter short translations (character names, status effects) or long translations (dialogue, descriptions)
- ✅ **Bulk Operations**: Select and retranslate all texts of specific length ranges
- ✅ **Content Analysis**: Understand distribution of text lengths in projects
- ✅ **Visual Control**: Intuitive slider interface with tooltips and live feedback
- ✅ **Dynamic Adaptation**: Automatically adapts to project's text length distribution
- ✅ **Combined Filtering**: Works seamlessly with search and other filters
- ✅ **Performance**: Efficient filtering with proper reactive updates
- ✅ **Accessibility**: Proper ARIA labels and keyboard navigation support

#### **✅ Ollama Provider Simplification - COMPLETED**
- [x] **Backend Service Update** (`src-tauri/src/llm/services/ollama.rs`):
  - [x] Always use `http://localhost:11434` without requiring user configuration
  - [x] Remove base_url dependency from Ollama service constructor
  - [x] Update `config_matches()` to not compare base_url (always same)
  - [x] Update `do_generate_with_usage()` to use hardcoded localhost URL
- [x] **Frontend UI Update** (`app/components/settings/AdvancedSettings.vue`):
  - [x] Hide base_url field for Ollama provider
  - [x] Show read-only "Endpoint" field with localhost URL
  - [x] Add helpful footer text explaining automatic localhost usage
- [x] **Settings Logic Update** (`app/pages/settings.vue`):
  - [x] Always save `http://localhost:11434` as base_url for Ollama
  - [x] Clear base_url field in UI for Ollama (not shown to user)
  - [x] Update provider change watcher to handle simplified logic

#### **✅ RunPod Provider Simplification - COMPLETED**
- [x] **Backend Service Update** (`src-tauri/src/llm/services/runpod.rs`):
  - [x] Accept only pod ID in base_url field (e.g., "abc123")
  - [x] Auto-format pod ID to full RunPod URL (`https://abc123-11434.proxy.runpod.net`)
  - [x] Handle multiple URL formats: pod ID, partial URL, or full URL
  - [x] Improved error messages for empty or invalid pod IDs
- [x] **Frontend UI Update** (`app/components/settings/AdvancedSettings.vue`):
  - [x] Show "Pod ID" field instead of "Base URL" for RunPod provider
  - [x] Update placeholder text to show pod ID example ("your-pod-id")
  - [x] Add helpful footer text explaining automatic URL formatting
- [x] **Settings Logic Update** (`app/pages/settings.vue`):
  - [x] Store pod ID in base_url field for RunPod
  - [x] Clear pod ID field when switching to RunPod provider
  - [x] Update provider change watcher for simplified pod ID handling

#### **✅ Type Definitions Update - COMPLETED**
- [x] **Provider Types Update** (`app/types/provider.ts`):
  - [x] Update documentation for `defaultOllamaConfig` explaining automatic localhost
  - [x] Update documentation for `defaultRunPodConfig` explaining pod ID usage
  - [x] Clarify that backend handles URL formatting automatically

#### **🎯 Benefits Achieved**
- ✅ **Simplified User Experience**: No need to configure localhost for Ollama
- ✅ **Reduced Configuration Errors**: Users only enter pod ID for RunPod, not full URLs
- ✅ **Automatic URL Formatting**: Backend handles URL construction automatically
- ✅ **Cleaner UI**: Provider-specific fields shown only when relevant
- ✅ **Better Error Messages**: Clear guidance on what to enter for each provider
- ✅ **Backward Compatibility**: Existing configurations continue to work
- ✅ **Improved Developer Experience**: Less configuration complexity in code

### ✅ **Phase 5.3: Automatic Updates Integration** - COMPLETED
**Goal:** Add automatic update functionality using [Tauri Updater Plugin](https://tauri.app/plugin/updater/)
**Timeline:** 1-2 weeks
**Status:** 100% Complete - All Objectives Achieved

#### **✅ Phase 5.3.1: Tauri Updater Plugin Setup** - COMPLETED
- [x] **Install Tauri Updater Plugin**:
  - [x] Run `pnpm tauri add updater` in project root
  - [x] Verify plugin installation in `Cargo.toml` and `lib.rs`
  - [x] Install frontend package: `pnpm add @tauri-apps/plugin-updater`
  - [x] Configure permissions in `capabilities/default.json`

#### **✅ Phase 5.3.2: Signing Keys Generation** - COMPLETED
- [x] **Generate Signing Keys**:
  - [x] Run `pnpm tauri signer generate -w ./ludolingua.key` in project directory
  - [x] Store private key securely (never share)
  - [x] Add public key to `tauri.conf.json` configuration
  - [x] Document key management and backup procedures

#### **✅ Phase 5.3.3: Update Server Configuration** - COMPLETED
- [x] **Update Server Setup**:
  - [x] Configure update endpoints in `tauri.conf.json` using GitHub API
  - [x] Set up GitHub API endpoint: `https://api.github.com/repos/blackat/ludolingua/releases/latest`
  - [x] Configure version comparison and update checking logic
  - [x] Test update flow with development builds on Windows and Linux

#### **✅ Phase 5.3.4: Frontend Update UI** - COMPLETED
- [x] **Update Notification UI**:
  - [x] Create `useUpdater.ts` composable with full update management
  - [x] Create `UpdateNotification.vue` component for update alerts
  - [x] Create `UpdateManager.vue` component for settings page
  - [x] Add update check functionality to settings with auto-check on startup
  - [x] Implement download progress indicators and native notifications
  - [x] Add manual update check option with proper error handling

#### **✅ Phase 5.3.5: Release Integration** - COMPLETED
- [x] **Automated Update Releases**:
  - [x] Verify GitHub Actions workflow already generates update artifacts (`includeUpdaterJson: true`)
  - [x] Add `TAURI_SIGNING_PRIVATE_KEY` and `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` environment variables
  - [x] Test update artifacts are properly uploaded to GitHub releases
  - [x] Test complete update flow from check to install on Windows and Linux
  - [x] Validate update process works with existing build workflow

#### **🎯 Benefits**
- ✅ **Automatic Updates**: Users get latest features and bug fixes automatically
- ✅ **Security**: Signed updates ensure authenticity and prevent tampering
- ✅ **User Experience**: Seamless update process without manual downloads
- ✅ **Professional Distribution**: Enterprise-grade update management
- ✅ **Cross-Platform**: Works on Windows and Linux (macOS support can be added later)
- ✅ **Flexible**: Support for multiple release channels (stable, beta)
- ✅ **GitHub Integration**: Leverages existing GitHub releases and API

---

## 🔮 **Future Enhancements** (Planned)

### **🍎 macOS Support & Code Signing** (Future)
- [ ] **macOS Build Support**: Add macOS runner to GitHub Actions workflow
- [ ] **macOS DMG Generation**: Implement `pnpm tauri build --bundles dmg` for macOS
- [ ] **macOS Update Support**: Extend updater configuration to include macOS
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
- **Text Processing**: Unified `utils/text/` utility with automatic Engine trait integration
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
With Phase 5.0-5.2 complete, the application has a solid foundation with automated builds ready for automatic updates in Phase 5.3.

---

**🏆 Status**: Phase 5.0-5.3 complete, automatic updates fully integrated!
