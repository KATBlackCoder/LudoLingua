# LudoLingua TODO.md - Development Roadmap
**Last Updated:** Phase 4 - Cleanup & Modernization 🧹
**Status:** 75% Complete - Framework Modernization ✅ Nuxt 4.1 + UI v4

---

# 📋 Current Phase: Cleanup & Modernization

## 🎯 **Phase 4: Cleanup & Modernization Plan**
**Goal:** Clean up naming inconsistencies, upgrade to latest tech stack, rebuild translation management
**Priority:** HIGH - Foundation for future development
**Platform:** Desktop Application

### ✅ **Phase 4.1: File Renaming - COMPLETED**
- [x] **Directory Rename**: `app/components/translation/` → `app/components/translator/`
- [x] **Page Rename**: `app/pages/translation.vue` → `app/pages/translator.vue`
- [x] **Navigation Updates**: Updated all route references from `/translation` to `/translator`
- [x] **Import Updates**: Fixed all import statements across components
- [x] **Consistency Achieved**: Clear distinction between translator (workflow) vs translations (management)

**Benefits:**
- ✅ No more naming confusion between workflow and management
- ✅ Clean, consistent naming convention throughout app
- ✅ Better code organization and maintainability

### ✅ **Phase 4.2: Nuxt 4.1 Upgrade - COMPLETED**
**Goal:** Upgrade from Nuxt 4.0 to 4.1 with latest features
**Reference:** [Nuxt 4.1 Release](https://nuxt.com/blog/v4-1)

**New Features Gained:**
- **Enhanced Chunk Stability**: Import maps for better caching
- **Experimental Rolldown**: Rust-powered bundling for faster builds  
- **Improved Lazy Hydration**: Better component loading
- **Performance Optimizations**: Route rules cache management

**Steps Completed:**
- [x] Update `package.json` dependencies to Nuxt 4.1.1
- [x] Run `pnpm install` and test build process
- [x] Update `nuxt.config.ts` if needed for new features
- [x] Test all existing functionality
- [x] Consider enabling Rolldown experimental support

### ✅ **Phase 4.3: Nuxt UI v4 Migration - COMPLETED** 
**Goal:** Migrate from Nuxt UI v3 to v4
**Reference:** [Nuxt UI v4 Migration Guide](https://ui4.nuxt.com/docs/getting-started/migration/v4)

**Key Changes Applied:**
- **Unified Package**: Pro components now included
- **Component API Updates**: New props and slots
- **Enhanced TypeScript**: Better type definitions
- **New Styling System**: Improved theming

**Migration Steps Completed:**
- [x] Update `package.json`: `"@nuxt/ui": "4.0.0-alpha.1"`
- [x] Review breaking changes in migration guide
- [x] Update component usage across all pages
- [x] Test glossary functionality (our reference implementation)
- [x] Update custom styling and theming
- [x] Verify TypeScript compatibility

### 🔄 **Phase 4.4: Rebuild Translation Management - PENDING**
**Goal:** Create simple, working translation management using glossary patterns
**Reference:** Proven `app/components/glossaries/` implementation

**Strategy:**
- **Follow Glossary Pattern**: Use `GlossaryTable.vue` as template
- **Simple CRUD**: List, view, edit, delete translations
- **Clean Architecture**: Store + composable + components pattern
- **Working TypeScript**: No complex type issues
- **Nuxt UI v4**: Use latest components and patterns

**Components to Create:**
- [ ] `app/components/translations/TranslationTable.vue` (based on GlossaryTable)
- [ ] `app/components/translations/TranslationForm.vue` (based on GlossaryForm)  
- [ ] `app/pages/translations.vue` (simple page like glossary)
- [ ] `app/composables/useTranslations.ts` (based on useGlossary)
- [ ] Navigation integration

---

# 🧪 **Testing & Quality Assurance**

## Post-Migration Testing Checklist
- [ ] **Core Functionality**: Project loading, text extraction, translation workflow
- [ ] **Navigation**: All routes work correctly with new naming
- [ ] **Components**: All UI components render properly
- [ ] **TypeScript**: No linting errors or type issues
- [ ] **Build Process**: `pnpm dev` and `pnpm build` work correctly
- [ ] **Cross-Platform**: Test on Windows/macOS/Linux

## Performance Validation
- [ ] **Bundle Size**: Verify no significant size increase
- [ ] **Load Times**: Check application startup performance  
- [ ] **Memory Usage**: Monitor for memory leaks
- [ ] **Build Speed**: Test Rolldown vs standard Vite builds

---

# 📚 **Documentation Updates**

## Files to Update Post-Migration
- [ ] **README.md**: Update with new naming and features
- [ ] **CHANGELOG.md**: Document all changes and improvements  
- [ ] **PROGRESS.MD**: Update milestone completion status
- [ ] **Component Documentation**: Update any component examples

---

# 🚀 **Success Metrics & Timeline**

## Phase 4 Timeline
- **✅ Phase 4.1**: File Renaming (30 minutes) - **COMPLETED**
- **✅ Phase 4.2**: Nuxt 4.1 Upgrade (1-2 hours) - **COMPLETED**
- **✅ Phase 4.3**: Nuxt UI v4 Migration (2-3 hours) - **COMPLETED**
- **📋 Phase 4.4**: Rebuild Translation Management (3-4 hours)
- **Total**: 6-9 hours estimated

## Success Criteria
- ✅ **Naming Consistency**: All files use clear, consistent naming
- ✅ **Modern Stack**: Latest Nuxt 4.1.1 and Nuxt UI v4.0.0-alpha.1
- ✅ **Working Features**: All existing functionality preserved
- 📋 **Translation Management**: Simple, working CRUD interface
- 📋 **Clean Codebase**: No TypeScript errors or warnings

## Current Progress
```
Phase 4 Progress: 75% Complete
├── ✅ File Renaming (100%) 
├── ✅ Nuxt 4.1 Upgrade (100%)
├── ✅ Nuxt UI v4 Migration (100%) 
└── 📋 Translation Management Rebuild (0%) - NEXT
```

---

## 🎯 **Next Steps**

**Immediate Priority**: Phase 4.4 - Rebuild Translation Management
1. Create `app/components/translations/` directory
2. Build `TranslationTable.vue` based on `GlossaryTable.vue`
3. Build `TranslationForm.vue` based on `GlossaryForm.vue`
4. Create `app/pages/translations.vue` page
5. Add navigation integration

**Strategy**: Follow the proven glossary implementation pattern for a simple, working solution.

This approach ensures we have a solid, modern foundation before rebuilding features!


