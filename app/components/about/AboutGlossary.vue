<template>
  <UCard>
    <template #header>
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div class="p-2 bg-primary-50 dark:bg-primary-900/20 rounded-lg">
            <UIcon name="i-lucide-book-open" class="text-primary w-5 h-5" />
          </div>
          <div>
            <h3 class="text-lg font-semibold text-gray-900 dark:text-white">Glossary & Prompt Types</h3>
            <p class="text-sm text-gray-500 dark:text-gray-400">Consistent terminology management</p>
          </div>
        </div>
        <UBadge color="primary" variant="soft" size="sm">
          <UIcon name="i-lucide-shield-check" class="mr-1" />
          Consistency
        </UBadge>
      </div>
    </template>

    <div class="space-y-6">
      <!-- How It Works -->
      <UCard>
        <template #header>
          <div class="flex items-center gap-2">
            <UIcon name="i-lucide-info" class="text-gray-500 w-4 h-4" />
            <span class="text-sm font-medium text-gray-700 dark:text-gray-300">How It Works</span>
          </div>
        </template>
        <div class="space-y-3">
          <p class="text-sm leading-relaxed">
            Glossary terms are stored in the database and injected into the LLM prompt based on the text unit's PromptType. 
            If DB terms exist, they are used (DB-only) for the vocabulary block; otherwise we fall back to the file 
            <code class="px-2 py-1 bg-gray-100 dark:bg-gray-800 rounded text-xs">prompts/vocabularies.txt</code>.
          </p>
          <div class="p-3 bg-info-50 dark:bg-info-900/20 rounded-lg border border-info-200 dark:border-info-800">
            <div class="flex items-start gap-2">
              <UIcon name="i-lucide-lightbulb" class="text-info-600 dark:text-info-400 mt-0.5" />
              <p class="text-sm text-info-800 dark:text-info-200">
                <strong>Pro Tip:</strong> For <strong>Character</strong> names, an exact source match in the glossary bypasses the LLM entirely.
              </p>
            </div>
          </div>
        </div>
      </UCard>

      <!-- Database Location -->
      <UCard>
        <template #header>
          <div class="flex items-center gap-2">
            <UIcon name="i-lucide-database" class="text-gray-500 w-4 h-4" />
            <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Database Location</span>
          </div>
        </template>
        <div class="space-y-3">
          <p class="text-sm">
            The glossary database is stored per user in the OS app data directory <code class="px-2 py-1 bg-gray-100 dark:bg-gray-800 rounded text-xs">ml.ludolingua.blackat</code> as <code class="px-2 py-1 bg-gray-100 dark:bg-gray-800 rounded text-xs">ludolingua.db</code>.
          </p>
          <div class="grid grid-cols-1 md:grid-cols-3 gap-3">
            <div class="p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
              <div class="flex items-center gap-2 mb-2">
                <UIcon name="i-lucide-monitor" class="text-info" />
                <span class="font-medium text-sm">Windows</span>
              </div>
              <code class="text-xs text-gray-600 dark:text-gray-400 break-all">%AppData%/ml.ludolingua.blackat/ludolingua.db</code>
            </div>
            <div class="p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
              <div class="flex items-center gap-2 mb-2">
                <UIcon name="i-lucide-apple" class="text-neutral" />
                <span class="font-medium text-sm">macOS</span>
              </div>
              <code class="text-xs text-gray-600 dark:text-gray-400 break-all">~/Library/Application Support/ml.ludolingua.blackat/ludolingua.db</code>
            </div>
            <div class="p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
              <div class="flex items-center gap-2 mb-2">
                <UIcon name="i-lucide-terminal" class="text-warning" />
                <span class="font-medium text-sm">Linux</span>
              </div>
              <code class="text-xs text-gray-600 dark:text-gray-400 break-all">~/.local/share/ml.ludolingua.blackat/ludolingua.db</code>
            </div>
          </div>
        </div>
      </UCard>

      <!-- Prompt Type Mapping -->
      <UCard>
        <template #header>
          <div class="flex items-center gap-2">
            <UIcon name="i-lucide-arrow-right-left" class="text-gray-500 w-4 h-4" />
            <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Prompt Type Mapping</span>
          </div>
        </template>
        <div class="space-y-4">
          <div v-for="mapping in promptMappings" :key="mapping.type" class="p-4 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
            <div class="flex items-center gap-3 mb-3">
              <UIcon :name="mapping.icon" class="text-primary" />
              <span class="font-medium">{{ mapping.type }}</span>
              <UIcon name="i-lucide-arrow-right" class="text-gray-400" />
            </div>
            <div class="flex flex-wrap gap-2">
              <UBadge 
                v-for="category in mapping.categories" 
                :key="category" 
                variant="soft" 
                :color="getCategoryColor(category)"
                class="text-xs"
              >
                {{ category }}
              </UBadge>
            </div>
          </div>
        </div>
      </UCard>

      <!-- Keep glossary entries short -->
      <UAlert color="warning" variant="soft" icon="i-lucide-triangle-alert">
        <template #title>Keep glossary entries short</template>
        <template #description>
          <div class="text-sm">
            Add only short terms/phrases (names, UI words, concise rules). Do not paste long paragraphs; those belong in translations, not the glossary. Long entries bloat prompts and hurt quality.
          </div>
        </template>
      </UAlert>

      <!-- Prompt Format -->
      <UCard>
        <template #header>
          <div class="flex items-center gap-2">
            <UIcon name="i-lucide-code" class="text-gray-500 w-4 h-4" />
            <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Prompt Format</span>
          </div>
        </template>
        <div class="space-y-3">
          <p class="text-sm">
            Add terms under the right <em>Category</em> in the Glossary. Each item renders into the prompt as:
          </p>
          <div class="p-4 bg-gray-900 dark:bg-gray-800 rounded-lg">
            <pre class="text-xs text-success-400 overflow-auto"><code>### Category
