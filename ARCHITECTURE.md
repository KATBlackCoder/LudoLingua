# LudoLingua Architecture

This document provides a comprehensive overview of the LudoLingua application architecture, including both the Rust backend and Nuxt.js frontend components and their interactions.

## System Overview

LudoLingua is built using a hybrid architecture:

- **Frontend**: A Nuxt.js Single-Page Application (SPA) that provides the user interface
- **Backend**: A Rust-based Tauri application that handles file operations, data processing, and AI integration

The application follows a clear separation of concerns pattern, with well-defined boundaries between different components.

## Component Interaction Diagram

```
┌─────────────────────────────────────────┐     ┌─────────────────────────────────────────┐
│           FRONTEND (Nuxt.js)            │     │             BACKEND (Rust)              │
│                                         │     │                                         │
│  ┌─────────────┐      ┌─────────────┐   │     │   ┌─────────────┐      ┌─────────────┐  │
│  │    Pages    │◄────►│    Stores   │◄──┼─────┼──►│  Commands   │◄────►│    Core     │  │
│  └─────────────┘      └─────────────┘   │     │   └─────────────┘      └─────────────┘  │
│         ▲                    ▲          │     │         ▲                    ▲          │
│         │                    │          │     │         │                    │          │
│         ▼                    ▼          │     │         ▼                    ▼          │
│  ┌─────────────┐      ┌─────────────┐   │     │   ┌─────────────┐      ┌─────────────┐  │
│  │ Components  │      │    Types    │   │     │   │   Models    │◄────►│   Engines   │  │
│  └─────────────┘      └─────────────┘   │     │   └─────────────┘      └─────────────┘  │
│                                         │     │         ▲                    ▲          │
│                                         │     │         │                    │          │
│                                         │     │         ▼                    ▼          │
│                                         │     │   ┌─────────────┐      ┌─────────────┐  │
│                                         │     │   │     DB      │      │     LLM     │  │
│                                         │     │   └─────────────┘      └─────────────┘  │
└─────────────────────────────────────────┘     └─────────────────────────────────────────┘
```

## Key Interaction Patterns

### 1. Frontend-Backend Communication

The frontend and backend communicate through Tauri's command system:

1. **Command Invocation**: The frontend's Pinia stores use Tauri's `invoke()` function to call backend commands defined in `src-tauri/src/commands/`.
2. **Data Transfer**: Data is serialized/deserialized using Serde on the backend and TypeScript types on the frontend.
3. **Response Handling**: The backend returns data that is processed by the store and updates the reactive state.

### 2. Backend Component Interactions

#### Commands Layer → Core/Models/Engines

- **Purpose**: The commands layer acts as an API gateway, receiving requests from the frontend and orchestrating the appropriate backend services.
- **Interaction**: Commands validate input, call the appropriate business logic in the engines or core modules, and return serializable responses.

#### Core → Engines

- **Purpose**: The core module defines contracts (traits) that engines must implement.
- **Interaction**: The `Engine` trait in `core/engine.rs` defines methods like `load_project` and `extract_text` that engine implementations must provide.

#### Engines → Models

- **Purpose**: Engines parse game files and transform them into the application's domain models.
- **Interaction**: Engine implementations read game files, extract data, and populate model structs from `models/` that represent game entities and translation units.

#### Models → DB/LLM

- **Purpose**: Models are passed to the database and LLM modules for persistence and processing.
- **Interaction**: The glossary database operations use model structs, and the LLM module receives translation models to process.

### 3. Frontend Component Interactions

#### Pages → Components

- **Purpose**: Pages compose the UI using smaller, reusable components.
- **Interaction**: Pages pass data from stores to components via props and listen for events emitted by components.

#### Pages/Components → Stores

- **Purpose**: Stores manage application state and communicate with the backend.
- **Interaction**: UI components dispatch actions to stores, which update reactive state and/or communicate with the backend.

#### Stores → Types

- **Purpose**: Types ensure type safety between the frontend and backend.
- **Interaction**: Stores use TypeScript interfaces that mirror the backend's Rust structs to ensure data consistency.

## Data Flow Examples

### Loading a Project

1. User clicks "Load Project" in the UI (`pages/index.vue`)
2. The page calls `projectStore.loadProject()`
3. The store invokes the backend command `load_project`
4. The backend command handler calls the engine factory to determine the project type
5. The appropriate engine implementation loads and parses the project files
6. The engine returns structured data using model types
7. The data is serialized and returned to the frontend
8. The store updates its state with the project data
9. Components reactively update to display the loaded project

### Translating Text

1. User selects text and clicks "Translate" in the editor component
2. The component emits a translation event to the parent page
3. The page calls `projectStore.translateText()`
4. The store invokes the backend command `translate_text`
5. The backend command handler retrieves relevant glossary terms from the database
6. The LLM module is called with the text and glossary context
7. The AI model generates a translation
8. The result is returned to the frontend
9. The store updates the translation state
10. The UI components reactively update to show the translated text

## Directory Structure

For detailed directory structures and module explanations, please refer to:

- [Backend Structure](BACKEND_STRUCTURE.md)
- [Frontend Structure](FRONTEND_STRUCTURE.md) 