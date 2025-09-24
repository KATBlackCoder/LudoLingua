<template>
  <div class="space-y-8">
    <!-- Page Header -->
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-3">
        <div class="p-2 bg-primary-50 dark:bg-primary-900/20 rounded-lg">
          <UIcon name="i-lucide-languages" class="text-primary w-6 h-6" />
        </div>
        <div>
          <h1 class="text-2xl font-bold text-gray-900 dark:text-white">Welcome to LudoLingua</h1>
          <p class="text-sm text-gray-500 dark:text-gray-400">Translate your RPG Maker games with AI-powered precision.</p>
        </div>
      </div>
      <div class="flex items-center gap-2">
        <UButton
          v-if="!engineStore.hasProject && !engineStore.isLoading"
          size="lg"
          color="primary"
          icon="i-lucide-folder-open"
          @click="pickProject"
        >
          Load Your Project
        </UButton>
        <div v-else-if="engineStore.hasProject && !engineStore.isLoading" class="flex gap-3">
          <UButton
            size="lg"
            color="primary"
            icon="i-lucide-languages"
            @click="goWorkspace"
          >
            Open Translation Workspace
          </UButton>
          <UButton
            size="lg"
            color="neutral"
            variant="outline"
            icon="i-lucide-folder-open"
            @click="pickProject"
          >
            Change Project
          </UButton>
        </div>
      </div>
    </div>

    <!-- Project Loading Indicator -->
    <UAlert
      v-if="engineStore.isLoading"
      color="info"
      variant="soft"
      title="Loading Project"
      description="Extracting project data and merging with database. This may take a while."
    >
      <template #description>
        <div class="space-y-3">
          <UProgress :value="50" size="lg" />
          <p class="text-xs text-gray-600 dark:text-gray-400">
            Loading text units and checking existing translations...
          </p>
        </div>
      </template>
    </UAlert>

    <!-- Project Status -->
    <div v-if="!engineStore.hasProject">
      <ProjectLoader />
    </div>

    <!-- Project Loaded State -->
    <div v-else class="space-y-6">
      <!-- Project Overview -->
      <UCard>
        <template #header>
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-3">
              <div class="p-2 bg-success-50 dark:bg-success-900/20 rounded-lg">
                <UIcon name="i-lucide-check-circle" class="text-success w-5 h-5" />
              </div>
              <div>
                <h3 class="text-lg font-semibold text-gray-900 dark:text-white">Project Ready</h3>
                <p class="text-sm text-gray-500 dark:text-gray-400">{{ engineStore.projectName }}</p>
              </div>
            </div>
            <div class="flex items-center gap-2">
              <UBadge color="success" variant="soft" size="lg">
                <UIcon name="i-lucide-check" class="w-3 h-3 mr-1" />
                Loaded
              </UBadge>
            </div>
          </div>
        </template>

        <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
          <div class="text-center p-4 bg-primary-50 dark:bg-primary-900/20 rounded-lg">
            <UIcon name="i-lucide-file-text" class="text-primary w-8 h-8 mx-auto mb-2" />
            <p class="text-2xl font-bold text-primary">{{ engineStore.totalTextUnits }}</p>
            <p class="text-sm text-gray-600 dark:text-gray-400">Text Units</p>
          </div>
          <div class="text-center p-4 bg-blue-50 dark:bg-blue-900/20 rounded-lg">
            <UIcon name="i-lucide-files" class="text-blue-500 w-8 h-8 mx-auto mb-2" />
            <p class="text-2xl font-bold text-blue-500">{{ engineStore.gameDataFiles.length }}</p>
            <p class="text-sm text-gray-600 dark:text-gray-400">Game Files</p>
          </div>
          <div class="text-center p-4 bg-purple-50 dark:bg-purple-900/20 rounded-lg">
            <UIcon name="i-lucide-brain" class="text-purple-500 w-8 h-8 mx-auto mb-2" />
            <p class="text-2xl font-bold text-purple-500">{{ settingsStore.userSettings.provider }}</p>
            <p class="text-sm text-gray-600 dark:text-gray-400">AI Provider</p>
          </div>
        </div>
      </UCard>

      <!-- Quick Actions -->
      <UCard>
        <template #header>
          <div class="flex items-center gap-2">
            <UIcon name="i-lucide-zap" class="text-yellow-500 w-5 h-5" />
            <h3 class="text-lg font-semibold text-gray-900 dark:text-white">Quick Actions</h3>
          </div>
        </template>

        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          <div class="p-4 bg-primary-50 dark:bg-primary-900/20 rounded-lg border border-primary-200 dark:border-primary-800">
            <div class="flex items-center gap-3 mb-3">
              <div class="p-2 bg-primary-100 dark:bg-primary-900/30 rounded-lg">
                <UIcon name="i-lucide-languages" class="text-primary w-5 h-5" />
              </div>
              <div>
                <h4 class="font-semibold text-gray-900 dark:text-white">Start Translating</h4>
                <p class="text-xs text-gray-600 dark:text-gray-400">Open Translation Workspace</p>
              </div>
            </div>
            <UButton
              size="lg"
              color="primary"
              :disabled="engineStore.isLoading"
              :loading="engineStore.isLoading"
              class="w-full"
              @click="goWorkspace"
            >
              <UIcon name="i-lucide-languages" class="w-4 h-4 mr-2" />
              Open Workspace
            </UButton>
          </div>

          <div class="p-4 bg-gray-50 dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700">
            <div class="flex items-center gap-3 mb-3">
              <div class="p-2 bg-gray-100 dark:bg-gray-700 rounded-lg">
                <UIcon name="i-lucide-settings" class="text-gray-600 dark:text-gray-400 w-5 h-5" />
              </div>
              <div>
                <h4 class="font-semibold text-gray-900 dark:text-white">Configure Settings</h4>
                <p class="text-xs text-gray-600 dark:text-gray-400">AI & Language Settings</p>
              </div>
            </div>
            <UButton
              size="lg"
              color="neutral"
              variant="outline"
              to="/settings"
              class="w-full"
            >
              <UIcon name="i-lucide-settings" class="w-4 h-4 mr-2" />
              Open Settings
            </UButton>
          </div>

          <div class="p-4 bg-gray-50 dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700">
            <div class="flex items-center gap-3 mb-3">
              <div class="p-2 bg-gray-100 dark:bg-gray-700 rounded-lg">
                <UIcon name="i-lucide-folder-open" class="text-gray-600 dark:text-gray-400 w-5 h-5" />
              </div>
              <div>
                <h4 class="font-semibold text-gray-900 dark:text-white">Change Project</h4>
                <p class="text-xs text-gray-600 dark:text-gray-400">Load Different Project</p>
              </div>
            </div>
            <UButton
              size="lg"
              color="neutral"
              variant="outline"
              class="w-full"
              @click="pickProject"
            >
              <UIcon name="i-lucide-folder-open" class="w-4 h-4 mr-2" />
              Select Project
            </UButton>
          </div>
        </div>
      </UCard>

      <!-- Getting Started Guide -->
      <UCard>
        <template #header>
          <div class="flex items-center gap-2">
            <UIcon name="i-lucide-book-open" class="text-green-500 w-5 h-5" />
            <h3 class="text-lg font-semibold text-gray-900 dark:text-white">Getting Started</h3>
          </div>
        </template>

        <div class="space-y-4">
          <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div class="flex items-start gap-3 p-4 bg-gray-50 dark:bg-gray-800 rounded-lg">
              <div class="flex-shrink-0 w-8 h-8 bg-primary-100 dark:bg-primary-900/20 rounded-full flex items-center justify-center">
                <span class="text-primary font-bold text-sm">1</span>
              </div>
              <div>
                <h4 class="font-semibold text-gray-900 dark:text-white mb-1">Load Project</h4>
                <p class="text-sm text-gray-600 dark:text-gray-400">Your RPG Maker MV/MZ project is already loaded and ready to translate.</p>
              </div>
            </div>

            <div class="flex items-start gap-3 p-4 bg-gray-50 dark:bg-gray-800 rounded-lg">
              <div class="flex-shrink-0 w-8 h-8 bg-blue-100 dark:bg-blue-900/20 rounded-full flex items-center justify-center">
                <span class="text-blue-500 font-bold text-sm">2</span>
              </div>
              <div>
                <h4 class="font-semibold text-gray-900 dark:text-white mb-1">Configure Settings</h4>
                <p class="text-sm text-gray-600 dark:text-gray-400">Set your AI provider, model, and language preferences in Settings.</p>
              </div>
            </div>

            <div class="flex items-start gap-3 p-4 bg-gray-50 dark:bg-gray-800 rounded-lg">
              <div class="flex-shrink-0 w-8 h-8 bg-green-100 dark:bg-green-900/20 rounded-full flex items-center justify-center">
                <span class="text-green-500 font-bold text-sm">3</span>
              </div>
              <div>
                <h4 class="font-semibold text-gray-900 dark:text-white mb-1">Start Translating</h4>
                <p class="text-sm text-gray-600 dark:text-gray-400">Use the Translation Workspace to translate and inject your game text.</p>
              </div>
            </div>
          </div>

          <UAlert
            color="info"
            variant="soft"
            icon="i-lucide-lightbulb"
            title="Pro Tip"
            description="You can change your AI provider and language settings anytime in Settings. The app will remember your preferences for future sessions."
          />
        </div>
      </UCard>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useEngineStore } from '../stores/engine';
import ProjectLoader from '../components/editor/ProjectLoader.vue';
import { open } from '@tauri-apps/plugin-dialog'
import { useSettingsStore } from '~/stores/settings'

const engineStore = useEngineStore();
const settingsStore = useSettingsStore();

async function pickProject() {
  const selected = await open({ directory: true, multiple: false, title: 'Select RPG Maker Project Folder' })
  if (selected) {
    await engineStore.loadProject(selected as string)
  }
}

function goWorkspace() {
  if (engineStore.isLoading || !engineStore.hasProject) return
  navigateTo('/translator')
}

onMounted(async () => {
  try {
    const exists = await settingsStore.hasPersistedUserSettings()
    if (!exists) {
      navigateTo('/settings')
    }
  } catch (e) {
    console.error('Failed to check settings existence', e)
  }
})
</script> 