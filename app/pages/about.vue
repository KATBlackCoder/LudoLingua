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
          <UTabs v-model="activeTab" :items="tabs" />
          <div>
            <AboutWhat v-if="activeTab === 'what'" />
            <AboutFeatures v-else-if="activeTab === 'features'" />
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
import AboutTechnology from '~/components/about/AboutTechnology.vue'
import AboutGlossary from '~/components/about/AboutGlossary.vue'
import AboutPlaceholders from '~/components/about/AboutPlaceholders.vue'

const tabs = [
  { label: 'What', icon: 'i-heroicons-information-circle', value: 'what' },
  { label: 'Features', icon: 'i-heroicons-sparkles', value: 'features' },
  { label: 'Technology', icon: 'i-heroicons-cpu-chip', value: 'technology' },
  { label: 'Glossary', icon: 'i-heroicons-book-open', value: 'glossary' },
  { label: 'Placeholders', icon: 'i-heroicons-code-bracket', value: 'placeholders' },
]
const activeTab = ref<'what' | 'features' | 'technology' | 'glossary' | 'placeholders'>('what')
</script>