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
      <div class="flex flex-wrap items-center gap-2">
        <UButton
          label="Select Folder"
          icon="i-heroicons-folder-open"
          color="primary"
          :loading="engineStore.isLoading"
          @click="openProjectDialog"
        />
        <UButton
          v-if="engineStore.hasProject"
          variant="outline"
          color="neutral"
          icon="i-heroicons-language"
          @click="goWorkspace"
        >Open Workspace</UButton>
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
import { useRouter } from 'vue-router'

const engineStore = useEngineStore();
const router = useRouter()
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
    }
  } catch (error) {
    console.error('Error opening project:', error);
  }
}

function goWorkspace() {
  router.push('/translation')
}
</script> 