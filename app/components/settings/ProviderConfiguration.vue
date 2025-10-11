<template>
  <UCard>
    <template #header>
      <div class="flex items-center gap-3">
        <div class="p-2 bg-orange-50 dark:bg-orange-900/20 rounded-lg">
          <UIcon name="i-lucide-cog" class="text-orange-500 w-5 h-5" />
        </div>
        <div>
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white">Provider Configuration</h3>
          <p class="text-sm text-gray-500 dark:text-gray-400">Configure your AI provider settings and model parameters</p>
        </div>
      </div>
    </template>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- Preset Selection -->
      <UCard>
        <template #header>
          <div class="flex items-center gap-2">
            <UIcon name="i-lucide-zap" class="text-gray-500 w-4 h-4" />
            <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Quick Presets</span>
          </div>
        </template>
        <UFormField label="Preset" description="Quick apply recommended values">
          <USelect 
            v-model="selectedPreset" 
            :items="presetItems" 
            class="w-full"
            @update:model-value="applyPreset"
          />
        </UFormField>
      </UCard>

      <!-- Provider Configuration -->
      <UCard>
        <template #header>
          <div class="flex items-center gap-2">
            <UIcon name="i-lucide-server" class="text-gray-500 w-4 h-4" />
            <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Provider Settings</span>
          </div>
        </template>
        
        <div class="space-y-4">
          <!-- RunPod Pod ID field - only show for RunPod provider -->
          <UFormField 
            v-if="currentProvider === 'RunPod'" 
            label="Pod ID" 
            description="Your RunPod pod identifier"
          >
            <UInput 
              :model-value="advancedSettings.base_url" 
              placeholder="your-pod-id"
              icon="i-lucide-hash"
              @update:model-value="updateField('base_url', $event)" 
            />
          </UFormField>

          <!-- Base URL field - only show for other providers (if any) -->
          <UFormField 
            v-else-if="currentProvider !== 'Ollama'" 
            label="Base URL" 
            description="Provider API endpoint"
          >
            <UInput 
              :model-value="advancedSettings.base_url" 
              placeholder="http://localhost:11434"
              icon="i-lucide-link"
              @update:model-value="updateField('base_url', $event)" 
            />
          </UFormField>

          <!-- Ollama info - show for Ollama provider -->
          <UFormField 
            v-if="currentProvider === 'Ollama'" 
            label="Endpoint" 
            description="Ollama local endpoint"
          >
            <UInput 
              value="http://localhost:11434" 
              readonly 
              disabled
              icon="i-lucide-check-circle"
            />
          </UFormField>
        </div>
      </UCard>

      <!-- Model Parameters -->
      <UCard>
        <template #header>
          <div class="flex items-center gap-2">
            <UIcon name="i-lucide-sliders" class="text-gray-500 w-4 h-4" />
            <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Model Parameters</span>
          </div>
        </template>
        
        <div class="grid grid-cols-1 gap-4">
          <UFormField label="Temperature" description="0.0 – 1.0 (randomness)">
            <UInput 
              :model-value="advancedSettings.temperature" 
              type="number" 
              min="0" 
              max="1" 
              step="0.1"
              icon="i-lucide-thermometer"
              @update:model-value="updateField('temperature', $event)" 
            />
          </UFormField>

          <UFormField label="Max Tokens" description="Maximum output tokens (100-150 recommended for single dialogue lines)">
            <UInput 
              :model-value="advancedSettings.max_tokens" 
              type="number" 
              min="50" 
              max="9999"
              icon="i-lucide-hash"
              @update:model-value="updateField('max_tokens', $event)" 
            />
          </UFormField>
        </div>
      </UCard>
    </div>

    <template #footer>
      <UAlert 
        :color="currentProvider === 'Ollama' ? 'info' : 'neutral'" 
        variant="soft"
        :icon="currentProvider === 'Ollama' ? 'i-lucide-info' : 'i-lucide-save'"
      >
        <template #title>
          {{ currentProvider === 'Ollama' ? 'Local Endpoint' : 'Settings Saved' }}
        </template>
        <template #description>
          <span v-if="currentProvider === 'Ollama'">
            Ollama uses local endpoint automatically.
          </span>
          <span v-else-if="currentProvider === 'RunPod'">
            Enter your RunPod pod ID (e.g., "abc123"). The app will format the URL automatically.
          </span>
          <span v-else>
            Changes are saved locally using Tauri Store.
          </span>
        </template>
      </UAlert>
    </template>
  </UCard>
</template>

<script setup lang="ts">
import { useSettingsStore } from '~/stores/settings'

interface Props {
  currentProvider: 'Ollama' | 'RunPod'
}

defineProps<Props>()
const settingsStore = useSettingsStore()

// Advanced settings form - initialize from store
const advancedSettings = ref({
  base_url: settingsStore.userSettings.base_url || '',
  api_key: '', // Void - not used in UI but kept for backend compatibility
  temperature: settingsStore.userSettings.temperature || 0.3,
  max_tokens: settingsStore.userSettings.max_tokens || 150,
})

// Update settings when store changes
watch(() => settingsStore.userSettings, (newSettings) => {
  advancedSettings.value = {
    base_url: newSettings.base_url || '',
    api_key: '', // Void - not used in UI but kept for backend compatibility
    temperature: newSettings.temperature || 0.3,
    max_tokens: newSettings.max_tokens || 150,
  }
}, { deep: true })

// Emit current settings for parent components to access
const emit = defineEmits<{
  'settings-updated': [settings: typeof advancedSettings.value]
}>()

// Watch for changes and emit them
watch(advancedSettings, (newSettings) => {
  emit('settings-updated', newSettings)
}, { deep: true })

// Also emit initial settings
onMounted(() => {
  emit('settings-updated', advancedSettings.value)
})

// Presets for temperature/max_tokens - optimized for one-by-one RPG dialogue translation
const presets = [
  { id: 'compact', label: 'Compact (0.3 · 100)', temperature: 0.3, max_tokens: 100 },
  { id: 'recommended', label: 'Recommended (0.3 · 150)', temperature: 0.3, max_tokens: 150 },
  { id: 'extended', label: 'Extended (0.3 · 200)', temperature: 0.3, max_tokens: 200 },
  { id: 'creative', label: 'Creative (0.5 · 150)', temperature: 0.5, max_tokens: 150 },
]
const presetItems = computed(() => presets.map(p => ({ label: p.label, value: p.id })))
const selectedPreset = ref('recommended')

function updateField(field: keyof typeof advancedSettings.value, value: string | number) {
  advancedSettings.value = {
    ...advancedSettings.value,
    [field]: value,
  }
}

function applyPreset(presetId: string) {
  const p = presets.find(x => x.id === presetId)
  if (!p) return
  
  advancedSettings.value = {
    ...advancedSettings.value,
    temperature: p.temperature,
    max_tokens: p.max_tokens,
  }
}
</script>
