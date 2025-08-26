# LudoLingua Backend Architecture

This document provides a detailed walkthrough of the Rust backend's structure for the LudoLingua application. The architecture is designed with a strong emphasis on the "Separation of Concerns" principle, ensuring the codebase is modular, scalable, and easy to maintain.

---

## 1. Directory Structure Overview

The following diagram illustrates the complete file and directory layout for the backend source code located in `/src-tauri/`. This structure has been updated to support the **Progress Preservation System** with a new `db/` layer and project-based architecture.

```
/src-tauri/
├── .cargo/
├── build.rs
├── Cargo.toml
├── capabilities/
│   └── default.json
├── migrations/       # SQL migration files managed by sqlx-cli
│   ├── 0001_create_glossary_terms.sql
│   ├── 0002_glossary_unique_index.sql
│   └── 0003_translation_progress.sql     # 🆕 NEW: Progress preservation
├── models/           # JSON configuration files for LLM provider models
│   ├── ollama.json        # Available Ollama models using ModelInfo structure
│   ├── openai.json        # Available OpenAI models (future)
│   └── anthropic.json     # Available Anthropic models (future)
├── prompts/          # Prompt templates for the LLM
│   └── [prompt_name].txt # e.g., item_name.txt, dialogue.txt
├── src/
│   ├── db/           # 🆕 NEW: Database layer with progress preservation
│   │   ├── mod.rs                   # Module exports
│   │   ├── state.rs                 # Database connection & setup
│   │   ├── translation/             # Progress tracking & session management
│   │   │   ├── mod.rs
│   │   │   └── repo.rs              # CRUD operations using shared models
│   │   └── glossaries/              # Terms management (moved from old glossaries/)
│   │       ├── mod.rs
│   │       └── repo.rs              # CRUD operations using shared models
│   ├── core/         # Core application contracts and shared utilities
│   │   ├── mod.rs
│   │   ├── engine.rs # Defines the `Engine` trait (UPDATED: project_id parameter)
│   │   └── error.rs  # Custom error types for the application
│   ├── models/       # Application-wide data structures (UPDATED)
│   │   ├── mod.rs                   # Add new model exports
│   │   ├── engine.rs      # Models for game project info, settings, and data files
│   │   ├── glossary.rs    # 🆕 NEW: GlossaryTerm (moved from glossaries/)
│   │   ├── translation.rs # Models for translation + 🆕 TranslationSession, TranslationProgress
│   │   └── ...                     # provider.rs, language.rs unchanged
│   ├── engines/      # Game engine-specific logic (MODIFIED: database storage)
│   │   ├── mod.rs
│   │   ├── factory.rs     # Selects the correct game engine implementation
│   │   └── [engine_name]/ # e.g., rpg_maker_mv/
│   │       ├── mod.rs     # Declares and exports the engine modules
│   │       ├── engine.rs  # Contains the `impl Engine` block (UPDATED: DB storage)
│   │       └── files/     # Modules for handling specific data files
│   │           ├── mod.rs
│   │           └── [file_name].rs # e.g., actors.rs, items.rs
│   ├── llm/          # AI translation logic and model interaction
│   │   ├── mod.rs
│   │   ├── services/      # Service-specific provider implementations
│   │   │   ├── mod.rs
│   │   │   └── ollama.rs  # Ollama provider using ollama-rs crate
│   │   └── factory.rs     # Selects the correct LLM provider/model
│   ├── utils/         # Application-wide utilities and shared functionality
│   │   ├── mod.rs
│   │   └── prompts/       # Prompt-specific utilities
│   │       ├── mod.rs
│   │       └── builder.rs  # Shared prompt building logic for all providers
│   ├── glossaries/   # 🗑️ DEPRECATED: Moved to db/glossaries/ (models moved to src/models/glossary.rs)
│   ├── commands/     # All Tauri commands exposed to the frontend
│   │   ├── mod.rs         # Declares and exports the command modules
│   │   ├── handler.rs     # Contains all #[tauri::command] wrappers (UPDATED: progress commands)
│   │   └── [command_group].rs # e.g., engine.rs, glossary.rs (contains pure logic)
│   ├── lib.rs        # The main library entry point (UPDATED: db initialization)
│   └── main.rs       # The binary entry point (just calls lib.rs)
└── tauri.conf.json
```

### Project File Structure (New)
```
MyGame/
├── data/                      # Game data files
├── js/                        # JavaScript files
├── index.html                 # Main game file
└── .ludolingua.json           # 🆕 Project metadata & unique ID
    {
      "project_id": "uuid-12345678-90ab-cdef-1234-567890abcdef",
      "project_name": "My Awesome Game",
      "engine_type": "RPGMakerMV",
      "created_at": 1704067200,
      "ludolingua_version": "1.0.0"
    }

# SQLite Database Location (OS App Data Directory)
# 📁 Linux: ~/.local/share/ludolingua/ludolingua.db
# 📁 macOS: ~/Library/Application Support/ludolingua/ludolingua.db
# 📁 Windows: %APPDATA%\ludolingua\ludolingua.db
#
# Contains:
# - Glossary terms (via src/glossaries/)
# - 🆕 Translation sessions & progress (after migration)
# - 🆕 Translation units & metadata (after migration)
```

