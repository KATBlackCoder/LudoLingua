<template>
  <div class="space-y-4">
    <UButton
      label="Load Project"
      icon="i-heroicons-folder-open"
      color="primary"
      size="lg"
      :loading="projectStore.isLoading"
      @click="openProjectDialog"
    />
  </div>
</template>

<script setup lang="ts">
import { open } from '@tauri-apps/plugin-dialog';
import { useProjectStore } from '~/stores/project';

const projectStore = useProjectStore();

async function openProjectDialog() {
  try {
    // Open a folder selection dialog
    const selected = await open({
      directory: true,
      multiple: false,
      title: 'Select RPG Maker Project Folder'
    });
    
    if (selected) {
      // If the user selected a folder, load the project
      await projectStore.loadProject(selected as string);
    }
  } catch (error) {
    console.error('Error opening project:', error);
  }
}
</script> 