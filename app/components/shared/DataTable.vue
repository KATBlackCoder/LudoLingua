<template>
  <div class="space-y-4">
    <!-- Header with Filters -->
    <FilterCard
      v-if="showFilters"
      :title="filterTitle"
      :icon="filterIcon"
      :badge-text="`${filteredData.length} items`"
      :badge-color="filteredData.length > 0 ? 'success' : 'neutral'"
      :show-clear-button="hasActiveFilters"
      @clear="clearFilters"
    >
      <template v-if="showSearch">
        <div class="space-y-2">
          <UInput
            v-model="searchQuery"
            placeholder="Search..."
            icon="i-lucide-search"
            size="sm"
          />
        </div>
      </template>

      <!-- Built-in filters -->
      <FilterField
        v-model="statusFilter"
        :show="showStatusFilter"
        label="Status"
        :items="statusFilterOptions"
        placeholder="All statuses"
        :total-filters="activeFilterCount"
      />

      <FilterField
        v-model="promptTypeFilter"
        :show="showPromptTypeFilter"
        label="Type"
        :items="promptTypeFilterOptions"
        placeholder="All types"
        :total-filters="activeFilterCount"
      />

      <FilterField
        v-model="categoryFilter"
        :show="showCategoryFilter"
        label="Category"
        :items="categoryFilterOptions"
        placeholder="All categories"
        :total-filters="activeFilterCount"
      />

      <FilterField
        v-model="sourceLanguageFilter"
        :show="showSourceLanguageFilter"
        label="Source Language"
        :items="languageFilterOptions"
        placeholder="Source language"
        :total-filters="activeFilterCount"
      />

      <FilterField
        v-model="targetLanguageFilter"
        :show="showTargetLanguageFilter"
        label="Target Language"
        :items="languageFilterOptions"
        placeholder="Target language"
        :total-filters="activeFilterCount"
      />

      <template v-if="customFilters">
        <slot name="filters" />
      </template>
    </FilterCard>

    <!-- Bulk Actions -->
    <BulkActions
      v-if="showBulkActions && selectedCount > 0"
      :selected-count="selectedCount"
      :actions="bulkActions"
      :alert-color="bulkAlertColor"
    />

    <!-- Table -->
    <UCard>
      <template #header>
        <TableHeader
          :title="title"
          :icon="icon"
          :show-pagination="false"
          :show-pagination-info="false"
        >
          <template #actions>
            <ActionButtonGroup
              :actions="headerActions"
              :spacing="'tight'"
            />
          </template>
        </TableHeader>
      </template>

      <UTable
        ref="tableRef"
        v-model:pagination="pagination"
        v-model:row-selection="tableConfig.rowSelection.value"
        :data="tableData"
        :columns="tableColumns"
        :pagination-options="paginationOptions"
        :loading="loading"
        :sticky="stickyHeaders"
        class="flex-1"
        v-bind="tableProps"
      >
        <!-- Custom cell slots -->
        <template v-for="(_, slotName) in $slots" :key="slotName" #[slotName]="slotData">
          <slot :name="slotName" v-bind="slotData" />
        </template>
      </UTable>

      <template #footer>
        <TableFooter
          :table-api="tableRef?.tableApi"
          :show-row-count="showRowCount"
          :show-selected-count="showSelectedCount"
          :selected-count="selectedCount"
          :show-stats="showStats"
          :stats="computedStats"
          :show-page-size-selector="showPageSizeSelector"
          :page-size-options="pageSizeOptions"
        />
      </template>
    </UCard>

    <!-- Pagination - Outside the table card -->
    <div v-if="showPagination" class="flex justify-center border-t border-default pt-4">
      <UPagination
        :default-page="(tableRef?.tableApi?.getState().pagination.pageIndex || 0) + 1"
        :items-per-page="tableRef?.tableApi?.getState().pagination.pageSize"
        :total="tableRef?.tableApi?.getFilteredRowModel().rows.length"
        @update:page="(p) => tableRef?.tableApi?.setPageIndex(p - 1)"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch, h, resolveComponent, type Component } from 'vue'
