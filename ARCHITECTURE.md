# LudoLingua Architecture

This document provides a comprehensive overview of the LudoLingua application architecture, including both the Rust backend and Nuxt.js frontend components and their interactions.

## System Overview

LudoLingua is built using a modern hybrid architecture:

- **Frontend**: Nuxt 4 Single-Page Application with Nuxt UI components and TypeScript
- **Backend**: Rust-based Tauri application with SQLite persistence and multi-provider LLM support

The application follows a clean separation of concerns with well-defined boundaries between components, supporting multiple game engines and AI translation providers.

## Component Interaction Diagram

```
┌─────────────────────────────────────────────┐     ┌─────────────────────────────────────────────┐
│            FRONTEND (Nuxt 4)                │     │             BACKEND (Rust)                  │
│                                             │     │                                             │
│  ┌─────────────┐      ┌─────────────┐       │     │   ┌─────────────┐      ┌─────────────┐      │
│  │    Pages    │◄────►│    Stores   │◄─────┼─────┼──►│  Commands   │◄────►│    Core     │      │
│  └─────────────┘      └─────────────┘       │     │   └─────────────┘      └─────────────┘      │
│         ▲                    ▲              │     │         ▲                    ▲              │
│         │                    │              │     │         │                    │              │
│         ▼                    ▼              │     │         ▼                    ▼              │
│  ┌─────────────┐      ┌─────────────┐       │     │   ┌─────────────┐      ┌─────────────┐      │
│  │ Components  │      │ Composables │       │     │   │   Models    │◄────►│   Engines   │      │
│  └─────────────┘      └─────────────┘       │     │   └─────────────┘      └─────────────┘      │
│         ▲                    ▲              │     │         ▲                    ▲              │
│         │                    │              │     │         │                    │              │
│         ▼                    ▼              │     │         ▼                    ▼              │
│  ┌─────────────┐      ┌─────────────┐       │     │   ┌─────────────┐      ┌─────────────┐      │
│  │    Types    │      │   Plugins   │       │     │   │     DB      │      │     LLM     │      │
│  └─────────────┘      └─────────────┘       │     │   └─────────────┘      └─────────────┘      │
└─────────────────────────────────────────────┘     └─────────────────────────────────────────────┘
```

## Key Interaction Patterns

### 1. Frontend-Backend Communication

The frontend and backend communicate through Tauri's command system:

1. **Store Coordination**: Frontend Pinia stores coordinate multi-store interactions (engine + provider + glossary)
2. **Command Invocation**: Stores use Tauri's `invoke()` to call backend commands
3. **Data Serialization**: Serde handles Rust ↔ TypeScript data transformation
4. **Reactive Updates**: Vue 3 reactivity automatically syncs UI with store state changes

### 2. Backend Component Interactions

#### Commands Layer → Core/Models/Engines

- **Purpose**: Commands layer serves as API gateway between frontend and backend services
- **Interaction**: Validates input, orchestrates business logic, returns serializable responses

#### Core → Engines

- **Purpose**: Core defines contracts and abstractions for game engine implementations
- **Interaction**: `Engine` trait provides `load_project`, `extract_text`, `inject_text_units`, `reconstruct_text_unit_id`

#### Engines → Models

- **Purpose**: Engines parse game files into domain models for processing
- **Interaction**: File parsers populate model structs representing game entities and text units

#### Models → DB/LLM

- **Purpose**: Models enable data persistence and AI processing
- **Interaction**: Database operations use models for CRUD, LLM receives models for translation

#### Multi-Provider LLM Integration

- **Purpose**: Support multiple AI providers with unified interface
- **Interaction**:
  - Factory pattern instantiates appropriate provider services
  - Shared prompt builder ensures consistency across providers
  - Token estimation provides cost/budgeting information

### 3. Frontend Component Interactions

#### Pages → Components

- **Purpose**: Pages orchestrate UI composition using reusable components
- **Interaction**: Pages pass reactive data from stores via props, handle component events

#### Components → Composables

