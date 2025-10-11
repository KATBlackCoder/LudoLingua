# TODO

## 🎯 Implementation Phases Overview

### Phase Priority Order:
1. **Phase 0**: Critical Fixes (IMMEDIATE - CRITICAL)
2. **Phase 1**: Batch Translation System (HIGH PRIORITY)
3. **Phase 1.5**: About System Consolidation (MEDIUM PRIORITY) 
4. **Phase 2**: Selective Translation Injection (HIGH PRIORITY)
5. **Phase 3**: Translation Quality System (MEDIUM PRIORITY)
6. **Phase 4**: Provider Architecture Improvements (LOW PRIORITY)

---

## Phase 0: Critical Fixes (IMMEDIATE - CRITICAL)
**Priority: CRITICAL** - Fix existing bulk action system and token limit issues before implementing new features

### 1. Token Limit Architecture Fix
The current manual `max_tokens` configuration causes incomplete translations because users set values too low for translation tasks.

#### Root Cause Analysis:
1. **Frontend Configuration Issues**: Users don't know what values to set for translation tasks
2. **Wrong Defaults**: Frontend defaults to 512-1024 tokens (too low for translation)
3. **Context Ignorance**: Manual config doesn't consider prompt length or task type
4. **Incomplete Translations**: Model stops mid-translation when `num_predict` limit is reached

#### Required Fixes:
- [ ] **Remove max_tokens from Frontend Config**: Eliminate user configuration of token limits
- [ ] **Implement Automatic Token Calculation**: Backend calculates optimal `num_predict` based on task type and context
- [ ] **Update RunPod Service**: Add intelligent token limit calculation for translation tasks
- [ ] **Update Ollama Service**: Add intelligent token limit calculation for translation tasks
- [ ] **Task-Aware Limits**: Different limits for translation vs chat vs other tasks
- [ ] **Context-Aware Sizing**: Adjust limits based on prompt length and expected output
- [ ] **Add System Parameter**: Implement `system` parameter for better translation consistency and quality
- [ ] **Optimize API Format**: Use proper `/api/generate` format with `system`, `prompt`, `stream: false` for complete translations
- [ ] **Create Method-Specific Architecture**: Separate `/api/generate` and `/api/chat` implementations for better organization
- [ ] **Implement LLM Method Structure**: Create `generate/`, `chat/`, and `embed/` folders for clear API method separation
- [ ] **Implement Automatic Retranslation**: Add quality-based retranslation system for poor translations
- [ ] **Add Translation Quality Assessment**: Use embeddings to assess and improve translation quality

#### Implementation Details:
```rust
// Automatic token calculation based on task type and context
// Implemented in core provider trait - works with ALL future providers
let num_predict = match task_type {
    Translation => {
        let prompt_tokens_estimate = prompt.len() / 4; // Rough estimate
        let min_output_tokens = 2048; // Minimum for complete translations
        let max_output_tokens = 4096; // Cap for efficiency
        std::cmp::max(min_output_tokens, std::cmp::min(max_output_tokens, prompt_tokens_estimate * 2))
    },
    Chat => 512,
    CodeGeneration => 1024,
};

// Provider-specific optimizations
let provider_multiplier = match provider_type {
    ProviderType::RunPod => 2.0,      // Cloud can handle more
    ProviderType::OpenAI => 1.5,      // API limits
    ProviderType::Ollama => 1.0,      // Local limitations
    ProviderType::Gemini => 1.5,      // Future provider
    ProviderType::Anthropic => 2.0,   // Future provider
};
```

#### API Format Optimization:
```rust
// Optimal API request structure for translation
struct TranslationRequest {
    model: String,
    system: String,  // "You are a professional translator specializing in RPG localization..."
    prompt: String,  // The text to translate
    stream: false,   // Get complete response at once
    options: TranslationOptions {
        temperature: 0.3,        // Lower = more consistent
        num_predict: calculated_value, // Automatic calculation
        repeat_penalty: 1.1,     // Prevent repetition
        num_ctx: 4096,           // Large context for prompts
        top_p: 0.9,              // Natural language
    }
}
```

