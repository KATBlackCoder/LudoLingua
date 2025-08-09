<template>
  <div class="space-y-3">
    <div class="flex items-center justify-between">
      <h3 class="text-base font-semibold">Translation Progress</h3>
      <span class="text-sm text-gray-600 dark:text-gray-400">{{ translationProgress }} / {{ translationTotal }}</span>
    </div>
    <UProgress :value="percentage" :max="100" />
    <UTable :data="viewRows">
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
    <div v-if="errors?.length" class="text-sm text-red-600 dark:text-red-400">
      {{ errors.length }} error(s). Check console for details.
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useTranslation } from '~/composables/useTranslation'
type ProcessRow = { id: string; source_text: string; target_text: string; status: 'pending' | 'processing' | 'done' | 'error' }

const props = defineProps<{ rows: ProcessRow[]; errors?: Array<{ id: string; message: string }> }>()
const { translationProgress, translationTotal } = useTranslation()

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


