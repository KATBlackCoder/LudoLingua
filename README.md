# Nuxt Minimal Starter

Look at the [Nuxt documentation](https://nuxt.com/docs/getting-started/introduction) to learn more.

## Setup

Install dependencies (pnpm is preferred):

```bash
# pnpm (recommended)
pnpm install

# npm
npm install

# yarn
yarn install

# bun
bun install
```

## Development Server

Start the development server on `http://localhost:3000`:

```bash
# pnpm (recommended)
pnpm dev

# npm
npm run dev

# yarn
yarn dev

# bun
bun run dev
```

## Production

Build the application for production:

```bash
# pnpm (recommended)
pnpm build

# npm
npm run build

# yarn
yarn build

# bun
bun run build
```

Locally preview production build:

```bash
# pnpm (recommended)
pnpm preview

# npm
npm run preview

# yarn
yarn preview

# bun
bun run preview
```

Check out the [deployment documentation](https://nuxt.com/docs/getting-started/deployment) for more information.


### Prompt loading and vocabulary filtering

- Dev vs Prod loading:
  - Dev builds read prompt templates from `src-tauri/prompts/` via filesystem for instant editing.
  - Prod builds embed the same files at compile time (`include_str!`) to avoid path issues in the bundle.

- Vocabulary filtering:
  - `src-tauri/src/utils/prompts/builder.rs` filters `prompts/vocabularies.txt` to include only sections relevant to each `PromptType`.
  - Sections are detected by headers that start with `### ` (e.g., `### Mechanics`). The header text must match exactly.
  - Current whitelist per type:

```rust
// In PromptBuilder::filter_vocabulary_sections
let wanted_sections: &[&str] = match prompt_type {
    PromptType::Dialogue | PromptType::Character => &[
        "### Characters",
        "### Essential Terms",
    ],
    PromptType::State | PromptType::Skill => &[
        "### Status Effects",
        "### Mechanics",
        "### Essential Terms",
    ],
    PromptType::Equipment => &[
        "### Mechanics",
        "### Essential Terms",
    ],
    PromptType::System | PromptType::Class | PromptType::Other => &[
        "### Mechanics",
        "### Essential Terms",
    ],
};
```

- Customize filtering:
  - To include another section (e.g., `### Locations`) for a type, add it to the corresponding array above.
  - Or, change headings/text inside `prompts/vocabularies.txt` (ensure exact header match).

- Add a new prompt type:
  - Add a variant to `PromptType` and its template file under `src-tauri/prompts/`.
  - Map it by implementing/using `PromptType::template_path()` (already provided in `src-tauri/src/models/translation.rs`).