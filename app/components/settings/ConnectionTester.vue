<template>
  <div class="flex items-center gap-2">
    <UBadge :color="badgeColor" variant="soft">{{ providerStore.connectionStatusText }}</UBadge>
    <UButton :loading="providerStore.isLoading" variant="outline" size="sm" icon="i-heroicons-wifi" @click="testConnection">Test</UButton>
  </div>
 </template>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useProviderStore } from '~/stores/provider'

// Use provider store for connection testing
const providerStore = useProviderStore()

// Methods
const testConnection = async () => {
  await providerStore.testConnection()
}

// Status badge color
const badgeColor = computed(() => {
  switch (providerStore.connectionStatus) {
    case 'connected':
      return 'success'
    case 'disconnected':
      return 'error'
    default:
      return 'neutral'
  }
})

// Keep minimal UI; last test can be shown in settings page

// Auto-test on mount
onMounted(async () => {
  if (providerStore.connectionStatus === 'unknown') {
    await providerStore.testConnection()
  }
})
</script> 