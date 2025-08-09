<template>
  <div>
    <div class="flex items-center justify-between mb-3">
      <h3 class="text-base font-semibold">Raw Text</h3>
      <UButton :disabled="isBusy" icon="i-heroicons-arrow-right" @click="startProcess">Translate All</UButton>
    </div>
    <UTable :data="rows">
      <template #source_text-data="{ row }">
        <span class="whitespace-pre-wrap">{{ row.original.source_text }}</span>
      </template>
      <template #actions-data="{ row }">
        <UButton size="xs" :disabled="isBusy" @click="translateOne(row.original.id)">Translate</UButton>
      </template>
    </UTable>
  </div>
  </template>

<script setup lang="ts">
import { computed } from 'vue'
import { useTranslation } from '~/composables/useTranslation'

const { textUnits, isBusy, startProcess, translateOne } = useTranslation()

const rows = computed(() => textUnits.value.map(u => ({ id: u.id, source_text: u.source_text })))
</script>


