<template>
  <div class="space-y-3">
    <div class="flex items-center justify-between">
      <h3 class="text-lg font-semibold">Raw Text</h3>
    </div>
    <UTable :data="pagedRows" class="text-base">
      <template #prompt_type-data="{ row }">
        <UBadge variant="soft">{{ row.original.prompt_type }}</UBadge>
      </template>
      <template #source_text-data="{ row }">
        <span class="whitespace-pre-wrap">{{ row.original.source_text }}</span>
      </template>
    </UTable>
    <div class="flex items-center justify-between">
      <span class="text-xs text-muted">Page {{ page }} / {{ pageCount }}</span>
      <UPagination v-model:page="page" :total="rows.length" :items-per-page="pageSize" />
    </div>
  </div>
  </template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { useTranslation } from '~/composables/useTranslation'

const { textUnits } = useTranslation()

const rows = computed(() => textUnits.value.map(u => ({ id: u.id, prompt_type: u.prompt_type, source_text: u.source_text })))
const page = ref(1)
const pageSize = ref(25)
const pageCount = computed(() => Math.max(1, Math.ceil(rows.value.length / pageSize.value)))
const pagedRows = computed(() => {
  const start = (page.value - 1) * pageSize.value
  return rows.value.slice(start, start + pageSize.value)
})
</script>


