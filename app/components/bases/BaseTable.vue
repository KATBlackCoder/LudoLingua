<template>
  <div class="space-y-6">
    <!-- Header Section -->
    <UCard v-if="showHeader">
      <template #header>
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <div class="p-2 bg-primary-50 dark:bg-primary-900/20 rounded-lg">
              <UIcon :name="headerIcon" class="text-primary w-5 h-5" :class="{ 'animate-spin': isProcessing }" />
            </div>
            <div>
              <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
                {{ headerTitle }}
              </h3>
              <p class="text-sm text-gray-500 dark:text-gray-400">
                {{ headerDescription }}
              </p>
            </div>
          </div>
          <div class="flex items-center gap-2">
            <UBadge
              :color="statusColor"
              variant="soft"
              size="sm"
            >
              {{ statusText }}
            </UBadge>
            <UBadge
              v-if="showProgress"
              color="primary"
              variant="soft"
              size="sm"
            >
              {{ progressPercentage }}% complete
            </UBadge>
            
            <!-- Display Mode Toggle -->
            <UButton
              v-if="showDisplayModeToggle"
              :variant="displayMode === 'all' ? 'solid' : 'soft'"
              color="primary"
              size="sm"
              :icon="displayMode === 'all' ? 'i-lucide-list' : 'i-lucide-grid-3x3'"
              @click="toggleDisplayMode"
            >
              {{ displayMode === 'all' ? 'All Data' : 'Paginated' }}
            </UButton>
            
            <!-- Export Button -->
            <UButton
              v-if="exportable"
              color="secondary"
              variant="soft"
              size="sm"
              icon="i-lucide-download"
              :loading="isExporting"
              @click="() => exportData()"
            >
              Export
            </UButton>
            
            <!-- Reset Button -->
            <UButton
              v-if="showResetButton"
              color="neutral"
              variant="soft"
              size="sm"
              icon="i-lucide-refresh-cw"
              @click="enhancedReset"
            >
              Reset
            </UButton>
          </div>
        </div>
      </template>

      <!-- Filters Section -->
      <div v-if="showFilters" class="space-y-4">
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
          <!-- Search Filter -->
          <div v-if="searchable" class="space-y-2">
            <label class="text-sm font-medium text-gray-700 dark:text-gray-300">
              Search
            </label>
            <UInput
              v-model="search"
              icon="i-lucide-search"
              :placeholder="searchPlaceholder"
              size="sm"
            />
          </div>

          <!-- Custom Filters Slot -->
          <slot name="filters" />

          <!-- Domain-specific Filters -->
          <div v-if="showDomainFilters" class="space-y-4">
            <!-- Status Filter -->
            <div v-if="showStatusFilter" class="space-y-2">
              <label class="text-sm font-medium text-gray-700 dark:text-gray-300">
                Status
              </label>
              <USelect
                v-model="statusFilter"
                :options="statusFilterOptions"
                placeholder="All statuses"
                size="sm"
                @change="(event) => onStatusFilterChange((event.target as HTMLSelectElement).value)"
              />
            </div>

            <!-- Prompt Type Filter -->
            <div v-if="showPromptTypeFilter" class="space-y-2">
              <label class="text-sm font-medium text-gray-700 dark:text-gray-300">
                Prompt Type
              </label>
              <USelect
                v-model="promptTypeFilter"
                :options="promptTypeFilterOptions"
                placeholder="All types"
                size="sm"
                @change="(event) => onPromptTypeFilterChange((event.target as HTMLSelectElement).value)"
              />
            </div>

            <!-- Field Type Filter -->
            <div v-if="showFieldTypeFilter" class="space-y-2">
              <label class="text-sm font-medium text-gray-700 dark:text-gray-300">
                Field Type
              </label>
              <UInput
                v-model="fieldTypeFilter"
                placeholder="Filter by field type..."
                size="sm"
                @input="onFieldTypeFilterChange"
              />
            </div>
          </div>

          <!-- Text Length Filter -->
          <div v-if="showTextLengthFilter" class="space-y-2">
            <label class="text-sm font-medium text-gray-700 dark:text-gray-300">
              Text Length: {{ textLengthRange[0] }}-{{ textLengthRange[1] }} chars
            </label>
            <USlider
              v-model="textLengthRange"
              :min="0"
              :max="maxTextLength"
              :step="5"
              size="sm"
            />
          </div>
        </div>
      </div>

      <!-- Statistics Section -->
      <div v-if="showStatistics" class="space-y-4">
        <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
          <div class="text-center p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
            <div class="text-2xl font-bold text-gray-900 dark:text-white">{{ statistics.totalItems }}</div>
            <div class="text-xs text-gray-500 dark:text-gray-400">Total Items</div>
          </div>
          <div class="text-center p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
            <div class="text-2xl font-bold text-gray-900 dark:text-white">{{ statistics.filteredItems }}</div>
            <div class="text-xs text-gray-500 dark:text-gray-400">Filtered</div>
          </div>
          <div class="text-center p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
            <div class="text-2xl font-bold text-gray-900 dark:text-white">{{ statistics.selectedItems }}</div>
            <div class="text-xs text-gray-500 dark:text-gray-400">Selected</div>
          </div>
          <div class="text-center p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
            <div class="text-2xl font-bold text-gray-900 dark:text-white">{{ statistics.pageCount }}</div>
            <div class="text-xs text-gray-500 dark:text-gray-400">Pages</div>
          </div>
        </div>
      </div>

      <!-- Progress Section -->
      <div v-if="showProgress" class="space-y-4">
        <!-- Progress Bar -->
        <div class="space-y-2">
          <div class="flex items-center justify-between text-sm">
            <span class="text-gray-600 dark:text-gray-400">Progress</span>
            <span class="font-medium text-gray-900 dark:text-white">{{ progressPercentage }}%</span>
          </div>
          <UProgress :value="progressPercentage" :max="100" size="lg" color="primary" />
        </div>

        <!-- Progress Stats -->
        <div class="grid grid-cols-3 gap-4">
          <div class="text-center p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
            <div class="text-2xl font-bold text-gray-900 dark:text-white">{{ progressCurrent }}</div>
            <div class="text-xs text-gray-500 dark:text-gray-400">Completed</div>
          </div>
          <div class="text-center p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
            <div class="text-2xl font-bold text-gray-900 dark:text-white">{{ progressTotal }}</div>
            <div class="text-xs text-gray-500 dark:text-gray-400">Total</div>
          </div>
          <div class="text-center p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
            <div class="text-2xl font-bold text-gray-900 dark:text-white">{{ progressTotal - progressCurrent }}</div>
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

    <!-- Bulk Actions -->
    <UAlert
      v-if="hasSelection && showBulkActions"
      color="info"
      variant="soft"
      icon="i-lucide-check-square"
      :title="`${selectedCount} item(s) selected`"
      class="p-4"
      :actions="bulkActions"
    />

    <!-- Table -->
    <UCard>
      <template #header>
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <UIcon name="i-lucide-table" class="text-gray-500 w-4 h-4" />
            <span class="text-sm font-medium text-gray-700 dark:text-gray-300">
              {{ tableTitle }}
            </span>
          </div>
          <div class="flex items-center gap-2">
            <span class="text-xs text-gray-500 dark:text-gray-400">
              Page {{ page }} of {{ pageCount }}
            </span>
            <UPagination
              v-model:page="page"
              :total="totalItems"
              :items-per-page="pageSize"
              size="sm"
            />
          </div>
        </div>
      </template>

      <UTable
        ref="table"
        v-model:row-selection="rowSelection"
        v-model:sorting="sorting"
        :data="data || []"
        :columns="columns || []"
        :loading="loading"
        :empty-state="{
          icon: 'i-lucide-table',
          label: 'No data available',
          description: 'No items match your current filters or search criteria.'
        }"
        class="text-sm"
        @select="onSelect"
      >
        <!-- Custom cell slots -->
        <template v-for="(_, slotName) in $slots" :key="slotName" #[slotName]="slotProps">
          <slot :name="slotName" v-bind="slotProps" />
        </template>
      </UTable>

      <template #footer>
        <div class="flex items-center justify-between text-xs text-gray-500 dark:text-gray-400">
          <div class="flex items-center gap-4">
            <span v-if="selectable">
              {{ selectedCount }} of {{ totalItems }} row(s) selected
            </span>
            <span v-else>
              Showing {{ data.length }} of {{ totalItems }} items
            </span>
            <span v-if="displayMode === 'all'">
              (All data mode)
            </span>
            <span v-else>
              (Page {{ page }} of {{ pageCount }})
            </span>
          </div>
          <div class="flex items-center gap-2">
            <span>
              {{ footerDescription }}
            </span>
            <UPagination
              v-if="displayMode === 'paginated'"
              v-model:page="page"
              :total="totalItems"
              :items-per-page="pageSize"
              size="sm"
            />
          </div>
        </div>
      </template>
    </UCard>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch, h, resolveComponent } from 'vue'
