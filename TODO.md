# LudoLingua Development TODO

## 🎉 **Phase 6: Advanced Features & Polish - READY TO START**

### ✅ **Phase 5: Enhanced Features & Polish** - COMPLETED
**Goal:** Add advanced features and polish the user experience
**Status:** 100% Complete - All Objectives Achieved
**Timeline:** 5-6 weeks completed successfully

**Completed Sub-phases:**
- ✅ **Phase 5.0**: Code Architecture Refinement - COMPLETED
- ✅ **Phase 5.1**: Native Notifications Integration - COMPLETED  
- ✅ **Phase 5.2**: GitHub Actions Cross-Platform Build Setup - COMPLETED
- ✅ **Phase 5.2.1**: Provider Configuration Simplification - COMPLETED
- ✅ **Phase 5.2.3**: Bulk Retranslation & Row Selection Enhancement - COMPLETED
- ✅ **Phase 5.2.4**: Text Length Filter Enhancement - COMPLETED
- ✅ **Phase 5.3**: Automatic Updates Integration - COMPLETED
- ✅ **Phase 5.4**: Sugoi Provider Integration - COMPLETED
- ✅ **Phase 5.5**: JSON Formatting Preservation Fix - COMPLETED
- ✅ **Phase 5.6**: Text Processing Architecture Enhancement - COMPLETED
- ✅ **Phase 5.7**: Update System Enhancement - COMPLETED
- ✅ **Phase 5.8**: UI Component Enhancement - COMPLETED
- ✅ **Phase 5.9**: Cross-Platform Update Enhancement - COMPLETED

---

## ✅ **Phase 6: Nuxt UI v4 Shared Architecture** - COMPLETED
**Goal:** Leverage Nuxt UI v4's built-in components and design system for consistent UX and reduced code duplication
**Timeline:** 1 week
**Status:** 100% Complete - All Objectives Achieved

