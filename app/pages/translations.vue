<template>
  <div class="space-y-6">
    <!-- Page Header -->
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-3">
        <div class="p-2 bg-primary-50 dark:bg-primary-900/20 rounded-lg">
          <UIcon name="i-lucide-database" class="text-primary w-5 h-5" />
        </div>
        <div>
          <h1 class="text-2xl font-bold text-gray-900 dark:text-white">Translation Management</h1>
          <p class="text-sm text-gray-500 dark:text-gray-400">View and manage all translations in your project database</p>
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

    <!-- Stats Overview -->
    <div v-if="translationStore.stats" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
      <UCard>
        <div class="p-6">
          <div class="flex items-center gap-3 mb-4">
            <div class="p-2 bg-blue-50 dark:bg-blue-900/20 rounded-lg">
              <UIcon name="i-lucide-file-text" class="text-blue-500 w-5 h-5" />
            </div>
            <div>
              <h3 class="text-sm font-medium text-gray-600 dark:text-gray-400">Total Items</h3>
              <p class="text-2xl font-bold text-gray-900 dark:text-white">{{ (translationStore.stats as any)?.total || 0 }}</p>
            </div>
          </div>
        </div>
      </UCard>

      <UCard>
        <div class="p-6">
          <div class="flex items-center gap-3 mb-4">
            <div class="p-2 bg-green-50 dark:bg-green-900/20 rounded-lg">
              <UIcon name="i-lucide-check-circle" class="text-green-500 w-5 h-5" />
            </div>
            <div>
              <h3 class="text-sm font-medium text-gray-600 dark:text-gray-400">Translated</h3>
              <p class="text-2xl font-bold text-gray-900 dark:text-white">{{ (translationStore.stats as any)?.translated || 0 }}</p>
            </div>
          </div>
        </div>
      </UCard>

      <UCard>
        <div class="p-6">
          <div class="flex items-center gap-3 mb-4">
            <div class="p-2 bg-orange-50 dark:bg-orange-900/20 rounded-lg">
              <UIcon name="i-lucide-clock" class="text-orange-500 w-5 h-5" />
            </div>
            <div>
              <h3 class="text-sm font-medium text-gray-600 dark:text-gray-400">Pending</h3>
              <p class="text-2xl font-bold text-gray-900 dark:text-white">{{ (translationStore.stats as any)?.pending || 0 }}</p>
            </div>
          </div>
        </div>
      </UCard>

      <UCard>
        <div class="p-6">
          <div class="flex items-center gap-3 mb-4">
            <div class="p-2 bg-purple-50 dark:bg-purple-900/20 rounded-lg">
              <UIcon name="i-lucide-bar-chart-3" class="text-purple-500 w-5 h-5" />
            </div>
            <div>
              <h3 class="text-sm font-medium text-gray-600 dark:text-gray-400">Progress</h3>
              <p class="text-2xl font-bold text-gray-900 dark:text-white">{{ progressPercentage }}%</p>
            </div>
          </div>
          <UProgress :value="progressPercentage" size="sm" class="mt-2" />
        </div>
      </UCard>
    </div>

    <!-- Translation Table -->
    <UCard>
      <TranslationView />
    </UCard>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
import TranslationView from '~/components/translations/TranslationView.vue'
import { useTranslationStore } from '~/stores/translation'

// Set page meta
definePageMeta({
  title: 'Translation Management'
})

const translationStore = useTranslationStore()

// Computed progress percentage
const progressPercentage = computed(() => {
  const currentStats = translationStore.stats
  if (!currentStats) return 0
  
  const statsData = currentStats as Record<string, unknown>
  const total = Number(statsData?.total) || 0
  const translated = Number(statsData?.translated) || 0
  
  if (total === 0) return 0
  return Math.round((translated / total) * 100)
})

// Load data on mount
onMounted(async () => {
  await translationStore.fetchTranslations()
  await translationStore.fetchStats()
})
</script>
