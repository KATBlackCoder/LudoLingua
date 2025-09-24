<template>
  <div class="space-y-6">
    <!-- Page Header -->
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-3">
        <div class="p-2 bg-primary-50 dark:bg-primary-900/20 rounded-lg">
          <UIcon name="i-lucide-info" class="text-primary w-5 h-5" />
        </div>
        <div>
          <h1 class="text-2xl font-bold text-gray-900 dark:text-white">About LudoLingua</h1>
          <p class="text-sm text-gray-500 dark:text-gray-400">Offline RPG Maker translation with modern AI technology</p>
        </div>
      </div>
      <div class="flex items-center gap-2">
        <UButton
          variant="outline"
          icon="i-lucide-arrow-left"
          to="/"
        >
          Back to Home
        </UButton>
      </div>
    </div>

    <!-- Main Content -->
    <UCard>
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
        <div class="flex items-center justify-between">
          <p class="text-sm text-gray-500 dark:text-gray-400">
            LudoLingua is an open-source project. Contributions are welcome!
          </p>
          <div class="flex items-center gap-2">
            <UButton
              size="sm"
              variant="outline"
              icon="i-lucide-github"
              to="https://github.com/KATBlackCoder/LudoLingua"
              target="_blank"
            >
              View on GitHub
            </UButton>
          </div>
        </div>
      </template>
    </UCard>
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