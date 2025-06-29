# LudoLingua Backend Architecture

This document provides a detailed walkthrough of the Rust backend's structure for the LudoLingua application. The architecture is designed with a strong emphasis on the "Separation of Concerns" principle, ensuring the codebase is modular, scalable, and easy to maintain.

---

## 1. Directory Structure Overview

The following diagram illustrates the complete file and directory layout for the backend source code located in `/src-tauri/`.

```
/src-tauri/
├── .cargo/
├── build.rs
├── Cargo.toml
├── capabilities/
│   └── default.json
├── migrations/       # SQL migration files for the glossary, managed by sqlx-cli
│   └── [timestamp]_create_glossary_table.sql
├── prompts/          # Prompt templates for the LLM
│   └── [prompt_name].txt # e.g., item_name.txt, dialogue.txt
├── src/
│   ├── core/         # Core application contracts and shared utilities
│   │   ├── mod.rs
│   │   ├── engine.rs # Defines the `Engine` trait, the contract for all engines
│   │   └── error.rs  # Custom error types for the application
│   ├── models/       # Application-wide data structures
│   │   ├── mod.rs
│   │   ├── engine.rs      # Models for game project info, settings, and data files
│   │   ├── glossary.rs    # Models for glossary terms (e.g., from the database)
│   │   └── translation.rs # Models for translation data and AI interaction
│   ├── engines/      # Game engine-specific logic (parsers, writers)
│   │   ├── mod.rs
│   │   ├── factory.rs     # Selects the correct game engine implementation
│   │   └── [engine_name]/ # e.g., rpg_maker_mv/
│   │       ├── mod.rs     # Declares and exports the engine modules
│   │       ├── engine.rs  # Contains the `impl Engine` block and orchestrates file modules
│   │       └── files/     # Modules for handling specific data files
│   │           ├── mod.rs
│   │           └── [file_name].rs # e.g., actors.rs, items.rs
│   ├── llm/          # AI translation logic and model interaction
│   │   ├── mod.rs
│   │   ├── trait.rs       # Defines the common `Translator` trait
│   │   ├── factory.rs     # Selects the correct LLM provider/model
│   │   └── [provider].rs  # e.g., ollama.rs, gguf.rs
│   ├── db/           # Database interaction layer using sqlx
│   │   ├── mod.rs
│   │   ├── connection.rs # Logic for establishing the SQLite connection pool
│   │   └── glossary.rs   # All CRUD (Create, Read, Update, Delete) functions for the glossary table
│   ├── commands/     # All Tauri commands exposed to the frontend
│   │   ├── mod.rs         # Declares and exports the command modules
│   │   ├── handler.rs     # Contains all #[tauri::command] wrappers and the handler
│   │   └── [command_group].rs # e.g., engine.rs, glossary.rs (contains pure logic)
│   ├── lib.rs        # The main library entry point where we build the app
│   └── main.rs       # The binary entry point (just calls lib.rs)
└── tauri.conf.json
```

---

## 2. Detailed Module Explanations

This section explains the purpose of each module and how they interact with each other.

### The Foundation: Core Contracts and Data Models

At the base of our architecture lie the modules that define *what* our application works with, but not *how*.

*   **`src/core`**: This is the most abstract part of our application. It defines the fundamental "contracts" or "interfaces" that other modules must follow.
    *   `engine.rs`: Contains the `Engine` trait. This is a critical contract that defines a set of capabilities (like `load_project`, `extract_text`) that any supported game engine must provide.
    *   `error.rs`: Defines custom error types used throughout the application, providing a unified way to handle and report problems.

*   **`src/models`**: This module contains all the passive data structures (structs) that represent the data in our application. It's the blueprint for our data.
    *   `engine.rs`: Holds structs that map directly to the game's project and data files (e.g., `GameItem`, `GameMap`).
    *   `glossary.rs`: Contains the `GlossaryTerm` struct, which mirrors the structure of our database table.
    *   `translation.rs`: Contains "in-flight" data structures used to manage the state of text being translated.

### The Logic Centers: Specialized Engines

These modules contain the active business logic. They are the "brains" of the application, each specializing in one domain. They implement the contracts defined in `src/core`.

*   **`src/engines`**: This module is responsible for all logic related to understanding and manipulating game project files.
    *   `factory.rs`: A smart component that inspects a project directory and determines which game engine it belongs to (e.g., RPG Maker MV vs. MZ).
    *   `[engine_name]/`: Each subdirectory contains the specific implementation for a given engine.
        *   `engine.rs`: The main logic file for the engine. It contains the `impl Engine for ...` block and orchestrates the file-specific modules to load, parse, and save project data.
        *   `files/`: This subdirectory contains modules (`[file_name].rs`), each dedicated to handling a specific game data file.
        *   `mod.rs`: Adhering to our "imports only" rule, this file simply declares the `engine` and `files` modules and re-exports the main engine struct (e.g., `pub use engine::RpgMakerMv;`).

*   **`src/llm`**: This module handles all communication with Large Language Models.
    *   `trait.rs`: Defines the `Translator` trait, a contract for any AI provider we want to support.
    *   `factory.rs`: Similar to the engine factory, this reads the user's settings to determine which AI provider and model to use (e.g., Ollama, a local file).
    *   `[provider].rs`: Each file is a self-contained implementation for a specific AI provider, handling the unique details of its API.

*   **`src/db`**: This module is the sole gatekeeper for our SQLite database.
    *   `connection.rs`: Manages the creation of a database connection pool, which is essential for performance.
    *   `glossary.rs`: Provides a safe, Rust-native API for all database operations on the glossary table (Create, Read, Update, Delete). No other module writes SQL.

### The Interface: Commands

*   **`src/commands`**: This module acts as the bridge between the Rust backend and the Nuxt.js frontend. It's designed to be a thin layer that orchestrates the more complex logic from the "engines" above.
    *   `[command_group].rs`: These files (`engine.rs`, `glossary.rs`) contain the pure business logic for user-facing actions. For testability, they contain no Tauri-specific code.
    *   `handler.rs`: This is the only file that contains the `#[tauri::command]` macro. It defines simple wrapper functions that call the pure logic functions from the other files, exposing them to the frontend. It also generates the single invoke handler for Tauri.
    *   `mod.rs`: Strictly adheres to our "imports only" rule, declaring the modules and exporting the handler.

### External Resources and Final Assembly

*   **`migrations/`**: Contains the version history of our database schema in raw `.sql` files. `sqlx-cli` is used to manage these. The application automatically applies them on startup to ensure the user's database is always up-to-date.
*   **`prompts/`**: Stores `.txt` files that are used as templates for prompting the AI. The application logic dynamically injects glossary terms and source text into these templates to create context-rich prompts.
*   **`lib.rs` & `main.rs`**: These are the final assembly points. `main.rs` is a minimal entrypoint that calls `lib.rs`. `lib.rs` uses the Tauri application builder to assemble all the pieces: it initializes the database connection pool, registers the command handler from `src/commands`, and starts the application.

This structure ensures a clean separation of concerns, making the backend easier to develop, debug, and maintain over time. 