import type { TableColumn } from '#ui/types'
import { getPaginationRowModel } from '@tanstack/vue-table'
import { useTableSelection } from '~/composables/features/useTableSelection'
import { useBulkActions } from '~/composables/features/useBulkActions'
import { 
  createTableWithRowSelection
} from '~/utils/table'
import { 
  statusFilterOptions as defaultStatusFilterOptions, 
  promptTypeFilterOptions as defaultPromptTypeFilterOptions 
} from '~/utils/translation'
import FilterCard from './FilterCard.vue'
import FilterField from './FilterField.vue'
import TableHeader from './TableHeader.vue'
import TableFooter from './TableFooter.vue'
import BulkActions from './BulkActions.vue'
import ActionButtonGroup from './ActionButtonGroup.vue'
import type { ActionButton } from './ActionButtonGroup.vue'
import type { BulkAction } from '~/composables/features/useBulkActions'

interface Props<T = unknown> {
  // Data
  data: T[]
  columns: TableColumn<T>[]
  loading?: boolean

  // Table configuration
  title?: string
  icon?: string
  stickyHeaders?: boolean
  tableProps?: Record<string, unknown>

  // Filters
  showFilters?: boolean
  showSearch?: boolean
  customFilters?: boolean
  filterTitle?: string
  filterIcon?: string
  
  // Built-in filter options
  statusFilterOptions?: Array<{ label: string; value: string }>
  promptTypeFilterOptions?: Array<{ label: string; value: string }>
  categoryFilterOptions?: Array<{ label: string; value: string }>
  languageFilterOptions?: Array<{ label: string; value: string }>
  
  // Filter visibility
  showStatusFilter?: boolean
  showPromptTypeFilter?: boolean
  showCategoryFilter?: boolean
  showLanguageFilter?: boolean
  showSourceLanguageFilter?: boolean
  showTargetLanguageFilter?: boolean

  // Selection
  showSelection?: boolean
  showSelectedCount?: boolean

  // Bulk actions
  showBulkActions?: boolean
  bulkActions?: BulkAction[]
  bulkAlertColor?: 'primary' | 'secondary' | 'tertiary' | 'info' | 'success' | 'warning' | 'error' | 'neutral'

  // Header actions
  headerActions?: ActionButton[]

  // Pagination
  showPagination?: boolean
  showPageSizeSelector?: boolean
  initialPageSize?: number
  pageSizeOptions?: Array<{ label: string; value: number }>

  // Stats
  showRowCount?: boolean
  showStats?: boolean
  stats?: Record<string, string | number>

  // Custom filtering function
  filterFunction?: (data: T[], searchQuery: string) => T[]

  // Row actions
  showRowActions?: boolean
  rowActions?: Array<{
    label: string
    icon?: string
    color?: 'primary' | 'secondary' | 'tertiary' | 'info' | 'success' | 'warning' | 'error' | 'neutral'
    variant?: 'solid' | 'soft' | 'outline' | 'subtle' | 'ghost'
    onClick: (row: T) => void | Promise<void>
    disabled?: (row: T) => boolean
    separator?: boolean
  }>
  rowActionItems?: (row: T) => Array<{
    type?: 'label' | 'separator'
    label?: string
    icon?: string
    disabled?: boolean
    onSelect?: () => void
  }>
  rowActionsType?: 'buttons' | 'dropdown' | 'menu'
}

interface Emits {
  (e: 'selection-change', selectedRows: unknown[]): void
  (e: 'bulk-action', action: BulkAction): void
  (e: 'row-action', action: { type: string; row: unknown }): void
}

const props = withDefaults(defineProps<Props>(), {
  // Table configuration
  title: 'Data Table',
  icon: 'i-lucide-table',
  stickyHeaders: false,
  tableProps: () => ({}),

  // Filters
  showFilters: true,
  showSearch: true,
  customFilters: false,
  filterTitle: 'Filters & Search',
  filterIcon: 'i-lucide-filter',
  
  // Built-in filter options
  statusFilterOptions: () => defaultStatusFilterOptions,
  promptTypeFilterOptions: () => defaultPromptTypeFilterOptions,
  categoryFilterOptions: () => [],
  languageFilterOptions: () => [],
  
  // Filter visibility
  showStatusFilter: false,
  showPromptTypeFilter: false,
  showCategoryFilter: false,
  showLanguageFilter: false,
  showSourceLanguageFilter: false,
  showTargetLanguageFilter: false,

  // Selection
  showSelection: false,
  showSelectedCount: true,

  // Bulk actions
  showBulkActions: false,
  bulkActions: () => [],
  bulkAlertColor: 'info',

  // Header actions
  headerActions: () => [],

  // Pagination
  showPagination: true,
  showPageSizeSelector: false,
  initialPageSize: 25,
  pageSizeOptions: () => [
    { label: '10', value: 10 },
    { label: '25', value: 25 },
    { label: '50', value: 50 },
    { label: '100', value: 100 }
  ],

  // Stats
  showRowCount: true,
  showStats: false,
  stats: () => ({}),

  // Custom filtering
  filterFunction: undefined,

  // Row actions
  showRowActions: false,
  rowActions: () => [],
  rowActionItems: undefined,
  rowActionsType: 'dropdown'
})

