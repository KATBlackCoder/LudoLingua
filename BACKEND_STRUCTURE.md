# LudoLingua Backend Architecture

This document provides a detailed walkthrough of the Rust backend's structure for the LudoLingua application. The architecture is designed with a strong emphasis on the "Separation of Concerns" principle, ensuring the codebase is modular, scalable, and easy to maintain. The backend now supports multiple LLM providers, database persistence, and engine-agnostic export functionality.

---

## 1. Directory Structure Overview

The following diagram illustrates the complete file and directory layout for the backend source code located in `/src-tauri/`. This structure supports multiple LLM providers, database persistence, and engine-agnostic export functionality.

```
/src-tauri/
├── .cargo/
├── build.rs
├── Cargo.toml
├── capabilities/
│   └── default.json
├── migrations/       # SQL migration files for database schema
│   ├── 0001_create_glossary_terms.sql
│   └── 0002_glossary_unique_index.sql
├── models/           # JSON configuration files for LLM provider models
│   ├── ollama.json        # Ollama models with pricing and context windows
│   ├── openai.json        # OpenAI models configuration
│   ├── groq.json          # Groq models configuration
│   ├── openrouter.json    # OpenRouter models configuration
│   └── language.json      # Language definitions and metadata
├── prompts/          # Prompt templates for the LLM
│   ├── basic.txt          # Base prompt template
│   ├── dialogue.txt       # Dialogue-specific prompts
│   ├── character.txt      # Character name prompts
│   ├── equipment.txt      # Equipment/item prompts
│   ├── class.txt          # Character class prompts
│   ├── skill.txt          # Skill prompts
│   ├── state.txt          # Status effect prompts
│   ├── system.txt         # System text prompts
│   └── vocabularies.txt   # Curated vocabulary for consistency
├── src/
│   ├── db/           # Database layer for persistence
│   │   ├── mod.rs                   # Module exports
│   │   ├── state.rs                 # Database connection & setup
│   │   ├── translation/             # Translation progress & session management
│   │   │   ├── mod.rs
│   │   │   ├── manifest.rs          # Project manifest handling
│   │   │   └── repo.rs              # CRUD operations for translations
│   │   └── glossary/                # Glossary term management
│   │       ├── mod.rs
│   │       └── repo.rs              # CRUD operations for glossary terms
│   ├── core/         # Core application contracts and shared utilities
│   │   ├── mod.rs
│   │   ├── engine.rs # Engine trait with reconstruct_text_unit_id
│   │   ├── error.rs  # Custom error types
│   │   └── provider.rs # LLM provider abstractions
│   ├── models/       # Application-wide data structures
│   │   ├── mod.rs                   # Module exports
│   │   ├── engine.rs      # Game project and engine models
│   │   ├── glossary.rs    # Glossary term models
│   │   ├── provider.rs    # LLM provider and model models
│   │   ├── language.rs    # Language models
│   │   ├── settings.rs    # Application settings models
│   │   └── translation.rs # Translation workflow models
│   ├── engines/      # Game engine-specific implementations
│   │   ├── mod.rs
│   │   ├── factory.rs     # Engine factory and dispatch logic
│   │   ├── common.rs      # Shared engine utilities
│   │   └── [engine_name]/ # Engine-specific implementations
│   │       ├── mod.rs
│   │       ├── engine.rs  # Engine trait implementation
│   │       └── files/     # File-specific parsing modules
│   │           ├── mod.rs
│   │           └── [file_name].rs # Individual file parsers
│   ├── llm/          # AI translation logic and providers
│   │   ├── mod.rs
│   │   ├── state.rs       # LLM state management
│   │   ├── factory.rs     # Provider factory
│   │   └── services/      # Provider service implementations
│   │       ├── mod.rs
│   │       ├── ollama.rs     # Ollama provider
│   │       ├── openai.rs     # OpenAI provider
│   │       ├── groq.rs       # Groq provider
│   │       └── openrouter.rs # OpenRouter provider
│   ├── utils/         # Application-wide utilities
│   │   ├── mod.rs
│   │   ├── prompts/       # Prompt building utilities
│   │   │   ├── mod.rs
│   │   │   └── builder.rs  # Shared prompt building logic
│   │   ├── text_processing.rs # Text processing utilities
│   │   └── token_estimation.rs # Token and cost estimation
│   ├── commands/     # Tauri commands exposed to frontend
│   │   ├── mod.rs         # Module declarations
│   │   ├── handler.rs     # Tauri command wrappers
│   │   ├── engine.rs      # Engine-related commands
│   │   ├── glossary.rs    # Glossary management commands
│   │   ├── languages.rs   # Language configuration commands
│   │   ├── provider.rs    # LLM provider commands
│   │   └── translation.rs # Translation workflow commands
│   ├── lib.rs        # Main library entry point
│   └── main.rs       # Binary entry point
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

### The Database Layer: Persistence and State Management

This layer provides robust data persistence and state management for the application.

*   **`src/db`**: Comprehensive database layer for application data persistence.
    *   `state.rs`: Manages SQLite database connection pool and provides shared database state.
    *   `translation/`: Translation progress and session management.
        *   `manifest.rs`: Project manifest handling with SHA-based project identification.
        *   `repo.rs`: CRUD operations for translation units with efficient querying.
    *   `glossary/`: Glossary term management and persistence.
        *   `repo.rs`: Database operations for glossary terms with language-specific support.

### The Logic Centers: Specialized Engines

These modules contain the active business logic. They are the "brains" of the application, each specializing in one domain. They implement the contracts defined in `src/core`.

*   **`src/engines`**: Game engine implementations and file processing logic.
    *   `factory.rs`: Engine detection and instantiation with factory pattern for extensibility.
    *   `common.rs`: Shared utilities for file processing across all engines.
    *   `[engine_name]/`: Engine-specific implementations (RPG Maker MV, MZ, Wolf RPG).
        *   `engine.rs`: Engine trait implementation with `reconstruct_text_unit_id` for export.
        *   `files/`: File-specific parsing modules with selective text extraction.

*   **`src/llm`**: Multi-provider LLM integration and translation services.
    *   `state.rs`: LLM state management with connection pooling and shared services.
    *   `factory.rs`: Provider factory for instantiating appropriate LLM services.
    *   `services/`: Provider-specific service implementations.
        *   `ollama.rs`: Local Ollama integration with model discovery.
        *   `openai.rs`: OpenAI API integration with token management.
        *   `groq.rs`: Groq API integration for high-performance translation.
        *   `openrouter.rs`: OpenRouter API integration for model access.
*   **`src/utils`**: Application-wide utilities and shared functionality.
    *   `prompts/`: Prompt building and template management.
        *   `builder.rs`: Shared prompt building logic with glossary integration.
    *   `text_processing.rs`: Text processing utilities for placeholders and formatting.
    *   `token_estimation.rs`: Token and cost estimation for translation budgeting.

### The Interface: Commands

*   **`src/commands`**: Tauri command interface between frontend and backend.
    *   `engine.rs`: Engine-related commands (load_project, extract_text, export).
    *   `glossary.rs`: Glossary management commands (CRUD operations).
    *   `languages.rs`: Language configuration commands.
    *   `provider.rs`: LLM provider management commands.
    *   `translation.rs`: Translation workflow commands.
    *   `handler.rs`: Tauri command wrappers and registration.
    *   `mod.rs`: Module declarations and exports.

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