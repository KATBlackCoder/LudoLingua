<template>
  <div>
    <UContainer class="max-w-6xl">
      <UCard class="max-w-6xl mx-auto">
        <template #header>
          <div class="flex items-center justify-between gap-2">
            <h2 class="text-xl font-semibold">Welcome to LudoLingua</h2>
          </div>
        </template>

        <div class="space-y-6">
          <UAlert
            v-if="!engineStore.hasProject"
            title="No project loaded"
            description="Select an RPG Maker project folder to begin."
            icon="i-lucide-info"
            color="neutral"
            variant="soft"
          />
          <div v-if="!engineStore.hasProject">
            <ProjectLoader />
          </div>

          <div v-else class="space-y-4">
            <div class="flex flex-wrap items-center gap-2">
              <UButton v-if="engineStore.hasProject" variant="outline" color="neutral" icon="i-lucide-folder-open" @click="pickProject">Change Project</UButton>
              <UButton :disabled="engineStore.isLoading || !engineStore.hasProject" :loading="engineStore.isLoading" icon="i-lucide-languages" @click="goWorkspace">Open Workspace</UButton>
              <UButton to="/settings" variant="outline" icon="i-lucide-settings">Settings</UButton>
              <UBadge color="neutral" variant="soft">{{ engineStore.totalTextUnits }} units • {{ engineStore.gameDataFiles.length }} files</UBadge>
            </div>

            <UCard>
              <template #header>
                <span class="font-medium">Getting Started</span>
              </template>
              <ol class="list-decimal pl-5 space-y-1 text-sm">
                <li>Load your RPG Maker MV/MZ project</li>
                <li>Configure model and languages in Settings</li>
                <li>Use the Translation Workspace to translate and inject</li>
              </ol>
            </UCard>
          </div>
        </div>

        <template #footer>
          <p class="text-sm text-gray-500">
            Tip: You can change provider and languages anytime in Settings.
          </p>
        </template>
      </UCard>
    </UContainer>
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