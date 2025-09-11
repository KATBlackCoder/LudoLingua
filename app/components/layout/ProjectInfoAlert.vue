<template>
  <UAlert v-if="engineStore.projectInfo" color="info" variant="soft" class="mb-4">
    <template #title>
      <div class="flex items-center">
        <span class="font-bold">{{ engineStore.projectInfo.name }}</span>
        <UBadge color="primary" class="ml-2">{{ engineStore.projectInfo.engine_type }}</UBadge>
        <UBadge v-if="engineStore.projectInfo.version" color="secondary" class="ml-2">v{{ engineStore.projectInfo.version }}</UBadge>
      </div>
    </template>
    <template #description>
      <div class="flex items-center gap-4 text-sm">
        <div>
          <UIcon name="i-lucide-file-text" class="mr-1" />
          <span class="font-medium">Files:</span> {{ engineStore.gameDataFiles.length }}
        </div>
        <div>
          <UIcon name="i-lucide-languages" class="mr-1" />
          <span class="font-medium">Text Units:</span> {{ engineStore.textUnits.length }}
        </div>
      </div>
    </template>
    <template #icon>
      <UIcon name="i-lucide-folder-open" />
    </template>
  </UAlert>
</template>

<script setup lang="ts">
import { useEngineStore } from '~/stores/engine';

const engineStore = useEngineStore();
</script> 