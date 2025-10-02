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

      <FilterField
        v-model="projectFilter"
        :show="showProjectFilter"
        label="Project"
        :items="projectFilterOptions"
        placeholder="All projects"
        :total-filters="activeFilterCount"
      />

      <FilterField
        v-model="textLengthValue"
        :show="showTextLengthFilter"
        label="Text Length"
        type="range"
        :range-min="textLengthMin"
        :range-max="textLengthMax"
        :range-step="textLengthStep"
        :range-label="textLengthLabel"
        :total-filters="activeFilterCount"
      />

      <FilterField
        v-model="placeholderFilter"
        :show="showPlaceholderFilter"
        label="Placeholder"
        :items="placeholderFilterOptions"
        placeholder="All placeholders"
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
        <!-- Pagination - Outside the table card -->
        <div v-if="showPagination" class="flex justify-center border-t border-default pt-4">
      <UPagination
        :default-page="(tableRef?.tableApi?.getState().pagination.pageIndex || 0) + 1"
        :items-per-page="tableRef?.tableApi?.getState().pagination.pageSize"
        :total="tableRef?.tableApi?.getFilteredRowModel().rows.length"
        @update:page="(p) => tableRef?.tableApi?.setPageIndex(p - 1)"
      />
    </div>

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
              :actions="[...headerActions, ...projectActions]"
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
        :columns="processedTableColumns"
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
import { computed, ref, watch, nextTick, h, resolveComponent, type Component } from 'vue'
import type { TableColumn } from '#ui/types'
import { getPaginationRowModel } from '@tanstack/vue-table'
import { useTableSelection } from '~/composables/features/useTableSelection'
import { useBulkActions } from '~/composables/features/useBulkActions'
import { 
  createTableWithRowSelection
} from '~/utils/table'
import { 
  statusFilterOptions as defaultStatusFilterOptions, 
  promptTypeFilterOptions as defaultPromptTypeFilterOptions,
  getPlaceholderFilterOptions,
  extractPlaceholderTypes
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
  projectFilterOptions?: Array<{ label: string; value: string }>
  
  // Filter visibility
  showStatusFilter?: boolean
  showPromptTypeFilter?: boolean
  showCategoryFilter?: boolean
  showLanguageFilter?: boolean
  showSourceLanguageFilter?: boolean
  showTargetLanguageFilter?: boolean
  showProjectFilter?: boolean
  showTextLengthFilter?: boolean
  showPlaceholderFilter?: boolean

  // Text length filter configuration
  textLengthMin?: number
  textLengthMax?: number
  textLengthStep?: number

  // Placeholder filter configuration
  placeholderFilterOptions?: Array<{ label: string; value: string }>

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

  // Search configuration
  searchFields?: string[]

  // Project management
  showProjectActions?: boolean
  onDeleteProject?: (projectHash: string, projectName: string) => Promise<boolean>

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
  (e: 'project-deleted', projectHash: string): void
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
  projectFilterOptions: () => [],
  
  // Filter visibility
  showStatusFilter: false,
  showPromptTypeFilter: false,
  showCategoryFilter: false,
  showLanguageFilter: false,
  showSourceLanguageFilter: false,
  showTargetLanguageFilter: false,
  showProjectFilter: false,
  showTextLengthFilter: false,
  showPlaceholderFilter: false,

  // Text length filter configuration
  textLengthMin: 0,
  textLengthMax: 200,
  textLengthStep: 5,

  // Placeholder filter configuration
  placeholderFilterOptions: () => [],

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

  // Search configuration
  searchFields: undefined,

  // Project management
  showProjectActions: false,
  onDeleteProject: undefined,

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
const projectFilter = ref('All')
const textLengthValue = ref(props.textLengthMin)
const placeholderFilter = ref('All')

// Project management state
const isDeletingProject = ref(false)

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
      // If specific search fields are provided, search only in those fields
      if (props.searchFields && props.searchFields.length > 0) {
        return props.searchFields.some(field => {
          const fieldValue = (item as Record<string, unknown>)[field]
          return fieldValue && String(fieldValue).toLowerCase().includes(query)
        })
      }
      // Otherwise, search in all fields (default behavior)
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

  if (props.showProjectFilter && projectFilter.value !== 'All') {
    filtered = filtered.filter((item: unknown) => {
      const record = item as Record<string, unknown>
      // Support both manifest_hash (for translations) and project_path (for other data)
      return record.manifest_hash === projectFilter.value || 
             record.project_path === projectFilter.value ||
             record.project_hash === projectFilter.value
    })
  }

  // Apply text length filter
  if (props.showTextLengthFilter && textLengthValue.value !== undefined) {
    const minLength = textLengthValue.value
    filtered = filtered.filter((item: unknown) => {
      const record = item as Record<string, unknown>
      const translatedText = record.translated_text as string || ''
      
      // Show if translated text is at least the minimum length
      return translatedText.length >= minLength
    })
  }

  // Apply placeholder filter
  if (props.showPlaceholderFilter && placeholderFilter.value !== 'All') {
    filtered = filtered.filter((item: unknown) => {
      const record = item as Record<string, unknown>
      const sourceText = record.source_text as string || ''
      const translatedText = record.translated_text as string || ''
      const combinedText = `${sourceText} ${translatedText}`
      
      if (placeholderFilter.value === 'all') {
        // Show items that contain any placeholder
        return /\[[A-Z_]+\d+\]/g.test(combinedText)
      } else {
        // Show items that contain the specific placeholder type
        const placeholderPattern = new RegExp(`\\[${placeholderFilter.value}_\\d+\\]`, 'g')
        return placeholderPattern.test(combinedText)
      }
    })
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

// Process table columns (adds row actions column if enabled)
const processedTableColumns = computed(() => {
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
         targetLanguageFilter.value !== 'All' ||
         projectFilter.value !== 'All' ||
         placeholderFilter.value !== 'All' ||
         (props.showTextLengthFilter && textLengthValue.value !== props.textLengthMin)
})

const selectedCount = computed(() => {
  return tableRef.value?.tableApi?.getFilteredSelectedRowModel().rows.length || 0
})

// Text length filter label
const textLengthLabel = computed(() => {
  if (!props.showTextLengthFilter || textLengthValue.value === undefined) return ''
  return `${textLengthValue.value}+ chars`
})

// Placeholder filter options - dynamically generated from data
const placeholderFilterOptions = computed(() => {
  if (!props.showPlaceholderFilter) return []
  
  // If custom options provided, use them
  if (props.placeholderFilterOptions && props.placeholderFilterOptions.length > 0) {
    return props.placeholderFilterOptions
  }
  
  // Otherwise, generate from current data
  const existingPlaceholders = new Set<string>()
  
  // Scan all data for placeholder patterns
  props.data.forEach((item) => {
    const record = item as Record<string, unknown>
    const sourceText = record.source_text as string || ''
    const translatedText = record.translated_text as string || ''
    const combinedText = `${sourceText} ${translatedText}`
    
    const placeholders = extractPlaceholderTypes(combinedText)
    placeholders.forEach(type => existingPlaceholders.add(type))
  })
  
  return getPlaceholderFilterOptions(existingPlaceholders)
})

// Project management computed properties
const selectedProject = computed(() => {
  if (!props.showProjectFilter || projectFilter.value === 'All') {
    return null
  }
  
  const project = props.projectFilterOptions?.find(p => p.value === projectFilter.value)
  return project ? { hash: project.value, name: project.label } : null
})

const projectActions = computed((): ActionButton[] => {
  if (!props.showProjectActions || !selectedProject.value) {
    return []
  }
  
  return [
    {
      id: 'delete-project',
      label: 'Delete Project',
      icon: 'i-lucide-trash-2',
      color: 'error',
      variant: 'soft',
      loading: isDeletingProject.value,
      disabled: isDeletingProject.value,
      onClick: () => handleDeleteProject()
    }
  ]
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
  if (props.showProjectFilter) count++
  if (props.showTextLengthFilter) count++
  if (props.showPlaceholderFilter) count++
  return count
})

// Watch for selection changes
watch(tableConfig.rowSelection, (_newSelection) => {
  // Use the same approach as TranslationView.vue
  const selectedRows = tableRef.value?.tableApi?.getFilteredSelectedRowModel().rows.map((row: { original: unknown }) => row.original) || []
  
  emit('selection-change', selectedRows)
}, { deep: true })

// Watch for project filter options changes and reset filter if selected project is no longer available
watch(() => props.projectFilterOptions, (newOptions) => {
  if (newOptions && projectFilter.value !== 'All') {
    const currentProject = newOptions.find(p => p.value === projectFilter.value)
    if (!currentProject) {
      console.log('Selected project no longer available, resetting filter to All')
      projectFilter.value = 'All'
    }
  }
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
  projectFilter.value = 'All'
  textLengthValue.value = props.textLengthMin
  placeholderFilter.value = 'All'
  clearSelection()
  // Clear table selection
  tableConfig.rowSelection.value = {}
}

// Project management methods
const handleDeleteProject = async () => {
  if (!selectedProject.value || !props.onDeleteProject) return
  
  const projectHash = selectedProject.value.hash
  const projectName = selectedProject.value.name
  
  try {
    isDeletingProject.value = true
    
    const success = await props.onDeleteProject(projectHash, projectName)
    
    if (success) {
      // Reset project filter and clear selection immediately
      projectFilter.value = 'All'
      clearSelection()
      // Clear table selection
      tableConfig.rowSelection.value = {}
      
      // Wait for the next tick to ensure the filter is reset
      await nextTick()
      
      // Emit event to parent component to refresh data
      emit('project-deleted', projectHash)
    }
  } finally {
    isDeletingProject.value = false
  }
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
