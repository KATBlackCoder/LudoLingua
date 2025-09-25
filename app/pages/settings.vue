<template>
  <div class="space-y-8">
    <!-- Page Header -->
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-3">
        <div class="p-2 bg-primary-50 dark:bg-primary-900/20 rounded-lg">
          <UIcon name="i-lucide-settings" class="text-primary w-6 h-6" />
        </div>
        <div>
          <h1 class="text-2xl font-bold text-gray-900 dark:text-white">Settings</h1>
          <p class="text-sm text-gray-500 dark:text-gray-400">Configure your AI model, language preferences, and application settings</p>
        </div>
      </div>
      <div class="flex items-center gap-2">
        <ConnectionTester :provider-settings="currentProviderSettings" />
      </div>
    </div>

    <!-- Status Messages -->
    <div class="space-y-4">
      <!-- Success Message -->
      <UAlert 
        v-if="showSuccessMessage" 
        color="success" 
        variant="soft" 
        icon="i-lucide-check-circle" 
        title="Settings saved successfully"
        description="Your configuration has been updated and will be used for future translations."
      />

      <!-- Initial Setup Warning -->
      <UAlert
        v-if="!hasSettings"
        color="warning"
        variant="soft"
        icon="i-lucide-settings"
        title="Complete Initial Setup"
        description="No settings found yet. Click Save at least once to create your configuration before continuing."
      />
    </div>

    <!-- Settings Sections -->
    <div class="space-y-8">
      <!-- Core Configuration -->
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <!-- AI Provider -->
        <ProviderSelector />

        <!-- Languages -->
        <LanguageSelector />
      </div>

      <!-- Provider Configuration -->
      <div class="space-y-6">
        <ProviderConfiguration 
          :current-provider="providerStore.selectedProvider"
          @settings-updated="onProviderSettingsUpdated"
        />
      </div>

      <!-- System Management -->
      <div class="space-y-6">
        <UpdateManager />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import ConnectionTester from '~/components/settings/ConnectionTester.vue'
import LanguageSelector from '~/components/settings/LanguageSelector.vue'
import ProviderSelector from '~/components/settings/ProviderSelector.vue'
import ProviderConfiguration from '~/components/settings/ProviderConfiguration.vue'
import UpdateManager from '~/components/settings/UpdateManager.vue'
import { useSettingsStore } from '~/stores/settings'
import { useProviderStore } from '~/stores/provider'

// Store references
const settingsStore = useSettingsStore()
const providerStore = useProviderStore()

// Reactive state
const showSuccessMessage = ref(false)
const hasSettings = ref(true)
const currentProviderSettings = ref({
  base_url: '',
  api_key: '',
  temperature: 0.3,
  max_tokens: 2048,
})

// Handle provider settings updates
const onProviderSettingsUpdated = (settings: typeof currentProviderSettings.value) => {
  currentProviderSettings.value = { ...settings }
}

// Initialize settings when component mounts
onMounted(async () => {
  try {
    await settingsStore.initializeStores()
    // Check persisted settings existence for first-run alert
    hasSettings.value = await settingsStore.hasPersistedUserSettings()
  } catch (error) {
    console.error('Failed to initialize settings:', error)
  }
})
</script>
