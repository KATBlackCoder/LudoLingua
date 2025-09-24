<template>
  <div class="space-y-6">
    <!-- Page Header -->
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-3">
        <div class="p-2 bg-primary-50 dark:bg-primary-900/20 rounded-lg">
          <UIcon name="i-lucide-languages" class="text-primary w-5 h-5" />
        </div>
        <div>
          <h1 class="text-2xl font-bold text-gray-900 dark:text-white">Translation Workspace</h1>
          <p class="text-sm text-gray-500 dark:text-gray-400">Translate your RPG Maker game with AI assistance</p>
        </div>
      </div>
      <div class="flex items-center gap-2">
        <UButton
          variant="outline"
          icon="i-lucide-arrow-left"
          to="/"
        >
          Back to Home
        </UButton>
      </div>
    </div>

    <!-- Project Stats -->
    <ProjectStats />

    <!-- Translation Controls -->
    <UCard>
      <template #header>
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <div class="p-2 bg-blue-50 dark:bg-blue-900/20 rounded-lg">
              <UIcon name="i-lucide-play" class="text-blue-500 w-5 h-5" />
            </div>
            <div>
              <h3 class="text-lg font-semibold text-gray-900 dark:text-white">Translation Controls</h3>
              <p class="text-sm text-gray-500 dark:text-gray-400">Manage your translation workflow</p>
            </div>
          </div>
          <div class="flex items-center gap-2">
            <UBadge v-if="failedCount" color="warning" variant="soft" size="lg">
              <UIcon name="i-lucide-alert-triangle" class="w-3 h-3 mr-1" />
              {{ failedCount }} failed
            </UBadge>
            <UBadge v-if="hasNotTranslated" color="warning" variant="soft" size="lg">
              {{ engineStore.textUnits.filter(u => u.status === 'NotTranslated').length }} raw
            </UBadge>
            <UBadge v-if="hasTranslated" color="success" variant="soft" size="lg">
              {{ translatedItems.length }} translated
            </UBadge>
          </div>
        </div>
      </template>

      <!-- Progress Indicator -->
      <div v-if="isBusy" class="mb-6">
        <UAlert
          color="info"
          variant="soft"
          icon="i-lucide-loader-2"
          title="Translation in Progress"
          :description="`Processing ${translationProgress} of ${translationTotal} items`"
          class="p-4"
        >
          <template #default>
            <div class="space-y-3 mt-3">
              <UProgress :value="progressPercent" size="lg" />
            </div>
          </template>
        </UAlert>
      </div>

      <!-- Export Progress -->
      <div v-if="isExportingSubset" class="mb-6">
        <UAlert
          color="success"
          variant="soft"
          icon="i-lucide-loader-2"
          title="Exporting Translations"
          description="Creating minimal copy of your project with translations..."
          class="p-4"
        />
      </div>

      <!-- Action Buttons -->
      <div class="grid grid-cols-1 gap-4 mb-6" :class="hasNotTranslated ? 'md:grid-cols-3' : 'md:grid-cols-2'">
        <div v-if="hasNotTranslated" class="p-4 bg-primary-50 dark:bg-primary-900/20 rounded-lg border border-primary-200 dark:border-primary-800">
          <div class="flex items-center gap-3 mb-3">
            <div class="p-2 bg-primary-100 dark:bg-primary-900/30 rounded-lg">
              <UIcon name="i-lucide-play" class="text-primary w-5 h-5" />
            </div>
            <div>
              <h4 class="font-semibold text-gray-900 dark:text-white">Start Translation</h4>
              <p class="text-xs text-gray-600 dark:text-gray-400">Begin AI translation process</p>
            </div>
          </div>
          <UButton
            size="lg"
            color="primary"
            :loading="isBusy"
            :disabled="isExportingSubset || !hasNotTranslated"
            class="w-full"
            @click="startProcess"
          >
            <UIcon name="i-lucide-play" class="w-4 h-4 mr-2" />
            Translate All
          </UButton>
        </div>

        <div class="p-4 bg-green-50 dark:bg-green-900/20 rounded-lg border border-green-200 dark:border-green-800">
          <div class="flex items-center gap-3 mb-3">
            <div class="p-2 bg-green-100 dark:bg-green-900/30 rounded-lg">
              <UIcon name="i-lucide-folder" class="text-green-500 w-5 h-5" />
            </div>
            <div>
              <h4 class="font-semibold text-gray-900 dark:text-white">Export Translations</h4>
              <p class="text-xs text-gray-600 dark:text-gray-400">Save to project files</p>
            </div>
          </div>
          <UButton
            size="lg"
            color="success"
            variant="outline"
            :loading="isExportingSubset"
            :disabled="!hasTranslated || isBusy"
            class="w-full"
            @click="exportSubset"
          >
            <UIcon name="i-lucide-folder" class="w-4 h-4 mr-2" />
            Export Data
          </UButton>
        </div>

        <div class="p-4 bg-gray-50 dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700">
          <div class="flex items-center gap-3 mb-3">
            <div class="p-2 bg-gray-100 dark:bg-gray-700 rounded-lg">
              <UIcon name="i-lucide-refresh-cw" class="text-gray-600 dark:text-gray-400 w-5 h-5" />
            </div>
            <div>
              <h4 class="font-semibold text-gray-900 dark:text-white">Reset All</h4>
              <p class="text-xs text-gray-600 dark:text-gray-400">Clear all translations</p>
            </div>
          </div>
          <UButton
            size="lg"
            color="neutral"
            variant="outline"
            :disabled="isBusy || isExportingSubset || !hasTranslated"
            class="w-full"
            @click="reset"
          >
            <UIcon name="i-lucide-refresh-cw" class="w-4 h-4 mr-2" />
            Reset
          </UButton>
        </div>
      </div>

      <!-- Mode Selector -->
      <div class="flex items-center justify-between mb-6">
        <h4 class="text-sm font-semibold text-gray-900 dark:text-white">View Mode</h4>
        <UFieldGroup>
          <UButton
            size="sm"
            variant="soft"
            :color="mode === 'raw' ? 'primary' : 'neutral'"
            :disabled="!hasNotTranslated && mode !== 'raw'"
            @click="mode = 'raw'"
          >
            <UIcon name="i-lucide-file-text" class="w-4 h-4 mr-2" />
            Raw Text
          </UButton>
          <UButton
            size="sm"
            variant="soft"
            :color="mode === 'process' ? 'warning' : 'neutral'"
            :disabled="isBusy"
            @click="mode = 'process'"
          >
            <UIcon name="i-lucide-cog" class="w-4 h-4 mr-2" />
            Processing
          </UButton>
          <UButton
            size="sm"
            variant="soft"
            :color="mode === 'result' ? 'success' : 'neutral'"
            :disabled="!hasTranslated && mode !== 'result'"
            @click="mode = 'result'"
          >
            <UIcon name="i-lucide-check-circle" class="w-4 h-4 mr-2" />
            Results
          </UButton>
        </UFieldGroup>
      </div>

      <!-- Content Display -->
      <div v-if="mode === 'raw'">
        <TranslationRaw />
        <!-- Mixed state notification -->
        <UAlert
          v-if="hasTranslated"
          color="info"
          variant="soft"
          icon="i-lucide-info"
          title="Partial Translation Complete"
          :description="`${translatedItems.length} items already translated`"
          class="mt-4"
        >
          <template #actions>
            <UButton size="sm" color="primary" @click="mode = 'result'">
              View Results
            </UButton>
          </template>
        </UAlert>
      </div>

      <div v-else-if="mode === 'process'">
        <TranslationProcess :rows="processRows" />
      </div>

      <div v-else-if="mode === 'result'">
        <TranslationResult :items="translatedItems" @save="saveEdit" @retranslate-selected="handleBulkRetranslation" />
        <!-- Mixed state notification -->
        <UAlert
          v-if="hasNotTranslated"
          color="warning"
          variant="soft"
          icon="i-lucide-clock"
          title="Translation Incomplete"
          :description="`${engineStore.textUnits.filter(u => u.status === 'NotTranslated').length} items still need translation`"
          class="mt-4"
        >
          <template #actions>
            <UButton size="sm" color="warning" @click="mode = 'raw'">
              Continue Translating
            </UButton>
          </template>
        </UAlert>
      </div>

      <!-- Completion notification -->
      <UAlert
        v-if="!hasNotTranslated && hasTranslated && mode !== 'result' && !isBusy"
        color="success"
        variant="soft"
        icon="i-lucide-check-circle"
        title="Translation Complete! 🎉"
        description="All items have been successfully translated."
        class="mt-4"
      >
        <template #actions>
          <UButton size="sm" color="success" @click="mode = 'result'">
            View All Results
          </UButton>
        </template>
      </UAlert>
    </UCard>
  </div>
</template>

<script setup lang="ts">
import TranslationProcess from '~/components/translator/TranslationProcess.vue'
import TranslationRaw from '~/components/translator/TranslationRaw.vue'
import TranslationResult from '~/components/translator/TranslationResult.vue'
import { useTranslator } from '~/composables/useTranslator'
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
  startBulkRetranslation,
  reset,
  saveEdit,
} = useTranslator()

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
      'i-lucide-check-circle'
    )
  } catch (e) {
    const msg = e instanceof Error ? e.message : 'Export failed'
    showToast(
      'Export Failed',
      msg,
      'error',
      7000,
      'i-lucide-triangle-alert'
    )
  } finally {
    isExportingSubset.value = false
  }
}

// Handle bulk retranslation from selected rows
async function handleBulkRetranslation(selectedRows: { id: string; source_text: string; translated_text: string; prompt_type: string; field_type: string }[]) {
  await startBulkRetranslation(selectedRows)
}

if (!engineStore.hasProject) {
  navigateTo('/')
}
</script>


