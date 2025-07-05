<template>
  <div>
    <UContainer>
      <UCard class="max-w-3xl mx-auto">
        <template #header>
          <div class="flex items-center justify-between">
            <h2 class="text-xl font-semibold">Welcome to LudoLingua</h2>
          </div>
        </template>
        
        <div class="space-y-6">
          <p>
            LudoLingua is a desktop application for translating RPG Maker game files.
            It helps you manage and translate game text using AI assistance.
          </p>
          
          <ProjectLoader />
          
          <FileExplorer :loading="projectStore.isLoading" />
        </div>
        
        <template #footer>
          <p v-if="!projectStore.projectInfo" class="text-sm text-gray-500">
            To get started, click the "Load Project" button to open an RPG Maker project.
          </p>
        </template>
      </UCard>
    </UContainer>
  </div>
</template>

<script setup lang="ts">
import { useProjectStore } from '../stores/project';
import ProjectLoader from '../components/editor/ProjectLoader.vue';
import FileExplorer from '../components/editor/FileExplorer.vue';

const projectStore = useProjectStore();
</script> 