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
    <div v-if="stats" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
      <UCard>
        <div class="p-6">
          <div class="flex items-center gap-3 mb-4">
            <div class="p-2 bg-blue-50 dark:bg-blue-900/20 rounded-lg">
              <UIcon name="i-lucide-file-text" class="text-blue-500 w-5 h-5" />
            </div>
            <div>
              <h3 class="text-sm font-medium text-gray-600 dark:text-gray-400">Total Items</h3>
              <p class="text-2xl font-bold text-gray-900 dark:text-white">{{ stats.total || 0 }}</p>
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
              <p class="text-2xl font-bold text-gray-900 dark:text-white">{{ stats.translated || 0 }}</p>
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
              <p class="text-2xl font-bold text-gray-900 dark:text-white">{{ stats.pending || 0 }}</p>
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
      <TranslationTable />
    </UCard>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
import TranslationTable from '~/components/translations/TranslationTable.vue'
import { useTranslations } from '~/composables/useTranslations'

// Set page meta
definePageMeta({
  title: 'Translation Management'
})

const { stats, loadTranslations } = useTranslations()

// Computed progress percentage
const progressPercentage = computed(() => {
  if (!stats.value || !stats.value.total || stats.value.total === 0) return 0
  const translated = stats.value.translated || 0
  return Math.round((translated / stats.value.total) * 100)
})

// Load data on mount
onMounted(async () => {
  await loadTranslations()
  // Note: Stats would need to be implemented in the composable to fetch from backend
})
</script>
