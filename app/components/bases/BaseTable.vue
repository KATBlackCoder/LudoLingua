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
              v-model="globalFilter"
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
                :options="props.statusFilterOptions.length > 0 ? props.statusFilterOptions : statusFilterOptions"
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
                :options="props.promptTypeFilterOptions.length > 0 ? props.promptTypeFilterOptions : promptTypeFilterOptions"
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
      v-if="selectedCount > 0 && showBulkActions"
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
        :data="tableData"
        :columns="tableColumns"
        :loading="loading"
        :loading-animation="loadingAnimation"
        :loading-color="loadingColor"
        :sticky="sticky"
        :sticky-header="stickyHeader"
        :sticky-footer="stickyFooter"
        :empty-state="{
          icon: 'i-lucide-table',
          label: 'No data available',
          description: 'No items match your current filters or search criteria.'
        }"
        class="text-sm"
        @select="onRowSelect"
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
              Showing {{ tableData.length }} of {{ totalItems }} items
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
import { computed, ref, watch, h, resolveComponent, useTemplateRef } from 'vue'
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
  statusFilterOptions?: Array<{ label: string; value: string }>
  promptTypeFilterOptions?: Array<{ label: string; value: string }>
  
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
  
  // Sticky features
  sticky?: boolean
  stickyHeader?: boolean
  stickyFooter?: boolean
  
  // Loading states
  loadingAnimation?: 'carousel' | 'carousel-inverse' | 'swing' | 'elastic'
  loadingColor?: 'primary' | 'secondary' | 'success' | 'info' | 'warning' | 'error' | 'neutral'
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
  statusFilterOptions: () => [],
  promptTypeFilterOptions: () => [],
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
  autoResetOnDataChange: false,
  sticky: false,
  stickyHeader: false,
  stickyFooter: false,
  loadingAnimation: 'carousel',
  loadingColor: 'primary'
})

// Emits
const emit = defineEmits<{
  (e: 'select', row: unknown): void
  (e: 'bulk-action', action: string, rows: unknown[]): void
  (e: 'filter-change', filters: Record<string, unknown>): void
  (e: 'search-change', search: string): void
}>()

// Template refs
const tableRef = useTemplateRef('table')

// Local state
const textLengthRange = ref<[number, number]>(props.textLengthRange)
const globalFilter = ref('')

// Domain-specific filter state
const statusFilter = ref('All')
const promptTypeFilter = ref('All')
const fieldTypeFilter = ref('')

// Validation
if (!props.data || !Array.isArray(props.data)) {
  console.warn('BaseTable: data prop must be an array')
}

if (!props.columns || !Array.isArray(props.columns)) {
  console.warn('BaseTable: columns prop must be an array')
}

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
      accessorKey: 'actions',
      header: props.actionColumnHeader,
      enableSorting: false,
      enableColumnFilter: false,
      enableHiding: false,
      meta: {
        class: {
          th: 'text-center',
          td: 'text-center'
        },
        style: {
          th: 'min-width: 120px;',
          td: 'min-width: 120px;'
        }
      },
      cell: ({ row }) => {
        try {
          const UButton = resolveComponent('UButton') as Component
          return h('div', { class: 'flex gap-1 justify-center' }, 
            props.actionButtons.map(button => 
              h(UButton, {
                size: button.size || 'xs',
                color: button.color || 'neutral',
                variant: button.variant || 'soft',
                icon: button.icon,
                disabled: button.disabled,
                onClick: () => {
                  try {
                    button.onClick?.(row.original)
                  } catch (error) {
                    console.warn('BaseTable: Error in action button click', error)
                  }
                }
              }, { default: () => button.label })
            )
          )
        } catch (error) {
          console.warn('BaseTable: Error rendering action buttons', error)
          return h('div', { class: 'text-center text-gray-500' }, 'Error')
        }
      }
    })
  }
  
  return columns
})

// Use enhanced shared table composable for table functionality
const {
  data: tableData,
  columns: tableColumns,
  rowSelection,
  page,
  pageSize,
  pageCount,
  selectedRows,
  selectedCount,
  totalItems,
  displayMode,
  isExporting,
  exportProgress,
  statistics,
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
  autoResetOnDataChange: props.autoResetOnDataChange || false,
  // Domain-specific filters
  domainFilters: {
    status: statusFilter.value,
    promptType: promptTypeFilter.value,
    textLength: textLengthRange.value,
    fieldType: fieldTypeFilter.value
  }
})


// Row selection handler for @select event
const onRowSelect = (row: TableRow<unknown>) => {
  try {
    emit('select', row.original)
  } catch (error) {
    console.warn('BaseTable: Error handling row selection', error)
  }
}


// Watch for search changes and sync with useTable
watch(globalFilter, (newSearch) => {
  setFilter('global', newSearch)
  emit('search-change', newSearch)
}, { deep: true })

// Watch for filter changes and sync with useTable
watch(textLengthRange, (newRange) => {
  setFilter('textLength', newRange)
  emit('filter-change', { textLength: newRange })
}, { deep: true })

// Watch for domain filter changes and sync with useTable
watch([statusFilter, promptTypeFilter, fieldTypeFilter], ([status, promptType, fieldType]) => {
  setFilter('status', status)
  setFilter('promptType', promptType)
  setFilter('fieldType', fieldType)
  emit('filter-change', { status, promptType, fieldType })
}, { deep: true })

// Domain-specific filter methods
const onStatusFilterChange = (value: string) => {
  statusFilter.value = value
  setStatusFilter(value)
  emit('filter-change', { status: value })
}

const onPromptTypeFilterChange = (value: string) => {
  promptTypeFilter.value = value
  setPromptTypeFilter(value)
  emit('filter-change', { promptType: value })
}

const onFieldTypeFilterChange = (value: string) => {
  fieldTypeFilter.value = value
  setFieldTypeFilter(value)
  emit('filter-change', { fieldType: value })
}

// Watch for external data changes and sync with useTable
watch(() => props.data, (newData) => {
  if (props.autoResetOnDataChange && newData) {
    enhancedReset()
  }
}, { deep: true })

// Watch for external filter changes and sync with useTable
watch(() => props.filters, (newFilters) => {
  if (newFilters) {
    Object.entries(newFilters).forEach(([key, value]) => {
      setFilter(key, value)
    })
  }
}, { deep: true })

// Expose methods and utilities for parent components
defineExpose({
  // Table ref for direct access
  tableRef,
  // Core table methods
  clearSelection,
  clearAllFilters,
  setFilter,
  clearFilter,
  selectedRows,
  selectedCount,
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
  isExporting,
  exportProgress,
  // Shared utilities from translation.ts
  statusFilterOptions,
  promptTypeFilterOptions,
  getStatusLabel,
  getStatusColor,
  // Filter state
  globalFilter,
  statusFilter,
  promptTypeFilter,
  fieldTypeFilter,
  textLengthRange
})
</script>
