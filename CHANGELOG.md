# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- **Method Naming Refactor (Phase 5 Preparation):**
  - Renamed `save_text_units` to `inject_text_units` throughout the codebase for accurate naming
  - Updated Engine trait method from `save_text_units` to `inject_text_units` in `src/core/engine.rs`
  - Updated RPG Maker MV engine implementation in `src/engines/rpg_maker_mv/engine.rs`
  - Updated command handler in `src/commands/engine.rs` and `src/commands/handler.rs`
  - Updated Tauri command registration in `src/lib.rs`
  - Updated frontend store in `stores/engine.ts` to use new command name
  - Enhanced documentation and logging messages to reflect injection vs saving semantics
  - Prepared foundation for upcoming backup-based save system implementation

- **JSON Model Configuration System (Phase 3.6):**
  - Replaced hardcoded model definitions with external JSON configuration files
  - Created `src-tauri/models/` directory for provider model definitions
  - Implemented `src-tauri/models/ollama.json` with comprehensive Ollama model catalog
  - Added template JSON files for future providers (openai.json, anthropic.json)
  - Updated `src/llm/services/ollama.rs` to load models from JSON instead of hardcoded lists
  - Added JSON parsing and validation with graceful fallback to hardcoded defaults
  - Enhanced maintainability by separating configuration data from code logic
  - Enabled model list updates without recompilation and user customization capabilities

### Added
- **Store Architecture Refactoring (Domain-Driven Design):**
  - Created dedicated `stores/translate.ts` for translation process management with comprehensive features:
    - Individual and batch translation operations with progress tracking
    - Translation history with undo functionality
    - Failed translation tracking and retry mechanisms
    - Real-time progress indicators with percentage completion
    - Toast notifications for user feedback throughout translation workflow
  - Renamed `stores/project.ts` to `stores/engine.ts` with focused responsibilities:
    - Project loading, refreshing, and state management
    - File operations and current file selection
    - Text unit CRUD operations and search functionality
    - Project statistics and translation progress metrics
    - Comprehensive error handling with user feedback
  - Refined `stores/settings.ts` for pure configuration management:
    - LLM provider settings with comprehensive validation
    - Real-time connection testing with status tracking and timestamps
    - Configuration validation with detailed error reporting
    - Settings persistence with automatic save/load functionality
  - Enhanced `stores/language.ts` for centralized language management:
    - Proper source and target language handling
    - Language validation and compatibility checking
    - Reusable language selection across components

### Added
- **LLM Architecture Enhancement (Pure Data Structures & Type Safety):**
  - Created type-safe `Provider` enum replacing string-based provider selection for compile-time validation
  - Added `ModelInfo` struct with display_name and model_name fields for better model management
  - Implemented comprehensive `OllamaModel` enum with 10 popular models (Mistral, Llama 3.2, Llama 3.1, CodeLlama, Phi-3, Gemma 2, Qwen 2.5)
  - Added model discovery, validation, and display name methods to `OllamaProvider`
  - Created specialized prompt templates for `item.txt`, `skill.txt`, and `other.txt` with context-specific guidelines
  - Enhanced frontend with `Provider` type union and enum validation in forms
  - Updated settings page with improved About section and status indicators
  - Added professional translation guidelines for items, skills, and general game text

- **Model Management System:**
  - Added `get_available_models()` method for dynamic model discovery
  - Implemented `is_model_supported()` for model validation
  - Created `model_from_name()` for string-to-enum conversion
  - Added `get_model_display_name()` for user-friendly model names
  - Enhanced UI with proper model selection and display

- **Enhanced Prompt System:**
  - Created dedicated prompt templates for different content types:
    - `item.txt`: Specialized for game items with focus on function and UI constraints
    - `skill.txt`: Optimized for abilities with emphasis on power and impact
    - `other.txt`: General game text with cultural appropriateness guidelines
  - Updated prompt selection logic to use dedicated templates instead of fallbacks
  - Added context-aware translation instructions for better quality

### Changed
- **Store Architecture:**
  - Refactored store system to follow domain-driven design principles with clear separation of concerns
  - Updated all component imports from `useProjectStore` to `useEngineStore` throughout the application
  - Enhanced component integration with direct store access for better performance
  - Improved error handling and user feedback across all store operations
  - Maintained backward compatibility while improving internal architecture

### Changed
- **Data Structure Architecture:**
  - Refactored `models/provider.rs` to use pure data structures without `impl` methods
  - Removed all `Default` implementations in favor of explicit construction
  - Updated `LlmFactory` to use pattern matching on `Provider` enum
  - Changed frontend forms to use `z.enum()` validation instead of string validation
  - Updated `TranslationSettingsForm.vue` to use `USelectMenu` and `Language` objects
  - Enhanced `LlmSettingsForm.vue` with proper enum handling and type casting

### Fixed
- **Type Safety Issues:**
  - Fixed linter errors in translation commands by updating to enum-based provider system
  - Resolved `toString()` trait issues by using pattern matching for provider names
  - Fixed form validation to use proper enum types instead of strings
  - Corrected TypeScript type mismatches between frontend and backend

