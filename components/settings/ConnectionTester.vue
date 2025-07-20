<template>
  <div class="space-y-4">
    <div class="flex justify-between items-center">
      <UButton
        :loading="providerStore.isLoading"
        variant="outline"
        @click="testConnection"
      >
        Test Connection
      </UButton>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useProviderStore } from '~/stores/provider';

// Use provider store for connection testing
const providerStore = useProviderStore();

// Methods
const testConnection = async () => {
  await providerStore.testConnection();
};
</script> 