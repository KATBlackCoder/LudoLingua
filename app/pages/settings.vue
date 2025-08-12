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

        <UAlert v-if="showSuccessMessage" color="success" variant="soft" icon="i-heroicons-check-circle" title="Settings saved" />

        <!-- Content grid -->
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
          <!-- Model -->
          <UCard>
            <template #header>
              <span class="font-medium">Model</span>
            </template>
            <div class="space-y-4">
              <ModelSelector />
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
          <UCard class="lg:col-span-2">
            <template #header>
              <span class="font-medium">Advanced</span>
            </template>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-3">
              <UFormField label="Preset" name="preset" description="Quick apply recommended values">
                <USelect v-model="selectedPreset" :items="presetItems" @update:model-value="applyPreset" />
              </UFormField>
              <UFormField label="Base URL" name="base_url" description="Provider API endpoint">
                <UInput v-model="advancedSettings.base_url" placeholder="http://localhost:11434" />
              </UFormField>

              <UFormField label="Temperature" name="temperature" description="0.0 – 1.0 (randomness)">
                <UInput v-model.number="advancedSettings.temperature" type="number" min="0" max="1" step="0.1" />
              </UFormField>

              <UFormField label="Max Tokens" name="max_tokens" description="Response token cap">
                <UInput v-model.number="advancedSettings.max_tokens" type="number" min="1" max="8192" />
              </UFormField>
            </div>
            <template #footer>
              <div class="text-xs text-muted">Changes are saved locally using Tauri Store.</div>
            </template>
          </UCard>
        </div>
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
  temperature: settingsStore.userSettings.temperature || 0.3,
  max_tokens: settingsStore.userSettings.max_tokens || 256,
})

// Presets for temperature/max_tokens
const presets = [
  { id: 'recommended', label: 'Recommended (0.3 · 256)', temperature: 0.3, max_tokens: 256 },
  { id: 'high', label: 'High (long lines) (0.3 · 512)', temperature: 0.3, max_tokens: 512 },
  { id: 'creative', label: 'Creative (0.7 · 256)', temperature: 0.7, max_tokens: 256 },
]
const presetItems = computed(() => presets.map(p => ({ label: p.label, value: p.id })))
const selectedPreset = ref('recommended')

function applyPreset(presetId: string) {
  const p = presets.find(x => x.id === presetId)
  if (!p) return
  advancedSettings.value.temperature = p.temperature
  advancedSettings.value.max_tokens = p.max_tokens
}

// Watch for settings changes and update form
watch(
  () => settingsStore.userSettings,
  (newSettings) => {
    advancedSettings.value = {
      base_url: newSettings.base_url || 'http://localhost:11434',
      api_key: newSettings.api_key || '',
      temperature: newSettings.temperature || 0.3,
      max_tokens: newSettings.max_tokens || 256,
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
