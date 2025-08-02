# LudoLingua Development TODO

## Current Status
**Phase:** Additional RPG Maker MV File Types Support  
**Last Completed:** 11 core data files (Actors, Items, Skills, Weapons, Armors, Classes, System, States, Enemies, CommonEvents, Troops)  
**Current Focus:** Expanding to map files

## Next Priorities (Immediate)

### 🎯 Priority 1: System Files (High Impact, Easy) - COMPLETED ✅
- [x] **System.json** - Game title, UI elements, currency, elements
- [x] **States.json** - Status effect names and descriptions  
- [x] **Enemies.json** - Enemy names and descriptions

### 🎯 Priority 2: Event Files (High Impact, Complex)
- [x] **CommonEvents.json** - Common events with dialogue
- [x] **Troops.json** - Troop names and event commands

### 🎯 Priority 3: Map Files (Complex but High Value)
- [ ] **MapXXX.json** - Individual map files with event dialogue
    - [ ] **Add text extraction from event commands (codes 101, 401, 102, 402, etc.)**
        - [ ] Extract text from commands with specific code values:
        - [ ] `101` (Show Text - Message Window attributes)
        - [ ] `401` (Show Text - Message content lines)
        - [ ] `102` (Show Choices)
        - [ ] `402` (When [Choice] selected - Choice content)
        - [ ] `405` (Show Scrolling Text)
        - [ ] `108` (Comment)
        - [ ] `408` (Comment continuation)
        - [ ] `320` (Change Actor Name)
        - [ ] `324` (Change Actor Nickname)
        - [ ] `355` (Script)
        - [ ] `655` (Script continuation)
## Quick Reference

### Architecture
- **Backend:** Rust + Tauri with trait-based engine system
- **Frontend:** Nuxt.js + Pinia stores
- **LLM:** Ollama integration with JSON model configs
- **Storage:** Tauri Store plugin for settings

### File Structure
```
src-tauri/src/engines/rpg_maker_mv/files/
├── actors.rs ✅
├── items.rs ✅
├── skills.rs ✅
├── weapons.rs ✅
├── armors.rs ✅
├── classes.rs ✅
├── system.rs ✅
├── states.rs ✅
├── enemies.rs ✅
├── common_events.rs ✅
├── troops.rs ✅
└── maps.rs [TODO]
```

### Implementation Pattern
Each file follows the same pattern:
1. Define structs matching JSON structure
2. Implement `extract_text()` function using common helpers
3. Implement `inject_translations()` function using common helpers
4. Add to engine file detection
5. Use appropriate prompt types (State, System, Character, etc.)
6. Test with sample project

### Prompt System
- **Base:** `erobasic.txt` (streamlined core guidelines)
- **Specialized:** `character.txt`, `equipment.txt`, `skill.txt`, `dialogue.txt`, `state.txt`, etc.
- **Features:** Direct translation mappings, flexible guidelines, aggressive output formatting

## Key Decisions

### ✅ Completed Optimizations
- **Prompt System:** Consolidated equipment prompts, added specific mappings
- **UI/UX:** Load New Project button, Reset functionality, improved workflow
- **Architecture:** Common helper functions, ~70% code reduction
- **LLM:** Best-of-breed approach with ollama-rs crate

### 🔄 Current Approach
- **File Priority:** System files first (easy wins), then events (high impact)
- **Implementation:** Follow established patterns, reuse common functions
- **Testing:** Each file type tested individually before moving to next

## Future Considerations
- **RPG Maker MZ Support** - After MV completion
- **Additional Engines** - Wolf RPG, etc.
- **Advanced Features** - Batch translation per file type, filtering UI
