<template>
  <div>
    <UContainer>
      <div class="space-y-6 max-w-6xl mx-auto">
        <!-- Page header -->
        <div class="flex items-start justify-between gap-3">
          <div>
            <h1 class="text-2xl font-bold">Settings</h1>
            <p class="text-gray-600 dark:text-gray-400 mt-1">Configure model, and language preferences.</p>
          </div>
          <div class="flex items-center gap-2">
            <ConnectionTester />
            <UButton variant="outline" size="sm" color="error" :loading="settingsStore.isLoading" @click="resetSettings">Reset</UButton>
            <UButton size="sm" :loading="settingsStore.isLoading" @click="saveSettings">Save</UButton>
          </div>
        </div>

        <UAlert v-if="showSuccessMessage" color="success" variant="soft" icon="i-lucide-check-circle" title="Settings saved" />

        <UAlert
          v-if="!hasSettings"
          color="warning"
          variant="soft"
          icon="i-lucide-settings"
          title="Finish initial setup"
          class="mt-2"
        >
          <template #description>
            <div class="text-sm">
              No settings found yet. Click <strong>Save</strong> at least once to create your configuration before continuing.
            </div>
          </template>
        </UAlert>


        <!-- Content grid -->
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
          <!-- Model -->
          <UCard>
            <template #header>
              <span class="font-medium">Model</span>
            </template>
            <div class="space-y-4">
              <ProviderSelector />
            </div>
          </UCard>

          <!-- Languages -->
          <UCard>
            <template #header>
              <span class="font-medium">Languages</span>
            </template>
            <LanguageSelector />
          </UCard>

          <!-- Advanced (spans full on lg via col-span-2) -->
          <AdvancedSettings 
            v-model:advanced-settings="advancedSettings"
            :current-provider="providerStore.selectedProvider"
          />

          <!-- Updates (spans full width) -->
          <div class="col-span-2">
            <UpdateManager />
          </div>
        </div>
      </div>
    </UContainer>
  </div>
 </template>

<script setup lang="ts">
import ConnectionTester from '~/components/settings/ConnectionTester.vue'
import LanguageSelector from '~/components/settings/LanguageSelector.vue'
import ProviderSelector from '~/components/settings/ProviderSelector.vue'
import AdvancedSettings from '~/components/settings/AdvancedSettings.vue'
import UpdateManager from '~/components/settings/UpdateManager.vue'
import { useSettingsStore } from '~/stores/settings'
import { useProviderStore } from '~/stores/provider'
import { useLanguageStore } from '~/stores/language'
import type { Language } from '~/types/language'

// Store references
const settingsStore = useSettingsStore()
const providerStore = useProviderStore()
const languageStore = useLanguageStore()

// Reactive state
const showSuccessMessage = ref(false)
const hasSettings = ref(true)

// Advanced settings form
const advancedSettings = ref({
  base_url: settingsStore.userSettings.base_url || '',
  api_key: '', // Void - not used in UI but kept for backend compatibility
  temperature: settingsStore.userSettings.temperature || 0.3,
  max_tokens: settingsStore.userSettings.max_tokens || 2048,
})


// Initialize settings when component mounts
onMounted(async () => {
  try {
    await settingsStore.initializeStores()
    // Check persisted settings existence for first-run alert
    hasSettings.value = await settingsStore.hasPersistedUserSettings()
    
    // Initialize form with current settings after stores are loaded
    updateFormFromSettings()
  } catch (error) {
    console.error('Failed to initialize settings:', error)
  }
})

// Update form from store settings
function updateFormFromSettings() {
  const currentSettings = settingsStore.userSettings
  
  // For Ollama, don't show base_url in UI (always uses localhost)
  // For RunPod, show the pod ID (stored in base_url)
  const displayBaseUrl = currentSettings.provider === 'Ollama' ? '' : (currentSettings.base_url || '')
  
  advancedSettings.value = {
    base_url: displayBaseUrl,
    api_key: '', // Void - not used in UI but kept for backend compatibility
    temperature: currentSettings.temperature || 0.3,
    max_tokens: currentSettings.max_tokens || 2048,
  }
}

// Watch for provider changes to update base_url defaults
watch(
  () => providerStore.selectedProvider,
  (newProvider) => {
    // For Ollama, clear the base_url field (always uses localhost)
    // For RunPod, clear the field to let user input their pod ID
    if (newProvider === 'Ollama') {
      advancedSettings.value.base_url = ''
    } else if (newProvider === 'RunPod') {
      // Clear pod ID field for RunPod to let user input their pod ID
      advancedSettings.value.base_url = ''
    }
  }
)

// Methods
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
    
    // Show success message
    showSuccessMessage.value = true
    hasSettings.value = true
    setTimeout(() => {
      showSuccessMessage.value = false
    }, 3000)
  } catch (error) {
    console.error('Failed to save settings:', error)
  }
}

const resetSettings = async () => {
  try {
    await settingsStore.resetUserSettings()
    hasSettings.value = true
    
    // Update form after reset
    updateFormFromSettings()
    
    // Show success message
    showSuccessMessage.value = true
    setTimeout(() => {
      showSuccessMessage.value = false
    }, 3000)
  } catch (error) {
    console.error('Failed to reset settings:', error)
  }
}
</script>
