<template>
  <div>
    <UContainer>
      <UCard class="max-w-4xl mx-auto">
        <template #header>
          <div class="flex items-center justify-between">
            <h2 class="text-xl font-semibold">Welcome to LudoLingua</h2>
            <UButton
              v-if="engineStore.hasProject"
              label="Load New Project"
              icon="i-heroicons-folder-open"
              color="secondary"
              size="sm"
              :loading="engineStore.isLoading"
              @click="loadNewProject"
            />
          </div>
        </template>
        
        <div class="space-y-6">
          <p class="text-gray-600 dark:text-gray-400">
            LudoLingua is a desktop application for translating RPG Maker game files.
            It helps you manage and translate game text using AI assistance.
          </p>
          
          <!-- Project Loading Section -->
          <div v-if="!engineStore.hasProject">
            <ProjectLoader />
          </div>
          
          <!-- Project Statistics Section -->
          <ProjectStats v-if="engineStore.hasProject" />
          
          <!-- Translation Table Section -->
          <div v-if="engineStore.hasProject">
            
            <TranslationTable />
          </div>
        </div>
        
        <template #footer>
          <p v-if="!engineStore.hasProject" class="text-sm text-gray-500">
            To get started, click the "Load Project" button to open an RPG Maker project.
          </p>
          <p v-else-if="engineStore.totalTextUnits === 0" class="text-sm text-gray-500">
            No translatable text found in this project.
          </p>
          <p v-else class="text-sm text-gray-500">
            Review the translation units above and click "Translate All" to begin translation.
          </p>
        </template>
      </UCard>
    </UContainer>
  </div>
</template>

<script setup lang="ts">
import { open } from '@tauri-apps/plugin-dialog';
import { useEngineStore } from '../stores/engine';
import ProjectLoader from '../components/editor/ProjectLoader.vue';
import ProjectStats from '../components/editor/ProjectStats.vue';
import TranslationTable from '../components/editor/TranslationTable.vue';

const engineStore = useEngineStore();

async function loadNewProject() {
  try {
    // Open a folder selection dialog
    const selected = await open({
      directory: true,
      multiple: false,
      title: 'Select RPG Maker Project Folder'
    });
    
    if (selected) {
      // If the user selected a folder, load the new project
      await engineStore.loadProject(selected as string);
    }
  } catch (error) {
    console.error('Error opening project:', error);
  }
}
</script> 