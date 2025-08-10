<template>
  <div class="flex flex-col gap-3">
    <UFormField label="Model" name="model" description="Model to use for translation">
      <USelect v-model="selectedModelName" :items="modelOptions" placeholder="Select a model" class="w-64" />
    </UFormField>

    <div v-if="selectedDisplay" class="text-xs text-muted">
      Selected: <UBadge color="neutral" variant="soft">{{ selectedDisplay }}</UBadge>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useProviderStore } from '~/stores/provider'

const providerStore = useProviderStore()

// Computed options for the select component
const modelOptions = computed(() =>
  providerStore.availableModels.map(m => ({
    label: m.display_name,
    value: m.model_name
  }))
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

// When the user selects a model, update the store
watch(selectedModelName, (modelName) => {
  if (modelName) {
    const model = providerStore.availableModels.find(m => m.model_name === modelName)
    if (model && model !== providerStore.selectedModel) {
      providerStore.setModel(model)
    }
  }
})

// Load models when component mounts
onMounted(() => {
  providerStore.fetchModels()
})
</script> 