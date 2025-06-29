# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
