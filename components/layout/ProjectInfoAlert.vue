<template>
  <UAlert v-if="projectStore.projectInfo" color="info" variant="soft" class="mb-4">
    <template #title>
      <div class="flex items-center">
        <span class="font-bold">{{ projectStore.projectInfo.name }}</span>
        <UBadge color="primary" class="ml-2">{{ projectStore.projectInfo.engine_type }}</UBadge>
        <UBadge v-if="projectStore.projectInfo.version" color="secondary" class="ml-2">v{{ projectStore.projectInfo.version }}</UBadge>
      </div>
    </template>
    <template #description>
      <div class="flex items-center gap-4 text-sm">
        <div>
          <UIcon name="i-heroicons-document-text" class="mr-1" />
          <span class="font-medium">Files:</span> {{ projectStore.gameDataFiles.length }}
        </div>
        <div>
          <UIcon name="i-heroicons-language" class="mr-1" />
          <span class="font-medium">Text Units:</span> {{ projectStore.textUnits.length }}
        </div>
      </div>
    </template>
    <template #icon>
      <UIcon name="i-heroicons-folder-open" />
    </template>
  </UAlert>
</template>

<script setup lang="ts">
import { useProjectStore } from '~/stores/project';

const projectStore = useProjectStore();
</script> 