### **✅ Nuxt UI v4 Shared Architecture** - COMPLETED
- ✅ **Nuxt UI v4 Integration**: Leverage [Nuxt UI v4](https://ui.nuxt.com/) components and design system
  - ✅ **UTable Component**: Use Nuxt UI's high-performance table with built-in virtualization, sorting, and filtering
  - ✅ **Design System**: Implement semantic color system and CSS-first configuration
- ✅ **Shared State Management**: Create stores and composables leveraging Nuxt UI v4 patterns
  - ✅ **Shared Stores**: Create stores for consistent state management across components
    - ✅ **Table Store**: Create `stores/shared/useTableStore.ts` for table state (pagination, sorting, filtering, selection)
  - ✅ **Shared Composables**: Create composables using Nuxt UI v4 patterns
    - ✅ **Table Composable**: Create `composables/shared/useTable.ts` using Nuxt UI's table API
  - ✅ **Test Implementation**: Create test components using Nuxt UI v4 components
    - ✅ **Test Table Component**: Create `components/test/TestTable.vue` using UTable for validation
    - ✅ **Test Page**: Create `pages/test-shared.vue` to demonstrate Nuxt UI v4 components
    - ✅ **Test Navigation**: Add test page link to `AppHeader.vue` navigation menu
    - ✅ **Test Data**: Create mock data and test scenarios for shared components
    - ✅ **Architecture Validation**: Verify Nuxt UI v4 components work correctly before refactoring
- ✅ **Component Refactoring**: Refactor existing components to use Nuxt UI v4 components
  - ✅ **Table Components**: Refactor to use UTable with built-in features
    - ✅ **TranslationResult.vue**: Use UTable with virtualization, sorting, and row selection
    - ✅ **TranslationRaw.vue**: Use UTable with simplified columns and pagination
    - ✅ **TranslationProcess.vue**: Use UTable with progress indicators and status badges
    - ✅ **TranslationTable.vue**: Use UTable with translation-specific features
    - ✅ **GlossaryTable.vue**: Use UTable with glossary-specific features
  - ✅ **Translation Components Unification**: Merge using Nuxt UI v4 patterns
    - ✅ **Unified Translation Component**: Create `components/translator/TranslationView.vue` with mode switching
    - ✅ **Mode-based Rendering**: Support 'raw', 'process', 'result' modes using UTable variants
    - ✅ **Shared Table Logic**: Use Nuxt UI's table API for pagination, filtering, and selection
    - ✅ **Component Consolidation**: Replace three separate components with one unified component
    - ✅ **Design System Integration**: Use semantic colors and CSS-first configuration
- ✅ **State Management**: Implement shared stores for consistent UX across app

### **🚀 Nuxt UI v4 Benefits for LudoLingua**
- **High-Performance Tables**: Built-in virtualization for 2000+ translation units
- **Accessibility**: WAI-ARIA compliance with Reka UI primitives
- **Design System**: Semantic color system with CSS-first configuration
- **Type Safety**: Full TypeScript support with auto-completion
- **Performance**: 5x faster builds with Tailwind CSS v4
- **Icon System**: 200,000+ icons from Iconify with Lucide integration
- **Dark Mode**: Automatic theme switching with CSS variables
- **Responsive Design**: Mobile-first approach with consistent breakpoints

---

## ✅ **Phase 7: Store & Command Consolidation** - COMPLETED
**Goal:** Consolidate overlapping stores and backend commands for better architecture and maintainability
**Timeline:** 1-2 weeks
**Status:** 100% Complete - All Objectives Achieved

### **✅ Phase 7 Objectives - COMPLETED**
- ✅ **Frontend Store Consolidation**: Merge `translations.ts` and `translator.ts` into unified `translation.ts` store
  - ✅ **Unified Store**: Create `stores/translation.ts` combining CRUD and workflow operations
  - ✅ **State Management**: Merge translation data, progress tracking, and error handling
  - ✅ **API Methods**: Combine fetch, update, delete, and translate operations
  - ✅ **Type Safety**: Ensure proper TypeScript types for all operations
  - ✅ **Backward Compatibility**: Maintain existing component interfaces during transition
- ✅ **Backend Command Consolidation**: Merge `translations.rs` and `translator.rs` into unified `translation.rs`
  - ✅ **Unified Commands**: Create `commands/translation.rs` with all translation operations
  - ✅ **Command Structure**: Organize commands by functionality (CRUD, AI, batch operations)
  - ✅ **Error Handling**: Consolidate error handling patterns
  - ✅ **Database Operations**: Merge database access patterns
  - ✅ **AI Integration**: Integrate AI translation with CRUD operations
- ✅ **Component Updates**: Update components to use unified store
  - ✅ **Translation Components**: Update all translation-related components
  - ✅ **Store Imports**: Update import statements across the application
  - ✅ **Method Calls**: Update method calls to use unified API
  - ✅ **Testing**: Verify all functionality works with unified store
- ✅ **Architecture Benefits**: Achieve cleaner, more maintainable codebase
  - ✅ **Reduced Complexity**: Single store for all translation operations
  - ✅ **Better Cohesion**: Related functionality in one place
  - ✅ **Simplified API**: Fewer commands to maintain
  - ✅ **Consistent State**: Single source of truth for translation data
  - ✅ **Easier Testing**: One store to test instead of two

### **🎯 Consolidation Strategy**

#### **Frontend Store Consolidation:**
```
Current:                    →    Unified:
┌─────────────────┐              ┌─────────────────┐
│  translations.ts │              │  translation.ts │
│  • CRUD ops     │              │  • CRUD ops     │
│  • Database     │              │  • AI workflow  │
└─────────────────┘              │  • Progress     │
┌─────────────────┐              │  • State mgmt   │
│  translator.ts  │              │  • Error handling│
│  • AI workflow  │              └─────────────────┘
│  • Progress     │
│  • State mgmt   │
└─────────────────┘
```

#### **Backend Command Consolidation:**
```
Current:                    →    Unified:
┌─────────────────┐              ┌─────────────────┐
│ translations.rs │              │  translation.rs │
│  • list_translations │         │  • list_translations │
│  • get_translation   │         │  • get_translation   │
│  • update_translation│         │  • update_translation│
│  • delete_translation│         │  • delete_translation│
└─────────────────┘              │  • translate_text_unit│
┌─────────────────┐              │  • batch_translate   │
│  translator.rs  │              │  • get_stats         │
│  • translate_text_unit│         └─────────────────┘
│  • AI integration│
└─────────────────┘
```

#### **Unified Store Structure:**
```typescript
// stores/translation.ts
export const useTranslationStore = defineStore('translation', () => {
  // State (combined from both stores)
  const translations = ref<TextUnitRecord[]>([])
  const isLoading = ref(false)
  const error = ref<string | null>(null)
  const stats = ref<Record<string, unknown> | null>(null)
  
  // Translation process state
  const isTranslating = ref(false)
  const translationProgress = ref(0)
  const translationTotal = ref(0)
  const currentTranslatingUnit = ref<TextUnit | null>(null)
  const failedTranslations = ref<Array<{ unit: TextUnit; error: string }>>([])
  
  // CRUD Operations (from translations.ts)
  const fetchTranslations = async (query: TextUnitQuery = {}) => { ... }
  const getTranslation = async (id: number) => { ... }
  const updateTranslation = async (id: number, translatedText: string, status?: TranslationStatus) => { ... }
  const deleteTranslation = async (id: number) => { ... }
  const bulkDeleteTranslations = async (ids: number[]) => { ... }
  const fetchStats = async (projectPath?: string) => { ... }
  
  // Translation Process Operations (from translator.ts)
  const translateTextUnit = async (textUnit: TextUnit) => { ... }
  const startBatchTranslation = async (textUnits: TextUnit[], onUnitTranslated?: (unit: TextUnit) => void) => { ... }
  const reset = () => { ... }
  
  // Computed properties
  const isTranslationInProgress = computed(() => isTranslating.value)
  const translationProgressPercentage = computed(() => { ... })
  const hasFailedTranslations = computed(() => failedTranslations.value.length > 0)
  
  return {
    // State
    translations, isLoading, error, stats,
    isTranslating, translationProgress, translationTotal, currentTranslatingUnit, failedTranslations,
    
    // Computed
    isTranslationInProgress, translationProgressPercentage, hasFailedTranslations,
    
    // CRUD Actions
    fetchTranslations, getTranslation, updateTranslation, deleteTranslation, bulkDeleteTranslations, fetchStats,
    
    // Translation Actions
    translateTextUnit, startBatchTranslation, reset,
  }
})
```

---

## ✅ **Phase 8: Enhanced Universal Table Architecture** - COMPLETED
**Goal:** Create enhanced universal table system with display mode management, custom data processing, export functionality, and statistics
**Timeline:** 1-2 weeks
**Status:** 100% Complete - All Objectives Achieved

### **✅ Phase 8 Objectives - COMPLETED**
- ✅ **Table Logic Consolidation**: Move table-specific logic from domain composables to shared `useTable.ts`
  - ✅ **Extract Table Logic**: Move pagination, filtering, sorting from `useTranslations.ts` to `useTable.ts`
  - ✅ **Domain Filtering**: Enhance `useTable.ts` with domain-specific filtering capabilities
  - ✅ **Shared Table State**: Centralize table state management in `useTableStore.ts`
  - ✅ **Composable Separation**: Keep domain logic in domain composables, table logic in shared composables
- ✅ **Enhanced Universal Table Architecture**: Create universal `BaseTable.vue` with enhanced `useTable.ts` utility
  - ✅ **Enhanced useTable.ts**: Add display mode management, custom data processing, export functionality
  - ✅ **Display Mode Support**: Add support for "paginated" vs "all data" display modes with toggle
  - ✅ **Custom Logic Support**: Allow custom data processors, filter processors, and pagination processors
  - ✅ **Enhanced Table Features**: Add export functionality, table statistics, reset functionality
  - ✅ **Pure Table Utility**: `useTable.ts` only used in `BaseTable.vue`, not directly in components
- ✅ **Component Refactoring**: Update components to use enhanced `BaseTable.vue` with custom processors
  - ✅ **BaseTable.vue Enhancement**: Updated to use enhanced `useTable.ts` with all new features
  - ✅ **Display Mode Toggle**: Added toggle button for switching between paginated and all data modes
  - ✅ **Export Functionality**: Added export button with CSV, JSON, and XLSX support
  - ✅ **Statistics Display**: Added statistics section showing total, filtered, selected items and pages
  - ✅ **Reset Functionality**: Added reset button with enhanced reset capabilities
  - ✅ **Enhanced Footer**: Updated footer to show display mode and conditional pagination
- ✅ **Architecture Benefits**: Achieve universal table system with custom processor pattern
  - ✅ **Universal Table**: Single `BaseTable.vue` component handles all table use cases
  - ✅ **Custom Processor Pattern**: Components provide custom data/filter/pagination processors as props
  - ✅ **Pure Table Utility**: `useTable.ts` only used in `BaseTable.vue`, not directly in components
  - ✅ **Enhanced Features**: Display mode toggle, export functionality, table statistics, reset functionality
  - ✅ **Backward Compatibility**: Existing `BaseTable` usage continues to work
  - ✅ **Code Reuse**: Eliminate duplicate table logic across components
  - ✅ **Maintainability**: Single source of truth for table functionality

### **✅ Phase 8 Progress - All Steps Completed**

#### **✅ Step 1: Enhanced `useTable.ts` with domain-specific filtering capabilities**
- ✅ **Added domain-specific filter interface**: Added `domainFilters` and `customFilters` to `TableConfig`
- ✅ **Enhanced filter logic**: Added support for status, promptType, textLength, and fieldType filters
- ✅ **Domain-specific filter methods**: Added `setStatusFilter`, `setPromptTypeFilter`, `setTextLengthFilter`, `setFieldTypeFilter`, and `applyCustomFilter`
- ✅ **Updated table store**: Enhanced `useTableStore.ts` with domain-specific filtering logic for translation data

#### **✅ Step 2: Refactored `useTranslations.ts` to remove table logic**
- ✅ **Removed table state**: Removed `search`, `statusFilter`, `promptTypeFilter`, `page`, `pageSize` UI state
- ✅ **Removed table computed**: Removed `filteredTranslations`, `pagedTranslations`, `pageCount` computed properties
- ✅ **Removed table methods**: Removed `clearFilters()` method
- ✅ **Kept business logic**: Preserved translation CRUD operations, store integration, and utility functions
- ✅ **Clean separation**: Now focuses purely on translation business logic, not table management

#### **✅ Step 3: Updated `BaseTable.vue` with domain-specific filter slots**
- ✅ **Added domain filter UI**: Added status, prompt type, and field type filter components
- ✅ **Enhanced props**: Added `showDomainFilters`, `showStatusFilter`, `showPromptTypeFilter`, `showFieldTypeFilter` props
- ✅ **Added filter options**: Added `statusOptions` and `promptTypeOptions` props
- ✅ **Domain filter methods**: Added `onStatusFilterChange`, `onPromptTypeFilterChange`, `onFieldTypeFilterChange` methods
- ✅ **Event handling**: Proper event handling for domain-specific filter changes
- ✅ **Integration**: Connected domain filters to the shared `useTable` composable

#### **✅ Step 4: Enhanced `useTable.ts` with universal table features**
- ✅ **Display Mode Management**: Added support for "paginated" vs "all data" display modes with toggle functionality
- ✅ **Custom Data Processing**: Added support for custom data processors, filter processors, and pagination processors
- ✅ **Export Functionality**: Added CSV, JSON, and XLSX export capabilities with progress tracking
- ✅ **Table Statistics**: Added comprehensive statistics including total, filtered, selected items and pages
- ✅ **Enhanced Reset**: Added enhanced reset functionality with auto-reset on data change
- ✅ **Type Safety**: Full TypeScript support with proper interfaces and type definitions

#### **✅ Step 5: Enhanced `BaseTable.vue` with universal table features**
- ✅ **Display Mode Toggle**: Added toggle button for switching between paginated and all data modes
- ✅ **Export Functionality**: Added export button with CSV, JSON, and XLSX support
- ✅ **Statistics Display**: Added statistics section showing total, filtered, selected items and pages
- ✅ **Reset Functionality**: Added reset button with enhanced reset capabilities
- ✅ **Enhanced Footer**: Updated footer to show display mode and conditional pagination
- ✅ **Enhanced Props**: Added props for display mode, export options, statistics, and reset functionality
- ✅ **Enhanced Expose**: Updated defineExpose to include all enhanced features

### **🏗️ Refactoring Strategy**

#### **✅ Current Architecture (Clean Separation Achieved):**
```
useTranslations.ts:
├── Translation business logic ✅
├── Translation data fetching ✅
├── Translation CRUD operations ✅
└── Translation utilities ✅

useTable.ts:
├── Table pagination logic ✅
├── Table filtering logic ✅
├── Table sorting logic ✅
├── Table state management ✅
└── Domain-specific filtering ✅

BaseTable.vue:
├── Table UI components ✅
├── Filter UI components ✅
├── Pagination UI components ✅
├── Domain-specific filter UI ✅
├── Uses useTable.ts internally ✅
└── Reusable table features ✅
```

#### **✅ Enhanced Universal Architecture - COMPLETED:**
```
Enhanced useTable.ts:
├── Display mode management ✅ (completed)
├── Custom data processing ✅ (completed)
├── Export functionality ✅ (completed)
├── Table statistics ✅ (completed)
└── Enhanced table utilities ✅ (completed)

Enhanced BaseTable.vue:
├── Display mode toggle ✅ (completed)
├── Custom processor props ✅ (completed)
├── Export functionality ✅ (completed)
├── Enhanced table features ✅ (completed)
└── Universal component ✅ (completed)

Component Migration:
├── TranslationTable.vue → Enhanced BaseTable with custom processors ⏳ (next step)
├── TranslationRaw.vue → Enhanced BaseTable with custom data processor ⏳ (future step)
├── TranslationResult.vue → Enhanced BaseTable with custom data processor ⏳ (future step)
└── GlossaryTable.vue → Enhanced BaseTable with custom processors ⏳ (future step)
```

#### **Implementation Steps:**
1. ✅ **Enhance `useTable.ts`**: Add domain-specific filtering capabilities
2. ✅ **Refactor `useTranslations.ts`**: Remove table logic, keep only translation business logic
3. ✅ **Update `BaseTable.vue`**: Enhance with domain-specific filter slots
4. ✅ **Enhance `useTable.ts`**: Add display mode management, custom data processing, export functionality
5. ✅ **Enhance `BaseTable.vue`**: Add display mode toggle, custom processor props, enhanced features
6. ⏳ **Migrate TranslationTable.vue**: Update TranslationTable.vue to use enhanced `BaseTable.vue` with custom processors (next step)
7. ⏳ **Migrate Remaining Components**: Update other table components to use enhanced `BaseTable.vue` (future step)
8. ⏳ **Test Architecture**: Verify all functionality works with universal table system (future step)
9. ⏳ **Cleanup**: Remove old specialized components after successful migration (future step)

### **🎯 Architecture Benefits Achieved**

#### **Clean Separation of Concerns**
- **Domain Logic**: Translation business logic stays in `useTranslations.ts`
- **Table Logic**: All table functionality centralized in `useTable.ts`
- **UI Logic**: Standardized table UI in `BaseTable.vue` with domain-specific filters
- **Code Reuse**: Eliminated duplicate table logic across components
- **Maintainability**: Single source of truth for table functionality

#### **Key Improvements**
- **Type Safety**: Proper TypeScript types for all operations
- **Performance**: Optimized filtering with domain-specific logic
- **Accessibility**: Built-in WAI-ARIA compliance with Reka UI primitives
- **Responsive Design**: Mobile-first approach with consistent breakpoints
- **Dark Mode**: Automatic theme switching with CSS variables
- **Icon System**: 200,000+ icons from Iconify with Lucide integration

#### **Ready for Enhanced Universal Architecture**
The shared architecture is now ready for the next phase where we can create a universal table system that supports:
- **Enhanced useTable.ts**: Display mode management, custom data processing, export functionality
- **Display Modes**: "Paginated" vs "All Data" with toggle functionality
- **Custom Processor Pattern**: Components provide custom data/filter/pagination processors as props
- **Enhanced Features**: Export functionality, table statistics, reset functionality
- **Universal Component**: Single `BaseTable.vue` handles all table use cases with custom processors

---

## 🎯 **Phase 9: TranslationTable.vue Migration** - READY TO START
**Goal:** Migrate TranslationTable.vue to use enhanced BaseTable.vue with custom processors
**Timeline:** 1-2 days
**Status:** Ready to Start

### **🎯 Phase 9 Objectives**
- [ ] **Replace UTable with BaseTable**: Replace direct UTable usage with enhanced BaseTable component
- [ ] **Remove Duplicate State Management**: Remove local state management that conflicts with BaseTable's enhanced state
- [ ] **Leverage Enhanced Features**: Enable display mode toggle, statistics, and reset capabilities
- [ ] **Update Props Interface**: Use enhanced props interface from BaseTable.vue
- [ ] **Simplify Event Handling**: Update event handling to use BaseTable's standardized approach
- [ ] **Test Migration**: Verify all existing functionality works with enhanced BaseTable
- [ ] **Code Reduction**: Eliminate ~200 lines of duplicate table logic

### **🎯 Migration Benefits**

#### **Enhanced Features Gained:**
- **Display Mode Toggle**: Switch between paginated and all data views
- **Table Statistics**: Real-time statistics display
- **Enhanced Filtering**: Domain-specific filtering with custom processors
- **Reset Functionality**: Enhanced reset with auto-reset on data change
- **Performance**: Optimized table rendering with virtualization
- **Accessibility**: Built-in WAI-ARIA compliance
- **Responsive Design**: Mobile-first approach with consistent breakpoints

#### **Code Reduction:**
- **Remove ~200 lines** of duplicate table logic
- **Eliminate custom filtering** implementation
- **Remove manual pagination** logic
- **Simplify state management**

#### **Maintainability:**
- **Single source of truth** for table functionality
- **Consistent UX** across all table components
- **Easier testing** with standardized table behavior
- **Future-proof** with enhanced table features

### **🎯 Implementation Steps:**
1. [ ] **Replace UTable with BaseTable**: Update template to use BaseTable component
2. [ ] **Remove Duplicate State**: Remove local state management that conflicts with BaseTable
3. [ ] **Update Props**: Configure BaseTable with enhanced features
4. [ ] **Update Event Handlers**: Simplify event handling to use BaseTable's approach
5. [ ] **Test Migration**: Verify all existing functionality works
6. [ ] **Leverage Enhanced Features**: Enable statistics, display mode toggle
7. [ ] **Code Cleanup**: Remove duplicate table logic and unused code

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
- **Shared Component Architecture**: Reusable shared components for consistent UX
  - **Shared Table Component**: Unified table behavior with pagination, filtering, selection
  - **Shared Modal Component**: Consistent modal patterns with form handling
  - **Shared State Management**: Common stores for table and modal state
  - **Composable Pattern**: Reusable logic for table and modal operations

### **Quality Standards:**
- ✅ No TypeScript errors or warnings
- ✅ Clean, consistent code structure
- ✅ Proper error handling with native dialogs
- ✅ Responsive design with proper spacing
- ✅ Modern UI patterns and best practices

---

**🏆 Status**: Phase 8 complete! All enhanced universal table architecture objectives achieved. Ready for Phase 9: TranslationTable.vue migration to enhanced BaseTable.vue.
