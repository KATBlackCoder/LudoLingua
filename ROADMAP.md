# LudoLingua Development Roadmap

## 📍 Current Status (v0.1.0-alpha)
- ✅ Basic translation functionality for RPG Maker MV/MZ and Wolf RPG
- ✅ Multi-provider AI integration (Ollama, OpenAI, OpenRouter, Groq, RunPod)
- ✅ SQLite database with sqlx (glossary system only)
- ✅ Tauri + Nuxt.js desktop application with Pinia stores
- ✅ Project loading and text extraction (in-memory processing)
- ❌ **Critical Gap**: No progress preservation - app crashes lose all progress
- ❌ **Scalability Issue**: Large projects overwhelm memory
- 🔄 **Database Architecture Refactor** - Shared `db/state.rs`, `db/glossaries/`, `db/translation/` with models in `src/models/`
- 🔄 **Engine Modification** - Store extracted texts directly in database with project-based sessions
- 🔄 **Glossary Migration** - Move `src/glossaries/` models to `src/models/glossary.rs`, logic to `db/glossaries/`
- 📄 **Migration Status**: 0001 & 0002 (glossary), creating 0003 (translation progress)

### **Current Sprint: 35-Step Progress Preservation Implementation**
- **Steps 1-2**: Architecture foundation and database structure
- **Step 3**: Database schema, project ID system, and session detection
- **Step 4**: Data models, repository pattern, and glossary migration
- **Step 5**: RPG Maker MV engine integration (test implementation)
- **Step 6**: Factory rework for progress preservation
- **Step 7**: Backend commands with session management (12 sub-steps)
- **Steps 8-11**: Engine integration for MZ and Wolf RPG
- **Steps 12-15**: UI components, testing, and final integration

**Key Focus: Project Isolation**
- Each project gets its own database session (identified by unique project ID)
- `.ludolingua.json` files in project roots for ID storage and metadata
- No cross-project data confusion or conflicts
- Independent progress tracking per project
- Robust project detection even if folders are moved

## 🎯 Phase 1: Stability & Reliability (v0.2.0) - IN PROGRESS

### Q1 2025 Goals (5-Day Sprint)
- **Progress Preservation System** ⭐ **CURRENT FOCUS**
  - **SOLVES:** No progress preservation - users lose hours of work on crashes
  - **Days 1-2**: Database foundation, models, and repository pattern
  - **Days 2-4**: Engine integration and factory rework
  - **Days 4-5**: Backend commands, UI components, and testing
  - **Result**: 100% progress preservation with database-first approach

- **Database Architecture Overhaul**
  - **SOLVES:** Memory limitations for large projects, no data persistence
  - **Days 1-2**: Create modular `db/` structure with shared state management
  - **Days 2-4**: Direct database storage of extracted texts during engine processing
  - **Result**: Handle 100K+ text projects without memory constraints

- **Translation Quality Improvements**
  - Enhanced AI prompt engineering and output cleaning
  - Retry mechanisms for failed translations
  - Batch processing optimization to reduce API costs

- **Build & Distribution**
  - Fix Linux AppImage packaging issues
  - Cross-platform build optimization
  - Automated testing pipeline

### Key Milestones
- [ ] **Steps 1-2**: Database architecture foundation and structure established
- [ ] **Step 3**: Project ID system, SQL migration 0003, and session detection implemented
- [ ] **Step 4**: Translation data models and repository pattern implemented
- [ ] **Step 5**: RPG Maker MV engine integration (test implementation)
- [ ] **Steps 6-7**: Complete backend integration, factory rework, and commands
- [ ] **Steps 8-11**: All game engines storing texts directly in database
- [ ] **Steps 12-15**: UI components, resume dialogs, and statistics tracking
- [ ] **Result**: 100% progress preservation, crash-resistant translation workflow
- [ ] Clean model separation with reusable components across all layers
- [ ] Project isolation with unique IDs and session management
- [ ] Comprehensive error recovery and auto-save functionality

### **Current Sprint Focus:** 35 Finishable Steps (2-3 Weeks)

#### **Step 1-2: Architecture Foundation** 🏗️
1. **Architecture Documentation Update** - Update BACKEND_STRUCTURE.md for new `db/` structure with DRY/SOLID principles
2. **Database Structure Creation** - Create `db/` directory structure (`db/mod.rs`, `db/state.rs`, `db/glossaries/`, `db/translation/`)

