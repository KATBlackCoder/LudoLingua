<template>
  <UContainer>
    <UCard class="max-w-6xl mx-auto">
      <template #header>
        <div class="flex items-center justify-between">
          <h2 class="text-xl font-semibold">Translation Workspace</h2>
        </div>
      </template>

      <div class="flex flex-wrap gap-2 mb-4">
        <UButton icon="i-heroicons-play" :loading="isBusy" @click="startProcess">Translate All</UButton>
        <UButton icon="i-heroicons-document-arrow-down" color="success" :disabled="!canInject || isBusy" @click="inject">Inject</UButton>
        <UButton icon="i-heroicons-arrow-path" color="neutral" :disabled="isBusy || !hasTranslated" @click="reset">Reset</UButton>
        <UButton icon="i-heroicons-document-arrow-up" color="warning" :disabled="!hasTranslated" @click="exportData">Export</UButton>
      </div>

      <div v-if="mode === 'raw'" class="mt-2">
        <TranslationRaw />
      </div>

      <div v-else-if="mode === 'process'" class="mt-2">
        <TranslationProcess :rows="processRows" />
      </div>

      <div v-else-if="mode === 'result'" class="mt-2">
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

const {
  mode,
  processRows,
  translatedItems,
  hasTranslated,
  canInject,
  isBusy,
  startProcess,
  inject,
  reset,
  exportData,
  saveEdit,
} = useTranslation()
</script>


