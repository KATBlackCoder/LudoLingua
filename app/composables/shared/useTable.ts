import { computed, watch, ref } from 'vue'
import { useTableStore } from '~/stores/shared/useTableStore'
import type { TableColumn, TableRow } from '#ui/types'
import { 
  statusFilterOptions, 
  promptTypeFilterOptions, 
  getStatusLabel, 
  getStatusColor 
} from '~/utils/translation'

export type DisplayMode = 'paginated' | 'all'

export interface DataProcessor<T = unknown> {
  process: (data: T[]) => T[]
  name: string
  description?: string
}

export interface FilterProcessor<T = unknown> {
  process: (data: T[], filters: Record<string, unknown>) => T[]
  name: string
  description?: string
}

export interface PaginationProcessor<T = unknown> {
  process: (data: T[], page: number, pageSize: number) => T[]
  name: string
  description?: string
}

export interface ExportOptions {
  format: 'csv' | 'json' | 'xlsx'
  filename?: string
  includeHeaders?: boolean
  selectedOnly?: boolean
}

export interface TableStatistics {
  totalItems: number
  filteredItems: number
  selectedItems: number
  pageCount: number
  currentPage: number
  itemsPerPage: number
  hasFilters: boolean
  hasSelection: boolean
  isEmpty: boolean
}

export interface TableConfig<T = unknown> {
  data: T[]
  columns: TableColumn<T>[]
  pageSize?: number
  searchable?: boolean
  sortable?: boolean
  selectable?: boolean
  filters?: Record<string, unknown>
  // Display mode management
  displayMode?: DisplayMode
  showDisplayModeToggle?: boolean
  // Domain-specific filtering
  domainFilters?: {
    status?: string
    promptType?: string
    textLength?: [number, number]
    fieldType?: string
  }
  // Custom filter functions
  customFilters?: Array<{
    key: string
    filter: (item: T, value: unknown) => boolean
  }>
  // Custom processors
  dataProcessor?: DataProcessor<T>
  filterProcessor?: FilterProcessor<T>
  paginationProcessor?: PaginationProcessor<T>
  // Export configuration
  exportable?: boolean
  exportOptions?: ExportOptions
  // Enhanced features
  showStatistics?: boolean
  showResetButton?: boolean
  autoResetOnDataChange?: boolean
}

/**
 * Enhanced shared table composable using Nuxt UI v4 patterns
 * Provides consistent table behavior with pagination, sorting, filtering, selection,
 * display mode management, custom data processing, export functionality, and statistics
 * 
 * Note: This composable works with Nuxt UI v4 Table component and uses proper
 * accessorKey properties for column identification as per Nuxt UI v4 standards.
 * According to https://ui.nuxt.com/docs/components/table, columns should use
 * accessorKey instead of id for proper Nuxt UI v4 Table integration.
 */