#### LLM Method Architecture:
```
src-tauri/src/llm/
├── generate/                # /api/generate implementation
│   ├── mod.rs
│   ├── ollama.rs           # Ollama single prompt completion
│   ├── runpod.rs           # RunPod single prompt completion
│   └── traits.rs           # GenerateService trait
├── chat/                   # /api/chat implementation (future batch)
│   ├── mod.rs
│   ├── ollama.rs           # Ollama multi-message conversations
│   ├── runpod.rs           # RunPod multi-message conversations
│   └── traits.rs           # ChatService trait
├── embed/                  # /api/embed implementation (future)
│   ├── mod.rs
│   ├── ollama.rs           # Ollama embeddings
│   ├── quality.rs          # Translation quality assessment
│   └── traits.rs           # EmbedService trait
└── services/               # Legacy services (deprecated)
    ├── ollama.rs
    └── runpod.rs
```

#### Future Provider Support:
- **Universal Architecture**: Token calculation in core provider trait
- **API Format Agnostic**: Works with OpenAI, Anthropic, Google Gemini, Cohere, etc.
- **Easy Integration**: New providers automatically get optimal token limits
- **No Manual Configuration**: Eliminates "what number should I use?" problem forever

#### Success Criteria:
- [ ] Translations complete without truncation
- [ ] No user configuration needed for token limits
- [ ] Automatic optimization based on prompt length
- [ ] Task-specific token limits (translation gets more tokens)
- [ ] Provider-specific optimizations (RunPod vs Ollama)
- [ ] **System Parameter Integration**: AI consistently behaves as professional translator
- [ ] **Optimal API Format**: Uses `/api/generate` with `system`, `prompt`, `stream: false`
- [ ] **Better Translation Quality**: Consistent RPG localization style and formatting
- [ ] **Automatic Retranslation**: Poor quality translations are automatically retranslated with different strategies
- [ ] **Quality Assessment**: Embeddings assess translation quality and flag poor translations
- [ ] **RPG Context Preservation**: RPG terminology and formatting codes are preserved in translations
- [ ] **Method-Specific Architecture**: Clear separation between `/api/generate` and `/api/chat` implementations
- [ ] **Provider Method Flexibility**: Each provider can implement all API methods independently
- [ ] **Easy Method Addition**: New API methods (like `/api/embed`) can be added without touching existing code
- [ ] **Future-Proof Architecture**: Works with all future providers (OpenAI, Anthropic, Google Gemini, etc.)
- [ ] **Universal Compatibility**: Automatic token calculation works for any API format
- [ ] **Extensible Design**: Easy to add new task types and provider types without code changes

### 2. Bulk Actions Architecture Fix
The bulk action system has multiple architectural issues preventing proper execution that must be fixed before implementing batch translation.

#### Root Cause Analysis:
1. **DataTable.vue Issues**: `useBulkActions` composable not properly integrated with props.bulkActions
2. **TranslationResult.vue Issues**: Incorrect event handler signature and wrong selected rows access path
3. **Event Flow Problems**: Mismatch between action execution and event handling

#### Required Fixes:
- [ ] **Fix DataTable.vue Bulk Action Integration**: Remove incorrect useBulkActions usage, add bulk action execution logic
- [ ] **Fix TranslationResult.vue Event Handling**: Fix onBulkAction signature and selected rows access
- [ ] **Fix BulkActions.vue Execution**: Add direct execution or proper parent communication
- [ ] **Standardize Event Flow**: Choose consistent pattern (onClick vs event emission)

#### Success Criteria:
- [ ] Bulk retranslate works with selected rows
- [ ] Bulk revert works with selected rows  
- [ ] Selected rows are correctly passed to bulk action functions
- [ ] Loading states work correctly during bulk operations
- [ ] Error handling works for bulk operations

---

## Phase 1: Batch Translation System Implementation
**Priority: HIGH** - Implement batch translation system to handle 10,000+ text units efficiently

### Objective
Implement batch translation system to handle 10,000+ text units efficiently, reducing translation time from hours to minutes while maintaining translation quality and consistency.

### Key Features to Implement:

