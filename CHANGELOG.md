# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
- **Architecture:**
  - Refactored command handling to follow the pattern where only handler.rs uses #[tauri::command]
  - Added detailed logging throughout the engine detection process
  - Made optional files truly optional in the detection criteria
  - Enhanced error messages for better diagnostics
  - Improved organization of game data files and text units in the UI
  - Removed example 'hello' command to clean up the codebase

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
