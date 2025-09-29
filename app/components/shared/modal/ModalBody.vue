<template>
  <div class="space-y-6">
    <!-- Loading State -->
    <div v-if="loading" class="text-center py-8">
      <UIcon name="i-lucide-loader-2" class="w-8 h-8 mx-auto animate-spin text-primary mb-4" />
      <p class="text-gray-600 dark:text-gray-400">{{ loadingMessage }}</p>
    </div>
    
    <!-- Error State -->
    <UAlert
      v-else-if="error"
      color="error"
      variant="soft"
      icon="i-lucide-alert-triangle"
      :title="errorTitle"
      :description="error"
    />
    
    <!-- Content -->
    <div v-else class="space-y-4">
      <slot />
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  loading?: boolean
  loadingMessage?: string
  error?: string
  errorTitle?: string
}

withDefaults(defineProps<Props>(), {
  loading: false,
  loadingMessage: 'Loading...',
  error: '',
  errorTitle: 'Error'
})
</script>