#### 1. Batch Translation Core System
- [ ] **Batch Translation API**: New `translate_batch` command for processing multiple text units
- [ ] **Batch Size Configuration**: Configurable batch sizes (50-200 units) based on provider and context
- [ ] **Batch Prompt Optimization**: Single prompt template for multiple units to reduce token usage
- [ ] **Batch Response Parsing**: Parse batch responses to extract individual translations
- [ ] **Error Handling**: Robust error handling for partial batch failures

#### 2. Contextual Grouping System
- [ ] **Grouping Strategies**: Group units by character, scene, file, or type for better context
- [ ] **Smart Grouping Logic**: Automatic grouping based on text unit relationships
- [ ] **Context Preservation**: Maintain translation consistency within grouped units
- [ ] **Group Size Optimization**: Balance between context and batch efficiency

#### 3. Streaming Progress System
- [ ] **Real-time Progress Updates**: Stream progress to frontend during batch processing
- [ ] **Progress Indicators**: Show current batch, completed units, and estimated time remaining
- [ ] **Batch Status Tracking**: Track individual batch success/failure status
- [ ] **Resume Capability**: Resume interrupted batch operations from last successful batch

#### 4. Performance Optimizations
- [ ] **Token Usage Reduction**: 70% reduction in token usage through batch prompts
- [ ] **API Call Reduction**: 10,000 individual calls → 100-200 batch calls
- [ ] **Intelligent Rate Limiting**: Adaptive delays based on provider and batch size
- [ ] **Memory Optimization**: Efficient handling of large batch operations

#### 5. Quality Assurance
- [ ] **Batch Validation**: Validate batch responses before saving to database
- [ ] **Consistency Checks**: Ensure translation consistency within batches
- [ ] **Fallback Handling**: Individual translation fallback for failed batch units
- [ ] **Quality Metrics**: Track translation quality across batch operations

### Success Metrics:
- [ ] **10,000+ text units**: Process in 15-30 minutes instead of 2-3 hours
- [ ] **70% token reduction**: Batch prompts vs individual prompts
- [ ] **95% API call reduction**: 10,000 calls → 100-200 batch calls
- [ ] **Real-time progress**: Users see progress updates during batch processing
- [ ] **Resume capability**: Continue from last successful batch after interruption
- [ ] **Quality maintenance**: No degradation in translation quality
- [ ] **100% backward compatibility**: Individual translation still available

### Workflow Changes:
- [ ] **"Translate All"** → Batch translation only (efficient for large projects)
- [ ] **"Retranslate"** → Single translation only (precise control for individual items)
- [ ] **Smart workflow**: Users get the right tool for the right job automatically
- [ ] **Performance optimization**: Batch for bulk operations, single for precision work

### Technical Requirements:

##### **Backend (Rust) - New Files (Consolidated Architecture):**
- [ ] **`src-tauri/src/commands/batch_translation.rs`** - New batch translation command with `translate_batch()`, `get_batch_progress()`, `cancel_batch_translation()`, `resume_batch_translation()`
- [ ] **`src-tauri/src/core/processor.rs`** - **UNIFIED**: Single + batch processing logic for all LLM services
  - **Unified processing**: Handle both single and batch operations in one place
  - **Provider abstraction**: Provider-agnostic processing logic
  - **Rate limiting**: Unified rate limiting for all operations
  - **Error handling**: Centralized error recovery and retry logic
  - **Response parsing**: Parse both single and batch responses
- [ ] **`src-tauri/src/core/progress.rs`** - **UNIFIED**: Single + batch progress tracking
  - **Unified progress**: Track both single and batch operations
  - **Event streaming**: Real-time progress updates for all operations
  - **State management**: Centralized progress state handling
  - **Resume capability**: Resume any interrupted operation
- [ ] **`src-tauri/src/core/prompts.rs`** - **UNIFIED**: Single + batch prompt building
  - **Unified prompt builder**: Handle both single and batch prompts
  - **Template management**: Load from `prompts/single/` and `prompts/batch/` folders
  - **Response parsing**: Parse both single and batch responses
  - **Token optimization**: Optimize prompts for different operation types