import { useTable } from '~/composables/shared/useTable'
import type { TableColumn, TableRow } from '#ui/types'
import type { Component } from 'vue'

// Props
interface Props {
  // Data
  data: unknown[]
  columns: TableColumn<unknown>[]
  
  // Configuration
  pageSize?: number
  searchable?: boolean
  sortable?: boolean
  selectable?: boolean
  loading?: boolean
  
  // Header
  showHeader?: boolean
  headerIcon?: string
  headerTitle?: string
  headerDescription?: string
  statusColor?: 'primary' | 'secondary' | 'tertiary' | 'info' | 'success' | 'warning' | 'error' | 'neutral'
  statusText?: string
  
  // Filters
  showFilters?: boolean
  searchPlaceholder?: string
  showTextLengthFilter?: boolean
  textLengthRange?: [number, number]
  maxTextLength?: number
  
  // Domain-specific filters
  showDomainFilters?: boolean
  showStatusFilter?: boolean
  showPromptTypeFilter?: boolean
  showFieldTypeFilter?: boolean
  // Note: statusOptions and promptTypeOptions now come from shared utilities
  
  // Progress
  showProgress?: boolean
  isProcessing?: boolean
  progressCurrent?: number
  progressTotal?: number
  
  // Table
  tableTitle?: string
  footerDescription?: string
  
