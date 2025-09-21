<template>
  <div class="space-y-4">
    <div class="flex items-center justify-between gap-3">
      <h3 class="text-lg font-semibold">Translation Progress</h3>
    </div>
    <UProgress :value="percentage" :max="100" class="h-2.5" />
    <UTable :data="pagedRows" class="text-base">
      <template #status-data="{ row }">
        <UBadge :color="statusColor(row.original.status)">{{ row.original.status }}</UBadge>
      </template>
      <template #source_text-data="{ row }">
        <span class="whitespace-pre-wrap">{{ row.original.source_text }}</span>
      </template>
      <template #target_text-data="{ row }">
        <span class="whitespace-pre-wrap">{{ row.original.target_text }}</span>
      </template>
    </UTable>
    <div v-if="errors?.length" class="text-sm text-error-600 dark:text-error-400">
      {{ errors.length }} error(s). Check console for details.
    </div>
    <div class="flex items-center justify-between">
      <span class="text-xs text-muted">Page {{ page }} / {{ pageCount }}</span>
      <UPagination v-model:page="page" :total="viewRows.length" :items-per-page="pageSize" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { useTranslator } from '~/composables/useTranslator'
type ProcessRow = { id: string; source_text: string; target_text: string; status: 'pending' | 'processing' | 'done' | 'error' }

const props = defineProps<{ rows: ProcessRow[]; errors?: Array<{ id: string; message: string }> }>()
const { translationProgress, translationTotal } = useTranslator()

const percentage = computed(() => {
  if (!translationTotal.value) return 0
  return Math.round((translationProgress.value / translationTotal.value) * 100)
})

// Hide internal id by projecting rows to only displayed fields
const viewRows = computed(() => props.rows.map(r => ({
  status: r.status,
  source_text: r.source_text,
  target_text: r.target_text
})))

const page = ref(1)
const pageSize = ref(25)
const pageCount = computed(() => Math.max(1, Math.ceil(viewRows.value.length / pageSize.value)))
const pagedRows = computed(() => {
  const start = (page.value - 1) * pageSize.value
  return viewRows.value.slice(start, start + pageSize.value)
})

function statusColor(status: ProcessRow['status']) {
  switch (status) {
    case 'processing':
      return 'primary'
    case 'done':
      return 'success'
    case 'error':
      return 'error'
    default:
      return 'neutral'
  }
}
</script>