Input: 源語 / Source term
Output: 翻訳 / Target term</code></pre>
          </div>
        </div>
      </UCard>

      <!-- Category Guide -->
      <UCard>
        <template #header>
          <div class="flex items-center gap-2">
            <UIcon name="i-lucide-bookmark" class="text-gray-500 w-4 h-4" />
            <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Category Guide</span>
          </div>
        </template>
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
          <div v-for="category in categories" :key="category.name" class="p-4 border border-gray-200 dark:border-gray-700 rounded-lg">
            <div class="flex items-center gap-2 mb-2">
              <UBadge :color="getCategoryColor(category.name)" variant="soft" class="text-xs">
                {{ category.name }}
              </UBadge>
            </div>
            <p class="text-sm leading-relaxed text-gray-600 dark:text-gray-400">
              {{ category.description }}
            </p>
          </div>
        </div>
        <template #footer>
          <div class="p-3 bg-info-50 dark:bg-info-900/20 rounded-lg">
            <div class="flex items-start gap-2">
              <UIcon name="i-lucide-lightbulb" class="text-info-600 dark:text-info-400 mt-0.5" />
              <div class="text-xs text-info-800 dark:text-info-200">
                <p class="font-medium mb-1">Pro Tips:</p>
                <ul class="space-y-1">
                  <li>• Terms are applied only when source/target languages match the current project and the term is enabled</li>
                  <li>• You can add terms directly from the Translation results using the "Add to glossary" button</li>
                  <li>• It auto-picks a Category based on the text unit's PromptType</li>
                </ul>
              </div>
            </div>
          </div>
        </template>
      </UCard>
    </div>
  </UCard>
</template>

<script setup lang="ts">
const promptMappings = [
  {
    type: 'Dialogue',
    icon: 'i-lucide-message-circle',
    categories: ['Characters', 'Essential Terms', 'Translation Rules', 'Locations', 'Time & Weather', 'Mechanics']
  },
  {
    type: 'Character',
    icon: 'i-lucide-user',
    categories: ['Characters', 'Essential Terms']
  },
  {
    type: 'State / Skill',
    icon: 'i-lucide-zap',
    categories: ['Status Effects', 'Mechanics', 'Essential Terms']
  },
  {
    type: 'Equipment',
    icon: 'i-lucide-sword',
    categories: ['Mechanics', 'Essential Terms']
  },
  {
    type: 'System / Class / Other',
    icon: 'i-lucide-settings',
    categories: ['Mechanics', 'Essential Terms']
  }
]

const categories = [
  {
    name: 'Characters',
    description: 'Proper names (people, monsters, places as names). Prefer consistent transliteration; avoid translating names unless the game localizes them officially. Exact matches may bypass the LLM.'
  },
  {
    name: 'Essential Terms',
    description: 'Game-specific vocabulary (currencies, factions, items). Single source of truth to keep wording uniform across files.'
  },
  {
    name: 'Translation Rules',
    description: 'Style and constraints (tone, honorifics, punctuation, brackets). These guide the model on how to write.'
  },
  {
    name: 'Locations',
    description: 'Place names used as common nouns. Keep canonical spelling; avoid inconsistent suffixes (e.g., "Forest of X").'
  },
  {
    name: 'Time & Weather',
    description: 'Temporal and weather terms (dawn, blizzard). Ensures consistent phrasing in UI and descriptions.'
  },
  {
    name: 'Mechanics',
    description: 'Mechanics and system terms (stats, damage types, item rarities). Avoid inventing new terms mid-project.'
  },
  {
    name: 'Status Effects',
    description: 'Buffs/debuffs and ailment names (Poison, Stun). Prefer standard localized names used by the genre.'
  }
]

const getCategoryColor = (category: string): 'primary' | 'secondary' | 'success' | 'info' | 'warning' | 'error' | 'neutral' => {
  const colorMap: Record<string, 'primary' | 'secondary' | 'success' | 'info' | 'warning' | 'error' | 'neutral'> = {
    'Characters': 'primary',
    'Essential Terms': 'success',
    'Translation Rules': 'secondary',
    'Locations': 'warning',
    'Time & Weather': 'info',
    'Mechanics': 'error',
    'Status Effects': 'warning'
  }
  return colorMap[category] || 'neutral'
}
</script>