---

## 2. Detailed Module Explanations

This section explains the purpose of each module and how they interact with each other.

### The Foundation: Core Contracts and Data Models

At the base of our architecture lie the modules that define *what* our application works with, but not *how*.

*   **`src/core`**: This is the most abstract part of our application. It defines the fundamental "contracts" or "interfaces" that other modules must follow.
    *   `engine.rs`: Contains the `Engine` trait. This is a critical contract that defines a set of capabilities (like `load_project`, `extract_text`) that any supported game engine must provide. **UPDATED**: Now includes `project_id` parameter for database integration.
    *   `error.rs`: Defines custom error types used throughout the application, providing a unified way to handle and report problems.

*   **`src/models`**: This module contains all the passive data structures (structs) that represent the data in our application. It's the blueprint for our data. **UPDATED**: Now includes new models for progress preservation.
    *   `engine.rs`: Holds structs that map directly to the game's project and data files (e.g., `GameItem`, `GameMap`).
    *   `glossary.rs`: **🆕 NEW**: Contains the `GlossaryTerm` and `GlossaryQuery` structs (moved from `src/glossaries/`).
    *   `translation.rs`: Contains "in-flight" data structures used to manage the state of text being translated. **UPDATED**: Now includes `TranslationSession` and `TranslationProgress` for progress preservation.
    *   `provider.rs`: Contains LLM provider configurations and model information.
    *   `language.rs`: Contains language definitions and metadata.

### The Database Layer: Progress Preservation System

**🆕 NEW**: This layer provides the foundation for robust progress tracking and project management.

*   **`src/db`**: The new database layer with comprehensive progress preservation capabilities.
    *   `state.rs`: Manages the database connection pool and provides shared database state to all modules.
    *   `translation/`: Handles all progress tracking and session management.
        *   `repo.rs`: Provides CRUD operations for translation progress using shared models from `src/models/`, enabling resume functionality and progress statistics.
    *   `glossaries/`: **MOVED** from the old `src/glossaries/` directory for better organization.
        *   `repo.rs`: Provides database operations for glossary terms using shared models from `src/models/` with improved error handling.

### The Logic Centers: Specialized Engines

These modules contain the active business logic. They are the "brains" of the application, each specializing in one domain. They implement the contracts defined in `src/core`.

*   **`src/engines`**: This module is responsible for all logic related to understanding and manipulating game project files. **UPDATED**: Now stores extracted text directly in database for progress preservation.
    *   `factory.rs`: A smart component that inspects a project directory and determines which game engine it belongs to (e.g., RPG Maker MV vs. MZ).
    *   `[engine_name]/`: Each subdirectory contains the specific implementation for a given engine.
        *   `engine.rs`: The main logic file for the engine. It contains the `impl Engine for ...` block and orchestrates the file-specific modules to load, parse, and save project data. **UPDATED**: Now creates translation sessions and stores extracted text in database.
        *   `files/`: This subdirectory contains modules (`[file_name].rs`), each dedicated to handling a specific game data file.
        *   `mod.rs`: Adhering to our "imports only" rule, this file simply declares the `engine` and `files` modules and re-exports the main engine struct (e.g., `pub use engine::RpgMakerMv;`).

*   **`src/llm`**: This module handles all communication with Large Language Models.
    *   `services/`: Contains service-specific provider implementations using the best available crates for each service.
        *   `ollama.rs`: Ollama provider implementation using the `ollama-rs` crate for optimal performance and features.
    *   `factory.rs`: Similar to the engine factory, this reads the user's settings to determine which AI provider and model to use (e.g., Ollama, OpenAI).
*   **`src/utils`**: This module contains application-wide utilities and shared functionality that can be reused across different parts of the application.
    *   `prompts/`: Contains utilities specifically for prompt building and management.
        *   `builder.rs`: Shared prompt building logic that can be used by all LLM providers, ensuring consistency and reducing code duplication.

*   **`src/glossaries/`**: **🗑️ DEPRECATED**: This module has been moved to `src/db/glossaries/` as part of the database reorganization. The `GlossaryTerm` and `GlossaryQuery` models have been moved to `src/models/glossary.rs` to maintain clean separation of concerns.

### The Interface: Commands

*   **`src/commands`**: This module acts as the bridge between the Rust backend and the Nuxt.js frontend. It's designed to be a thin layer that orchestrates the more complex logic from the "engines" above. **UPDATED**: Now includes progress preservation commands.
    *   `[command_group].rs`: These files (`engine.rs`, `glossary.rs`, `translation.rs`) contain the pure business logic for user-facing actions. For testability, they contain no Tauri-specific code. **NEW**: `translation.rs` now includes progress tracking integration.
    *   `handler.rs`: This is the only file that contains the `#[tauri::command]` macro. It defines simple wrapper functions that call the pure logic functions from the other files, exposing them to the frontend. **UPDATED**: Now includes progress preservation commands like `load_project`, `save_progress`, `resume_translation`.
    *   `mod.rs`: Strictly adheres to our "imports only" rule, declaring the modules and exporting the handler.