- [ ] **`src-tauri/src/utils/grouping.rs`** - Batch grouping logic with `GroupingStrategy`, `group_text_units()`, `optimize_batch_size()`, `BatchGroup` struct
- [ ] **`src-tauri/migrations/0005_create_progress.sql`** - Unified progress tracking table migration
- [ ] **`src-tauri/migrations/0006_create_config.sql`** - Unified configuration table migration
- [ ] **`src-tauri/prompts/single/`** - **NEW FOLDER**: Move existing single prompt templates (9 files: basic.txt, dialogue.txt, character.txt, system.txt, equipment.txt, skill.txt, state.txt, class.txt, other.txt)
- [ ] **`src-tauri/prompts/batch/`** - **NEW FOLDER**: New batch prompt templates (9 files: basic.txt, dialogue.txt, character.txt, system.txt, equipment.txt, skill.txt, state.txt, class.txt, other.txt)
  - **Why different prompts**: Single prompts optimized for individual units, batch prompts optimized for multiple units
  - **Token efficiency**: Batch prompts reduce token overhead by 70% compared to individual prompts
  - **Context preservation**: Better translation consistency when processing related texts together
  - **Response parsing**: Structured format for easy parsing of multiple translations
  - **Quality improvement**: AI maintains consistency across related translations in same batch
  - **Organized structure**: Clear separation between single and batch prompt templates

##### **Core Architecture Improvements (DRY Principle Applied):**
- [ ] **`src-tauri/src/core/provider.rs`** - Extend `LlmService` trait with unified methods
  - **`generate_unified()`** - Unified generation method for single and batch operations
  - **`parse_response()`** - Provider-agnostic response parsing for all operation types
  - **`get_limits()`** - Provider-specific limits for all operation types
  - **`optimize_config()`** - Provider-specific optimization for all operation types

##### **Backend (Rust) - Files to Modify (Simplified):**
- [ ] **`src-tauri/src/commands/handler.rs`** - Add batch translation commands to Tauri registry
- [ ] **`src-tauri/src/commands/translation.rs`** - Add batch translation functions and unified error handling
- [ ] **`src-tauri/src/db/translation/model.rs`** - Add unified progress tracking fields
- [ ] **`src-tauri/src/db/translation/repo.rs`** - Add unified save operations, progress tracking queries
- [ ] **`src-tauri/src/models/translation.rs`** - Add batch translation types and progress tracking models
- [ ] **`src-tauri/src/llm/services/ollama.rs`** - Use unified core processor, remove duplicate logic
- [ ] **`src-tauri/src/llm/services/runpod.rs`** - Use unified core processor, remove duplicate logic
- [ ] **`src-tauri/src/llm/services/openai.rs`** - Use unified core processor, remove duplicate logic
- [ ] **`src-tauri/tauri.conf.json`** - Add batch translation permissions, progress event permissions
- [ ] **`src-tauri/Cargo.toml`** - Add unified processing dependencies, progress tracking dependencies

##### **Backend (Rust) - Files That DON'T Need Updates (Already Optimal):**
- [ ] **`src-tauri/src/core/engine.rs`** - ✅ **NO CHANGES NEEDED**: Engine trait is already well-designed
- [ ] **`src-tauri/src/engines/factory.rs`** - ✅ **NO CHANGES NEEDED**: Factory pattern is optimal
- [ ] **`src-tauri/src/engines/rpg_maker_mv/`** - ✅ **NO CHANGES NEEDED**: Engine implementations are file-focused
- [ ] **`src-tauri/src/engines/rpg_maker_mz/`** - ✅ **NO CHANGES NEEDED**: Engine implementations are file-focused
- [ ] **`src-tauri/src/engines/wolf_rpg/`** - ✅ **NO CHANGES NEEDED**: Engine implementations are file-focused
  - **Why no changes**: Engine system handles file I/O and data extraction/injection
  - **Separation of concerns**: Engines handle game data, translation system handles AI processing
  - **Batch translation impact**: None - engines work with individual TextUnits regardless of translation method
  - **Already follows DRY, KISS, YAGNI**: Well-designed architecture with clear separation