#### **Step 3: Database Schema** 🗄️
3. **Project ID System** - Create `.ludolingua.json` files with unique project IDs in project roots
4. **SQL Migration 0003** - Create translation_sessions, translation_units, translation_progress tables (use project_id instead of project_path)
5. **Project Detection** - Implement logic to read `.ludolingua.json` and detect existing sessions
6. **Update Existing Glossary** - Move glossary_terms to use new `db/glossaries/` structure

#### **Step 4: Data Models & Repository** 📊
7. **Translation Data Models** - Implement TranslationSession, TranslationProgress in `src/models/translation.rs`
8. **TranslationProgressRepository** - Implement basic CRUD operations in `db/translation/repo.rs`
9. **Repository Integration** - Connect repository to shared `db/state.rs` connection
10. **Move Glossary Repository** - Migrate existing glossary logic to `db/glossaries/repo.rs`
11. **Update Prompt Builder** - Fix imports in `utils/prompts/builder.rs` to use new `db/glossaries/` path

#### **Step 5: Engine Integration - RPG Maker MV** 🎮
12. **Extract Text Modification** - Update `rpg_maker_mv/engine.rs` to store texts in database
13. **Database Session Creation** - Create translation session when project loads
14. **Text Storage** - Store extracted texts directly in `translation_units` table

#### **Step 6: Factory Rework** 🏭
15. **Progress Factory Pattern** - Adapt `factory.rs` export pattern for progress preservation
16. **Session Management** - Integrate session creation with engine factory
17. **Progress Queries** - Add database queries for progress statistics

#### **Step 7: Backend Commands** ⚙️
18. **Progress Commands** - Add progress preservation commands to `handler.rs`
19. **Project Detection** - Implement `detect_project_progress()` to check existing sessions
20. **Resume Logic** - Add `resume_project_translation()` with position detection
21. **Auto-save Integration** - Hook into translation workflow for automatic saves
22. **Session Management** - Handle project-specific sessions to avoid confusion
23. **Update translation.rs** - Modify `translate_text_unit()` for database progress tracking
24. **Update engine.rs** - Modify `load_project()` and `extract_text()` for database storage
25. **Update glossary.rs** - Migrate to new `db/glossaries/` structure
26. **Update mod.rs** - Update module exports for database integration
27. **Update core/engine.rs** - Modify Engine trait for database operations (CRITICAL)
28. **Update core/mod.rs** - Add db module exports
29. **Update core/provider.rs** - Add progress tracking to LlmService trait
30. **Update models/mod.rs** - Add new model exports (translation session, progress)
31. **Move Glossary Models** - Move GlossaryTerm and GlossaryQuery from src/glossaries/ to src/models/glossary.rs

#### **Steps 8-11: Engine Integration & UI** 🎨
32. **Engine Integration - RPG Maker MZ** - Update `rpg_maker_mz/engine.rs` to store texts in database
33. **Engine Integration - Wolf RPG** - Update `wolf_rpg/engine.rs` to store texts in database
34. **UI Components** - Progress bar, resume dialog, statistics tracking
35. **Integration Testing** - End-to-end testing of complete progress preservation workflow

### **Key Changes We're Making:**

**🔄 Architecture Changes:**
- **Database-First Approach**: Extracted texts stored directly in DB (not memory)
- **Project ID System**: `.ludolingua.json` files with UUIDs (robust project identification)
- **Project Isolation**: Each project has its own translation session (no cross-project confusion)
- **Clean Model Separation**: Models in `src/models/`, persistence logic in `src/db/`
- **Modular Database Structure**: `db/glossaries/` + `db/translation/` + shared `db/state.rs`
- **SOLID Principles**: Single responsibility for each module, open/closed for extensions
- **DRY Implementation**: Shared database state, reusable repository patterns, no model duplication
- **Glossary Migration**: Move models to `src/models/glossary.rs`, logic to `db/glossaries/`
- **Factory Pattern Adaptation**: Reuse export logic for progress preservation
- **Documentation Updates**: Keep BACKEND_STRUCTURE.md and FRONTEND_STRUCTURE.md current

**⚡ Performance & Reliability:**
- Memory-efficient: Handle 100K+ text projects without memory issues
- Crash-resistant: Resume from exact position after app restart
- Real-time tracking: Live progress statistics from database queries

