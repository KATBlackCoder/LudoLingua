# LudoLingua Development Roadmap

## Current Status: Phase 4 - Cleanup & Modernization 🧹

---

## ✅ **Phase 1: Project Setup & Core Foundation** - COMPLETED
**Goal:** Establish a solid, working project structure with all necessary dependencies and a basic UI shell.

- [x] Nuxt.js + Tauri integration
- [x] Basic UI components and navigation
- [x] Backend command system
- [x] Project structure and architecture

---

## ✅ **Phase 2: File Handling & Data Parsing** - COMPLETED
**Goal:** Implement the ability to open RPG Maker projects, read data files, and display extracted text.

- [x] RPG Maker MV/MZ project detection
- [x] JSON parsing and text extraction
- [x] Project loading and validation
- [x] Basic text display interface

---

## ✅ **Phase 3: Translation Core & Smart Features** - COMPLETED
**Goal:** Integrate AI translation functionality and build comprehensive translation workflow.

- [x] LLM integration (Ollama, RunPod)
- [x] Translation pipeline with glossary support
- [x] Smart project loading and export system
- [x] Database-backed translation persistence

---

## 🔄 **Phase 4: Cleanup & Modernization** - IN PROGRESS
**Goal:** Clean up naming inconsistencies, upgrade to latest tech stack, rebuild translation management.
**Status:** 25% Complete - File Renaming Done
**Platform:** Desktop Application

### ✅ **Phase 4.1: File Renaming for Consistency** - COMPLETED
**Goal:** Eliminate confusion between translation workflow vs translation management

- [x] **Directory Rename**: `app/components/translation/` → `app/components/translator/`
- [x] **Page Rename**: `app/pages/translation.vue` → `app/pages/translator.vue`
- [x] **Navigation Updates**: Updated all route references from `/translation` to `/translator`
- [x] **Import Updates**: Fixed all import statements across components
- [x] **Clear Naming**: Translator (workflow) vs Translations (management)

**Benefits Achieved:**
- ✅ No more naming confusion
- ✅ Clean, consistent naming convention
- ✅ Better code organization

