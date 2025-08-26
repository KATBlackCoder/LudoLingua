<template>
  <div class="flex flex-col gap-4">
    <div class="flex items-center gap-4">
      <UFormField label="Provider" name="provider" description="Choose an LLM provider">
        <USelect
          v-model="selectedProvider"
          :items="providerOptions"
          placeholder="Select a provider"
          class="w-64"
          @update:model-value="onProviderChange"
        />
      </UFormField>
      <UFormField label="Model" name="model" description="Model to use for translation">
        <USelect
          v-model="selectedModelName"
          :items="modelOptions"
          placeholder="Select a model"
          class="w-64"
        />
      </UFormField>
    </div>

    <div v-if="selectedDisplay" class="text-xs text-muted">
      Selected: <UBadge color="neutral" variant="soft">{{ selectedDisplay }}</UBadge>
    </div>
  </div>
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
