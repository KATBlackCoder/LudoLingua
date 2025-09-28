<template>
  <div class="space-y-6">
    <DataTable
      ref="dataTableRef"
      :data="allTranslations as any"
      :columns="tableColumns as any"
      :loading="isLoading"
      title="Translations"
      icon="i-lucide-languages"
      :show-filters="true"
      :show-search="true"
      :show-selection="true"
      :show-selected-count="true"
      :show-bulk-actions="true"
      :bulk-actions="bulkActions"
      :show-pagination="true"
      :show-page-size-selector="true"
      :show-stats="true"
      :stats="translationStats"
      :custom-filters="false"
      filter-title="Translation Filters"
      filter-icon="i-lucide-filter"
      :show-status-filter="true"
      :show-prompt-type-filter="true"
      :show-project-filter="true"
      :project-filter-options="projectOptions"
      :show-project-actions="true"
      :on-delete-project="handleDeleteProject"
      :show-row-actions="true"
      :row-actions="rowActions"
      @selection-change="onSelectionChange as any"
      @bulk-action="onBulkAction"
      @row-action="onRowAction"
      @project-deleted="onProjectDeleted"
    />
  </div>
</template>

<script setup lang="ts">
import { computed, ref, onMounted, h, resolveComponent, type Component } from 'vue'
import type { TableColumn } from '#ui/types'
import type { TextUnitRecord } from '~/types/translation'
import { useTranslations } from '~/composables/useTranslations'
import { getPromptTypeColor } from '~/utils/table'
import DataTable from '~/components/shared/DataTable.vue'
import type { BulkAction } from '~/composables/features/useBulkActions'

// Composables
const {
  allTranslations,
  isLoading,
  loadTranslations,
  deleteTranslation: deleteTranslationAction,
  bulkDeleteTranslations,
  getStatusLabel,
  getStatusColor,
  loadAvailableProjects,
  deleteProject
} = useTranslations()


// Project options for the filter
const projectOptions = ref<Array<{ label: string; value: string }>>([])

// DataTable reference
const dataTableRef = ref()

// Selection state
const selectedRows = ref<TextUnitRecord[]>([])

// Table columns configuration for TranslationView
const tableColumns: TableColumn<TextUnitRecord>[] = [
  {
    accessorKey: 'status',
    header: 'Status',
    size: 140,
    cell: ({ row }) => {
      const UBadge = resolveComponent('UBadge') as Component
      return h(UBadge, {
        color: getStatusColor((row.original as TextUnitRecord).status) as 'primary' | 'secondary' | 'tertiary' | 'info' | 'success' | 'warning' | 'error' | 'neutral',
        variant: 'soft',
        size: 'sm'
      }, {
        default: () => getStatusLabel((row.original as TextUnitRecord).status)
      })
    }
  },
  {
    accessorKey: 'prompt_type',
    header: 'Type',
    size: 120,
    cell: ({ row }) => {
      const UBadge = resolveComponent('UBadge') as Component
      return h(UBadge, {
        color: getPromptTypeColor((row.original as TextUnitRecord).prompt_type),
        variant: 'soft',
        size: 'sm'
      }, {
        default: () => (row.original as TextUnitRecord).prompt_type
      })
    }
  },
  {
    accessorKey: 'project_path',
    header: 'Project',
    size: 200,
    cell: ({ row }) => {
      const projectPath = (row.original as TextUnitRecord).project_path
      const projectName = projectPath ? projectPath.split('/').pop() || projectPath : '—'
      return h('div', {
        class: 'max-w-xs truncate',
        title: projectPath || ''
      }, projectName)
    }
  },
  {
    accessorKey: 'source_text',
    header: 'Source Text',
    size: 300,
    cell: ({ row }) => {
      return h('div', {
        class: 'max-w-xs truncate',
        title: (row.original as TextUnitRecord).source_text
      }, (row.original as TextUnitRecord).source_text)
    }
  },
  {
    accessorKey: 'translated_text',
    header: 'Translation',
    size: 300,
    cell: ({ row }) => {
      const translatedText = (row.original as TextUnitRecord).translated_text
      return h('div', {
        class: 'max-w-xs truncate',
        title: translatedText || ''
      }, translatedText || '—')
    }
  },
]

