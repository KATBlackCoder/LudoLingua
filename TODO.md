# TODO

## 🚨 Critical Refactoring Tasks

### Component Architecture Refactoring
**Priority: HIGH** - Components are too large and violate DRY principles

#### Current Issues:
- `TranslationResult.vue`: 812 lines (should be ~200)
- ✅ `TranslationTable.vue`: Removed (replaced by modern `TranslationView.vue`) 
- `GlossaryTable.vue`: 282 lines (should be ~100)
- Duplicated code across 5 table components
- 5x fullscreen detection logic duplication
- 4x pagination logic duplication
- 4x table header duplication

#### Refactoring Tasks:

##### 1. Create Shared Composables ✅ COMPLETED
- [x] `composables/useFullscreen.ts` - Extract fullscreen detection logic
- [x] `composables/usePagination.ts` - Create TanStack Table pagination composable
  - [x] Extract TanStack pagination logic from TranslationResult.vue
  - [x] Create unified pagination interface for all table components
  - [x] Add reactive pagination state management
  - [x] Include pagination options for TanStack Table integration
- [x] `composables/useTableSelection.ts` - Extract row selection logic
  - [x] Create pure selection state management
  - [x] Add TanStack Table selection column factory
  - [x] Include selection utilities (clear, selectAll, etc.)
  - [x] NO bulk actions (separate concern)
- [x] `composables/useBulkActions.ts` - Extract bulk operation logic
  - [x] Create bulk action processor
  - [x] Handle action state (loading, disabled, etc.)
  - [x] Support dynamic action labels with counts
- [x] `composables/useModal.ts` - Extract modal state management
- [x] `utils/table.ts` - Pure table utility functions
  - [x] Table column factory functions (no state)
  - [x] Cell formatting utilities (pure functions)
  - [x] Table validation helpers (pure functions)
- [x] `utils/ui.ts` - Pure UI utility functions
  - [x] Badge color mapping functions (pure)
  - [x] Icon mapping utilities (pure)
  - [x] Text formatting helpers (pure)

##### 2. Create Shared Components ✅ COMPLETED
- [x] `components/shared/FilterCard.vue` - Reusable filter card wrapper
- [x] `components/shared/TableHeader.vue` - Standardized table header with pagination
- [x] `components/shared/BulkActions.vue` - Reusable bulk action alerts
- [x] `components/shared/TableFooter.vue` - Standardized table footer with stats
- [x] `components/shared/ActionButtonGroup.vue` - Reusable action button groups
- [x] `components/shared/DataTable.vue` - Complete table with common features

##### 3. Refactor Existing Components
- [x] ✅ `TranslationResult.vue`: Migrated to DataTable component with utility functions
- [x] ✅ `TranslationTable.vue`: Removed (replaced by modern `TranslationView.vue`) 
- [x] ✅ `GlossaryTable.vue`: Migrated to DataTable component with utility functions
- [x] ✅ `TranslationRaw.vue`: Migrated to DataTable component with utility functions
- [x] ✅ `TranslationProcess.vue`: Migrated to DataTable component with utility functions
- [x] ✅ `GlossaryTable.vue`: Migrated to DataTable component with TanStack Table pagination
- [x] ✅ All table components: Standardized to use DataTable component with TanStack Table

##### 4. Refactor Modal Components
- [x] ✅ Create comprehensive `Modal.vue` component in `components/shared/modal/`:
  - [x] ✅ **Main Modal Component**: `Modal.vue` - Complete modal with all features (like DataTable.vue)
    - [x] ✅ Built-in header with icon, title, description, and actions
    - [x] ✅ Built-in body with consistent spacing and layout
    - [x] ✅ Built-in footer with save/cancel buttons and status
    - [x] ✅ Built-in loading and error states
    - [x] ✅ Built-in form validation and submission handling
    - [x] ✅ Built-in keyboard shortcuts (Enter to save, Escape to cancel)
    - [x] ✅ Built-in responsive design and accessibility
  - [x] ✅ **Supporting Components**:
    - [x] ✅ `ModalHeader.vue` - Standardized modal header with icon, title, and description
    - [x] ✅ `ModalActions.vue` - Standardized modal actions with badge display
    - [x] ✅ `ModalBody.vue` - Standardized modal body with consistent spacing
    - [x] ✅ `ModalFooter.vue` - Standardized modal footer with save/cancel buttons and status
    - [x] ✅ `TextCard.vue` - Reusable text display card with character count and styling
    - [x] ✅ `FormCard.vue` - Reusable form input card with header and footer actions
    - [x] ✅ `MetadataCard.vue` - Reusable metadata display card for configuration sections
- [x] ✅ **Modal Component Analysis**: Analyzed existing modal components for compatibility
  - [x] ✅ **TranslationEditor.vue Analysis**: Perfect match for two-column layout with source/translation cards
  - [x] ✅ **TranslationForm.vue Analysis**: Perfect match for two-column layout with metadata integration
  - [x] ✅ **GlossaryForm.vue Analysis**: Perfect match for two-column layout with configuration sections
  - [x] ✅ **Implementation Strategy**: All three components can be replaced with Modal.vue using slots and props
