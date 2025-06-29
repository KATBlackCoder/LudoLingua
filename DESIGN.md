# LudoLingua Design Document

## 1. Introduction

LudoLingua is a desktop application designed to streamline the translation of video games created with RPG Maker engines (MV, MZ, etc.). It functions as an offline translation editor, allowing users to load game project files, manage text, and leverage AI for quick and accurate translations. The application modifies the game's data files directly and is not intended for real-time translation while the game is running.

## 2. Key Features

*   **Project Loading:** Directly open and parse RPG Maker MV/MZ project files.
*   **Text Extraction:** Automatically extract all user-facing text from game files (maps, common events, items, etc.).
*   **AI-Powered Translation:** Integrate with AI models to provide instant translation suggestions. 
*   **Side-by-Side Editing:** A dual-pane view showing original text and the translation for easy comparison and editing.
*   **Project Saving:** Save translated text back into the original file format, ready to be used in the game engine.
*   **Format Support:** Initial support for RPG Maker MV and MZ, with the possibility of extending to other engines.
*   **Glossary / Termbase:** Manage a local database of key terms (names, items, locations) to ensure consistent translations throughout the project.

## 3. Technology Stack

### Backend & Core

| Category               | Crate / Plugin              | Justification                                                                                                                                                                                 | Documentation                                                 |
| :--------------------- | :-------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------ |
| **Async Runtime**      | `tokio`                     | The foundation for Tauri. It's an asynchronous runtime for Rust, essential for handling non-blocking tasks like file I/O and network requests efficiently without freezing the UI.                 | [tokio.rs](https://tokio.rs/)                                 |
| **AI Translation**     | `llm`                       | As we decided, this crate will provide the core AI functionality, allowing the app to interface with various large language models for translation.                                             | [crates.io](https://crates.io/crates/llm)                     |
| **Data Serialization** | `serde`                     | The standard for high-performance JSON (and other formats) serialization and deserialization in Rust. Crucial for parsing RPG Maker's `.json` project files into strongly-typed Rust structs.    | [serde.rs](https://serde.rs/)                                 |
| **Error Handling**     | `anyhow`                    | Provides a flexible and ergonomic error handling system. It simplifies bubbling up errors from different libraries into a single, consistent error type for our Tauri commands.                  | [crates.io](https://crates.io/crates/anyhow)                  |
| **Termbase**           | `sqlx` with `sqlx-sqlite`   | Provides an embedded SQLite database for storing and managing a glossary of key terms, ensuring translation consistency.                                                                      | [crates.io](https://crates.io/crates/sqlx)                                    |
| **Database Tooling**   | `sqlx-cli`                  | A command-line utility for `sqlx` to manage database migrations (creating, applying, reverting) and to verify queries at compile time.                                                         | [crates.io](https://crates.io/crates/sqlx-cli)                              |
| **File System**        | `tauri-plugin-dialog`       | Provides the native OS "Open File/Folder" dialogs. We'll use this to let the user select the root folder of their RPG Maker project.                                                            | [tauri.app](https://tauri.app/plugin/dialog/)                 |
|                        | `tauri-plugin-fs`           | Offers a secure and sandboxed way to read and write files. This is essential for both reading the game data and saving the translated files back to the disk.                                     | [tauri.app](https://tauri.app/plugin/file-system/)            |
| **Settings Storage**   | `tauri-plugin-store`        | A persistent key-value store. Perfect for securely saving user settings like AI provider API keys, theme preferences, and default language choices directly on the user's machine.                | [tauri.app](https://tauri.app/plugin/store/)                  |
| **App Management**     | `tauri-plugin-updater`      | Enables automatic application updates, which is a critical feature for any production desktop application to deliver bug fixes and new features seamlessly.                                     | [tauri.app](https://tauri.app/plugin/updater/)                |
|                        | `tauri-plugin-log`          | A robust logging utility that can write to stdout, the web console, and log files. Invaluable for debugging issues, especially those related to file I/O or the AI API.                            | [tauri.app](https://tauri.app/plugin/logging/)                |
|                        | `tauri-plugin-opener`       | Allows opening paths and URLs in the default application. We'll use this to open the project folder in the user's file explorer after translation.                                              | [tauri.app](https://tauri.app/plugin/opener/)                 |
|                        | `tauri-plugin-window-state` | Automatically saves and restores the window's size and position between sessions, providing a better user experience.                                                                           | [tauri.app](https://tauri.app/plugin/window-state/)           |

### Frontend

| Category                   | Library/Tool                                     | Justification                                                                                                                                                                     | Documentation                                                    |
| :------------------------- | :----------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------- |
| **UI Framework**           | `Nuxt`                                           | A Vue.js framework with TypeScript that provides a great developer experience with features like file-based routing and auto-imports. We will disable server-side rendering for Tauri compatibility. | [nuxt.com](https://nuxt.com/)                                    |
| **Component Library**      | `Nuxt UI`                                        | A beautiful and customizable component library built for Nuxt. It provides the building blocks for our UI (forms, buttons, modals, tables) and has built-in i18n support. **Only the free, open-source version will be used.** | [ui.nuxt.com](https://ui.nuxt.com/)                              |
| **Styling**                | `Tailwind CSS`                                   | A utility-first CSS framework integrated with Nuxt UI, allowing for rapid and consistent styling without writing custom CSS.                                                    | [tailwindcss.com](https://tailwindcss.com/)                    |
| **State Management**       | `Pinia`                                          | The official state management library for Vue. It provides a simple and type-safe way to manage global application state, such as loaded project files and translation progress.   | [pinia.vuejs.org](https://pinia.vuejs.org/)                      |
| **Form Handling/Validation** | `Zod`                                            | A TypeScript-first schema library used with Nuxt UI's `<UForm>` to define the structure and validation rules for our translation data and settings, ensuring data integrity.        | [zod.dev](https://zod.dev/)                                      |
| **Localization**           | `@nuxtjs/i18n`                                   | Manages the internationalization of the application's own user interface. This allows us to translate UI elements like buttons, labels, and menus.                                | [i18n.nuxtjs.org](https://i18n.nuxtjs.org/)                    |
| **Icons**                  | `@iconify/vue`                                   | Bundled with Nuxt UI, providing access to over 100,000 icons from various sets, essential for a polished and intuitive user interface.                                            | [icon-sets.iconify.design](https://icon-sets.iconify.design/)    |
| **Testing**                | `Vitest`                                         | A blazing-fast unit test framework. It's essential for writing tests for our data transformation logic (e.g., parsing game files, structuring text data) to prevent regressions.   | [vitest.dev](https://vitest.dev/)                                |
| **Linting**                | `ESLint`                                         | Statically analyzes code to quickly find problems. It helps enforce code style and catch common errors before they become bugs.                                                    | [eslint.org](httpss://eslint.org/)                                |
| **Formatting**             | `Prettier`                                       | An opinionated code formatter that keeps the codebase consistent and readable, saving time and preventing debates over style.                                                      | [prettier.io](https://prettier.io/)                              |

## 4. Architecture

The application will use Tauri's hybrid architecture:

*   The **Frontend** will be a Nuxt.js single-page application (SPA). It will be responsible for rendering the entire user interface, from file selection to the translation editing view. It will communicate with the backend to request data and trigger actions. 
*   The **Backend** will be a Rust binary. It will expose a set of `commands` that the frontend can invoke. These commands will handle all the heavy lifting:
    *   Reading and writing to the file system.
    *   Parsing the specific structure of RPG Maker project files.
    *   Making calls to the AI translation models via the `llm` crate.
    *   Managing application state and data.

This separation ensures that the core logic is efficient and secure, while the UI remains modern, flexible, and easy to develop. 