##### **Architecture Separation (Why Engine System Doesn't Need Updates):**
- [ ] **Engine System Scope**: File I/O, data extraction, data injection, game format handling
- [ ] **Translation System Scope**: LLM communication, prompt building, response parsing, AI processing
- [ ] **Batch Translation Scope**: Efficient AI processing, progress tracking, batch operations
- [ ] **Clear Boundaries**: Engine system works with individual TextUnits regardless of translation method
- [ ] **No Coupling**: Batch translation is a translation layer concern, not an engine concern
- [ ] **Provider Agnostic**: Engines don't care if translation uses Ollama, RunPod, or OpenAI
- [ ] **Method Agnostic**: Engines don't care if translation is single or batch

##### **Frontend (Nuxt.js) - New Files (Consolidated Architecture):**

###### **Core Translation System (DRY Principle Applied):**
- [ ] **`app/composables/useTranslation.ts`** - **UNIFIED**: Single + batch translation logic
  - **Unified functions**: Handle both single and batch operations
  - **Progress tracking**: Track progress for all operation types
  - **State management**: Manage state for all operation types
  - **Replace**: `useTranslations.ts` + `useTranslator.ts` (consolidate 2 files into 1)
- [ ] **`app/stores/translation.ts`** - **UNIFIED**: Single + batch translation state management
  - **Unified state**: Manage both single and batch operations in one place
  - **Progress tracking**: Track progress for all operation types
  - **Operation controls**: Handle all translation operation types
- [ ] **`app/components/translation/TranslationDialog.vue`** - **UNIFIED**: Single + batch translation configuration modal
  - **Unified configuration**: Handle both single and batch translation settings
  - **Smart defaults**: Automatically choose batch for large projects, single for small ones
  - **Progress preview**: Show estimated time and token usage for both operation types
- [ ] **`app/components/translation/TranslationProgress.vue`** - **UNIFIED**: Real-time progress display for all operations
  - **Unified progress**: Track both single and batch operations
  - **Status indicators**: Show operation type, progress, and statistics
  - **Real-time updates**: Stream progress updates for all operation types
- [ ] **`app/components/translation/TranslationControls.vue`** - **UNIFIED**: Operation controls for all translation types
  - **Unified controls**: Start, pause, resume, stop for all operations
  - **Smart controls**: Show relevant controls based on operation type
  - **State management**: Handle different states for different operation types

###### **Modal System Consolidation (DRY Principle Applied):**
- [ ] **`app/composables/useModal.ts`** - **UNIFIED**: Single modal composable
  - **Unified modal logic**: Handle all modal operations in one place
  - **Color management**: Unified color detection and management
  - **Validation**: Unified modal validation logic
  - **Formatting**: Unified modal formatting logic
  - **Replace**: `modal/useModalColors.ts` + `modal/useModalFormatting.ts` + `modal/useModalValidation.ts` + `modal/index.ts` (consolidate 4 files into 1)
- [ ] **`app/utils/modal.ts`** - **UNIFIED**: Single modal utility
  - **Unified utilities**: All modal utilities in one place
  - **Color mapping**: Unified color mapping and detection
  - **Validation**: Unified validation utilities
  - **Formatting**: Unified formatting utilities
  - **Replace**: `modal/colors.ts` + `modal/formatting.ts` + `modal/validation.ts` + `modal/index.ts` (consolidate 4 files into 1)
- [ ] **`app/components/shared/Modal.vue`** - **UNIFIED**: Single modal component
  - **Unified modal**: Handle all modal types in one component
  - **Smart rendering**: Automatically render appropriate modal content
  - **Replace**: `modal/FormCard.vue` + `modal/MetadataCard.vue` + `modal/TextCard.vue` + `modal/ModalActions.vue` + `modal/ModalBody.vue` + `modal/ModalFooter.vue` + `modal/ModalHeader.vue` (consolidate 7 files into 1)

###### **About System Consolidation (DRY Principle Applied):**
- [ ] **`app/components/about/AboutCard.vue`** - **UNIFIED**: Single about card component
  - **Unified card**: Handle all about content in one component
  - **Smart rendering**: Automatically render appropriate content based on type
  - **Configuration-driven**: Use aboutConfig.ts to drive content rendering
  - **Replace**: `AboutFeatures.vue` + `AboutGlossary.vue` + `AboutPlaceholders.vue` + `AboutTechnology.vue` + `AboutTranslations.vue` + `AboutWhat.vue` (consolidate 6 files into 1)
