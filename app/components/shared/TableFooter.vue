<template>
  <div class="flex items-center justify-between px-4 py-3 border-t border-gray-200 dark:border-gray-700">
    <!-- Left side - Stats -->
    <div class="flex items-center gap-4 text-sm text-gray-500 dark:text-gray-400">
      <span v-if="showRowCount">
        {{ rowCountText }}
      </span>
      <span v-if="showSelectedCount && selectedCount > 0" class="flex items-center gap-1">
        <UIcon name="i-lucide-check-square" class="w-4 h-4" />
        {{ selectedCount }} selected
      </span>
      <span v-if="showStats && stats" class="flex items-center gap-2">
        <template v-for="(value, key) in stats" :key="key">
          <UBadge :color="getBadgeColor(value)" variant="soft" size="xs">
            {{ key }}: {{ value }}
          </UBadge>
        </template>
      </span>
    </div>

    <!-- Right side - Page size selector -->
    <div class="flex items-center gap-2">
      <div v-if="showPageSizeSelector" class="flex items-center gap-2">
        <span class="text-sm text-gray-500 dark:text-gray-400">Rows per page:</span>
        <USelect
          :model-value="tableApi?.getState().pagination.pageSize || 25"
          :items="pageSizeOptions"
          size="sm"
          @update:model-value="(size) => tableApi?.setPageSize(size)"
        />
      </div>
      
      <div class="text-sm text-gray-500 dark:text-gray-400">
        {{ paginationInfo }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  // TanStack Table API
  tableApi?: {
    getState: () => { pagination: { pageIndex: number; pageSize: number } }
    setPageIndex: (index: number) => void
    setPageSize: (size: number) => void
    getFilteredRowModel: () => { rows: unknown[] }
  }

  // Stats
  showRowCount?: boolean
  showSelectedCount?: boolean
  selectedCount?: number
  showStats?: boolean
  stats?: Record<string, string | number>

  // Page size selector
  showPageSizeSelector?: boolean
  pageSizeOptions?: Array<{ label: string; value: number }>
}

const props = withDefaults(defineProps<Props>(), {
  tableApi: undefined,
  // Stats
  showRowCount: true,
  showSelectedCount: false,
  selectedCount: 0,
  showStats: false,
  stats: () => ({}),

  // Page size selector
  showPageSizeSelector: false,
  pageSizeOptions: () => [
    { label: '10', value: 10 },
    { label: '25', value: 25 },
    { label: '50', value: 50 },
    { label: '100', value: 100 }
  ]
})

const rowCountText = computed(() => {
  const total = props.tableApi?.getFilteredRowModel().rows.length || 0
  if (total === 0) return 'No items'
  if (total === 1) return '1 item'
  return `${total} items`
})

const paginationInfo = computed(() => {
  const pagination = props.tableApi?.getState().pagination
  const totalRows = props.tableApi?.getFilteredRowModel().rows.length || 0
  
  if (totalRows === 0) return 'No items'
  
  const pageIndex = pagination?.pageIndex || 0
  const pageSize = pagination?.pageSize || 25
  const start = pageIndex * pageSize + 1
  const end = Math.min((pageIndex + 1) * pageSize, totalRows)
  
  return `${start}-${end} of ${totalRows}`
})

const getBadgeColor = (value: string | number): 'primary' | 'secondary' | 'tertiary' | 'info' | 'success' | 'warning' | 'error' | 'neutral' => {
  const stringValue = String(value).toLowerCase()
  
  if (stringValue.includes('success') || stringValue.includes('complete')) {
    return 'success'
  }
  if (stringValue.includes('error') || stringValue.includes('failed')) {
    return 'error'
  }
  if (stringValue.includes('warning') || stringValue.includes('pending')) {
    return 'warning'
  }
  if (stringValue.includes('info') || stringValue.includes('processing')) {
    return 'info'
  }
  
  return 'neutral'
}
</script>
