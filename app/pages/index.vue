<template>
  <div>
    <UContainer>
      <UCard class="max-w-4xl mx-auto">
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
            icon="i-heroicons-information-circle"
            color="neutral"
            variant="soft"
          />

          <div v-if="!engineStore.hasProject">
            <ProjectLoader />
          </div>

          <div v-else class="space-y-4">
            <div class="flex flex-wrap items-center gap-2">
              <UButton v-if="engineStore.hasProject" variant="outline" color="neutral" icon="i-heroicons-folder-open" @click="pickProject">Change Project</UButton>
              <UButton to="/translation" icon="i-heroicons-language">Open Workspace</UButton>
              <UButton to="/settings" variant="outline" icon="i-heroicons-cog-6-tooth">Settings</UButton>
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

const engineStore = useEngineStore();

async function pickProject() {
  const selected = await open({ directory: true, multiple: false, title: 'Select RPG Maker Project Folder' })
  if (selected) {
    await engineStore.loadProject(selected as string)
  }
}
</script> 