<template>
  <div class="flex items-center gap-4">
    <!-- Connection Status -->
    <div class="flex items-center gap-3">
      <div class="flex items-center gap-2">
        <UIcon name="i-lucide-wifi" class="text-gray-500 w-4 h-4" />
        <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Connection</span>
      </div>
      <UBadge :color="badgeColor" variant="soft" size="sm">
        {{ providerStore.connectionStatusText }}
      </UBadge>
    </div>

    <!-- Provider Info -->
    <div class="flex items-center gap-2">
      <UIcon name="i-lucide-server" class="text-gray-400 w-4 h-4" />
      <span class="text-sm text-gray-600 dark:text-gray-400">{{ providerStore.selectedProvider }}</span>
    </div>

    <!-- Action Buttons -->
    <div class="flex items-center gap-2">
      <UButton
        :loading="providerStore.isLoading"
        color="primary"
        variant="soft"
        size="sm"
        icon="i-lucide-wifi"
        :disabled="!ready"
        @click="testConnection"
      >
        Test
      </UButton>
      <UButton 
        variant="outline" 
        size="sm" 
        color="error" 
        :loading="settingsStore.isLoading" 
        @click="resetSettings"
      >
        <UIcon name="i-lucide-refresh-cw" class="w-4 h-4 mr-1" />
        Reset
      </UButton>
      <UButton 
        size="sm" 
        :loading="settingsStore.isLoading" 
        @click="saveSettings"
      >
        <UIcon name="i-lucide-save" class="w-4 h-4 mr-1" />
        Save
      </UButton>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { useProviderStore } from '~/stores/provider'
import { useSettingsStore } from '~/stores/settings'
import { useLanguageStore } from '~/stores/language'
import type { Language } from '~/types/language'

// Use provider store for connection testing
const providerStore = useProviderStore()
const settingsStore = useSettingsStore()
const languageStore = useLanguageStore()
const ready = ref(true)

// Advanced settings form - get from current settings
const advancedSettings = computed(() => ({
  base_url: settingsStore.userSettings.base_url || '',
  api_key: '', // Void - not used in UI but kept for backend compatibility
  temperature: settingsStore.userSettings.temperature || 0.3,
  max_tokens: settingsStore.userSettings.max_tokens || 2048,
}))

// Methods
const testConnection = async () => {
  if (!ready.value) return
  await providerStore.testConnection(settingsStore.providerConfig, { silent: false })
}

const saveSettings = async () => {
  try {
    // Collect all current settings
    const currentSettings = {
      provider: providerStore.selectedProvider,
      model: providerStore.selectedModel || settingsStore.userSettings.model,
      source_language: languageStore.getLanguage.source_language as Language,
      target_language: languageStore.getLanguage.target_language as Language,
      // For Ollama, always use localhost (backend handles this)
      // For RunPod, store the pod ID as base_url
      base_url: providerStore.selectedProvider === 'Ollama' ? 'http://localhost:11434' : advancedSettings.value.base_url,
      api_key: undefined, // Void - not used in UI but kept for backend compatibility
      temperature: advancedSettings.value.temperature,
      max_tokens: advancedSettings.value.max_tokens,
    }

    // Save to persistent storage
    await settingsStore.saveUserSettings(currentSettings)
  } catch (error) {
    console.error('Failed to save settings:', error)
  }
}

const resetSettings = async () => {
  try {
    await settingsStore.resetUserSettings()
  } catch (error) {
    console.error('Failed to reset settings:', error)
  }
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