  // Actions
  showBulkActions?: boolean
  bulkActions?: Array<{
    label?: string
    color?: 'primary' | 'secondary' | 'tertiary' | 'info' | 'success' | 'warning' | 'error' | 'neutral'
    variant?: 'solid' | 'soft' | 'outline' | 'ghost'
    icon?: string
    loading?: boolean
    disabled?: boolean
    size?: 'xs' | 'sm' | 'md' | 'lg'
    onClick?: () => void
  }>
  
  // Action Columns
  showActionColumn?: boolean
  actionColumnHeader?: string
  actionButtons?: Array<{
    label?: string
    color?: 'primary' | 'secondary' | 'tertiary' | 'info' | 'success' | 'warning' | 'error' | 'neutral'
    variant?: 'solid' | 'soft' | 'outline' | 'ghost'
    icon?: string
    size?: 'xs' | 'sm' | 'md' | 'lg'
    disabled?: boolean
    onClick?: (row: unknown) => void
  }>
  
  // Errors
  errors?: Array<{ id: string; message: string }>
  
  // Filters
  filters?: Record<string, unknown>
  
  // Enhanced features
  displayMode?: 'paginated' | 'all'
  showDisplayModeToggle?: boolean
  exportable?: boolean
  exportOptions?: {
    format: 'csv' | 'json' | 'xlsx'
    filename?: string
    includeHeaders?: boolean
    selectedOnly?: boolean
  }
  showStatistics?: boolean
  showResetButton?: boolean
  autoResetOnDataChange?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  pageSize: 25,
  searchable: true,
  sortable: true,
  selectable: false,
  loading: false,
  showHeader: true,
  headerIcon: 'i-lucide-table',
  headerTitle: 'Data Table',
  headerDescription: 'Manage your data',
  statusColor: 'neutral',
  statusText: 'Ready',
  showFilters: true,
  searchPlaceholder: 'Search items...',
  showTextLengthFilter: false,
  textLengthRange: () => [0, 200],
  maxTextLength: 200,
  showDomainFilters: false,
  showStatusFilter: false,
  showPromptTypeFilter: false,
  showFieldTypeFilter: false,
  showProgress: false,
  isProcessing: false,
  progressCurrent: 0,
  progressTotal: 0,
  tableTitle: 'Data Table',
  footerDescription: 'Data management',
  showBulkActions: false,
  bulkActions: () => [],
  showActionColumn: false,
  actionColumnHeader: 'Actions',
  actionButtons: () => [],
  errors: () => [],
  filters: () => ({}),
  displayMode: 'paginated',
  showDisplayModeToggle: false,
  exportable: false,
  exportOptions: () => ({
    format: 'csv' as const,
    filename: undefined,
    includeHeaders: true,
    selectedOnly: false
  }),
  showStatistics: false,
  showResetButton: false,
  autoResetOnDataChange: false
})

// Emits
const emit = defineEmits<{
  (e: 'select', row: unknown): void
  (e: 'bulk-action', action: string, rows: unknown[]): void
  (e: 'filter-change', filters: Record<string, unknown>): void
  (e: 'search-change', search: string): void
}>()

