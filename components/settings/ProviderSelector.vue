<template>
  <UFormField
    label="Provider"
    name="provider"
    description="Select the LLM provider"
  >
    <USelect
      v-model="selectedProvider"
      :items="providerOptions"
      placeholder="Select a provider"
    />
  </UFormField>
</template>

<script setup lang="ts">
import { useProviderStore } from '~/stores/provider'

const providerStore = useProviderStore()

// Computed options for the select component
const providerOptions = computed(() => 
  providerStore.availableProviders.map(p => ({
    label: p.charAt(0).toUpperCase() + p.slice(1),
    value: p
  }))
)

const selectedProvider = ref<string>(providerStore.selectedProvider)

// Keep selectedProvider in sync with store
watch(
  () => providerStore.selectedProvider,
  (newProvider) => {
    selectedProvider.value = newProvider
  },
  { immediate: true }
)

// When the user selects a provider, update the store
watch(selectedProvider, (providerName) => {
  if (providerName && providerName !== providerStore.selectedProvider) {
    providerStore.setProvider(providerName)
  }
})

// Load providers when component mounts
onMounted(() => {
  providerStore.fetchProviders()
})
</script>

<style>

</style>