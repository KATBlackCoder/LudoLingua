<template>
  <UCard v-if="gameDataFiles.length > 0">
    <template #header>
      <div class="flex items-center justify-between">
        <h3 class="text-lg font-semibold">Game Files</h3>
      </div>
    </template>
    
    <div class="flex">
      <!-- File selection sidebar -->
      <div class="w-1/4 border-r pr-4">
        <UButton
          v-for="file in gameDataFiles"
          :key="file.path"
          :label="file.name"
          :color="file === currentFile ? 'primary' : 'neutral'"
          variant="ghost"
          block
          class="mb-2 justify-start"
          @click="setCurrentFile(file)"
        >
          <template #leading>
            <UIcon name="i-heroicons-document-text" class="mr-1" />
          </template>
          <template #trailing>
            <UBadge size="xs">{{ file.text_unit_count }}</UBadge>
          </template>
        </UButton>
      </div>
      
      <!-- Translation table for selected file -->
      <div class="w-3/4 pl-4">
        <div v-if="!currentFile" class="py-10 text-center text-gray-500">
          Select a file to view its translatable text
        </div>
        <TranslationTable 
          v-else
          :game-data-file="currentFile"
          :loading="loading"
        />
      </div>
    </div>
  </UCard>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useProjectStore } from '~/stores/project';
import type { GameDataFile } from '~/types/engine';
import TranslationTable from './TranslationTable.vue';

defineProps<{
  loading?: boolean;
}>();

const projectStore = useProjectStore();

// Computed properties to access project store data
const gameDataFiles = computed(() => projectStore.gameDataFiles);
const currentFile = computed(() => projectStore.currentFile);

// Method to set the current file
function setCurrentFile(file: GameDataFile) {
  projectStore.setCurrentFile(file);
}
</script> 