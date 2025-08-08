<template>
  <div>
    <UContainer>
      <div class="space-y-8">
        <!-- Header -->
        <div>
          <h1 class="text-2xl font-bold">Settings</h1>
          <p class="text-gray-600 dark:text-gray-400 mt-2">
            Configure your translation preferences and provider settings
          </p>
        </div>

        <!-- Settings Form -->
        <UCard>
          <template #header>
            <div class="flex items-center justify-between">
              <h2 class="text-lg font-semibold">Translation Settings</h2>
              <div class="flex gap-2">
                <ConnectionTester />
                <UButton
                  variant="outline"
                  size="sm"
                  :loading="settingsStore.isLoading"
                  @click="resetSettings"
                >
                  Reset to Defaults
                </UButton>
                <UButton
                  :loading="settingsStore.isLoading"
                  @click="saveSettings"
                >
                  Save Settings
                </UButton>
              </div>
            </div>
          </template>

          <div class="space-y-6">
            <!-- Model Settings Section (Ollama-only) -->
            <div>
              <h3 class="text-md font-medium mb-4">Model Configuration</h3>
              <div class="space-y-4">
                <div class="flex flex-wrap items-start gap-4">
                  <ModelSelector />
                </div>
              </div>
            </div>

            <!-- Language Settings Section -->
            <div>
              <h3 class="text-md font-medium mb-4">Language Configuration</h3>
              <LanguageSelector />
            </div>

            <!-- Advanced Settings Section -->
            <div>
              <h3 class="text-md font-medium mb-4">Advanced Settings</h3>
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <UFormField
                  label="Base URL"
                  name="base_url"
                  description="Provider API endpoint"
                >
                  <UInput
                    v-model="advancedSettings.base_url"
                    placeholder="http://localhost:11434"
                  />
                </UFormField>

                <UFormField
                  label="API Key"
                  name="api_key"
                  description="API key for the provider (optional for Ollama)"
                >
                  <UInput
                    v-model="advancedSettings.api_key"
                    type="password"
                    placeholder="Enter API key"
                  />
                </UFormField>

                <UFormField
                  label="Temperature"
                  name="temperature"
                  description="Controls randomness in responses (0.0 to 1.0)"
                >
                  <UInput
                    v-model.number="advancedSettings.temperature"
                    type="number"
                    min="0"
                    max="1"
                    step="0.1"
                  />
                </UFormField>

                <UFormField
                  label="Max Tokens"
                  name="max_tokens"
                  description="Maximum tokens in response"
                >
                  <UInput
                    v-model.number="advancedSettings.max_tokens"
                    type="number"
                    min="1"
                    max="8192"
                  />
                </UFormField>
              </div>
            </div>
          </div>
        </UCard>
      </div>
    </UContainer>
  </div>
</template>

<script setup lang="ts">
import ConnectionTester from '~/components/settings/ConnectionTester.vue'
import LanguageSelector from '~/components/settings/LanguageSelector.vue'
import ModelSelector from '~/components/settings/ModelSelector.vue'
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

// Advanced settings form
const advancedSettings = ref({
  base_url: settingsStore.userSettings.base_url || 'http://localhost:11434',
  api_key: settingsStore.userSettings.api_key || '',
  temperature: settingsStore.userSettings.temperature || 0.7,
  max_tokens: settingsStore.userSettings.max_tokens || 2048,
})

// Watch for settings changes and update form
watch(
  () => settingsStore.userSettings,
  (newSettings) => {
    advancedSettings.value = {
      base_url: newSettings.base_url || 'http://localhost:11434',
      api_key: newSettings.api_key || '',
      temperature: newSettings.temperature || 0.7,
      max_tokens: newSettings.max_tokens || 2048,
    }
  },
  { immediate: true, deep: true }
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
      base_url: advancedSettings.value.base_url,
      api_key: advancedSettings.value.api_key || undefined,
      temperature: advancedSettings.value.temperature,
      max_tokens: advancedSettings.value.max_tokens,
    }

    // Save to persistent storage
    await settingsStore.saveUserSettings(currentSettings)
    
    // Show success message
    showSuccessMessage.value = true
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
    
    // Show success message
    showSuccessMessage.value = true
    setTimeout(() => {
      showSuccessMessage.value = false
    }, 3000)
  } catch (error) {
    console.error('Failed to reset settings:', error)
  }
}

// Initialize settings when component mounts
onMounted(async () => {
  try {
    await settingsStore.initializeStores()
  } catch (error) {
    console.error('Failed to initialize settings:', error)
  }
})
</script>