const emit = defineEmits<Emits>()

// Composables
const tableRef = ref()

// Resolve UCheckbox component for table selection
const UCheckbox = resolveComponent('UCheckbox') as Component

// Search and filtering
const searchQuery = ref('')

// Built-in filter state
const statusFilter = ref('All')
const promptTypeFilter = ref('All')
const categoryFilter = ref('All')
const sourceLanguageFilter = ref('All')
const targetLanguageFilter = ref('All')

// Selection
const { 
  clearSelection 
} = useTableSelection(tableRef)

// Bulk actions
useBulkActions(computed(() => tableConfig.selectedRowCount.value), {
  onActionStart: (action) => emit('bulk-action', action as unknown as BulkAction)
})

// Pagination - TanStack Table integration
const pagination = ref({
  pageIndex: 0,
  pageSize: props.initialPageSize
})

const paginationOptions = {
  getPaginationRowModel: getPaginationRowModel()
}

// TanStack Table handles all pagination logic

// Computed data
const filteredData = computed(() => {
  let filtered = props.data

  // Apply search filter first
  if (searchQuery.value.trim()) {
    const query = searchQuery.value.toLowerCase()
    filtered = filtered.filter(item => {
      return JSON.stringify(item).toLowerCase().includes(query)
    })
  }

  // Apply built-in filters
  if (props.showStatusFilter && statusFilter.value !== 'All') {
    filtered = filtered.filter((item: unknown) => (item as Record<string, unknown>).status === statusFilter.value)
  }

  if (props.showPromptTypeFilter && promptTypeFilter.value !== 'All') {
    filtered = filtered.filter((item: unknown) => (item as Record<string, unknown>).prompt_type === promptTypeFilter.value)
  }

  if (props.showCategoryFilter && categoryFilter.value !== 'All') {
    filtered = filtered.filter((item: unknown) => (item as Record<string, unknown>).category === categoryFilter.value)
  }

  if (props.showSourceLanguageFilter && sourceLanguageFilter.value !== 'All') {
    filtered = filtered.filter((item: unknown) => (item as Record<string, unknown>).source_lang === sourceLanguageFilter.value)
  }

  if (props.showTargetLanguageFilter && targetLanguageFilter.value !== 'All') {
    filtered = filtered.filter((item: unknown) => (item as Record<string, unknown>).target_lang === targetLanguageFilter.value)
  }

  // Apply custom filter function if provided
  if (props.filterFunction) {
    filtered = props.filterFunction(filtered, searchQuery.value)
  }

  return filtered
})

// Remove manual pagination - let TanStack Table handle it
const tableData = computed(() => filteredData.value)

// Create table with row selection using the new utility
const tableConfig = createTableWithRowSelection(
  props.columns,
  {
    enableRowSelection: props.showSelection,
    onRowSelect: (selectedRows) => {
      emit('selection-change', selectedRows)
    }
  }
)

// Override the selection column to use the resolved UCheckbox component
if (props.showSelection && tableConfig.columns.value.length > 0) {
  const selectionColumn = tableConfig.columns.value[0]
  if (selectionColumn && selectionColumn.id === 'select') {
    selectionColumn.header = ({ table }) => {
      return h(UCheckbox, {
        modelValue: table.getIsAllPageRowsSelected(),
        indeterminate: table.getIsSomePageRowsSelected() && !table.getIsAllPageRowsSelected(),
        'onUpdate:modelValue': (value: boolean | 'indeterminate') =>
          table.toggleAllPageRowsSelected(!!value),
        'aria-label': 'Select all'
      })
    }
    selectionColumn.cell = ({ row }) => {
      return h(UCheckbox, {
        modelValue: row.getIsSelected(),
        'onUpdate:modelValue': (value: boolean | 'indeterminate') =>
          row.toggleSelected(!!value),
        'aria-label': 'Select row'
      })
    }
  }
}