- [ ] Update `TranslationEditor.vue` to use `Modal.vue` component
  - [ ] Replace custom modal structure with `Modal.vue`
  - [ ] Use two-column layout with source text and translation form cards
  - [ ] Leverage built-in loading and error states
  - [ ] Use custom content slots for re-translate button and prompt type selection
- [x] ✅ Update `TranslationForm.vue` to use `Modal.vue` component
  - [x] ✅ Replace custom modal structure with `Modal.vue`
  - [x] ✅ Use two-column layout with source text and translation form cards
  - [x] ✅ Integrate metadata section with status, file path, and field type
  - [x] ✅ Leverage built-in form validation and submission
  - [x] ✅ Move status utilities to Modal.vue for better encapsulation
  - [x] ✅ Use template ref to access Modal's exposed utilities
- [ ] Update `GlossaryForm.vue` to use `Modal.vue` component
  - [ ] Replace custom modal structure with `Modal.vue`
  - [ ] Use two-column layout with source term and target term cards
  - [ ] Add configuration section with category, language, and enabled settings
  - [ ] Leverage built-in modal behavior and keyboard shortcuts
- [ ] Benefits Achieved:
  - [ ] Eliminate 3x modal structure duplication across form components
  - [ ] Consistent modal behavior with shared components
  - [ ] Standardized loading and error states across all modals
  - [ ] Reusable card components for different content types
  - [ ] Centralized modal state management with `useModal.ts`

##### 5. Testing & Validation
- [ ] Test all refactored components individually
- [ ] Verify no functionality is broken
- [ ] Test responsive behavior across screen sizes
- [ ] Validate accessibility features are preserved

#### Expected Benefits:
- **Reduce Total LOC**: From ~2000 lines to ~800 lines
- **Improve Maintainability**: Single source of truth for common patterns
- **Better Testing**: Isolated, focused components with clear separation of concerns
- **Consistent UX**: Standardized table behavior with TanStack Table
- **Performance**: Shared logic, fewer event listeners, optimized pagination
- **Unified API**: All table components use same pagination and selection systems
- **Modular Design**: Separate selection logic from bulk actions for better reusability
- **Shared Utilities**: Common functions like `translation.ts` eliminate code duplication
- **Consistent Styling**: Unified badge colors, icons, and UI patterns across components
- **Modal Consistency**: Standardized modal behavior across all forms with built-in loading/error states
- **Reduced Duplication**: Eliminate 3x modal state management duplication across form components

#### Progress Status:
- ✅ **Phase 1A Complete**: All shared composables and utilities created
- ✅ **Phase 1B Complete**: All shared components created (FilterCard, TableHeader, TableFooter, BulkActions, ActionButtonGroup, DataTable)
- ✅ **Phase 1C Complete**: TranslationResult.vue, TranslationRaw.vue, and TranslationProcess.vue migrated to DataTable
- ✅ **Phase 1D Complete**: GlossaryTable.vue migrated to DataTable with language filter optimization
- ✅ **Phase 1E Complete**: Modal component refactoring - Supporting components created
- ✅ **Phase 1F Complete**: TanStack Table standardization across all components

---

## 🎯 Current Development Tasks

### Translation Features
- [x] Add multiple select filter for prompt types in TranslationResult.vue
- [ ] Implement bulk retranslation functionality
- [ ] Add translation progress indicators
- [ ] Improve translation quality scoring

### UI/UX Improvements
- [ ] Add keyboard shortcuts for common actions
- [ ] Implement drag-and-drop for file operations
- [ ] Add search highlighting in tables
- [ ] Improve mobile responsiveness

### Performance Optimizations
- [ ] Implement virtual scrolling for large datasets
- [ ] Add lazy loading for translation results
- [ ] Optimize bundle size
- [ ] Add caching for frequently accessed data

---

## 🐛 Bug Fixes

### Known Issues
- [ ] Table pagination sometimes resets unexpectedly
- [ ] Filter state not persisted across navigation
- [ ] Memory leaks in long-running translation sessions
- [ ] Inconsistent error handling across components
- [ ] Mixed pagination systems (TanStack vs manual) causing inconsistencies
- [ ] Pagination state not synced between header and footer
- [ ] Row selection logic duplicated across components
- [x] ✅ Bulk action patterns now consistent (TranslationTable.vue removed, using TranslationView.vue)
- [ ] Modal state management duplicated across TranslationEditor, TranslationForm, and GlossaryForm
- [ ] Inconsistent modal loading and error states across form components

---

## 📚 Documentation

### Technical Documentation
- [ ] Update component architecture documentation
- [ ] Add composable usage examples
- [ ] Document shared component APIs
- [ ] Create refactoring migration guide

### User Documentation
- [ ] Update user manual with new features
- [ ] Add troubleshooting section
- [ ] Create video tutorials for key workflows