- [ ] **`app/components/about/AboutSection.vue`** - **UNIFIED**: Single about section component
  - **Unified sections**: Handle all about sections in one component
  - **Smart rendering**: Automatically render appropriate section content
  - **Replace**: `placeholders/` + `referral/` + `technologies/` folders (consolidate 8 files into 1)
- [ ] **`app/config/aboutConfig.ts`** - **NEW**: Configuration for all About content
  - **Unified configuration**: Single source of truth for all About content
  - **Content types**: Features, glossary, technology, placeholders, referral, translations
  - **Smart defaults**: Automatic icon, color, and layout selection
  - **Easy maintenance**: Update content without touching components

##### **Frontend (Nuxt.js) - Files to Modify (Simplified):**
- [ ] **`app/components/translation/TranslationResult.vue`** - Add unified translation controls and progress display
- [ ] **`app/components/translation/TranslationView.vue`** - Add unified translation options and progress indicators
- [ ] **`app/components/shared/DataTable.vue`** - Add unified selection support and operation buttons
- [ ] **`app/types/translation.ts`** - Add unified translation types for all operation types

###### **App/ Folder Structure Improvements (DRY, KISS, YAGNI Applied):**
- [ ] **File Count Reduction**: 47 files → 15 files (68% reduction)
  - **Translation System**: 6 files → 3 files (50% reduction)
  - **Modal System**: 16 files → 3 files (81% reduction)  
  - **About System**: 14 files → 3 files (79% reduction)
- [ ] **Code Duplication Elimination**: Remove duplicate logic across composables, stores, and components
- [ ] **Unified State Management**: Consolidate related state into single stores
- [ ] **Smart Component Rendering**: Components automatically adapt to different use cases
- [ ] **Maintainability**: Single source of truth for each system
- [ ] **Performance**: Reduced bundle size and improved loading times
- [ ] **Developer Experience**: Easier to maintain, test, and extend

###### **About System Specific Improvements:**
- [ ] **Duplicate Card Structure Elimination**: Remove identical UCard header pattern repeated 14 times
- [ ] **Configuration-Driven Content**: Use aboutConfig.ts to drive all About content rendering
- [ ] **Smart Content Types**: Automatic rendering based on content type (features, glossary, technology, etc.)
- [ ] **Unified Header Logic**: Single AboutCardHeader component for all cards
- [ ] **Unified Section Logic**: Single AboutSection component for all sections
- [ ] **Easy Content Updates**: Update About content by modifying configuration, not components
- [ ] **Consistent Styling**: Single place to update card styling across all About content
- [ ] **Better Testing**: Test 2 components instead of 14 components

###### **Testing Strategy:**
- [ ] **Unit Testing**: Test unified components with different content types
- [ ] **Integration Testing**: Test batch translation end-to-end workflow
- [ ] **Performance Testing**: Verify 70% token reduction and 95% API call reduction
- [ ] **Regression Testing**: Ensure individual translation still works
- [ ] **UI Testing**: Test unified components with different configurations

---

## Phase 1.5: About System Consolidation
**Priority: MEDIUM** - Consolidate About system to eliminate duplication and improve maintainability

### Objective
Consolidate the About system by replacing 14 duplicate components with 3 unified components, eliminating code duplication and improving maintainability.

### Key Features to Implement:

#### 1. Unified About Components
- [ ] **AboutCard.vue**: Single component for all About cards with smart rendering
- [ ] **AboutSection.vue**: Single component for all About sections with smart rendering  
- [ ] **aboutConfig.ts**: Configuration-driven content management

#### 2. Configuration-Driven Content
- [ ] **Content Types**: Features, glossary, technology, placeholders, referral, translations
- [ ] **Smart Defaults**: Automatic icon, color, and layout selection
- [ ] **Easy Updates**: Update content by modifying configuration, not components