### Improved
- **Store Architecture Quality:**
  - Achieved proper separation of concerns with each store having a focused, single responsibility
  - Enhanced maintainability by isolating domain logic to prevent cross-contamination
  - Improved testability with isolated stores that are easier to test and debug
  - Better reusability with translation logic that can be used across different contexts
  - Scalable design that makes it easy to add new features without bloating existing stores
  - Enhanced user experience with better error handling and progress feedback

- **Architecture Quality:**
  - Achieved better separation of concerns with pure data structures
  - Enhanced maintainability by removing complex `impl` methods from models
  - Improved type safety with compile-time validation throughout the system
  - Simplified code by pushing logic to factory and service layers
  - Enhanced extensibility for adding new providers and models

- **Translation Quality:**
  - Significantly improved translation accuracy with specialized prompt templates
  - Better context awareness for different types of game content
  - More appropriate guidelines for translating items, skills, and general text
  - Enhanced consistency across different content types

### Added
- **LLM Architecture Refactoring (Best-of-Breed Approach):**
  - Migrated to dedicated `ollama-rs` crate for Ollama integration (857 stars, feature-rich API)
  - Created `src/llm/services/` directory for service-specific adapters
  - Implemented `LlmProvider` trait with `async_trait` for object-safe async methods
  - Added `TranslationLanguages` struct for comprehensive translation request handling
  - Created `Language` interface with code, name, and native_name fields for better UX
  - Enhanced prompt template loading from `prompts/` directory with fallback handling
  - Updated `LlmConfig` in `models/provider.rs` with proper `Default` implementation
  - Removed custom HTTP client code in favor of optimized service-specific implementations
  - Updated all frontend components to use new `Language` objects instead of strings
  - Enhanced translation workflow to one-by-one processing (removed batch translation)
  - Fixed all TypeScript interfaces to match new backend architecture
  - Added native language names for improved language selection UX

### Added
- **LLM Translation System (Phase 3):**
  - Implemented complete LLM abstraction layer with trait-based architecture
  - Created Ollama provider integration with professional prompt engineering
  - Added single and batch translation commands for text units
  - Implemented persistent LLM configuration using Tauri store plugin
  - Created comprehensive settings UI with connection testing and validation
  - Added translation workflow with status tracking and visual indicators
  - Integrated context-aware prompt selection based on text type
  - Embedded prompt templates in binary for better architecture and distribution
  - Added translation functions to project store for seamless UI integration
  - Created professional prompt templates for character names, descriptions, and dialogue

- **Settings Management:**
  - Built complete settings page with tabbed navigation and modern UI
  - Created LLM configuration form with connection testing
  - Added translation settings for source and target language selection
  - Implemented reactive settings store with persistent state management
  - Added configuration validation and error handling throughout the workflow

- **Translation User Interface:**
  - Enhanced TranslationTable component with individual and batch translate buttons
  - Added translation status indicators with color-coded badges
  - Implemented settings validation modal to guide users through proper configuration
  - Added translation progress feedback and error handling
  - Created intuitive card-based layout for better text unit visualization

- **Architecture Improvements:**
  - Moved prompt templates from frontend to backend (`src-tauri/prompts/`) for proper separation of concerns
  - Embedded prompt templates in binary using `include_str!()` macro for better distribution
  - Added context-aware prompt selection to choose appropriate templates based on text type
  - Implemented proper error handling with LLM-specific error types
  - Added comprehensive logging throughout the translation pipeline

### Added
- **Engine Architecture:**
  - Implemented the engine factory system for detecting and instantiating appropriate game engine handlers
  - Created the RPG Maker MV engine implementation with project detection criteria
  - Added functionality to extract project metadata from package.json files
  - Designed an extensible architecture for supporting multiple game engines in the future
  - Added `as_any` method to Engine trait for downcasting to specific engine implementations
  - Created a structured GameDataFile model to organize text units by their source files
  
- **Project Loading:**
  - Added Tauri commands for loading projects and extracting text
  - Implemented frontend-backend communication for project loading
  - Created UI for displaying loaded project information
  - Updated the project store to manage project state
  - Added new `extract_game_data_files` command to retrieve structured game data files
  
- **Text Extraction:**
  - Implemented Actors.json parsing for RPG Maker MV projects
  - Created data structures to represent RPG Maker MV actor data
  - Added text extraction for character names, nicknames, profiles, and notes
  - Created a translation table UI component to display extracted text units
  - Implemented a file browser alongside translation table for better organization of game files

- **Frontend Improvements:**
  - Refactored layout into component-based architecture with `AppHeader` and `AppFooter` components
  - Enhanced UI with Nuxt UI components for a more modern and consistent interface
  - Added dark mode toggle functionality in the footer
  - Created About page to showcase application information
  - Improved navigation with icons and active state indicators
  - Fixed TypeScript errors in the TranslationTable component
  - Modularized the project loading functionality into a dedicated `ProjectLoader` component
  - Created a `FileExplorer` component to handle game file selection and display
  - Improved home page layout with better component organization and responsiveness
  - Added container component for better layout consistency
  - Created a `ProjectInfoAlert` component to display project information at application level
  - Moved project info from `ProjectLoader` to a global alert in the default layout

