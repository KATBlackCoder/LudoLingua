<template>
  <UCard>
    <template #header>
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div class="p-2 bg-primary-50 dark:bg-primary-900/20 rounded-lg">
            <UIcon name="i-lucide-folder-open" class="text-primary w-5 h-5" />
          </div>
          <div>
            <h3 class="text-lg font-semibold text-gray-900 dark:text-white">Load Project</h3>
            <p class="text-sm text-gray-500 dark:text-gray-400">Select an RPG Maker project to start translating</p>
          </div>
        </div>
        <UBadge v-if="engineStore.hasProject" color="success" variant="soft" size="lg">
          <UIcon name="i-lucide-check-circle" class="w-3 h-3 mr-1" />
          {{ engineStore.projectName }}
        </UBadge>
      </div>
    </template>

    <div class="space-y-6">
      <!-- Project Status -->
      <div v-if="!engineStore.hasProject" class="text-center py-8">
        <div class="inline-flex items-center justify-center w-16 h-16 bg-gray-100 dark:bg-gray-800 rounded-full mb-4">
          <UIcon name="i-lucide-folder-plus" class="text-gray-500 w-8 h-8" />
        </div>
        <h4 class="text-lg font-semibold text-gray-900 dark:text-white mb-2">No Project Loaded</h4>
        <p class="text-sm text-gray-600 dark:text-gray-400 mb-6">
          Select an RPG Maker MV/MZ project folder to start translating your game.
        </p>
        <UButton
          size="lg"
          color="primary"
          icon="i-lucide-folder-open"
          :loading="engineStore.isLoading"
          @click="openProjectDialog"
        >
          Select Project Folder
        </UButton>
      </div>

      <!-- Loading State -->
      <UAlert
        v-if="engineStore.isLoading"
        color="info"
        variant="soft"
        icon="i-lucide-loader-2"
        title="Loading Project"
        description="Extracting project data and merging with database. This may take a while."
        class="p-4"
      >
        <template #default>
          <div class="space-y-3 mt-3">
            <UProgress :value="50" size="lg" />
            <p class="text-xs text-gray-600 dark:text-gray-400">
              Loading text units and checking existing translations...
            </p>
          </div>
        </template>
      </UAlert>

      <!-- Project Loaded State -->
      <div v-if="engineStore.hasProject && !engineStore.isLoading" class="space-y-4">
        <UCard :ui="{ body: 'p-0' }" class="ring-2 ring-success-200 dark:ring-success-800">
          <div class="p-6">
            <div class="flex items-center gap-3 mb-4">
              <div class="p-2 bg-success-50 dark:bg-success-900/20 rounded-lg">
                <UIcon name="i-lucide-check-circle" class="text-success w-5 h-5" />
              </div>
              <div>
                <h4 class="text-lg font-semibold text-gray-900 dark:text-white">Project Loaded Successfully</h4>
                <p class="text-sm text-gray-600 dark:text-gray-400">{{ engineStore.projectName }}</p>
              </div>
            </div>
            
            <div class="grid grid-cols-2 gap-4 text-sm">
              <div class="space-y-1">
                <p class="text-gray-600 dark:text-gray-400">Text Units</p>
                <p class="text-xl font-bold text-primary">{{ engineStore.totalTextUnits }}</p>
              </div>
              <div class="space-y-1">
                <p class="text-gray-600 dark:text-gray-400">Files</p>
                <p class="text-xl font-bold text-blue-500">{{ engineStore.gameDataFiles.length }}</p>
              </div>
            </div>
          </div>
        </UCard>

        <div class="flex gap-3">
          <UButton
            size="lg"
            color="primary"
            icon="i-lucide-folder-open"
            @click="openProjectDialog"
          >
            Load Different Project
          </UButton>
          <UButton
            size="lg"
            color="neutral"
            variant="outline"
            icon="i-lucide-refresh-cw"
            @click="refreshProject"
          >
            Refresh Project
          </UButton>
        </div>
      </div>

      <!-- Security Notice -->
      <UAlert
        color="neutral"
        variant="soft"
        icon="i-lucide-shield-check"
        title="Your Data is Safe"
        description="We only access project data files (e.g., /www/data/*.json). You remain in control of your project files."
        class="p-4"
      />
    </div>
  </UCard>
</template>

<script setup lang="ts">
import { open } from '@tauri-apps/plugin-dialog';
import { useEngineStore } from '~/stores/engine';
import { useAppToast } from '~/composables/useAppToast'

const engineStore = useEngineStore();
const { showToast } = useAppToast()

const refreshProject = async () => {
  try {
    await engineStore.refreshProject()
    showToast(
      'Project Refreshed',
      'Project data has been updated successfully',
      'success',
      2000,
      'i-lucide-refresh-cw'
    )
  } catch (error) {
    console.error('Error refreshing project:', error);
    showToast(
      'Failed to Refresh Project',
      error instanceof Error ? error.message : 'Unexpected error',
      'error',
      1800,
      'i-lucide-triangle-alert'
    )
  }
}

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
        `${engineStore.totalTextUnits} units across ${engineStore.gameDataFiles.length} files - Database merged successfully`,
        'success',
        2000,
        'i-lucide-server'
      )
    }
  } catch (error) {
    console.error('Error opening project:', error);
    showToast(
      'Failed to Load Project',
      error instanceof Error ? error.message : 'Unexpected error',
      'error',
      1800,
      'i-lucide-triangle-alert'
    )
  }
}

</script> 