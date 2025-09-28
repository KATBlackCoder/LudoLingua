<template>
  <div class="space-y-6 w-full">
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

    <!-- Process Table using DataTable -->
    <DataTable
      :data="rows"
      :columns="tableColumns"
      :loading="false"
      title="Translation Status"
      icon="i-lucide-table"
      :show-filters="false"
      :show-search="false"
      :show-pagination="true"
      :show-row-count="true"
      :show-stats="true"
      :stats="computedStats"
      :initial-page-size="25"
      :sticky-headers="true"
      class="w-full"
    >
      <!-- Custom cell slots for status and text content -->
      <template #status-data="{ row }">
        <UBadge :color="statusColor((row.original as ProcessRow).status)" variant="soft" size="sm">
          <UIcon 
            :name="getStatusIconFromUtils((row.original as ProcessRow).status)" 
            class="w-3 h-3 mr-1"
          />
          {{ (row.original as ProcessRow).status }}
        </UBadge>
      </template>
      
      <template #source_text-data="{ row }">
        <div class="max-w-md">
          <span class="whitespace-pre-wrap break-words text-sm">{{ (row.original as ProcessRow).source_text }}</span>
        </div>
      </template>
      
      <template #target_text-data="{ row }">
        <div class="max-w-md">
          <span class="whitespace-pre-wrap break-words text-sm">{{ (row.original as ProcessRow).target_text }}</span>
        </div>
      </template>
    </DataTable>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useTranslator } from '~/composables/useTranslator'
import type { TableColumn } from '#ui/types'
import DataTable from '~/components/shared/DataTable.vue'
import { getBadgeColor, getStatusIcon } from '~/utils/ui'

type ProcessRow = { id: string; source_text: string; target_text: string; status: 'pending' | 'processing' | 'done' | 'error' }

defineProps<{ rows: ProcessRow[]; errors?: Array<{ id: string; message: string }> }>()
const { translationProgress, translationTotal } = useTranslator()

const percentage = computed(() => {
  if (!translationTotal.value) return 0
  return Math.round((translationProgress.value / translationTotal.value) * 100)
})

// Table columns configuration
const tableColumns: TableColumn<unknown>[] = [
  {
    accessorKey: 'status',
    header: 'Status',
    size: 120,
    enableSorting: true
  },
  {
    accessorKey: 'source_text',
    header: 'Source Text',
    size: 300,
    enableSorting: false
  },
  {
    accessorKey: 'target_text',
    header: 'Target Text',
    size: 300,
    enableSorting: false
  }
]


// Stats for DataTable
const computedStats = computed(() => ({
  completed: translationProgress.value,
  total: translationTotal.value,
  remaining: translationTotal.value - translationProgress.value,
  percentage: percentage.value
}))

// Use utility functions for status styling
const statusColor = getBadgeColor
const getStatusIconFromUtils = getStatusIcon
</script>


