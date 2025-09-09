# LudoLingua Frontend Architecture

This document provides a detailed walkthrough of the Nuxt.js frontend's structure for the LudoLingua application. The architecture is designed to be modular, maintainable, and to fully leverage the features of Nuxt 4 and Pinia for a reactive and efficient user experience.

---

## 1. Directory Structure Overview

The following diagram illustrates the file and directory layout for the frontend source code, located in the project's root directory.

```
/
├── app/              # Nuxt 4 app directory structure
│   ├── app.vue       # Root application component
│   ├── assets/       # Static assets and global CSS
│   │   └── css/
│   │       └── main.css
│   ├── components/   # Reusable Vue components, organized by feature
│   │   ├── layout/   # Main layout components (AppHeader, AppFooter, etc.)
│   │   ├── settings/ # Settings page components (ProviderSelector, etc.)
│   │   ├── glossary/ # Glossary management components
│   │   ├── translation/ # Translation workflow components
│   │   ├── editor/   # Legacy editor components
│   │   └── about/    # About page components
│   ├── layouts/      # Application layouts
│   │   └── default.vue
│   ├── pages/        # Application pages (auto-routed)
│   │   ├── index.vue     # Main translation page
│   │   ├── glossary.vue  # Glossary management
│   │   ├── settings.vue  # Application settings
│   │   ├── translation.vue # Translation workflow page
│   │   └── about.vue     # About/application info
│   ├── stores/       # Pinia stores for state management
│   │   ├── engine.ts     # Game engine and project management
│   │   ├── settings.ts   # User settings and configuration
│   │   ├── provider.ts   # LLM provider management
│   │   ├── language.ts   # Language configuration
│   │   ├── translate.ts  # Translation workflow state
│   │   └── glossary.ts   # Glossary term management
│   ├── types/        # TypeScript type definitions
│   │   ├── engine.ts     # Game project data types
│   │   ├── glossary.ts   # Glossary term types
│   │   ├── provider.ts   # LLM provider types
│   │   ├── language.ts   # Language types
│   │   ├── settings.ts   # Settings configuration types
│   │   ├── tokens.ts     # Token estimation types
│   │   └── translation.ts # Translation workflow types
│   └── composables/  # Vue composables for shared logic
│       ├── useAppToast.ts   # Toast notification system
│       ├── useGlossary.ts   # Glossary operations
│       └── useTranslation.ts # Translation utilities
├── plugins/          # Nuxt plugins
│   └── pinia.client.ts # Pinia client-side initialization
├── nuxt.config.ts    # Nuxt configuration file
└── package.json      # Dependencies and scripts
```

---

## 2. Detailed Module Explanations

This section explains the purpose of each directory and how they work together to form the application.

### The Foundation: Configuration and Global Assets

*   **`nuxt.config.ts`**: The main configuration file for the Nuxt 4 application. Configures Tauri compatibility (SSR disabled), registers Nuxt UI and Pinia modules, and defines global application settings for the desktop app environment.
*   **`app/assets/`**: Contains globally used static files including the main CSS file for global styles and overrides that apply across the entire application.

### The UI Layer: Components, Layouts, and Pages

This layer is responsible for everything the user sees and interacts with. It's built on a modern Nuxt 4 component hierarchy.

*   **`app/layouts/`**: Defines the main application shell with persistent UI elements like headers, footers, and navigation. Uses `<NuxtPage />` to render the current page content.
*   **`app/pages/`**: Contains top-level Vue components for each application route. Nuxt's file-based routing automatically creates routes from filenames. Pages orchestrate data flow between stores and components.
*   **`app/components/`**: Contains all reusable Vue components organized by feature:
    *   `layout/`: Main layout components (AppHeader, AppFooter, ColorModeButton)
    *   `settings/`: Settings page components (ProviderSelector, ConnectionTester)
    *   `glossary/`: Glossary management components (GlossaryTable, GlossaryForm)
    *   `translation/`: Translation workflow components (TranslationEditor, TranslationResult)
    *   `editor/`: Legacy editor components (ProjectLoader, ProjectStats)
    *   `about/`: About page components with comprehensive documentation

### The Logic & State Layer: Pinia Stores

This is the "brain" of the frontend, managing application state and serving as the bridge to the Rust backend.

*   **`app/stores/`**: Contains all Pinia stores using the Setup Store pattern for optimal Vue 3 Composition API integration:
    *   `engine.ts`: Manages game engine detection, project loading, and text extraction operations
    *   `settings.ts`: Handles user settings persistence and configuration management
    *   `provider.ts`: Manages LLM provider selection and model configurations
    *   `language.ts`: Handles language selection and RTL support
    *   `translate.ts`: Manages translation workflow state, progress tracking, and batch operations
    *   `glossary.ts`: Handles glossary term CRUD operations and database synchronization

### Composables and Shared Logic

*   **`app/composables/`**: Contains Vue composables for reusable logic:
    *   `useAppToast.ts`: Centralized toast notification system
    *   `useGlossary.ts`: Glossary operations and state management
    *   `useTranslation.ts`: Translation utilities and workflow helpers

### Type Safety and Type Definitions

*   **`app/types/`**: Comprehensive TypeScript type definitions organized by domain:
    *   `engine.ts`: Game project data and engine-specific types
    *   `glossary.ts`: Glossary term and category types
    *   `provider.ts`: LLM provider and model configuration types
    *   `language.ts`: Language and locale types
    *   `settings.ts`: Application settings and configuration types
    *   `tokens.ts`: Token estimation and pricing types
    *   `translation.ts`: Translation workflow and status types

### Plugins and Initialization

*   **`plugins/`**: Nuxt plugins for client-side initialization:
    *   `pinia.client.ts`: Ensures Pinia store availability on the client side

---

## 3. Frontend Data Flow: Modern Translation Workflow

Understanding the flow of data is key to understanding the architecture. Here's how the modern translation workflow operates:

### Translation Workflow Example:

1.  **User Action:** User clicks "Load Project" on the main page
2.  **Store Dispatch:** Page calls `engineStore.loadProject()` action
3.  **Multi-Store Coordination:** Engine store coordinates with settings store for configuration
4.  **Backend Communication:** Store invokes Tauri command `invoke('load_project')`
5.  **State Updates:** Engine store updates reactive state with project data
6.  **UI Synchronization:** Components automatically re-render with new project information

### Advanced Translation Flow:

1.  **Batch Translation:** User initiates batch translation in `TranslationProcess` component
2.  **Progress Tracking:** `translateStore` manages progress state and cancellation
3.  **Provider Integration:** `providerStore` provides current LLM configuration
4.  **Glossary Enhancement:** `glossaryStore` supplies relevant terms for context
5.  **Toast Notifications:** `useAppToast` composable provides user feedback
6.  **Export Workflow:** User triggers export via `engineStore.exportTranslatedSubset()`
7.  **File Operations:** Backend handles file copying and translation injection
8.  **Success Feedback:** Toast notifications confirm successful export

### Key Architectural Patterns:

- **Store Composition:** Multiple stores work together (engine + provider + glossary)
- **Composable Integration:** Shared logic via composables (toast, translation utils)
- **Reactive Updates:** Vue 3 reactivity automatically syncs UI with store state
- **Type Safety:** TypeScript interfaces ensure data consistency across layers
- **Clean Separation:** UI components focus on display, stores handle business logic

This modern architecture supports complex workflows while maintaining clean separation of concerns and excellent developer experience.