const tableColumns = computed(() => {
  const columns = [...tableConfig.columns.value]
  
  // Add row actions column if enabled
  if (props.showRowActions) {
    const actionsColumn: TableColumn<unknown> = {
      id: 'actions',
      header: 'Actions',
      size: 120,
      enableSorting: false,
      cell: ({ row }) => {
        const UDropdownMenu = resolveComponent('UDropdownMenu') as Component
        const UButton = resolveComponent('UButton') as Component
        
        const menuItems = (props.rowActions || []).flatMap(action => {
          const item = {
            label: action.label,
            icon: action.icon,
            disabled: action.disabled?.(row.original),
            onSelect: () => {
              action.onClick(row.original)
              emit('row-action', { type: action.label, row: row.original })
            }
          }
          
          return action.separator ? [item, { type: 'separator' }] : [item]
        })
        
        return h('div', { class: 'text-right' }, 
          h(UDropdownMenu, {
            content: { align: 'end' },
            items: menuItems,
            'aria-label': 'Row actions'
          }, {
            default: () => h(UButton, {
              icon: 'i-lucide-ellipsis-vertical',
              color: 'neutral',
              variant: 'ghost',
              size: 'sm',
              'aria-label': 'Actions dropdown'
            })
          })
        )
      }
    }
    
    columns.push(actionsColumn)
  }
  
  return columns
})

const computedStats = computed(() => {
  const baseStats = { ...props.stats }
  
  if (props.showStats && filteredData.value.length !== props.data.length) {
    baseStats.filtered = filteredData.value.length
    baseStats.total = props.data.length
  }
  
  return baseStats
})

const hasActiveFilters = computed(() => {
  return searchQuery.value.trim() !== '' ||
         statusFilter.value !== 'All' ||
         promptTypeFilter.value !== 'All' ||
         categoryFilter.value !== 'All' ||
         sourceLanguageFilter.value !== 'All' ||
         targetLanguageFilter.value !== 'All'
})

const selectedCount = computed(() => {
  return tableRef.value?.tableApi?.getFilteredSelectedRowModel().rows.length || 0
})

// Count active filters for responsive design
const activeFilterCount = computed(() => {
  let count = 0
  if (props.showSearch) count++
  if (props.showStatusFilter) count++
  if (props.showPromptTypeFilter) count++
  if (props.showCategoryFilter) count++
  if (props.showSourceLanguageFilter) count++
  if (props.showTargetLanguageFilter) count++
  return count
})

// Watch for selection changes
watch(tableConfig.rowSelection, (_newSelection) => {
  // Use the same approach as TranslationView.vue
  const selectedRows = tableRef.value?.tableApi?.getFilteredSelectedRowModel().rows.map((row: { original: unknown }) => row.original) || []
  
  emit('selection-change', selectedRows)
}, { deep: true })

// Pagination is now handled directly by TanStack Table and UPagination

// Methods
const clearFilters = () => {
  searchQuery.value = ''
  statusFilter.value = 'All'
  promptTypeFilter.value = 'All'
  categoryFilter.value = 'All'
  sourceLanguageFilter.value = 'All'
  targetLanguageFilter.value = 'All'
  clearSelection()
  // Clear table selection
  tableConfig.rowSelection.value = {}
}

// Row actions helper
const createRowActionsWithEvents = (actions: typeof props.rowActions) => {
  if (!actions) return []
  
  return actions.map(action => ({
    ...action,
    onClick: (row: unknown) => {
      // Emit row action event
      emit('row-action', { type: action.label, row })
      // Call original onClick if provided
      if (action.onClick) {
        action.onClick(row)
      }
    }
  }))
}

// Expose methods and table ref for parent components
defineExpose({
  clearFilters,
  clearSelection,
  selectedRows: tableConfig.rowSelection,
  selectedCount,
  tableRef,
  createRowActionsWithEvents,
  tableConfig
})
</script>
