<template>
  <div class="container mx-auto p-6">
    <div class="space-y-6">
      <!-- Page Header -->
      <div class="space-y-2">
        <UBreadcrumb :links="breadcrumbLinks" />
        <div class="flex items-center justify-between">
          <div>
            <h1 class="text-3xl font-bold">Translation Management</h1>
            <p class="text-muted-foreground">
              View and manage all translations in your project database
            </p>
          </div>
          <UButton 
            variant="outline" 
            icon="i-heroicons-arrow-left" 
            to="/translator"
          >
            Back to Translator
          </UButton>
        </div>
      </div>

      <!-- Stats Cards -->
      <div v-if="stats" class="grid grid-cols-1 md:grid-cols-4 gap-4">
        <UCard>
          <div class="flex items-center gap-3">
            <div class="p-2 bg-blue-100 dark:bg-blue-900 rounded-lg">
              <Icon name="i-heroicons-document-text" class="h-5 w-5 text-blue-600 dark:text-blue-400" />
            </div>
            <div>
              <p class="text-sm text-muted-foreground">Total</p>
              <p class="text-2xl font-bold">{{ stats.total || 0 }}</p>
            </div>
          </div>
        </UCard>

        <UCard>
          <div class="flex items-center gap-3">
            <div class="p-2 bg-green-100 dark:bg-green-900 rounded-lg">
              <Icon name="i-heroicons-check-circle" class="h-5 w-5 text-green-600 dark:text-green-400" />
            </div>
            <div>
              <p class="text-sm text-muted-foreground">Translated</p>
              <p class="text-2xl font-bold">{{ stats.translated || 0 }}</p>
            </div>
          </div>
        </UCard>

        <UCard>
          <div class="flex items-center gap-3">
            <div class="p-2 bg-yellow-100 dark:bg-yellow-900 rounded-lg">
              <Icon name="i-heroicons-clock" class="h-5 w-5 text-yellow-600 dark:text-yellow-400" />
            </div>
            <div>
              <p class="text-sm text-muted-foreground">Pending</p>
              <p class="text-2xl font-bold">{{ stats.pending || 0 }}</p>
            </div>
          </div>
        </UCard>

        <UCard>
          <div class="flex items-center gap-3">
            <div class="p-2 bg-purple-100 dark:bg-purple-900 rounded-lg">
              <Icon name="i-heroicons-chart-bar" class="h-5 w-5 text-purple-600 dark:text-purple-400" />
            </div>
            <div>
              <p class="text-sm text-muted-foreground">Progress</p>
              <p class="text-2xl font-bold">{{ progressPercentage }}%</p>
            </div>
          </div>
        </UCard>
      </div>

      <!-- Main Table -->
      <TranslationTable />
    </div>
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

// Breadcrumb navigation
const breadcrumbLinks = [
  { label: 'Home', to: '/' },
  { label: 'Translator', to: '/translator' },
  { label: 'Translation Management' }
]

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
