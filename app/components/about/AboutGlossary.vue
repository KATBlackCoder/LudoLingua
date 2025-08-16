<template>
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

    <!-- Placeholder guide moved to AboutPlaceholders.vue -->

    <UAlert color="warning" variant="soft" icon="i-heroicons-exclamation-triangle" class="mt-3">
      <template #title>Keep glossary entries short</template>
      <template #description>
        <div class="text-sm">
          Add only short terms/phrases (names, UI words, concise rules). Do not paste long paragraphs; those belong in translations, not the glossary. Long entries bloat prompts and hurt quality.
        </div>
      </template>
    </UAlert>

    <div class="mt-3 text-sm space-y-2">
      <p>
        Add terms under the right <em>Category</em> in the Glossary. Each item renders into the prompt as:
      </p>
      <pre class="bg-gray-100 dark:bg-gray-800 p-3 rounded text-xs overflow-auto"><code>### Category
Input: 源語 / Source term
Output: 翻訳 / Target term
</code></pre>
    </div>

    <!-- Category explanations -->
    <UCard class="mt-4">
      <template #header>
        <span class="font-medium">Category Guide</span>
      </template>
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 text-sm">
        <div class="flex items-start gap-2">
          <UBadge variant="soft">Characters</UBadge>
          <p class="leading-relaxed">
            Proper names (people, monsters, places as names). Prefer consistent transliteration; avoid translating names unless the game localizes them officially. Exact matches may bypass the LLM.
          </p>
        </div>
        <div class="flex items-start gap-2">
          <UBadge variant="soft">Essential Terms</UBadge>
          <p class="leading-relaxed">
            Game-specific vocabulary (currencies, factions, items). Single source of truth to keep wording uniform across files.
          </p>
        </div>
        <div class="flex items-start gap-2">
          <UBadge variant="soft">Translation Rules</UBadge>
          <p class="leading-relaxed">
            Style and constraints (tone, honorifics, punctuation, brackets). These guide the model on how to write.
          </p>
        </div>
        <div class="flex items-start gap-2">
          <UBadge variant="soft">Locations</UBadge>
          <p class="leading-relaxed">
            Place names used as common nouns. Keep canonical spelling; avoid inconsistent suffixes (e.g., “Forest of X”).
          </p>
        </div>
        <div class="flex items-start gap-2">
          <UBadge variant="soft">Time & Weather</UBadge>
          <p class="leading-relaxed">
            Temporal and weather terms (dawn, blizzard). Ensures consistent phrasing in UI and descriptions.
          </p>
        </div>
        <div class="flex items-start gap-2">
          <UBadge variant="soft">Mechanics</UBadge>
          <p class="leading-relaxed">
            Mechanics and system terms (stats, damage types, item rarities). Avoid inventing new terms mid-project.
          </p>
        </div>
        <div class="flex items-start gap-2">
          <UBadge variant="soft">Status Effects</UBadge>
          <p class="leading-relaxed">
            Buffs/debuffs and ailment names (Poison, Stun). Prefer standard localized names used by the genre.
          </p>
        </div>
      </div>
      <template #footer>
        <div class="text-xs text-muted">
          Tip: Terms are applied only when source/target languages match the current project and the term is enabled.
          You can add terms directly from the Translation results using the “Add to glossary” button; it auto-picks a Category based on the text unit’s PromptType.
        </div>
      </template>
    </UCard>
  </section>
</template>


