# LudoLingua Frontend Architecture

This document provides a detailed walkthrough of the Nuxt.js frontend's structure for the LudoLingua application. The architecture is designed to be modular, maintainable, and to fully leverage the features of Nuxt 3 and Pinia for a reactive and efficient user experience.

---

## 1. Directory Structure Overview

The following diagram illustrates the file and directory layout for the frontend source code, located in the project's root directory.

```
/
├── .vscode/          # VS Code settings (e.g., for recommended extensions)
├── assets/           # For static assets like fonts and global CSS
│   └── css/
│       └── main.css
├── components/       # Reusable Vue components, organized by feature
│   ├── layout/       # Components for the main layout (e.g., AppHeader, AppSidebar)
│   ├── settings/     # Components specific to the settings page
│   ├── glossary/     # Components specific to the glossary page
│   └── editor/       # Components for the main translation editor view
├── layouts/          # For the main application layouts
│   └── default.vue
├── pages/            # Application pages, which map to routes
│   ├── index.vue     # Main editor page
│   ├── glossary.vue  # Glossary management page
│   └── settings.vue  # Application settings page
├── stores/           # Pinia stores for state management
│   ├── project.ts    # State for the currently loaded project and its text
│   ├── settings.ts   # State for user settings (e.g., API keys, theme)
│   └── ui.ts         # State for UI elements (e.g., loading states, modal visibility)
├── types/            # TypeScript type definitions, organized by domain
│   ├── engine.ts      # Types related to the game project data
│   ├── glossary.ts    # Types for glossary terms
│   ├── settings.ts    # Types for application settings
│   └── translation.ts # Types for translation units and status
└── nuxt.config.ts    # Nuxt configuration file
```

---

## 2. Detailed Module Explanations

This section explains the purpose of each directory and how they work together to form the application.

### The Foundation: Configuration and Global Assets

*   **`nuxt.config.ts`**: The main configuration file for the entire Nuxt application. This is where we will disable Server-Side Rendering (SSR) for Tauri compatibility, register Nuxt modules (like Nuxt UI and Pinia), and define global application settings.
*   **`assets/`**: This directory holds globally used static files. The primary use case for us will be `assets/css/main.css`, where we can define any global CSS styles or overrides that apply across the entire application.

### The UI Layer: Components, Layouts, and Pages

This layer is responsible for everything the user sees and interacts with. It's built on a hierarchy of components.

*   **`layouts/`**: Defines the main "shell" of the application. For example, `layouts/default.vue` will contain the persistent UI elements like the main header or sidebar, and a `<NuxtPage />` component. The content of the current page is rendered inside `<NuxtPage />`.
*   **`pages/`**: Contains the top-level Vue components for each "page" or "route" of our application. Nuxt's file-based routing automatically creates routes based on the filenames here (e.g., `glossary.vue` becomes the `/glossary` route). Pages are responsible for fetching data from Pinia stores and passing it down to the display components.
*   **`components/`**: This is the heart of the UI. It contains all our reusable Vue components, which are the building blocks of the application. They are organized into subdirectories by feature or page (e.g., `editor/`, `glossary/`) to keep them organized. These components receive data via props and emit events to notify their parent pages of user actions.

### The Logic & State Layer: Pinia Stores

This is the "brain" of the frontend. It manages the application's state and is the sole bridge to the Rust backend. This separation keeps our UI components clean and focused on display logic.

*   **`stores/`**: This directory houses all our Pinia state management stores. Each store is a self-contained module responsible for a specific slice of the application's state.
    *   `project.ts`: Will hold all data related to the currently loaded RPG Maker project, such as the list of game files and the extracted text units for translation.
    *   `settings.ts`: Manages the user's saved settings. It will be responsible for calling the Tauri commands to load settings from the `tauri-plugin-store` on startup and save them when they change.
    *   `ui.ts`: Manages the state of the UI itself, such as loading spinners, error message visibility, or which modal is currently open.

### Type Safety

*   **`types/`**: A centralized location for all our custom TypeScript interfaces and type definitions. To keep them organized, types are broken into files based on their application domain (e.g., `engine.ts`, `glossary.ts`), mirroring the backend's data models. This helps ensure type safety across the entire frontend.

---

## 3. Frontend Data Flow: A Complete Example

Understanding the flow of data is key to understanding the architecture. Here's a typical user interaction:

1.  **User Action:** A user clicks the "Translate" button inside a component in `editor/`.
2.  **Event Emission:** The component doesn't perform the translation itself. Instead, it emits an event (e.g., `@translate-clicked`).
3.  **Page Handler:** The page (`pages/index.vue`) listens for this event and calls an **action** in the appropriate Pinia store, for example, `projectStore.translateSelectedText()`.
4.  **Store Action & Backend Communication:** The `projectStore` action contains the core logic. It gets the current state, fetches the user's settings from the `settingsStore`, and then calls `invoke()` from the Tauri API to execute a Rust command (e.g., `invoke('translate_text', { ... })`).
5.  **State Mutation:** When the Rust backend returns the translated text, the store action updates its own state by modifying its reactive refs (e.g., `translationUnits.value[i].translated_text = result`).
6.  **Automatic UI Update:** Because our Vue components get their data from the Pinia store, they are inherently reactive. As soon as the state in the store changes, Nuxt and Vue automatically re-render the affected components to display the new translated text. The component doesn't need to be explicitly told to update.

This clean, one-way data flow makes the application easy to reason about, debug, and scale.

