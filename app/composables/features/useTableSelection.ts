import { ref, computed, type Ref } from 'vue'

export interface TableSelectionState<T = unknown> {
  rowSelection: Ref<Record<string, boolean>>
  selectedRows: Ref<T[]>
  selectedRowsCount: Ref<number>
  isAllSelected: Ref<boolean>
  isSomeSelected: Ref<boolean>
  isIndeterminate: Ref<boolean>
  hasSelection: Ref<boolean>
}

export interface TableSelectionActions<_T = unknown> {
  clearSelection: () => void
  selectAll: () => void
  toggleRow: (rowId: string) => void
  selectRow: (rowId: string) => void
  deselectRow: (rowId: string) => void
  toggleAllRows: () => void
}

export function useTableSelection<T = unknown>(
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  tableRef?: Ref<any>
): TableSelectionState<T> & TableSelectionActions<T> {
  
  // State
  const rowSelection = ref<Record<string, boolean>>({})

  // Computed
  const selectedRows = computed((): T[] => {
    if (tableRef?.value?.tableApi) {
      const selectedRowModel = tableRef.value.tableApi.getFilteredSelectedRowModel()
      return selectedRowModel?.rows?.map((row: { original: T }) => row.original) || []
    }
    return []
  })

  const selectedRowsCount = computed(() => {
    if (tableRef?.value?.tableApi) {
      return tableRef.value.tableApi.getFilteredSelectedRowModel().rows.length || 0
    }
    return Object.keys(rowSelection.value).filter(key => rowSelection.value[key]).length
  })

  const isAllSelected = computed(() => {
    if (tableRef?.value?.tableApi) {
      return tableRef.value.tableApi.getIsAllPageRowsSelected()
    }
    return selectedRowsCount.value > 0 && selectedRowsCount.value === totalRows.value
  })

  const isSomeSelected = computed(() => {
    if (tableRef?.value?.tableApi) {
      return tableRef.value.tableApi.getIsSomePageRowsSelected()
    }
    return selectedRowsCount.value > 0 && selectedRowsCount.value < totalRows.value
  })

  const isIndeterminate = computed(() => {
    return isSomeSelected.value && !isAllSelected.value
  })

  const hasSelection = computed(() => selectedRowsCount.value > 0)

  const totalRows = computed(() => {
    if (tableRef?.value?.tableApi) {
      return tableRef.value.tableApi.getFilteredRowModel().rows.length || 0
    }
    return 0
  })

  // Actions
  const clearSelection = () => {
    rowSelection.value = {}
    if (tableRef?.value?.tableApi) {
      tableRef.value.tableApi.resetRowSelection()
    }
  }

  const selectAll = () => {
    if (tableRef?.value?.tableApi) {
      tableRef.value.tableApi.toggleAllPageRowsSelected(true)
    }
  }

  const toggleRow = (rowId: string) => {
    if (tableRef?.value?.tableApi) {
      const row = tableRef.value.tableApi.getRow(rowId)
      if (row) {
        row.toggleSelected()
      }
    } else {
      rowSelection.value[rowId] = !rowSelection.value[rowId]
    }
  }

  const selectRow = (rowId: string) => {
    if (tableRef?.value?.tableApi) {
      const row = tableRef.value.tableApi.getRow(rowId)
      if (row) {
        row.toggleSelected(true)
      }
    } else {
      rowSelection.value[rowId] = true
    }
  }

  const deselectRow = (rowId: string) => {
    if (tableRef?.value?.tableApi) {
      const row = tableRef.value.tableApi.getRow(rowId)
      if (row) {
        row.toggleSelected(false)
      }
    } else {
      rowSelection.value[rowId] = false
    }
  }

  const toggleAllRows = () => {
    if (tableRef?.value?.tableApi) {
      tableRef.value.tableApi.toggleAllPageRowsSelected()
    } else {
      const allSelected = isAllSelected.value
      Object.keys(rowSelection.value).forEach(key => {
        rowSelection.value[key] = !allSelected
      })
    }
  }

  return {
    // State
    rowSelection,
    selectedRows,
    selectedRowsCount,
    isAllSelected,
    isSomeSelected,
    isIndeterminate,
    hasSelection,
    // Actions
    clearSelection,
    selectAll,
    toggleRow,
    selectRow,
    deselectRow,
    toggleAllRows
  }
}