// Local state
const textLengthRange = ref<[number, number]>(props.textLengthRange)

// Domain-specific filter state
const statusFilter = ref('All')
const promptTypeFilter = ref('All')
const fieldTypeFilter = ref('')

// Computed properties
const progressPercentage = computed(() => {
  if (!props.progressTotal) return 0
  return Math.round((props.progressCurrent / props.progressTotal) * 100)
})

// Enhanced columns with action column if enabled
const enhancedColumns = computed(() => {
  const columns = [...props.columns]
  
  if (props.showActionColumn && props.actionButtons.length > 0) {
    columns.push({
      id: 'actions',
      header: props.actionColumnHeader,
      enableSorting: false,
      cell: ({ row }) => {
        const UButton = resolveComponent('UButton') as Component
        return h('div', { class: 'flex gap-2' }, 
          props.actionButtons.map(button => 
            h(UButton, {
              size: button.size || 'xs',
              color: button.color || 'neutral',
              variant: button.variant || 'soft',
              icon: button.icon,
              disabled: button.disabled,
              onClick: () => button.onClick?.(row.original)
            }, { default: () => button.label })
          )
        )
      }
    })
  }
  
  return columns
})

// Use enhanced shared table composable
const {
  data,
  columns,
  rowSelection,
  sorting,
  page,
  pageSize,
  pageCount,
  search,
  selectedRows,
  selectedCount,
  hasSelection,
  totalItems,
  displayMode,
  isExporting,
  exportProgress,
  statistics,
  processedData,
  filteredData,
  sortedData,
  paginatedData,
  onSelect: onSelectRow,
  clearSelection,
  setFilter,
  clearFilter,
  clearAllFilters,
  setStatusFilter,
  setPromptTypeFilter,
  setFieldTypeFilter,
  toggleDisplayMode,
  setDisplayMode,
  exportData,
  enhancedReset,
  // Shared utilities from translation.ts
  statusFilterOptions,
  promptTypeFilterOptions,
  getStatusLabel,
  getStatusColor
} = useTable({
  data: props.data,
  columns: enhancedColumns.value,
  pageSize: props.pageSize,
  searchable: props.searchable,
  sortable: props.sortable,
  selectable: props.selectable,
  filters: props.filters,
  displayMode: props.displayMode || 'paginated',
  showDisplayModeToggle: props.showDisplayModeToggle || false,
  exportable: props.exportable || false,
  exportOptions: props.exportOptions,
  showStatistics: props.showStatistics || false,
  showResetButton: props.showResetButton || false,
  autoResetOnDataChange: props.autoResetOnDataChange || false
})

// Enhanced select handler
const onSelect = (row: unknown) => {
  onSelectRow(row as TableRow<unknown>)
  emit('select', row)
}

// Watch for search changes
watch(search, (newSearch) => {
  emit('search-change', newSearch)
}, { deep: true })

// Watch for filter changes
watch(textLengthRange, (newRange) => {
  setFilter('textLength', newRange)
  emit('filter-change', { textLength: newRange })
}, { deep: true })

// Domain-specific filter methods
const onStatusFilterChange = (value: string) => {
  setStatusFilter(value)
  emit('filter-change', { status: value })
}

const onPromptTypeFilterChange = (value: string) => {
  setPromptTypeFilter(value)
  emit('filter-change', { promptType: value })
}

const onFieldTypeFilterChange = (value: string) => {
  setFieldTypeFilter(value)
  emit('filter-change', { fieldType: value })
}

// Watch for external data changes
watch(() => props.data, (_newData) => {
  // The useTable composable will handle this automatically
}, { deep: true })

// Watch for external filter changes
watch(() => props.filters, (_newFilters) => {
  // The useTable composable will handle this automatically
}, { deep: true })

// Expose methods and utilities for parent components
defineExpose({
  clearSelection,
  clearAllFilters,
  setFilter,
  clearFilter,
  selectedRows,
  selectedCount,
  hasSelection,
  totalItems,
  page,
  pageCount,
  // Enhanced features
  displayMode,
  toggleDisplayMode,
  setDisplayMode,
  exportData,
  enhancedReset,
  statistics,
  processedData,
  filteredData,
  sortedData,
  paginatedData,
  isExporting,
  exportProgress,
  // Shared utilities from translation.ts
  statusFilterOptions,
  promptTypeFilterOptions,
  getStatusLabel,
  getStatusColor
})
</script>
