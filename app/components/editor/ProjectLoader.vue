<template>
  <UCard>
    <template #header>
      <div class="flex items-center justify-between">
        <h3 class="text-lg font-semibold">Load Project</h3>
        <UBadge v-if="engineStore.hasProject" color="success" variant="soft" size="sm">{{ engineStore.projectName }}</UBadge>
      </div>
    </template>

    <div class="space-y-4">
      <p class="text-sm text-muted">Select an RPG Maker MV/MZ project folder to start translating.</p>
      <div v-if="engineStore.isLoading" class="flex items-center gap-2 text-sm text-muted">
        <UIcon name="i-heroicons-arrow-path" class="w-4 h-4 animate-spin" />
        <span>Extracting project data… This may take a while.</span>
      </div>
      <div class="flex flex-wrap items-center gap-2">
        <UButton
          label="Select Folder"
          icon="i-heroicons-folder-open"
          color="primary"
          :loading="engineStore.isLoading"
          @click="openProjectDialog"
        />
        
      </div>
    </div>

    <template #footer>
      <p class="text-xs text-muted">We only access project data files (e.g., /www/data/*.json). You remain in control.</p>
    </template>
  </UCard>
</template>

<script setup lang="ts">
import { open } from '@tauri-apps/plugin-dialog';
import { useEngineStore } from '~/stores/engine';
import { useAppToast } from '~/composables/useAppToast'

const engineStore = useEngineStore();
const { showToast } = useAppToast()
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
      await engineStore.loadProject(selected as string);
      showToast(
        'Project Loaded',
        `${engineStore.totalTextUnits} units across ${engineStore.gameDataFiles.length} files`,
        'success',
        1600,
        'i-heroicons-check-circle'
      )
    }
  } catch (error) {
    console.error('Error opening project:', error);
    showToast(
      'Failed to Load Project',
      error instanceof Error ? error.message : 'Unexpected error',
      'error',
      1800,
      'i-heroicons-exclamation-triangle'
    )
  }
}

</script> 