### External Resources and Final Assembly

*   **`migrations/`**: Contains the version history of our database schema in raw `.sql` files. `sqlx-cli` is used to manage these. The application automatically applies them on startup to ensure the user's database is always up-to-date.
    *   `0001_create_glossary_terms.sql`: Initial glossary table schema
    *   `0002_glossary_unique_index.sql`: Performance optimization for glossary lookups
    *   `0003_translation_progress.sql`: **🆕 NEW**: Progress preservation tables (`translation_sessions`, `translation_units`, `translation_progress`)
*   **`models/`**: Contains JSON configuration files that define available models for each LLM provider. Each file follows the `ModelInfo` structure defined in `src/models/provider.rs` with `display_name` and `model_name` fields. This approach enables:
    *   **Runtime Configuration**: Model lists can be updated without recompiling the application.
    *   **User Customization**: Advanced users can modify these files to add custom models or remove unwanted ones.
    *   **Provider Isolation**: Each provider's models are cleanly separated in dedicated JSON files.
    *   **Graceful Fallback**: If JSON files are missing or corrupt, providers fall back to hardcoded default models.
    *   **Maintainability**: Easier to keep model catalogs up-to-date as providers release new models.
*   **`prompts/`**: Stores `.txt` files that are used as templates for prompting the AI. The application logic dynamically injects glossary terms and source text into these templates to create context-rich prompts.
*   **`lib.rs` & `main.rs`**: These are the final assembly points. `main.rs` is a minimal entrypoint that calls `lib.rs`. `lib.rs` uses the Tauri application builder to assemble all the pieces: it initializes the database connection pool, registers the command handler from `src/commands`, and starts the application. **UPDATED**: Now initializes the new `db/` layer with progress preservation capabilities.

### 🆕 Progress Preservation System

The new architecture includes a comprehensive **Progress Preservation System** that enables users to resume translation work after app crashes, power failures, or system updates.

#### Key Features
*   **Project-Based Sessions**: Each project gets a unique ID stored in `.ludolingua.json`
*   **Database-First Storage**: All translation progress stored in SQLite, not memory
*   **Resume Capability**: Users can resume from exact position after interruptions
*   **Progress Statistics**: Real-time tracking of completion status and error counts
*   **Auto-Save Integration**: Automatic progress saves during translation workflow

#### Database Schema Overview
```sql
-- Project metadata (UUID-based, not path-based)
CREATE TABLE translation_sessions (
    project_id TEXT PRIMARY KEY,
    project_path TEXT,
    engine_type TEXT,
    created_at INTEGER,
    last_modified INTEGER,
    total_texts INTEGER,
    completed_texts INTEGER,
    status TEXT
);

-- Individual text units for translation
CREATE TABLE translation_units (
    id INTEGER PRIMARY KEY,
    project_id TEXT,
    file_path TEXT,
    text_key TEXT,
    original_text TEXT,
    translated_text TEXT,
    status TEXT,
    created_at INTEGER,
    updated_at INTEGER
);

-- Progress metadata for resume functionality
CREATE TABLE translation_progress (
    project_id TEXT PRIMARY KEY,
    last_processed_file TEXT,
    last_processed_key TEXT,
    completed_count INTEGER,
    error_count INTEGER,
    last_save_at INTEGER
);
```

#### Project Structure Integration
```
MyGame/
├── .ludolingua.json    # 🆕 Project metadata & unique ID
├── data/               # Game data files
├── js/                 # JavaScript files
└── index.html          # Main game file

# SQLite Database: Stored in OS app data directory
# 📁 Linux: ~/.local/share/ludolingua/ludolingua.db
# 📁 macOS: ~/Library/Application Support/ludolingua/ludolingua.db
# 📁 Windows: %APPDATA%\ludolingua\ludolingua.db
#
# Current: Glossary terms (via src/glossaries/)
# Future: Translation sessions & progress (after migration)
```

This updated structure ensures a clean separation of concerns while adding robust progress preservation capabilities, making the backend easier to develop, debug, and maintain over time. The architecture now supports large-scale translation projects with reliable progress tracking and resume functionality.

## 🏗️ Key Architectural Principles

* **Model Reuse**: Data models in `src/models/` are used throughout the application (UI, commands, engines, database)
* **Persistence Separation**: Database layer (`src/db/`) handles storage while importing shared models
* **Clean Contracts**: `src/core/` defines interfaces that all modules implement
* **Progress Preservation**: All translation state is persisted in SQLite with project-based sessions
* **Engine Agnosticism**: Game engines work with shared models and database interfaces 