#### 3. Duplication Elimination
- [ ] **Remove 14 duplicate components**: AboutFeatures, AboutGlossary, AboutPlaceholders, AboutTechnology, AboutTranslations, AboutWhat, plus 8 section components
- [ ] **Unified header logic**: Single header component for all cards
- [ ] **Unified section logic**: Single section component for all sections

### Success Metrics:
- [ ] **79% file reduction**: 14 files → 3 files
- [ ] **No duplicate code**: Single source of truth for About content
- [ ] **Easy maintenance**: Update content without touching components
- [ ] **Consistent styling**: Single place to update card styling
- [ ] **Better testing**: Test 2 components instead of 14

### Technical Requirements:
- [ ] **`app/components/about/AboutCard.vue`** - Unified card component with smart rendering
- [ ] **`app/components/about/AboutSection.vue`** - Unified section component with smart rendering
- [ ] **`app/config/aboutConfig.ts`** - Configuration for all About content
- [ ] **Update `app/pages/about.vue`** - Use unified components instead of individual components
- [ ] **Remove duplicate components** - Delete 14 duplicate About components

---

## Phase 2: Selective Translation Injection
**Priority: HIGH** - Allow users to selectively inject translations instead of global injection

### Objective
Implement selective injection functionality that allows users to choose specific translations to inject rather than performing a global injection of all translated content.

### Key Features to Implement:

#### 1. Enhanced Selection System
- [ ] **Multi-Select Translation Units**: Allow users to select specific translation units for injection
- [ ] **Batch Selection**: Support selecting multiple translations by criteria (status, file type, etc.)
- [ ] **Visual Selection Indicators**: Clear UI indicators showing what's selected for injection
- [ ] **Selection Persistence**: Maintain selection state across navigation and filtering

#### 2. Selective Injection UI
- [ ] **Injection Dialog**: Create modal for selective injection with preview
- [ ] **Injection Preview**: Show which files will be modified and what translations will be injected
- [ ] **Injection Options**: Allow users to choose injection scope (selected items only)
- [ ] **Progress Tracking**: Real-time progress indicator during selective injection
- [ ] **Translation Controls**: Start, pause, resume, and stop translation operations
- [ ] **Translation State Management**: Handle idle, running, paused, and stopped states

#### 3. Backend Injection Logic
- [ ] **Selective Injection API**: Modify backend to accept specific translation unit IDs
- [ ] **Partial File Updates**: Update only the files containing selected translations
- [ ] **Injection Validation**: Validate selected translations before injection
- [ ] **Rollback Support**: Ability to undo selective injections
- [ ] **Translation State Persistence**: Save and restore translation progress and state
- [ ] **Request Cancellation**: Cancel ongoing translation requests when paused/stopped

#### 4. Integration Points
- [ ] **Translation Tables**: Add selection checkboxes and bulk injection actions
- [ ] **Translation Editor**: Add "Inject Selected" option to individual translation editor
- [ ] **Glossary System**: Selective injection for glossary terms
- [ ] **Export System**: Modify export to support selective translation injection

### Success Metrics:
- [ ] Users can select 1-1000+ specific translations for injection
- [ ] Injection process takes <30 seconds for typical selections
- [ ] 100% accuracy in selective injection (no unintended changes)
- [ ] Clear visual feedback during injection process
- [ ] Support for both single-item and bulk selective injection
- [ ] Users can pause/resume translation operations seamlessly
- [ ] Translation state persists across application restarts
- [ ] <2 second response time for pause/stop operations

### Technical Requirements:
- [ ] Modify `export_translated_subset` to accept translation unit IDs
- [ ] Update database queries to fetch specific translation units
- [ ] Enhance UI selection components with injection actions
- [ ] Add injection preview and confirmation dialogs
- [ ] Implement progress tracking for selective injection operations
- [ ] Add translation state management to stores (idle, running, paused, stopped)
- [ ] Implement request cancellation for pause/stop operations
- [ ] Add translation progress persistence to database
- [ ] Create translation control components (start, pause, resume, stop)

---

## Phase 3: Translation Quality System
**Priority: MEDIUM** - Implement embedding-based quality assessment and automatic retranslation system