### 🔄 **Phase 4.2: Nuxt 4.1 Upgrade** - PENDING
**Goal:** Upgrade from Nuxt 4.0 to 4.1 with latest features
**Reference:** [Nuxt 4.1 Release](https://nuxt.com/blog/v4-1)

**New Features to Leverage:**
- **Enhanced Chunk Stability**: Import maps prevent cascading hash changes
- **Experimental Rolldown**: Rust-powered bundling for faster builds
- **Improved Lazy Hydration**: Better component loading performance
- **Module Development**: Enhanced tools for better development experience

**Steps:**
- [ ] Update `package.json` dependencies to Nuxt 4.1
- [ ] Run `pnpm install` and test build process
- [ ] Update `nuxt.config.ts` if needed for new features
- [ ] Test all existing functionality
- [ ] Consider enabling Rolldown experimental support

### 🔄 **Phase 4.3: Nuxt UI v4 Migration** - PENDING
**Goal:** Migrate from Nuxt UI v3 to v4
**Reference:** [Nuxt UI v4 Migration Guide](https://ui4.nuxt.com/docs/getting-started/migration/v4)

**Key Improvements:**
- **Unified Package**: Pro components now included for free
- **Component API Updates**: New props, slots, and improved DX
- **Enhanced TypeScript**: Better type definitions and IntelliSense
- **New Styling System**: Improved theming and customization

**Migration Steps:**
- [ ] Update `package.json`: `"@nuxt/ui": "^4.0.0"`
- [ ] Review breaking changes in migration guide
- [ ] Update component usage across all pages and components
- [ ] Test glossary functionality (our reference implementation)
- [ ] Update custom styling and theming
- [ ] Verify TypeScript compatibility

### 🔄 **Phase 4.4: Rebuild Translation Management** - PENDING
**Goal:** Create simple, working translation management using proven glossary patterns
**Reference:** Working `app/components/glossaries/` implementation

**Strategy:**
- **Follow Glossary Pattern**: Use `GlossaryTable.vue` and `GlossaryForm.vue` as templates
- **Simple CRUD**: List, view, edit, delete translations without complexity
- **Clean Architecture**: Store + composable + components pattern that works
- **Working TypeScript**: No complex type issues or module resolution problems
- **Nuxt UI v4**: Use latest components and established patterns

**Components to Create:**
- [ ] `app/components/translations/TranslationTable.vue` (based on GlossaryTable)
- [ ] `app/components/translations/TranslationForm.vue` (based on GlossaryForm)
- [ ] `app/pages/translations.vue` (simple page like glossary)
- [ ] `app/composables/useTranslations.ts` (based on useGlossary)
- [ ] Add navigation integration

---

## 🎨 **Phase 5: Enhanced Features & Polish** - PLANNED
**Goal:** Add advanced features and polish the user experience
**Status:** PLANNED - Post modernization completion
**Priority:** MEDIUM

### 🔧 Advanced Translation Management
- [ ] **Advanced Filtering**: Multi-criteria filtering (status, type, project, date)
- [ ] **Full-Text Search**: Search across source and translated text
- [ ] **Bulk Operations**: Multi-select actions and batch processing
- [ ] **Export/Import**: JSON/CSV export and import capabilities
- [ ] **Statistics Dashboard**: Visual progress tracking and metrics

### 🎨 UI/UX Improvements
- [ ] **Performance Optimization**: Virtual scrolling for large datasets
- [ ] **Keyboard Shortcuts**: Power user keyboard navigation
- [ ] **Loading States**: Better skeleton screens and loading indicators
- [ ] **Error Handling**: User-friendly error messages and recovery
- [ ] **Mobile Support**: Responsive design for touch devices

### 🧪 Quality & Testing
- [ ] **Unit Tests**: Component and composable testing
- [ ] **Integration Tests**: End-to-end workflow testing
- [ ] **Performance Tests**: Large dataset handling validation
- [ ] **Cross-Platform**: Windows/macOS/Linux compatibility testing

---

## 📈 **Development Timeline & Progress**

### ⏱️ Phase 4 Timeline (Current)
- **✅ Phase 4.1**: File Renaming (30 minutes) - **COMPLETED**
- **🔄 Phase 4.2**: Nuxt 4.1 Upgrade (1-2 hours) - **NEXT**
- **📋 Phase 4.3**: Nuxt UI v4 Migration (2-3 hours)
- **📋 Phase 4.4**: Rebuild Translation Management (3-4 hours)
- **Total**: 6-9 hours estimated

### 🎯 Success Metrics
- **✅ Naming Consistency**: All files use clear, consistent naming
- **📋 Modern Stack**: Latest Nuxt 4.1 and Nuxt UI v4
- **📋 Working Features**: All existing functionality preserved and improved
- **📋 Translation Management**: Simple, working CRUD interface
- **📋 Clean Codebase**: No TypeScript errors or warnings

### 📊 Current Progress
```
Phase 4 Progress: 25% Complete
├── ✅ File Renaming (100%) 
├── 📋 Nuxt 4.1 Upgrade (0%) - NEXT
├── 📋 Nuxt UI v4 Migration (0%)
└── 📋 Translation Management Rebuild (0%)
```

---

## 🏗️ **Technical Architecture**

### **Current Naming Convention (Established)**
- **`translator.rs/ts`** → Translation workflow (LLM calls, AI processing, state management)
- **`translations.rs/ts`** → Translation management (CRUD operations for saved translations)
- **Clear separation** between workflow and management concerns

### **Technology Stack**
- **Backend**: Rust with Tauri, SQLite database, LLM integration
- **Frontend**: Nuxt 4.1, Nuxt UI v4, TypeScript, Pinia state management
- **Build**: Vite with optional Rolldown experimental support
- **Patterns**: Composable-based architecture, proven by glossary implementation

### **Database Foundation**
- **✅ Complete**: All CRUD operations, bulk operations, statistics
- **✅ Optimized**: Efficient queries and proper indexing
- **✅ Reliable**: Tested with large datasets and proven stability

---

## 🚀 **Next Steps**

**Immediate Priority**: Phase 4.2 - Nuxt 4.1 Upgrade
1. Update package.json dependencies
2. Test build and development workflow
3. Consider enabling new features (Rolldown, import maps)
4. Verify all existing functionality

**Then**: Phase 4.3 - Nuxt UI v4 Migration
**Finally**: Phase 4.4 - Rebuild Translation Management

This methodical approach ensures we have a solid, modern foundation before rebuilding features, leading to more maintainable and reliable code.