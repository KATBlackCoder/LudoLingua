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
        </div>
      </template>

      <div class="flex flex-wrap items-center gap-3 mb-3">
        <UButtonGroup>
          <UButton icon="i-heroicons-play" :loading="isBusy" @click="startProcess">Translate All</UButton>
          <UButton icon="i-heroicons-document-arrow-down" color="success" :disabled="!canInject || isBusy" @click="inject">Inject</UButton>
          <UButton icon="i-heroicons-arrow-path" color="neutral" :disabled="isBusy || !hasTranslated" @click="reset">Reset</UButton>
          <UButton icon="i-heroicons-document-arrow-up" color="warning" :disabled="!hasTranslated" @click="exportData">Export</UButton>
        </UButtonGroup>

        <UButtonGroup class="ml-auto">
          <UButton size="xs" variant="soft" :color="mode === 'raw' ? 'secondary' : 'neutral'" @click="mode = 'raw'">Raw</UButton>
          <UButton size="xs" variant="soft" :color="mode === 'process' ? 'warning' : 'neutral'" @click="mode = 'process'">Process</UButton>
          <UButton size="xs" variant="soft" :color="mode === 'result' ? 'primary' : 'neutral'" @click="mode = 'result'">Result</UButton>
        </UButtonGroup>
      </div>

      <div v-if="mode === 'raw'">
        <TranslationRaw />
      </div>

      <div v-else-if="mode === 'process'">
        <TranslationProcess :rows="processRows" />
      </div>

      <div v-else-if="mode === 'result'">
        <TranslationResult :items="translatedItems" @save="saveEdit" />
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

const {
  mode,
  processRows,
  translatedItems,
  hasTranslated,
  canInject,
  isBusy,
  translationProgress,
  translationTotal,
  failedCount,
  startProcess,
  inject,
  reset,
  exportData,
  saveEdit,
} = useTranslation()

const progressPercent = computed(() => {
  return translationTotal.value ? Math.round((translationProgress.value / translationTotal.value) * 100) : 0
})
</script>


