<template>
  <div class="space-y-6">
    <!-- Progress Header -->
    <UCard>
      <template #header>
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <div class="p-2 bg-primary-50 dark:bg-primary-900/20 rounded-lg">
              <UIcon name="i-lucide-loader-2" class="text-primary w-5 h-5 animate-spin" />
            </div>
            <div>
              <h3 class="text-lg font-semibold text-gray-900 dark:text-white">Translation Progress</h3>
              <p class="text-sm text-gray-500 dark:text-gray-400">AI is processing your translations</p>
            </div>
          </div>
          <div class="flex items-center gap-2">
            <UBadge color="primary" variant="soft" size="sm">
              {{ percentage }}% complete
            </UBadge>
          </div>
        </div>
      </template>

      <div class="space-y-4">
        <!-- Progress Bar -->
        <div class="space-y-2">
          <div class="flex items-center justify-between text-sm">
            <span class="text-gray-600 dark:text-gray-400">Translation Progress</span>
            <span class="font-medium text-gray-900 dark:text-white">{{ percentage }}%</span>
          </div>
          <UProgress :value="percentage" :max="100" size="lg" color="primary" />
        </div>

        <!-- Progress Stats -->
        <div class="grid grid-cols-3 gap-4">
          <div class="text-center p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
            <div class="text-2xl font-bold text-gray-900 dark:text-white">{{ translationProgress }}</div>
            <div class="text-xs text-gray-500 dark:text-gray-400">Completed</div>
          </div>
          <div class="text-center p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
            <div class="text-2xl font-bold text-gray-900 dark:text-white">{{ translationTotal }}</div>
            <div class="text-xs text-gray-500 dark:text-gray-400">Total</div>
          </div>
          <div class="text-center p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
            <div class="text-2xl font-bold text-gray-900 dark:text-white">{{ translationTotal - translationProgress }}</div>
            <div class="text-xs text-gray-500 dark:text-gray-400">Remaining</div>
          </div>
        </div>
      </div>
    </UCard>

    <!-- Error Alert -->
    <UAlert
      v-if="errors?.length"
      color="error"
      variant="soft"
      icon="i-lucide-alert-triangle"
      :title="`${errors.length} error(s) occurred`"
      description="Check console for details"
      class="p-4"
    />

    <!-- Process Table -->
    <UCard>
      <template #header>
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <UIcon name="i-lucide-table" class="text-gray-500 w-4 h-4" />
            <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Translation Status</span>
          </div>
          <div class="flex items-center gap-2">
            <span class="text-xs text-gray-500 dark:text-gray-400">
              Page {{ page }} of {{ pageCount }}
            </span>
            <UPagination 
              v-model:page="page" 
              :total="viewRows.length" 
              :items-per-page="pageSize"
              size="sm"
            />
          </div>
        </div>
      </template>

      <UTable :data="pagedRows" class="text-sm">
        <template #status-data="{ row }">
          <UBadge :color="statusColor(row.original.status)" variant="soft" size="sm">
            <UIcon 
              :name="getStatusIcon(row.original.status)" 
              class="w-3 h-3 mr-1"
            />
            {{ row.original.status }}
          </UBadge>
        </template>
        <template #source_text-data="{ row }">
          <div class="max-w-md">
            <span class="whitespace-pre-wrap break-words text-sm">{{ row.original.source_text }}</span>
          </div>
        </template>
        <template #target_text-data="{ row }">
          <div class="max-w-md">
            <span class="whitespace-pre-wrap break-words text-sm">{{ row.original.target_text }}</span>
          </div>
        </template>
      </UTable>

      <template #footer>
        <div class="flex items-center justify-between text-xs text-gray-500 dark:text-gray-400">
          <span>
            Showing {{ pagedRows.length }} of {{ viewRows.length }} items
          </span>
          <span>
            Real-time translation progress
          </span>
        </div>
      </template>
    </UCard>
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

function getStatusIcon(status: ProcessRow['status']) {
  switch (status) {
    case 'processing':
      return 'i-lucide-loader-2'
    case 'done':
      return 'i-lucide-check-circle'
    case 'error':
      return 'i-lucide-x-circle'
    default:
      return 'i-lucide-clock'
  }
}
</script>


