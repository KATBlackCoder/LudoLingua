import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { TableColumn } from '#ui/types'

export interface TableState<T = unknown> {
  data: T[]
  columns: TableColumn<T>[]
  rowSelection: Record<string, boolean>
  sorting: Array<{ id: string; desc: boolean }>
  page: number
  pageSize: number
  search: string
  filters: Record<string, unknown>
}

export interface TableActions<T = unknown> {
  setData: (data: T[]) => void
  setColumns: (columns: TableColumn<T>[]) => void
  setRowSelection: (selection: Record<string, boolean>) => void
  setSorting: (sorting: Array<{ id: string; desc: boolean }>) => void
  setPage: (page: number) => void
  setPageSize: (pageSize: number) => void
  setSearch: (search: string) => void
  setFilters: (filters: Record<string, unknown>) => void
  clearSelection: () => void
  reset: () => void
}

/**
 * Shared table store for consistent table state management across components
 * Provides pagination, sorting, filtering, and row selection functionality
 */
export const useTableStore = defineStore('shared-table', () => {
  // State
  const data = ref<unknown[]>([])
  const columns = ref<TableColumn<unknown>[]>([])
  const rowSelection = ref<Record<string, boolean>>({})
  const sorting = ref<Array<{ id: string; desc: boolean }>>([])
  const page = ref(1)
  const pageSize = ref(25)
  const search = ref('')
  const filters = ref<Record<string, unknown>>({})

  // Computed
  const filteredData = computed(() => {
    let filtered = data.value

    // Apply search filter
    if (search.value.trim()) {
      const query = search.value.trim().toLowerCase()
      filtered = filtered.filter((item) => {
        if (typeof item === 'object' && item !== null) {
          return Object.values(item).some((value) =>
            String(value).toLowerCase().includes(query)
          )
        }
        return String(item).toLowerCase().includes(query)
      })
    }

    // Apply custom filters
    Object.entries(filters.value).forEach(([key, value]) => {
      if (value !== null && value !== undefined && value !== '') {
        filtered = filtered.filter((item) => {
          if (typeof item === 'object' && item !== null) {
            const itemRecord = item as Record<string, unknown>
            
            // Handle domain-specific filters
            if (key === 'status' && 'status' in itemRecord) {
              return itemRecord.status === value
            }
            
            if (key === 'promptType' && 'prompt_type' in itemRecord) {
              return itemRecord.prompt_type === value
            }
            
            if (key === 'fieldType' && 'field_type' in itemRecord) {
              return itemRecord.field_type === value
            }
            
            if (key === 'textLength' && Array.isArray(value) && value.length === 2) {
              const [min, max] = value as [number, number]
              const sourceText = itemRecord.source_text as string
              const textLength = sourceText ? sourceText.length : 0
              return textLength >= min && textLength <= max
            }
            
            // Handle generic filters
            if (key in itemRecord) {
              const itemValue = itemRecord[key]
              if (typeof value === 'string') {
                return String(itemValue).toLowerCase().includes(value.toLowerCase())
              }
              if (typeof value === 'number') {
                return Number(itemValue) === value
              }
              if (Array.isArray(value)) {
                return value.includes(itemValue)
              }
              return itemValue === value
            }
          }
          return false
        })
      }
    })

    return filtered
  })

  const sortedData = computed(() => {
    if (!sorting.value.length) return filteredData.value

    return [...filteredData.value].sort((a, b) => {
      for (const sort of sorting.value) {
        if (typeof a === 'object' && a !== null && typeof b === 'object' && b !== null) {
          const aRecord = a as Record<string, unknown>
          const bRecord = b as Record<string, unknown>
          const aValue = aRecord[sort.id]
          const bValue = bRecord[sort.id]
          
          if (aValue === bValue) continue
          
          // Handle comparison safely for unknown types
          let comparison = 0
          if (typeof aValue === 'string' && typeof bValue === 'string') {
            comparison = aValue.localeCompare(bValue)
          } else if (typeof aValue === 'number' && typeof bValue === 'number') {
            comparison = aValue - bValue
          } else {
            comparison = String(aValue).localeCompare(String(bValue))
          }
          
          return sort.desc ? -comparison : comparison
        }
        return 0
      }
      return 0
    })
  })

  const pageCount = computed(() => 
    Math.max(1, Math.ceil(sortedData.value.length / pageSize.value))
  )

  const pagedData = computed(() => {
    const start = (page.value - 1) * pageSize.value
    return sortedData.value.slice(start, start + pageSize.value)
  })

  const selectedRows = computed(() => {
    return data.value.filter((_, index) => rowSelection.value[index])
  })

  const selectedCount = computed(() => selectedRows.value.length)

  const hasSelection = computed(() => selectedCount.value > 0)

  // Actions
  const setData = (newData: unknown[]) => {
    data.value = newData
    // Reset page when data changes
    page.value = 1
  }

  const setColumns = (newColumns: TableColumn<unknown>[]) => {
    columns.value = newColumns
  }

  const setRowSelection = (selection: Record<string, boolean>) => {
    rowSelection.value = selection
  }

  const setSorting = (newSorting: Array<{ id: string; desc: boolean }>) => {
    sorting.value = newSorting
  }

  const setPage = (newPage: number) => {
    page.value = Math.max(1, Math.min(newPage, pageCount.value))
  }

  const setPageSize = (newPageSize: number) => {
    pageSize.value = Math.max(1, newPageSize)
    page.value = 1 // Reset to first page
  }

  const setSearch = (newSearch: string) => {
    search.value = newSearch
    page.value = 1 // Reset to first page when searching
  }

  const setFilters = (newFilters: Record<string, unknown>) => {
    filters.value = newFilters
    page.value = 1 // Reset to first page when filtering
  }

  const clearSelection = () => {
    rowSelection.value = {}
  }

  const reset = () => {
    data.value = []
    columns.value = []
    rowSelection.value = {}
    sorting.value = []
    page.value = 1
    pageSize.value = 25
    search.value = ''
    filters.value = {}
  }

  return {
    // State
    data,
    columns,
    rowSelection,
    sorting,
    page,
    pageSize,
    search,
    filters,

    // Computed
    filteredData,
    sortedData,
    pageCount,
    pagedData,
    selectedRows,
    selectedCount,
    hasSelection,

    // Actions
    setData,
    setColumns,
    setRowSelection,
    setSorting,
    setPage,
    setPageSize,
    setSearch,
    setFilters,
    clearSelection,
    reset,
  }
})