export function useTable<T = unknown>(config: TableConfig<T>) {
  const tableStore = useTableStore()

  // Validate config
  if (!config.data || !Array.isArray(config.data)) {
    console.warn('useTable: config.data must be an array')
    config.data = []
  }
  
  if (!config.columns || !Array.isArray(config.columns)) {
    console.warn('useTable: config.columns must be an array')
    config.columns = []
  }

  // Validate column properties for Nuxt UI v4 compliance
  config.columns.forEach((col, index) => {
    const column = col as unknown as Record<string, unknown>
    if (!column.accessorKey && !column.id) {
      console.warn(`useTable: Column at index ${index} is missing accessorKey property. This may cause issues with Nuxt UI v4 Table.`)
    }
  })

  // Enhanced state management
  const displayMode = ref<DisplayMode>(config.displayMode || 'paginated')
  const isExporting = ref(false)
  const exportProgress = ref(0)

  // Initialize store with config
  tableStore.setData(config.data as unknown[])
  tableStore.setColumns(config.columns as TableColumn<unknown>[])
  
  if (config.pageSize && config.pageSize > 0) {
    tableStore.setPageSize(config.pageSize)
  }
  
  // Combine regular filters with domain filters
  const combinedFilters = {
    ...config.filters,
    ...config.domainFilters
  }
  
  if (Object.keys(combinedFilters).length > 0) {
    tableStore.setFilters(combinedFilters)
  }

  // Watch for data changes and update store
  watch(() => config.data, (newData) => {
    if (Array.isArray(newData)) {
      tableStore.setData(newData as unknown[])
    }
  }, { deep: true })

  // Watch for columns changes and update store
  watch(() => config.columns, (newColumns) => {
    if (Array.isArray(newColumns)) {
      tableStore.setColumns(newColumns as TableColumn<unknown>[])
    }
  }, { deep: true })

  // Watch for filters changes and update store
  watch(() => config.filters, (newFilters) => {
    if (newFilters && typeof newFilters === 'object') {
      tableStore.setFilters(newFilters)
    }
  }, { deep: true })

  // Enhanced data processing
  const processedData = computed(() => {
    let data = config.data
    
    // Apply custom data processor if provided
    if (config.dataProcessor) {
      data = config.dataProcessor.process(data)
    }
    
    return data
  })

  // Enhanced filtering with custom filter processor
  const filteredData = computed(() => {
    let filtered = processedData.value
    
    // Apply custom filter processor if provided
    if (config.filterProcessor) {
      filtered = config.filterProcessor.process(filtered, tableStore.filters)
    } else {
      // Use default filtering logic from store
      filtered = tableStore.filteredData as T[]
    }
    
    return filtered
  })

  // Enhanced sorting
  const sortedData = computed(() => {
    if (!tableStore.sorting.length) return filteredData.value
    return tableStore.sortedData as T[]
  })

  // Enhanced pagination with custom pagination processor
  const paginatedData = computed(() => {
    if (displayMode.value === 'all') {
      return sortedData.value
    }
    
    // Apply custom pagination processor if provided
    if (config.paginationProcessor) {
      return config.paginationProcessor.process(sortedData.value, tableStore.page, tableStore.pageSize)
    }
    
    return tableStore.pagedData
  })

  // Computed properties
  const data = computed(() => displayMode.value === 'all' ? sortedData.value : paginatedData.value)
  const columns = computed(() => tableStore.columns)
  const rowSelection = computed({
    get: () => tableStore.rowSelection,
    set: (value) => tableStore.setRowSelection(value)
  })
  const sorting = computed({
    get: () => tableStore.sorting,
    set: (value) => tableStore.setSorting(value)
  })
  const page = computed({
    get: () => tableStore.page,
    set: (value) => tableStore.setPage(value)
  })
  const pageSize = computed({
    get: () => tableStore.pageSize,
    set: (value) => tableStore.setPageSize(value)
  })
  const search = computed({
    get: () => tableStore.search,
    set: (value) => tableStore.setSearch(value)
  })
  const filters = computed({
    get: () => tableStore.filters,
    set: (value) => tableStore.setFilters(value)
  })

  const pageCount = computed(() => tableStore.pageCount)
  const selectedRows = computed(() => tableStore.selectedRows)
  const selectedCount = computed(() => tableStore.selectedCount)
  const hasSelection = computed(() => tableStore.hasSelection)
  const totalItems = computed(() => sortedData.value.length)

  // Enhanced statistics
  const statistics = computed<TableStatistics>(() => ({
    totalItems: processedData.value.length,
    filteredItems: filteredData.value.length,
    selectedItems: selectedCount.value,
    pageCount: pageCount.value,
    currentPage: page.value,
    itemsPerPage: pageSize.value,
    hasFilters: Object.keys(filters.value).length > 0 || search.value.trim() !== '',
    hasSelection: hasSelection.value,
    isEmpty: totalItems.value === 0
  }))

  // Table API methods (Nuxt UI v4 compliant)
  const onSelect = (row: TableRow<T>) => {
    if (!config.selectable || !row) return
    try {
      // Use Nuxt UI v4 TableRow API
      if (typeof row.toggleSelected === 'function') {
        row.toggleSelected(!row.getIsSelected())
      } else {
        // Fallback for compatibility
        const rowId = String(row.index)
        const newSelection = { ...rowSelection.value }
        newSelection[rowId] = !newSelection[rowId]
        tableStore.setRowSelection(newSelection)
      }
    } catch (error) {
      console.warn('useTable: Error toggling row selection', error)
    }
  }

  const selectAll = () => {
    if (!config.selectable || !data.value.length) return
    try {
      const allSelected = data.value.reduce((acc: Record<string, boolean>, _, index) => {
        acc[index] = true
        return acc
      }, {} as Record<string, boolean>)
      tableStore.setRowSelection(allSelected)
    } catch (error) {
      console.warn('useTable: Error selecting all rows', error)
    }
  }

  const clearSelection = () => {
    tableStore.clearSelection()
  }

  const reset = () => {
    tableStore.reset()
    displayMode.value = config.displayMode || 'paginated'
    isExporting.value = false
    exportProgress.value = 0
  }

  // Display mode management
  const toggleDisplayMode = () => {
    displayMode.value = displayMode.value === 'paginated' ? 'all' : 'paginated'
  }

  const setDisplayMode = (mode: DisplayMode) => {
    displayMode.value = mode
  }

  // Export functionality
  const exportData = async (options?: Partial<ExportOptions>) => {
    if (!config.exportable) {
      console.warn('useTable: Export is not enabled for this table')
      return
    }

    const exportOptions = { ...config.exportOptions, ...options }
    isExporting.value = true
    exportProgress.value = 0

    try {
      const dataToExport = exportOptions.selectedOnly ? selectedRows.value : sortedData.value
      const filename = exportOptions.filename || `table-export-${new Date().toISOString().split('T')[0]}`
      
      let exportContent: string
      let mimeType: string
      let fileExtension: string

      switch (exportOptions.format) {
        case 'csv':
          exportContent = convertToCSV(dataToExport, columns.value, exportOptions.includeHeaders)
          mimeType = 'text/csv'
          fileExtension = 'csv'
          break
        case 'json':
          exportContent = JSON.stringify(dataToExport, null, 2)
          mimeType = 'application/json'
          fileExtension = 'json'
          break
        case 'xlsx':
          // For XLSX, we'd need a library like xlsx
          // For now, fallback to CSV
          exportContent = convertToCSV(dataToExport, columns.value, exportOptions.includeHeaders)
          mimeType = 'text/csv'
          fileExtension = 'csv'
          break
        default:
          throw new Error(`Unsupported export format: ${exportOptions.format}`)
      }

      // Create and download file
      const blob = new Blob([exportContent], { type: mimeType })
      const url = URL.createObjectURL(blob)
      const link = document.createElement('a')
      link.href = url
      link.download = `${filename}.${fileExtension}`
      document.body.appendChild(link)
      link.click()
      document.body.removeChild(link)
      URL.revokeObjectURL(url)

      exportProgress.value = 100
    } catch (error) {
      console.error('useTable: Export failed', error)
      throw error
    } finally {
      isExporting.value = false
    }
  }

  // Helper function to convert data to CSV
  const convertToCSV = (data: unknown[], columns: TableColumn<unknown>[], includeHeaders: boolean = true): string => {
    if (!data.length || !columns.length) return ''

    const headers = columns.map(col => {
      const column = col as unknown as Record<string, unknown>
      return column.header || column.accessorKey
    }).join(',')
    const rows = data.map(item => {
      return columns.map(col => {
        const column = col as unknown as Record<string, unknown>
        const value = (item as Record<string, unknown>)[column.accessorKey as string || '']
        // Escape CSV values
        const stringValue = String(value || '')
        if (stringValue.includes(',') || stringValue.includes('"') || stringValue.includes('\n')) {
          return `"${stringValue.replace(/"/g, '""')}"`
        }
        return stringValue
      }).join(',')
    })

    return includeHeaders ? [headers, ...rows].join('\n') : rows.join('\n')
  }

  // Enhanced reset with auto-reset on data change
  const enhancedReset = () => {
    reset()
    if (config.autoResetOnDataChange) {
      // Reset page to 1 when data changes
      tableStore.setPage(1)
    }
  }

  // Filter helpers
  const setFilter = (key: string, value: unknown) => {
    const newFilters = { ...filters.value, [key]: value }
    tableStore.setFilters(newFilters)
  }

  const clearFilter = (key: string) => {
    const newFilters = { ...filters.value }
    if (key in newFilters) {
      const { [key]: _, ...rest } = newFilters
      tableStore.setFilters(rest)
    }
  }

  const clearAllFilters = () => {
    tableStore.setFilters({})
  }

  // Domain-specific filter helpers (using shared utilities)
  const setStatusFilter = (status: string) => {
    setFilter('status', status === 'All' ? undefined : status)
  }

  const setPromptTypeFilter = (promptType: string) => {
    setFilter('promptType', promptType === 'All' ? undefined : promptType)
  }

  const setTextLengthFilter = (range: [number, number]) => {
    setFilter('textLength', range)
  }

  const setFieldTypeFilter = (fieldType: string) => {
    setFilter('fieldType', fieldType === 'All' ? undefined : fieldType)
  }

  // Custom filter helpers
  const applyCustomFilter = (key: string, value: unknown) => {
    if (config.customFilters) {
      const customFilter = config.customFilters.find(f => f.key === key)
      if (customFilter) {
        // Apply custom filter logic
        const filteredData = config.data.filter(item => customFilter.filter(item, value))
        tableStore.setData(filteredData as unknown[])
      }
    }
  }

  // Search helpers
  const clearSearch = () => {
    tableStore.setSearch('')
  }

  // Pagination helpers
  const goToFirstPage = () => {
    if (pageCount.value > 0) {
      tableStore.setPage(1)
    }
  }

  const goToLastPage = () => {
    if (pageCount.value > 0) {
      tableStore.setPage(pageCount.value)
    }
  }

  const goToNextPage = () => {
    if (page.value < pageCount.value && pageCount.value > 0) {
      tableStore.setPage(page.value + 1)
    }
  }

  const goToPreviousPage = () => {
    if (page.value > 1) {
      tableStore.setPage(page.value - 1)
    }
  }

  // Utility methods
  const isFirstPage = computed(() => page.value === 1)
  const isLastPage = computed(() => page.value === pageCount.value)
  const hasData = computed(() => totalItems.value > 0)
  const isEmpty = computed(() => totalItems.value === 0)

  return {
    // Data
    data,
    columns,
    totalItems,
    processedData,
    filteredData,
    sortedData,
    paginatedData,

    // State
    rowSelection,
    sorting,
    page,
    pageSize,
    search,
    filters,
    displayMode,
    isExporting,
    exportProgress,

    // Computed
    pageCount,
    selectedRows,
    selectedCount,
    hasSelection,
    isFirstPage,
    isLastPage,
    hasData,
    isEmpty,
    statistics,

    // Shared utilities
    statusFilterOptions,
    promptTypeFilterOptions,
    getStatusLabel,
    getStatusColor,

    // Actions
    onSelect,
    selectAll,
    clearSelection,
    reset,
    enhancedReset,

    // Display mode actions
    toggleDisplayMode,
    setDisplayMode,

    // Export actions
    exportData,

    // Filter actions
    setFilter,
    clearFilter,
    clearAllFilters,

    // Domain-specific filter actions
    setStatusFilter,
    setPromptTypeFilter,
    setTextLengthFilter,
    setFieldTypeFilter,
    applyCustomFilter,

    // Search actions
    clearSearch,

    // Pagination actions
    goToFirstPage,
    goToLastPage,
    goToNextPage,
    goToPreviousPage,
  }
}
