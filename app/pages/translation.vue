<template>
  <UContainer>
    <div class="max-w-6xl mx-auto">
      <div class="flex items-center justify-between gap-3 mb-3">
        <h2 class="text-xl font-semibold">Translation Workspace</h2>
      </div>
      <!-- Stats uses its own UCard internally -->
      <ProjectStats />
    </div>

    <!-- Translations card -->
    <UCard class="max-w-8xl mx-auto mt-4">
      <template #header>
        <div class="flex items-center gap-3 w-full">
          <h3 class="text-lg font-semibold">Translations</h3>
          <UBadge v-if="failedCount" color="warning" variant="soft">{{ failedCount }} failed</UBadge>
          <div v-if="isBusy" class="flex items-center gap-2 ml-auto">
            <UProgress :value="progressPercent" />
            <span class="text-xs text-muted">{{ translationProgress }}/{{ translationTotal }}</span>
          </div>
          <div v-else-if="isExportingSubset" class="flex items-center gap-2 ml-auto text-muted">
            <UIcon name="i-heroicons-arrow-path" class="w-4 h-4 animate-spin" />
            <span class="text-xs">Exporting minimal copy…</span>
          </div>
        </div>
      </template>

      <div class="flex flex-wrap items-center gap-3 mb-3">
        <UButtonGroup>
          <UButton
            icon="i-heroicons-play"
            :loading="isBusy"
            :disabled="isExportingSubset || !hasNotTranslated"
            @click="startProcess"
          >
            Translate All
          </UButton>
          <UButton
            icon="i-heroicons-folder"
            color="success"
            variant="soft"
            :loading="isExportingSubset"
            :disabled="!hasTranslated || isBusy"
            @click="exportSubset"
          >
            Export data
          </UButton>
          <UButton
            icon="i-heroicons-arrow-path"
            color="neutral"
            :disabled="isBusy || isExportingSubset || !hasTranslated"
            @click="reset"
          >
            Reset
          </UButton>
        </UButtonGroup>

        <!-- Enhanced mode indicators -->
        <div class="flex items-center gap-2 ml-auto">
          <UBadge v-if="hasNotTranslated" color="warning" variant="soft">
            {{ engineStore.textUnits.filter(u => u.status === 'NotTranslated').length }} raw
          </UBadge>
          <UBadge v-if="hasTranslated" color="success" variant="soft">
            {{ translatedItems.length }} translated
          </UBadge>

          <UButtonGroup>
            <UButton
              size="xs"
              variant="soft"
              :color="mode === 'raw' ? 'secondary' : 'neutral'"
              :disabled="!hasNotTranslated && mode !== 'raw'"
              @click="mode = 'raw'"
            >
              Raw
            </UButton>
            <UButton
              size="xs"
              variant="soft"
              :color="mode === 'process' ? 'warning' : 'neutral'"
              :disabled="isBusy"
              @click="mode = 'process'"
            >
              Process
            </UButton>
            <UButton
              size="xs"
              variant="soft"
              :color="mode === 'result' ? 'primary' : 'neutral'"
              :disabled="!hasTranslated && mode !== 'result'"
              @click="mode = 'result'"
            >
              Result
            </UButton>
          </UButtonGroup>
        </div>
      </div>

      <!-- Enhanced content display with mixed state support -->
      <div v-if="mode === 'raw'">
        <TranslationRaw />
        <!-- Show translated items summary in mixed state -->
        <div v-if="hasTranslated" class="mt-4 p-4 bg-primary-50 dark:bg-primary-950 rounded-lg">
          <div class="flex items-center justify-between">
            <span class="text-sm font-medium text-primary-700 dark:text-primary-300">
              {{ translatedItems.length }} items already translated
            </span>
            <UButton size="xs" variant="soft" @click="mode = 'result'">
              View Results
            </UButton>
          </div>
        </div>
      </div>

      <div v-else-if="mode === 'process'">
        <TranslationProcess :rows="processRows" />
      </div>

      <div v-else-if="mode === 'result'">
        <TranslationResult :items="translatedItems" @save="saveEdit" />
        <!-- Show raw items summary in mixed state -->
        <div v-if="hasNotTranslated" class="mt-4 p-4 bg-warning-50 dark:bg-warning-950 rounded-lg">
          <div class="flex items-center justify-between">
            <span class="text-sm font-medium text-warning-700 dark:text-warning-300">
              {{ engineStore.textUnits.filter(u => u.status === 'NotTranslated').length }} items still need translation
            </span>
            <UButton size="xs" variant="soft" @click="mode = 'raw'">
              Continue Translating
            </UButton>
          </div>
        </div>
      </div>

      <!-- Auto-navigation helpers -->
      <div v-if="!hasNotTranslated && hasTranslated && mode !== 'result'" class="mt-4 p-4 bg-success-50 dark:bg-success-950 rounded-lg">
        <div class="flex items-center justify-between">
          <span class="text-sm font-medium text-success-700 dark:text-success-300">
            All items translated! 🎉
          </span>
          <UButton size="xs" color="primary" @click="mode = 'result'">
            View All Results
          </UButton>
        </div>
      </div>
    </UCard>
  </UContainer>
</template>

<script setup lang="ts">
import TranslationProcess from '~/components/translation/TranslationProcess.vue'
import TranslationRaw from '~/components/translation/TranslationRaw.vue'
import TranslationResult from '~/components/translation/TranslationResult.vue'
import { useTranslation } from '~/composables/useTranslation'
import ProjectStats from '~/components/editor/ProjectStats.vue'
import { useEngineStore } from '~/stores/engine'
import { useAppToast } from '~/composables/useAppToast'

const {
  mode,
  processRows,
  translatedItems,
  hasTranslated,
  isBusy,
  translationProgress,
  translationTotal,
  failedCount,
  hasNotTranslated,
  startProcess,
  reset,
  saveEdit,
} = useTranslation()

const { showToast } = useAppToast()

const progressPercent = computed(() => {
  return translationTotal.value ? Math.round((translationProgress.value / translationTotal.value) * 100) : 0
})

const engineStore = useEngineStore()

// removed exportTranslated()

const isExportingSubset = ref(false)

// REMOVED: loadExistingTranslations - now handled automatically by smart loading

async function exportSubset() {
  try {
    // Use fixed location: project/ludolingua/ (next to original project)
    const projectPath = engineStore.projectInfo?.path
    if (!projectPath) {
      throw new Error('No project loaded')
    }
    const fixedExportPath = `${projectPath}/ludolingua`

    isExportingSubset.value = true
    const _exportedPath = await engineStore.exportTranslatedSubset(fixedExportPath)
    showToast(
      'Export Completed',
      `Translations exported successfully to ludolingua/ folder`,
      'success',
      5000,
      'i-heroicons-check-circle'
    )
  } catch (e) {
    const msg = e instanceof Error ? e.message : 'Export failed'
    showToast(
      'Export Failed',
      msg,
      'error',
      7000,
      'i-heroicons-exclamation-triangle'
    )
  } finally {
    isExportingSubset.value = false
  }
}

</script>


