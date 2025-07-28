# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Items.json Support:** Complete implementation for RPG Maker MV Items.json file parsing and translation
  - Added `src/engines/rpg_maker_mv/files/items.rs` with `Item` struct and translatable fields
  - Implemented `extract_text` and `inject_translations` functions for Items.json
  - Updated engine to detect and process Items.json alongside Actors.json
  - Added proper ID prefixing (`item_`) for text unit identification
- **Skills.json Support:** Complete implementation for RPG Maker MV Skills.json file parsing and translation
  - Added `src/engines/rpg_maker_mv/files/skills.rs` with `Skill` struct and translatable fields
  - Implemented `extract_text` and `inject_translations` functions for Skills.json
  - Updated engine to detect and process Skills.json alongside Actors.json and Items.json
  - Added proper ID prefixing (`skill_`) for text unit identification
  - Supports extraction of skill names, descriptions, messages, and notes
- **Weapons.json Support:** Complete implementation for RPG Maker MV Weapons.json file parsing and translation
  - Added `src/engines/rpg_maker_mv/files/weapons.rs` with `Weapon` struct and translatable fields
  - Implemented `extract_text` and `inject_translations` functions for Weapons.json
  - Updated engine to detect and process Weapons.json alongside other file types
  - Added proper ID prefixing (`weapon_`) for text unit identification
  - Supports extraction of weapon names, descriptions, and notes
- **Armors.json Support:** Complete implementation for RPG Maker MV Armors.json file parsing and translation
  - Added `src/engines/rpg_maker_mv/files/armors.rs` with `Armor` struct and translatable fields
  - Implemented `extract_text` and `inject_translations` functions for Armors.json
  - Updated engine to detect and process Armors.json alongside other file types
  - Added proper ID prefixing (`armor_`) for text unit identification
  - Supports extraction of armor names, descriptions, and notes
- **Classes.json Support:** Complete implementation for RPG Maker MV Classes.json file parsing and translation
  - Added `src/engines/rpg_maker_mv/files/classes.rs` with `Class` struct and translatable fields
  - Implemented `extract_text` and `inject_translations` functions for Classes.json
  - Updated engine to detect and process Classes.json alongside other file types
  - Added proper ID prefixing (`class_`) for text unit identification
  - Supports extraction of class names and notes

### Changed
- **Common Helper Functions Architecture:** Refactored engine implementation to use reusable helper functions
  - Created `src/engines/common.rs` with generic `extract_file_type_text` and `inject_file_type_translations` functions
  - Updated RPG Maker MV engine to use common helper functions instead of private implementations
  - Eliminated code duplication between engine implementations
  - Improved maintainability and consistency across all file types
  - Made it easier to add new file types (just 3 lines of code per file type)
  - Enhanced type safety with proper generic function signatures
- **Advanced Common Functions Architecture:** Further refactored file modules to use higher-level abstraction functions
  - Created `src/engines/rpg_maker_mv/files/common.rs` with `extract_text_from_file_with_objects` and `inject_translations_into_file_with_objects` functions
  - Refactored all file modules (`actors.rs`, `items.rs`, `skills.rs`, `weapons.rs`, `armors.rs`, `classes.rs`) to use new common functions
  - Achieved ~70% reduction in code duplication across file modules
  - File-specific modules now focus only on their unique data structures and field logic
  - Eliminated boilerplate code for file existence checks, JSON parsing, and iteration
  - Improved maintainability and readability of all file processing code

### Technical Improvements
- **Engine Architecture:** Reduced code complexity by ~100 lines through common helper functions
- **Error Handling:** Consistent error handling and logging across all file types
- **Reusability:** Common functions can be used across different RPG Maker versions (MV, MZ, etc.)
- **Type Safety:** Enhanced generic function implementations with proper error handling
- **File Module Architecture:** Achieved ~70% code reduction in file modules through advanced common functions
- **Code Maintainability:** File-specific modules now focus only on unique data structures and field logic
- **Boilerplate Elimination:** Removed repetitive file I/O, JSON parsing, and iteration code from all file modules

## [0.1.0] - 2024-01-XX

### Added
- Initial release with Actors.json translation support
- Basic RPG Maker MV project loading and text extraction
- AI translation integration with Ollama support
- Settings management and configuration persistence
- Translation workflow with status tracking and error handling