### Fixed
- **LLM Architecture:**
  - Fixed async trait object safety issues by implementing `async_trait` for `LlmProvider`
  - Resolved linter errors in translation commands by updating function signatures
  - Fixed TypeScript type mismatches between frontend and backend interfaces
  - Corrected translation status enum handling for new backend variants
  - Fixed form field validation in settings components to use proper Nuxt UI components
  - Resolved import errors by reorganizing LLM configuration to proper module structure
  - Fixed glossary terms handling to use simple tuple format instead of complex structs

- **Architecture:**
  - Fixed prompt template location by moving from frontend to backend directory
  - Resolved runtime path issues with prompt loading by embedding templates in binary
  - Fixed distribution problems where prompt files were not accessible after packaging
  - Corrected architecture violation where backend logic depended on frontend file structure

- **Engine Detection:**
  - Improved detection criteria for RPG Maker MV projects to handle various folder structures
  - Enhanced error handling for projects without package.json files
  - Added fallback to use folder name as project name when metadata is unavailable
  - Fixed detection logic to properly identify projects with non-standard layouts

- **Data Parsing:**
  - Fixed Actors.json parsing to match the actual array structure of the file
  - Corrected GameDataFile path handling to use relative paths for better serialization
  - Updated the actor model to include the note field as translatable text
  - Added default values and flexible field handling to Actor struct to handle varying JSON structures
  - Fixed parsing error caused by missing fields in some actor entries
  - Added proper field name mapping using serde rename attributes for camelCase JSON fields
  - Added missing fields like equips and traits to the Actor struct
  - Added more derive traits (Clone, Default) to the Actor struct for better compatibility

### Improved
- **LLM System Architecture:**
  - Refactored to use dedicated service crates for optimal performance and maintainability
  - Enhanced provider abstraction with unified `LlmProvider` trait interface
  - Improved configuration system with proper type safety and validation
  - Enhanced translation request handling with comprehensive `TranslationLanguages` struct
  - Streamlined one-by-one translation workflow for better user control and feedback
  - Added support for native language names in UI for improved user experience
  - Enhanced prompt template system with better error handling and fallback mechanisms

- **Translation System:**
  - Enhanced prompt engineering with professional game localization guidelines
  - Improved context-aware template selection for different text types
  - Added comprehensive error handling with user-friendly error messages
  - Implemented robust connection testing with detailed feedback
  - Enhanced translation workflow with proper state management and persistence

- **Architecture:**
  - Refactored command handling to follow the pattern where only handler.rs uses #[tauri::command]
  - Added detailed logging throughout the engine detection process
  - Made optional files truly optional in the detection criteria
  - Enhanced error messages for better diagnostics
  - Improved organization of game data files and text units in the UI
  - Removed example 'hello' command to clean up the codebase
  - Moved to embedded templates for better deployment and security

- **User Interface:**
  - Restructured application layout for better component reusability
  - Enhanced visual consistency by using Nuxt UI components throughout the application
  - Improved user experience with better navigation and layout
  - Added support for dark mode to improve accessibility and user preference
  - Refactored index page to use modular components for better maintainability
  - Improved conditional rendering to hide test functionality when a project is loaded
  - Removed test functionality from the UI for a cleaner interface
  - Simplified TranslationTable component by removing unused functions and using basic UTable
  - Enhanced user experience by showing loaded project information consistently across all pages

### Ongoing
- **Frontend Refactoring:**
  - Continuing to improve the UI components and layout
  - Enhancing the project loading experience
  - Creating reusable components for common UI patterns
  - Implementing responsive design for different screen sizes
  - Adding proper loading states and transitions

## [0.1.0] - 2024-07-01

### Added
- **Project Foundation:** Initialized the LudoLingua project with Nuxt.js for the frontend and Rust/Tauri for the backend.
- **Core Backend Structure:**
    - Established the core module structure (`src/core`) with error handling.
    - Set up the command module structure (`src/commands`) for handling frontend invocations.
    - Integrated essential plugins: `tauri-plugin-log` and `tauri-plugin-window-state`.
    - Added a sample `hello` command to verify backend-frontend communication.
- **Core Frontend Structure:**
    - Configured Nuxt.js with required modules: `@nuxt/ui`, `pinia`, `@nuxt/eslint`, and `@nuxt/icon`.
    - Created a default layout with navigation between pages.
    - Added placeholder pages for Home, Glossary, and Settings.
    - Implemented a Pinia store (`ui.ts`) for managing global UI state.
    - Added global CSS styles and application head metadata.
    - Integrated UApp component for proper Nuxt UI configuration.
    - Improved layout structure following Nuxt best practices.
