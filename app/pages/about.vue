<template>
  <div>
    <UContainer>
      <UCard class="max-w-4xl mx-auto">
        <template #header>
          <div class="flex items-center justify-between">
            <h2 class="text-xl font-semibold">About LudoLingua</h2>
          </div>
        </template>

        <div class="space-y-6">
          <section>
            <h3 class="text-lg font-medium mb-2">What is LudoLingua?</h3>
            <p class="text-muted">LudoLingua is a desktop application designed for translating RPG Maker game files. It helps game developers and translators manage and translate game text using AI assistance.</p>
          </section>

          <USeparator />

          <section>
            <h3 class="text-lg font-medium mb-2">Features</h3>
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-2 text-sm">
              <UBadge color="primary" variant="soft">Load/parse RPG Maker MV/MZ</UBadge>
              <UBadge color="primary" variant="soft">Extract translatable text</UBadge>
              <UBadge color="primary" variant="soft">Organize by file/type</UBadge>
              <UBadge color="primary" variant="soft">AI-assisted translation</UBadge>
              <UBadge color="primary" variant="soft">Glossary/termbase</UBadge>
              <UBadge color="primary" variant="soft">Save back to files</UBadge>
            </div>
          </section>

          <USeparator />

          <section>
            <h3 class="text-lg font-medium mb-2">Technology</h3>
            <div class="flex flex-wrap gap-2 mt-2">
              <UBadge color="primary">Tauri</UBadge>
              <UBadge color="primary">Rust</UBadge>
              <UBadge color="primary">Nuxt.js</UBadge>
              <UBadge color="primary">TypeScript</UBadge>
              <UBadge color="primary">Nuxt UI</UBadge>
              <UBadge color="primary">Tailwind CSS</UBadge>
            </div>
          </section>

          <USeparator />

          <section>
            <h3 class="text-lg font-medium mb-2">Glossary and Prompt Types</h3>
            <p class="text-sm text-muted mb-3">
              Glossary terms are stored in the database and injected into the LLM prompt based on the text unit's PromptType. If DB terms exist, they are used (DB-only) for the vocabulary block; otherwise we fall back to the file <code>prompts/vocabularies.txt</code>. For <strong>Character</strong> names, an exact source match in the glossary bypasses the LLM.
            </p>

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