**🎯 User Experience:**
- Progress bars with completion percentages
- Resume dialogs on project load
- Auto-save after each translation
- Batch processing to reduce API calls

## 🚀 Phase 2: Enhanced Features (v0.3.0)

### Q2 2025 Goals
- **Advanced AI Integration**
  - Custom model configurations
  - Model performance comparison tools
  - Context-aware translations
  - Multi-language support

- **User Experience Overhaul**
  - Modern UI with improved workflows
  - Real-time translation progress
  - Advanced project management
  - Keyboard shortcuts and productivity features

- **Extended Engine Support**
  - Enhanced Wolf RPG support
  - Additional game engine formats
  - Plugin architecture for new engines
  - Improved text extraction accuracy

### Key Milestones
- [ ] Professional-grade translation workflow
- [ ] Support for 5+ game engines
- [ ] Advanced AI model management
- [ ] Comprehensive project templates

## 🔬 Phase 3: Power User Features (v0.4.0)

### Q3 2025 Goals
- **Translation Memory System**
  - Reusable translation segments
  - Consistency checking across projects
  - Automated terminology management
  - Quality assurance tools

- **Collaboration Features**
  - Multi-user project support
  - Review and approval workflows
  - Version control integration
  - Team productivity analytics

- **Performance & Scalability**
  - Large project optimization
  - Distributed processing support
  - Advanced caching strategies
  - Memory usage optimization

### Key Milestones
- [ ] Handle projects with 100K+ text units
- [ ] Team collaboration workflows
- [ ] Enterprise-grade quality assurance
- [ ] Advanced performance monitoring

## 🎨 Phase 4: Ecosystem Expansion (v0.5.0)

### Q4 2025 Goals
- **Plugin Ecosystem**
  - Third-party engine plugins
  - Custom AI provider integrations
  - Workflow automation plugins
  - Community plugin marketplace

- **Advanced Analytics**
  - Translation quality metrics
  - Productivity dashboards
  - Usage pattern analysis
  - Performance benchmarking

- **Industry Integration**
  - CAT tool compatibility
  - API for third-party integrations
  - Enterprise deployment options
  - Cloud synchronization

### Key Milestones
- [ ] Active plugin ecosystem
- [ ] Professional translation studio integration
- [ ] Enterprise deployment capabilities
- [ ] Comprehensive analytics platform

## 🏗️ Phase 5: Platform Evolution (v1.0.0)

### 2026 Goals
- **Web Platform**
  - Browser-based translation interface
  - Cloud-based processing options
  - WebAssembly optimizations
  - Progressive Web App features

- **Mobile Support**
  - iOS and Android applications
  - Touch-optimized interfaces
  - Offline translation capabilities
  - Cross-device synchronization

- **AI-Powered Features**
  - Automated quality assessment
  - Intelligent context detection
  - Machine learning optimizations
  - Advanced terminology extraction

### Key Milestones
- [ ] Full platform availability (Desktop + Web + Mobile)
- [ ] AI-first translation experience
- [ ] Global accessibility and localization
- [ ] Industry-leading translation technology

## 🔄 Maintenance & Evolution

### Ongoing Activities
- **Community Engagement**
  - User feedback integration
  - Feature request prioritization
  - Documentation improvements
  - Community event participation

- **Technology Updates**
  - Dependency management
  - Security updates
  - Performance optimizations
  - Platform compatibility

- **Research & Innovation**
  - AI/ML advancements monitoring
  - Game development trends
  - Translation technology evolution
  - Competitive analysis

## 📊 Success Metrics

### Technical Metrics
- **Performance**: Handle 10K+ translations/hour
- **Reliability**: 99.9% uptime, zero data loss
- **Compatibility**: Support 10+ game engines
- **Scalability**: Support projects with 1M+ text units

### User Metrics
- **Satisfaction**: 4.5+ star rating across platforms
- **Adoption**: 10K+ active users
- **Productivity**: 50%+ time savings for translators
- **Retention**: 80%+ user retention rate

### Business Metrics
- **Market Share**: Top 3 translation tools for indie games
- **Revenue**: Sustainable business model
- **Partnerships**: 5+ industry partnerships
- **Ecosystem**: 100+ community plugins

---

*This roadmap is adaptive and will be updated based on user feedback, technological advancements, and market conditions.*
