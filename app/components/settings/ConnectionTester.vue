<template>
  <div class="flex items-center gap-2">
    <UBadge :color="badgeColor" variant="soft">{{ providerStore.connectionStatusText }}</UBadge>
    <UBadge color="neutral" variant="soft">{{ providerStore.selectedProvider }}</UBadge>
    <UButton
      :loading="providerStore.isLoading"
      variant="outline"
      size="sm"
      icon="i-heroicons-wifi"
      :disabled="!ready"
      @click="testConnection"
    >
      Test {{ providerStore.selectedProvider }}
    </UButton>
  </div>
 </template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { useProviderStore } from '~/stores/provider'
import { useSettingsStore } from '~/stores/settings'

// Use provider store for connection testing
const providerStore = useProviderStore()
const settingsStore = useSettingsStore()
const ready = ref(true)

// Methods
const testConnection = async () => {
  if (!ready.value) return
  await providerStore.testConnection(settingsStore.providerConfig, { silent: false })
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
  try {
    ready.value = await settingsStore.hasPersistedUserSettings()
  } catch {
    ready.value = false
  }
  if (ready.value && providerStore.connectionStatus === 'unknown') {
    await providerStore.testConnection(settingsStore.providerConfig, { silent: true })
  }
})
</script> 