# TODO

## 🎯 Current Phase: Text Processing Performance Optimization

### Phase 1: Text Processing Performance Optimization
**Priority: CRITICAL** - Optimize text formatting pipeline for 3-5x performance improvement

#### Objective
Implement performance optimizations for the text processing pipeline to achieve 3-5x faster processing while maintaining 100% correctness.

#### Key Features to Implement:

##### 1. Regex Compilation Optimization ✅ COMPLETED
- [x] **Pre-compiled Regexes**: Use `once_cell::sync::Lazy` to pre-compile all regex patterns
- [x] **Static Regex Storage**: Store compiled regexes as static constants
- [x] **Performance Gain**: 70-80% faster regex operations
- [x] **Integration**: OptimizedTextFormatter integrated into pipeline.rs

##### 2. Early Exit Optimization ✅ COMPLETED
- [x] **Formatting Code Detection**: Quick check for formatting codes before processing
- [x] **Plain Text Bypass**: Skip expensive processing for plain text
- [x] **Performance Gain**: 50-90% faster for plain text (most common case)
- [x] **Integration**: Early exit logic integrated into OptimizedTextFormatter

##### 3. Batch Processing Optimization ❌ NOT COMPATIBLE
- [x] **Analysis Complete**: Sequential pipeline cannot be parallelized
- [x] **Architecture Limitation**: Text processing is inherently sequential
- [x] **Alternative**: Focus on other optimizations instead

##### 4. String Allocation Optimization
- [ ] **Pre-allocation**: Use `String::with_capacity()` for known sizes
- [ ] **Reduced Cloning**: Minimize unnecessary string allocations
- [ ] **Performance Gain**: 20-30% faster string operations

##### 5. Caching System ❌ NOT COMPATIBLE
- [x] **Analysis Complete**: Text processing happens only once during extraction
- [x] **Database Architecture**: Processed text is stored in database permanently
- [x] **No Re-processing**: Frontend loads pre-processed text from database

##### 6. Engine-Specific Optimization ✅ COMPLETED
- [x] **RPG Maker Formatter**: Optimized formatter for RPG Maker projects
- [x] **Wolf RPG Formatter**: Optimized formatter for Wolf RPG projects
- [x] **Performance Gain**: 40-60% faster per engine type
- [x] **Integration**: EngineTextProcessor automatically routes to appropriate formatter

#### Success Metrics:
- [ ] 3-5x overall performance improvement
- [x] 70-80% faster regex operations
- [x] 50-90% faster plain text processing
- [x] ~~2-4x faster batch processing~~ (Not compatible)
- [x] ~~90% faster cached operations~~ (Not compatible)
- [x] 100% backward compatibility maintained

#### Technical Requirements:
- [x] Create `formatting_optimized.rs` with pre-compiled regexes (REMOVED - replaced by engine-specific formatters)
- [x] ~~Implement `batch_processor.rs` for parallel processing~~ (Not compatible)
- [x] ~~Add `cache.rs` for text processing cache~~ (Not compatible)
- [x] Update `pipeline.rs` with optimized methods (REMOVED - replaced by `EngineTextProcessor`)
- [x] Add engine-specific formatters
- [x] Create `universal_formatter.rs` for common patterns across all engines
- [x] Maintain backward compatibility with existing code

---

### Phase 2: Selective Translation Injection
**Priority: HIGH** - Allow users to selectively inject translations instead of global injection

#### Objective
Implement selective injection functionality that allows users to choose specific translations to inject rather than performing a global injection of all translated content.

#### Key Features to Implement:

##### 1. Enhanced Selection System
- [ ] **Multi-Select Translation Units**: Allow users to select specific translation units for injection
- [ ] **Batch Selection**: Support selecting multiple translations by criteria (status, file type, etc.)
- [ ] **Visual Selection Indicators**: Clear UI indicators showing what's selected for injection
- [ ] **Selection Persistence**: Maintain selection state across navigation and filtering

##### 2. Selective Injection UI
- [ ] **Injection Dialog**: Create modal for selective injection with preview
- [ ] **Injection Preview**: Show which files will be modified and what translations will be injected
- [ ] **Injection Options**: Allow users to choose injection scope (selected items only)
- [ ] **Progress Tracking**: Real-time progress indicator during selective injection

##### 3. Backend Injection Logic
- [ ] **Selective Injection API**: Modify backend to accept specific translation unit IDs
- [ ] **Partial File Updates**: Update only the files containing selected translations
- [ ] **Injection Validation**: Validate selected translations before injection
- [ ] **Rollback Support**: Ability to undo selective injections

##### 4. Integration Points
- [ ] **Translation Tables**: Add selection checkboxes and bulk injection actions
- [ ] **Translation Editor**: Add "Inject Selected" option to individual translation editor
- [ ] **Glossary System**: Selective injection for glossary terms
- [ ] **Export System**: Modify export to support selective translation injection

#### Success Metrics:
- [ ] Users can select 1-1000+ specific translations for injection
- [ ] Injection process takes <30 seconds for typical selections
- [ ] 100% accuracy in selective injection (no unintended changes)
- [ ] Clear visual feedback during injection process
- [ ] Support for both single-item and bulk selective injection

#### Technical Requirements:
- [ ] Modify `export_translated_subset` to accept translation unit IDs
- [ ] Update database queries to fetch specific translation units
- [ ] Enhance UI selection components with injection actions
- [ ] Add injection preview and confirmation dialogs
- [ ] Implement progress tracking for selective injection operations

---

## 🎯 Future Development Tasks

### Provider Architecture
- [ ] **Ollama-Compatible Provider Structure**: Create shared `OllamaCompatibleConfig` structure in `provider.rs` for providers using Ollama API format (Ollama, RunPod, etc.)
- [ ] **Unified Ollama Options**: Centralize Ollama API options (temperature, num_predict, repeat_penalty, etc.) for all Ollama-compatible providers
- [ ] **Provider Abstraction**: Distinguish between Ollama-compatible providers (Ollama, RunPod) and custom API providers (OpenAI, DeepSeek, Anthropic)

### Translation Features
- [ ] Implement bulk retranslation functionality
- [ ] Add translation progress indicators
- [ ] Improve translation quality scoring
- [ ] Add translation comparison tools

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

## 🐛 Known Issues

### Current Issues
- [ ] Table pagination sometimes resets unexpectedly
- [ ] Filter state not persisted across navigation
- [ ] Memory leaks in long-running translation sessions
- [ ] Inconsistent error handling across components

---

## 📚 Documentation

### Technical Documentation
- [ ] Document selective injection API
- [ ] Update component architecture documentation
- [ ] Add composable usage examples
- [ ] Document shared component APIs

### User Documentation
- [ ] Update user manual with selective injection features
- [ ] Add troubleshooting section
- [ ] Create video tutorials for selective injection workflow