// Computed stats for the table (DataTable will handle filtering)
const translationStats = computed(() => {
  const total = allTranslations.value.length
  const byStatus = allTranslations.value.reduce((acc, item) => {
    acc[item.status] = (acc[item.status] || 0) + 1
    return acc
  }, {} as Record<string, number>)

  return {
    total,
    ...byStatus
  }
})

// Bulk actions
const bulkActions = computed<BulkAction[]>(() => [
  {
    label: 'Delete Selected',
    color: 'error',
    variant: 'soft',
    icon: 'i-lucide-trash',
    onClick: async () => {
      if (selectedRows.value.length === 0) return
      const ids = selectedRows.value.map(row => row.id!).filter(id => id !== undefined)
      await bulkDeleteTranslations(ids)
      // Clear selection after bulk delete
      if (dataTableRef.value?.clearSelection) {
        dataTableRef.value.clearSelection()
      }
    }
  }
])

// Row actions configuration
const rowActions = computed(() => [
  {
    label: 'Edit',
    icon: 'i-lucide-edit',
    color: 'primary' as const,
    variant: 'soft' as const,
    onClick: (row: unknown) => editTranslation(row as TextUnitRecord)
  },
  {
    label: 'Delete',
    icon: 'i-lucide-trash',
    color: 'error' as const,
    variant: 'soft' as const,
    onClick: (row: unknown) => deleteTranslation((row as TextUnitRecord).id!),
    separator: true
  }
])

// Filter logic is now handled by DataTable built-in filters

// Event handlers
const onSelectionChange = (rows: TextUnitRecord[]) => {
  selectedRows.value = rows
  console.log('Selection changed:', rows.length, 'rows selected')
}

const onBulkAction = (action: BulkAction) => {
  // Handle bulk actions - the action.onClick is already called by DataTable
  console.log('Bulk action triggered:', action.label)
}

const onRowAction = (action: { type: string; row: unknown }) => {
  // Handle row actions - actions are already handled by onClick in rowActions
  console.log('Row action triggered:', action.type, action.row as TextUnitRecord)
}

const editTranslation = (translation: TextUnitRecord) => {
  // TODO: Implement edit functionality
  console.log('Edit translation:', translation)
}

const deleteTranslation = async (id?: number) => {
  if (!id) return
  await deleteTranslationAction(id)
}

// Load project options for the filter
const loadProjectOptions = async () => {
  projectOptions.value = await loadAvailableProjects()
}

// Project management functions
const handleDeleteProject = async (projectHash: string, projectName: string): Promise<boolean> => {
  try {
    const success = await deleteProject(projectHash, projectName)
    return success
  } catch (error) {
    console.error('Error deleting project:', error)
    return false
  }
}

const onProjectDeleted = async (projectHash: string) => {
  console.log('Project deleted:', projectHash)
  
  try {
    // Small delay to ensure database operations are complete
    await new Promise(resolve => setTimeout(resolve, 100))
    
    // Reload project options from database
    console.log('Reloading project options from database...')
    await loadProjectOptions()
    
    // Reload translations from database to reflect the deletion
    console.log('Reloading translations from database...')
    await loadTranslations()
    
    // Clear any table selections since the data has changed
    if (dataTableRef.value?.clearSelection) {
      dataTableRef.value.clearSelection()
    }
    
    console.log('Database successfully reloaded after project deletion')
  } catch (error) {
    console.error('Error reloading data after project deletion:', error)
  }
}

// Load data on mount
onMounted(async () => {
  await loadTranslations()
  await loadProjectOptions()
})
</script>