<template>
  <UCard class="lg:col-span-2">
    <template #header>
      <span class="font-medium">Advanced</span>
    </template>
    <div class="grid grid-cols-1 md:grid-cols-3 gap-3">
      <UFormField label="Preset" name="preset" description="Quick apply recommended values">
        <USelect v-model="selectedPreset" :items="presetItems" @update:model-value="applyPreset" />
      </UFormField>
      
      <!-- RunPod Pod ID field - only show for RunPod provider -->
      <UFormField 
        v-if="currentProvider === 'RunPod'" 
        label="Pod ID" 
        name="pod_id" 
        description="Your RunPod pod identifier"
      >
        <UInput 
          :model-value="advancedSettings.base_url" 
          placeholder="your-pod-id"
          @update:model-value="updateField('base_url', $event)" 
        />
      </UFormField>

      <!-- Base URL field - only show for other providers (if any) -->
      <UFormField 
        v-else-if="currentProvider !== 'Ollama'" 
        label="Base URL" 
        name="base_url" 
        description="Provider API endpoint"
      >
        <UInput 
          :model-value="advancedSettings.base_url" 
          placeholder="http://localhost:11434"
          @update:model-value="updateField('base_url', $event)" 
        />
      </UFormField>

      <!-- Ollama info - show for Ollama provider -->
      <UFormField 
        v-if="currentProvider === 'Ollama'" 
        label="Endpoint" 
        name="endpoint_info" 
        description="Ollama local endpoint"
      >
        <UInput 
          value="http://localhost:11434" 
          readonly 
          disabled
        />
      </UFormField>

      <UFormField label="Temperature" name="temperature" description="0.0 – 1.0 (randomness)">
        <UInput 
          :model-value="advancedSettings.temperature" 
          type="number" 
          min="0" 
          max="1" 
          step="0.1"
          @update:model-value="updateField('temperature', $event)" 
        />
      </UFormField>

      <UFormField label="Max Tokens" name="max_tokens" description="Response token cap">
        <UInput 
          :model-value="advancedSettings.max_tokens" 
          type="number" 
          min="1" 
          max="8192"
          @update:model-value="updateField('max_tokens', $event)" 
        />
      </UFormField>
    </div>
    <template #footer>
      <div class="text-xs text-muted">
        <span v-if="currentProvider === 'Ollama'">
          Ollama uses local endpoint automatically.
        </span>
        <span v-else-if="currentProvider === 'RunPod'">
          Enter your RunPod pod ID (e.g., "abc123"). The app will format the URL automatically.
        </span>
        <span v-else>
          Changes are saved locally using Tauri Store.
        </span>
      </div>
    </template>
  </UCard>
</template>

<script setup lang="ts">
interface Props {
  advancedSettings: {
    base_url: string
    api_key: string
    temperature: number
    max_tokens: number
  }
  currentProvider: 'Ollama' | 'RunPod'
}

interface Emits {
  (e: 'update:advancedSettings', value: Props['advancedSettings']): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

// Presets for temperature/max_tokens
const presets = [
  { id: 'recommended', label: 'Recommended (0.3 · 2048)', temperature: 0.3, max_tokens: 2048 },
  { id: 'high', label: 'High Quality (0.3 · 4096)', temperature: 0.3, max_tokens: 4096 },
  { id: 'creative', label: 'Creative (0.7 · 2048)', temperature: 0.7, max_tokens: 2048 },
  { id: 'maximum', label: 'Maximum Context (0.3 · 8192)', temperature: 0.3, max_tokens: 8192 },
]
const presetItems = computed(() => presets.map(p => ({ label: p.label, value: p.id })))
const selectedPreset = ref('recommended')

function updateField(field: keyof Props['advancedSettings'], value: string | number) {
  const updatedSettings = {
    ...props.advancedSettings,
    [field]: value,
  }
  emit('update:advancedSettings', updatedSettings)
}

function applyPreset(presetId: string) {
  const p = presets.find(x => x.id === presetId)
  if (!p) return
  
  const updatedSettings = {
    ...props.advancedSettings,
    temperature: p.temperature,
    max_tokens: p.max_tokens,
  }
  emit('update:advancedSettings', updatedSettings)
}
</script>
