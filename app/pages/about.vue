<template>
  <div>
    <UContainer class="max-w-6xl">
      <UCard class="max-w-6xl mx-auto">
        <template #header>
          <div class="flex items-center gap-3">
            <UIcon name="i-lucide-info" class="w-6 h-6 text-primary-500" />
            <div>
              <h2 class="text-xl font-semibold">About LudoLingua</h2>
              <p class="text-sm text-gray-500 dark:text-gray-400">Offline RPG Maker translation with a modern Nuxt + Tauri UI</p>
            </div>
          </div>
        </template>

        <div class="space-y-6">
          <UTabs v-model="activeTab" :items="tabs" />
          <div>
            <AboutWhat v-if="activeTab === 'what'" />
            <AboutFeatures v-else-if="activeTab === 'features'" />
            <AboutTranslations v-else-if="activeTab === 'translations'" />
            <AboutTechnology v-else-if="activeTab === 'technology'" />
            <AboutGlossary v-else-if="activeTab === 'glossary'" />
            <AboutPlaceholders v-else-if="activeTab === 'placeholders'" />
          </div>
        </div>

        <template #footer>
          <p class="text-sm text-gray-500">LudoLingua is an open-source project. Contributions are welcome!</p>
        </template>
      </UCard>
    </UContainer>
  </div>
</template>

<script setup lang="ts">
import AboutWhat from '~/components/about/AboutWhat.vue'
import AboutFeatures from '~/components/about/AboutFeatures.vue'
import AboutTranslations from '~/components/about/AboutTranslations.vue'
import AboutTechnology from '~/components/about/AboutTechnology.vue'
import AboutGlossary from '~/components/about/AboutGlossary.vue'
import AboutPlaceholders from '~/components/about/AboutPlaceholders.vue'

const tabs = [
  { label: 'What', icon: 'i-lucide-info', value: 'what' },
  { label: 'Features', icon: 'i-lucide-sparkles', value: 'features' },
  { label: 'Translations', icon: 'i-lucide-languages', value: 'translations' },
  { label: 'Technology', icon: 'i-lucide-cpu', value: 'technology' },
  { label: 'Glossary', icon: 'i-lucide-book-open', value: 'glossary' },
  { label: 'Placeholders', icon: 'i-lucide-code', value: 'placeholders' },
]
const activeTab = ref<'what' | 'features' | 'translations' | 'technology' | 'glossary' | 'placeholders'>('what')
</script>