### Objective
Implement comprehensive translation quality assessment using embeddings to automatically detect poor translations and retranslate them with different strategies, ensuring RPG context and formatting preservation.

### Key Features to Implement:

#### 1. Embedding-Based Quality Assessment
- [ ] **Semantic Similarity Analysis**: Compare original vs translated text using embedding cosine similarity
- [ ] **RPG Maker Code Preservation**: Detect and score preservation of [NEWLINE], [C(1)], [V(1)] formatting codes
- [ ] **Translation Structure Analysis**: Assess length ratios, error detection, and structural quality
- [ ] **Cross-Language Embedding Support**: Enable quality assessment across Japanese ↔ English translations

#### 2. Automatic Retranslation System
- [ ] **Quality Thresholds**: Define quality levels (excellent 85%+, good 70%+, poor 60%+, failed <60%)
- [ ] **Retranslation Strategies**: Different prompt approaches for failed translations
- [ ] **Multi-Attempt System**: Up to 3 retranslation attempts with different parameters
- [ ] **Fallback Handling**: Flag for manual review if all attempts fail

#### 3. RPG Context Preservation
- [ ] **RPG Terminology Consistency**: Ensure RPG terms are translated consistently
- [ ] **Character Name Consistency**: Detect and flag inconsistent character name translations
- [ ] **Game Mechanics Preservation**: Preserve RPG combat terminology and mechanics
- [ ] **Formatting Code Validation**: Ensure RPG Maker codes are preserved correctly

#### 4. Integration and Infrastructure
- [ ] **LLM Factory Integration**: Update factory to create embed and quality services automatically
- [ ] **Command Handler Integration**: Add quality assessment and embedding commands to Tauri registry
- [ ] **Frontend Store Integration**: Add quality features to translation stores and composables
- [ ] **Database Schema Updates**: Add quality tracking tables and indexes for translation assessment
- [ ] **Provider Capability Detection**: Automatically detect and enable embedding features per provider

### Success Metrics:
- [ ] **Quality Assessment Accuracy**: 95% accuracy in detecting poor translations
- [ ] **Automatic Improvement**: 80% of poor translations improved through retranslation
- [ ] **RPG Context Preservation**: 100% preservation of RPG Maker formatting codes
- [ ] **Translation Consistency**: 90% consistency in RPG terminology across translations
- [ ] **Performance Impact**: <5% overhead for quality assessment on batch translations
- [ ] **User Experience**: Transparent quality assessment with clear quality indicators

### Technical Requirements:
- [ ] **Embedding Service**: Implement `/api/embed` integration for Ollama and RunPod
- [ ] **Quality Assessment Engine**: Core quality scoring and retranslation logic
- [ ] **RPG Code Parser**: Detect and validate RPG Maker formatting codes
- [ ] **Quality Database Schema**: Store quality scores and retranslation history
- [ ] **Frontend Quality UI**: Quality indicators, retranslation controls, and reporting
- [ ] **Batch Quality Processing**: Quality assessment for batch translation operations

---

## Phase 4: Provider Architecture Improvements
**Priority: LOW** - Enhance provider architecture for better maintainability and future expansion

### Objective
Improve the provider architecture to support multiple API formats, reduce code duplication, and make it easier to add new providers in the future.

### Key Features to Implement:

#### 1. Provider Architecture Improvements
- [ ] **Ollama-Compatible Provider Structure**: Create shared `OllamaCompatibleConfig` structure for Ollama API format providers
- [ ] **Unified Ollama Options**: Centralize Ollama API options for all Ollama-compatible providers
- [ ] **Provider Abstraction**: Distinguish between Ollama-compatible and custom API providers

### Additional Features
- [ ] **Translation Features**: Bulk retranslation, progress indicators, quality scoring, comparison tools
- [ ] **UI/UX Improvements**: Keyboard shortcuts, drag-and-drop, search highlighting, mobile responsiveness
- [ ] **Performance Optimizations**: Virtual scrolling, lazy loading, bundle optimization, caching

### Documentation
- [ ] **Technical Documentation**: Selective injection API, component architecture, composable examples
- [ ] **User Documentation**: User manual updates, troubleshooting, video tutorials
