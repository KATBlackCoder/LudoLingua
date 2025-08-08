<template>
  <div class="space-y-2">
    <div class="flex items-center gap-3">
      <span :class="['inline-block h-2.5 w-2.5 rounded-full', dotClass]" />
      <span class="text-sm">{{ providerStore.connectionStatusText }}</span>
      <span class="text-xs text-gray-500 dark:text-gray-400">
        <template v-if="providerStore.lastConnectionTest">
          Last test: {{ formattedLastTest }}
        </template>
        <template v-else>
          Not tested yet
        </template>
      </span>
    </div>

    <div class="flex items-center">
      <UButton
        :loading="providerStore.isLoading"
        variant="outline"
        size="sm"
        @click="testConnection"
      >
        Test Connection
      </UButton>
    </div>
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

// Status dot color
const dotClass = computed(() => {
  switch (providerStore.connectionStatus) {
    case 'connected':
      return 'bg-green-500'
    case 'disconnected':
      return 'bg-red-500'
    default:
      return 'bg-gray-400'
  }
})

// Last test formatted
const formattedLastTest = computed(() => {
  const d = providerStore.lastConnectionTest
  return d ? new Date(d).toLocaleString() : ''
})

// Auto-test on mount
onMounted(async () => {
  if (providerStore.connectionStatus === 'unknown') {
    await providerStore.testConnection()
  }
})
</script> 