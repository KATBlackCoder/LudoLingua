<template>
  <div>
    <UContainer>
      <UCard class="max-w-4xl mx-auto">
        <template #header>
          <div class="flex items-center gap-3">
            <UIcon name="i-heroicons-information-circle" class="w-6 h-6 text-primary-500" />
            <div>
              <h2 class="text-xl font-semibold">About LudoLingua</h2>
              <p class="text-sm text-gray-500 dark:text-gray-400">Offline RPG Maker translation with a modern Nuxt + Tauri UI</p>
            </div>
          </div>
        </template>

        <div class="space-y-6">
          <section>
            <h3 class="text-lg font-medium mb-2">What is LudoLingua?</h3>
            <p class="text-muted">LudoLingua is a desktop application designed for translating RPG Maker game files. It helps game developers and translators manage and translate game text using AI assistance.</p>

            <div class="mt-4 flex flex-wrap gap-2 text-xs">
              <UBadge color="primary" variant="soft" icon="i-heroicons-cpu-chip">Offline-first</UBadge>
              <UBadge color="primary" variant="soft" icon="i-heroicons-shield-check">Local files</UBadge>
              <UBadge color="primary" variant="soft" icon="i-heroicons-language">RPG Maker MV</UBadge>
              <UBadge color="primary" variant="soft" icon="i-heroicons-swatch">Nuxt UI</UBadge>
            </div>
          </section>

          <USeparator />

          <section>
            <h3 class="text-lg font-medium mb-2">Features</h3>
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 text-sm">
              <div class="flex items-center gap-2">
                <UIcon name="i-heroicons-folder-open" class="w-4 h-4" />
                <span>Load and parse RPG Maker MV projects</span>
              </div>
              <div class="flex items-center gap-2">
                <UIcon name="i-heroicons-document-text" class="w-4 h-4" />
                <span>Extract translatable text</span>
              </div>
              <div class="flex items-center gap-2">
                <UIcon name="i-heroicons-rectangle-stack" class="w-4 h-4" />
                <span>Organize by file and type</span>
              </div>
              <div class="flex items-center gap-2">
                <UIcon name="i-heroicons-sparkles" class="w-4 h-4" />
                <span>AI-assisted translation</span>
              </div>
              <div class="flex items-center gap-2">
                <UIcon name="i-heroicons-book-open" class="w-4 h-4" />
                <span>Glossary / termbase with prompts</span>
              </div>
              <div class="flex items-center gap-2">
                <UIcon name="i-heroicons-arrow-down-tray" class="w-4 h-4" />
                <span>Inject translations back to files</span>
              </div>
            </div>
          </section>

          <USeparator />

          <section>
            <h3 class="text-lg font-medium mb-2">Technology</h3>
            <UGrid :cols="{ default: 2, sm: 3, md: 6 }" class="gap-2 mt-2">
              <UBadge color="primary" variant="soft" class="justify-center">Tauri</UBadge>
              <UBadge color="primary" variant="soft" class="justify-center">Rust</UBadge>
              <UBadge color="primary" variant="soft" class="justify-center">Nuxt 4</UBadge>
              <UBadge color="primary" variant="soft" class="justify-center">TypeScript</UBadge>
              <UBadge color="primary" variant="soft" class="justify-center">Nuxt UI</UBadge>
              <UBadge color="primary" variant="soft" class="justify-center">Tailwind CSS</UBadge>
            </UGrid>
          </section>

          <USeparator />

          <section>
            <div class="flex items-center gap-2 mb-2">
              <h3 class="text-lg font-medium">Glossary and Prompt Types</h3>
              <UBadge color="primary" variant="soft" size="xs">Consistency</UBadge>
            </div>
            <p class="text-sm text-muted mb-3">
              Glossary terms are stored in the database and injected into the LLM prompt based on the text unit's PromptType. If DB terms exist, they are used (DB-only) for the vocabulary block; otherwise we fall back to the file <code>prompts/vocabularies.txt</code>. For <strong>Character</strong> names, an exact source match in the glossary bypasses the LLM.
            </p>

            <UAlert
              color="primary"
              variant="soft"
              icon="i-heroicons-circle-stack"
              title="Database location"
            >
              <template #description>
                <div class="text-sm space-y-1">
                  <p>The glossary database is stored per user in the OS app data directory as <code>ludolingua.db</code>.</p>
                  <ul class="list-disc pl-5">
                    <li>Windows: <code>%AppData%/LudoLingua/ludolingua.db</code></li>
                    <li>macOS: <code>~/Library/Application Support/LudoLingua/ludolingua.db</code></li>
                    <li>Linux: <code>~/.local/share/LudoLingua/ludolingua.db</code></li>
                  </ul>
                  <p class="text-xs text-gray-500 dark:text-gray-400">Note: the app folder name may appear as <code>LudoLingua</code> or as the identifier (e.g., <code>ml.ludolingua.blackat</code>) depending on the platform.</p>
                </div>
              </template>
            </UAlert>

            <div class="space-y-2 text-sm">
              <div>
                <span class="font-medium">Dialogue</span>
                <span class="mx-2">→</span>
                <UBadge variant="soft" class="mr-1">Characters</UBadge>
                <UBadge variant="soft" class="mr-1">Essential Terms</UBadge>
                <UBadge variant="soft" class="mr-1">Translation Rules</UBadge>
                <UBadge variant="soft" class="mr-1">Locations</UBadge>
                <UBadge variant="soft" class="mr-1">Time & Weather</UBadge>
                <UBadge variant="soft" class="mr-1">Mechanics</UBadge>
              </div>
              <div>
                <span class="font-medium">Character</span>
                <span class="mx-2">→</span>
                <UBadge variant="soft" class="mr-1">Characters</UBadge>
                <UBadge variant="soft" class="mr-1">Essential Terms</UBadge>
              </div>
              <div>
                <span class="font-medium">State / Skill</span>
                <span class="mx-2">→</span>
                <UBadge variant="soft" class="mr-1">Status Effects</UBadge>
                <UBadge variant="soft" class="mr-1">Mechanics</UBadge>
                <UBadge variant="soft" class="mr-1">Essential Terms</UBadge>
              </div>
              <div>
                <span class="font-medium">Equipment</span>
                <span class="mx-2">→</span>
                <UBadge variant="soft" class="mr-1">Mechanics</UBadge>
                <UBadge variant="soft" class="mr-1">Essential Terms</UBadge>
              </div>
              <div>
                <span class="font-medium">System / Class / Other</span>
                <span class="mx-2">→</span>
                <UBadge variant="soft" class="mr-1">Mechanics</UBadge>
                <UBadge variant="soft" class="mr-1">Essential Terms</UBadge>
              </div>
            </div>

            <div class="mt-3 text-sm space-y-2">
              <p>
                Add terms under the right <em>Category</em> in the Glossary. Each item renders into the prompt as:
              </p>
              <pre class="bg-gray-100 dark:bg-gray-800 p-3 rounded text-xs overflow-auto"><code>### Category
Input: 源語 / Source term
Output: 翻訳 / Target term
</code></pre>
              <ul class="list-disc pl-5 space-y-1">
                <li><strong>Languages</strong>: Terms apply only when <code>source_lang</code> and <code>target_lang</code> match the current project.</li>
                <li><strong>Enabled</strong>: Toggle to include/exclude individual terms quickly.</li>
                <li><strong>Exact match (Character)</strong>: Exact source equals glossary input → we use the glossary output directly.</li>
              </ul>
            </div>
          </section>
        </div>

        <template #footer>
          <p class="text-sm text-gray-500">LudoLingua is an open-source project. Contributions are welcome!</p>
        </template>
      </UCard>
    </UContainer>
  </div>
</template>

<script setup lang="ts">
// About page setup code can go here
</script> 