- **Purpose**: Composables provide shared logic and state management
- **Interaction**: Components use composables for toast notifications, translation utilities, glossary operations

#### Pages/Components → Stores

- **Purpose**: Stores manage application state and backend communication
- **Interaction**: Components trigger store actions, stores handle async operations and state updates

#### Stores → Types

- **Purpose**: TypeScript ensures type safety across the application
- **Interaction**: Stores use typed interfaces matching backend Rust structs

#### Multi-Store Coordination

- **Purpose**: Complex workflows require coordination between multiple stores
- **Interaction**: Stores reference each other (engine + provider + glossary) for unified operations

## Data Flow Examples

### Modern Project Loading & Translation Workflow

1. **User Action**: User clicks "Load Project" in translation page
2. **Store Coordination**: `engineStore.loadProject()` coordinates with settings store
3. **Backend Command**: Store invokes `invoke('load_project')` with project path
4. **Engine Detection**: Backend factory detects engine type (MV/MZ/Wolf RPG)
5. **File Processing**: Engine extracts text units with manifest creation
6. **Database Persistence**: Text units stored in SQLite with project identification
7. **State Update**: Frontend stores update reactive state
8. **UI Synchronization**: Components automatically re-render with project data

### Advanced Translation Flow

1. **Batch Translation**: User initiates translation in `TranslationProcess` component
2. **Multi-Store Coordination**: `translateStore` coordinates with `providerStore` and `glossaryStore`
3. **Provider Selection**: `providerStore` provides current LLM configuration (Ollama/OpenAI/Groq)
4. **Glossary Integration**: `glossaryStore` supplies relevant terms for context
5. **Prompt Building**: Backend shared prompt builder creates context-rich prompts
6. **Token Estimation**: `token_estimation.rs` provides cost/budgeting information
7. **Translation Execution**: LLM service processes text with provider-specific optimizations
8. **Progress Tracking**: Real-time progress updates with cancellation support
9. **Database Updates**: Translation results persisted with status tracking
10. **UI Feedback**: Toast notifications and progress indicators update user

### Export Workflow

1. **Export Trigger**: User clicks "Export" in translation interface
2. **Database Query**: Backend queries translated units by manifest hash
3. **Engine Dispatch**: Factory instantiates appropriate engine implementation
4. **ID Reconstruction**: Engine's `reconstruct_text_unit_id()` recreates TextUnit objects
5. **File Operations**: Engine copies project files and injects translations
6. **User Feedback**: Toast notifications confirm successful export to `ludolingua/` folder

## Directory Structure

For detailed directory structures and module explanations, please refer to:

- [Backend Structure](BACKEND_STRUCTURE.md)
- [Frontend Structure](FRONTEND_STRUCTURE.md)

## Technology Stack

### Frontend Technologies
- **Framework**: Nuxt 4 with Vue 3 Composition API
- **UI Library**: Nuxt UI with Tailwind CSS
- **State Management**: Pinia stores with TypeScript
- **Package Manager**: pnpm
- **Build Tool**: Vite
- **Type Safety**: TypeScript throughout

### Backend Technologies
- **Core**: Rust with Tokio async runtime
- **Desktop Framework**: Tauri for cross-platform desktop apps
- **Database**: SQLite with SQLx for persistence
- **AI Integration**: Multi-provider LLM support (Ollama, OpenAI, Groq, OpenRouter)
- **Serialization**: Serde for JSON handling
- **Error Handling**: Custom error types with comprehensive error recovery

### Supported Game Engines
- **RPG Maker MV**: Full support with 12+ file types
- **RPG Maker MZ**: Full support with core files
- **Wolf RPG**: Experimental support with MPS/DB files

### Key Features
- **Database Persistence**: SQLite storage with ACID transactions
- **Multi-Provider LLM**: Unified interface for different AI providers
- **Engine-Agnostic Export**: Factory pattern for extensible game engine support
- **Glossary System**: Database-backed term management with prompt integration
- **Token Estimation**: Pre-translation cost and budget planning
- **Progress Preservation**: Resume functionality after interruptions 