# TODO

## 🎯 Current Phase: Selective Injection System

### Phase 1: Selective Translation Injection
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
