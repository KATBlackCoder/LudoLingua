<template>
  <UCard>
    <template #header>
      <div class="flex items-center gap-3">
        <div class="p-2 bg-purple-50 dark:bg-purple-900/20 rounded-lg">
          <UIcon name="i-lucide-brain" class="text-purple-500 w-5 h-5" />
        </div>
        <div>
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white">AI Provider</h3>
          <p class="text-sm text-gray-500 dark:text-gray-400">Choose your AI provider and model for translation</p>
        </div>
      </div>
    </template>

    <div class="space-y-6">
      <!-- Provider and Model Selection -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <UFormField label="Provider" description="Choose an LLM provider">
          <USelect
            v-model="selectedProvider"
            :items="providerOptions"
            placeholder="Select a provider"
            class="w-full"
            icon="i-lucide-server"
            @update:model-value="onProviderChange"
          />
        </UFormField>
        
        <UFormField label="Model" description="Model to use for translation">
          <USelect
            v-model="selectedModelName"
            :items="modelOptions"
            placeholder="Select a model"
            class="w-full"
            icon="i-lucide-cpu"
          />
        </UFormField>
      </div>

      <!-- Current Selection Display -->
      <div v-if="selectedDisplay" class="flex items-center gap-3 p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
        <UIcon name="i-lucide-check-circle" class="text-green-500 w-4 h-4" />
        <div class="flex-1">
          <div class="text-sm font-medium text-gray-900 dark:text-white">Selected Configuration</div>
          <div class="text-xs text-gray-500 dark:text-gray-400">
            {{ selectedProvider }} • {{ selectedDisplay }}
          </div>
        </div>
        <UBadge color="success" variant="soft" size="sm">
          {{ selectedDisplay }}
        </UBadge>
      </div>

      <!-- Provider Info -->
      <UAlert 
        color="info" 
        variant="soft" 
        icon="i-lucide-info"
        title="Provider Setup"
        description="After selecting your provider, configure the settings in the Provider Configuration section below."
      />
    </div>
  </UCard>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import { useProviderStore } from '~/stores/provider'
import type { Provider } from '~/types/provider'

const providerStore = useProviderStore()

// Provider options and v-model
const providerOptions = computed(() => providerStore.getProvider)
const selectedProvider = ref(providerStore.selectedProvider)

// Model options for current provider
const modelOptions = computed(() =>
  providerStore.availableModels.map(m => ({ label: m.display_name, value: m.model_name }))
)

const selectedModelName = ref<string | undefined>(providerStore.selectedModel?.model_name)
const selectedDisplay = computed(() => providerStore.selectedModel?.display_name || '')

// Keep selectedModelName in sync with store
watch(
  () => providerStore.selectedModel,
  (model) => {
    selectedModelName.value = model?.model_name
  },
  { immediate: true }
)

// When the user selects a provider, update store and refresh models
const onProviderChange = async (provider: string) => {
  await providerStore.setProvider(provider as Provider)
  if (providerStore.availableModels.length > 0) {
    providerStore.setModel(providerStore.availableModels[0]!)
  }
}

// When the user selects a model, update the store
watch(selectedModelName, (modelName) => {
  if (modelName) {
    const model = providerStore.availableModels.find(m => m.model_name === modelName)
    if (model && model !== providerStore.selectedModel) {
      providerStore.setModel(model)
    }
  }
})

// Load providers and models when component mounts
onMounted(async () => {
  await providerStore.fetchProviders()
  await providerStore.fetchModels()
})
</script>
