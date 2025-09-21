<template>
  <div class="space-y-4">
    <UCard>
      <template #header>
        <div class="flex flex-col gap-3">
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-3">
              <h2 class="text-xl font-semibold">Translations</h2>
              <UBadge variant="soft">{{ filteredTranslations.length }} translations</UBadge>
            </div>
            <div class="flex gap-2">
              <UButton 
                v-if="selectedRowsCount > 0" 
                color="error" 
                icon="i-lucide-trash" 
                @click="handleBulkDelete"
              >
                Delete {{ selectedRowsCount }}
              </UButton>
              <UButton 
                v-if="selectedRowsCount > 0" 
                color="neutral" 
                variant="soft"
                icon="i-lucide-x" 
                @click="clearSelection"
              >
                Clear Selection
              </UButton>
              <UButton color="primary" icon="i-lucide-refresh-cw" :loading="isLoading" @click="() => loadTranslations()">
                Reload
              </UButton>
            </div>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-4 gap-3">
            <UInput 
              v-model="search" 
              icon="i-lucide-search" 
              placeholder="Search source/translated/field text…" 
            />

            <USelect 
              v-model="statusFilter" 
              :items="statusOptions" 
              placeholder="All statuses" 
            />

            <USelect 
              v-model="promptTypeFilter" 
              :items="promptTypeOptions" 
              placeholder="All types" 
            />

            <UButton 
              variant="soft" 
              icon="i-lucide-x" 
              @click="clearFilters"
            >
              Clear Filters
            </UButton>
          </div>

          <!-- Project Management Section -->
          <div class="flex items-center gap-3 pt-3 border-t border-gray-200 dark:border-gray-700">
            <USelect 
              v-model="selectedProject" 
              :items="projectOptions" 
              placeholder="Select a project to manage…"
              icon="i-lucide-folder"
              class="flex-1"
            />
            
            <UButton 
              v-if="selectedProject && selectedProject !== 'none'" 
              color="error" 
              variant="soft"
              icon="i-lucide-trash-2" 
              :loading="isDeletingProject"
              @click="handleDeleteProject"
            >
              Delete Project
            </UButton>
          </div>
        </div>
      </template>

      <div v-if="error" class="text-error-500 text-sm mb-2">{{ error }}</div>

      <UTable 
        ref="table"
        v-model:row-selection="rowSelection"
        :data="pagedTranslations" 
        :columns="columns" 
        :loading="isLoading" 
        class="text-base"
        :empty-state="{ 
          icon: 'i-lucide-file-text', 
          label: 'No translations found', 
          description: 'Try loading a project first or adjusting your filters.' 
        }"
      >
        <template #source_text-data="{ row }">
          <div class="max-w-xs truncate" :title="row.original.source_text">
            {{ row.original.source_text }}
          </div>
        </template>

        <template #translated_text-data="{ row }">
          <div v-if="row.original.translated_text" class="max-w-xs truncate" :title="row.original.translated_text">
            {{ row.original.translated_text }}
          </div>
          <span v-else class="text-gray-400 italic">Not translated</span>
        </template>

        <template #status-data="{ row }">
          <UBadge :color="getStatusColor(row.original.status) as any" size="sm">
            {{ getStatusLabel(row.original.status) }}
          </UBadge>
        </template>

        <!-- Actions column now uses render function approach in columns definition -->
      </UTable>

      <template #footer>
        <div class="flex items-center justify-between w-full">
          <div class="flex items-center gap-4">
            <span class="text-xs text-muted">
              Showing {{ pagedTranslations.length }} of {{ filteredTranslations.length }} translations
            </span>
            <span class="text-xs text-muted">
              {{ selectedRowsCount }} of {{ pagedTranslations.length }} row(s) selected
            </span>
          </div>
          <div class="flex items-center gap-3">
            <span class="text-xs text-muted">Page {{ page }} / {{ pageCount }}</span>
            <UPagination v-model:page="page" :total="filteredTranslations.length" :items-per-page="pageSize" />
          </div>
        </div>
      </template>
    </UCard>

    <TranslationForm 
      v-model:open="modalOpen" 
      :translation="activeTranslation" 
      @save="handleFormSave" 
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, h, resolveComponent, type Component } from 'vue'
import type { TableColumn } from '#ui/types'
import TranslationForm from '~/components/translations/TranslationForm.vue'
import type { TextUnitRecord } from '~/stores/translations'
import { useTranslations } from '~/composables/useTranslations'
import { useAppToast } from '~/composables/useAppToast'
// Tauri API will be imported dynamically when needed

const { showToast } = useAppToast()

const {
  // State
  isLoading,
  error,
  
  // UI State
  search,
  statusFilter,
  promptTypeFilter,
  page,
  pageSize,
  
  // Filter options
  statusOptions,
  promptTypeOptions,
  
  // Computed
  filteredTranslations,
  pagedTranslations,
  pageCount,
  
  // Actions
  loadTranslations,
  updateTranslation,
  deleteTranslation,
  bulkDeleteTranslations,
  getTranslation,
  clearFilters,
  getStatusLabel,
  getStatusColor
} = useTranslations()

const UCheckbox = resolveComponent('UCheckbox') as Component

const table = useTemplateRef<{ tableApi?: { getFilteredSelectedRowModel: () => { rows: { original: TextUnitRecord }[] } } }>('table')
const rowSelection = ref({})

// Computed property to get selected row count
const selectedRowsCount = computed((): number => {
  return table.value?.tableApi?.getFilteredSelectedRowModel().rows.length || 0
})

// Get selected rows for bulk operations
const getSelectedRows = (): TextUnitRecord[] => {
  return table.value?.tableApi?.getFilteredSelectedRowModel().rows.map((row: { original: TextUnitRecord }) => row.original) || []
}

// Clear selection
function clearSelection() {
  rowSelection.value = {}
}

// Project management state
const selectedProject = ref<string>('')
const isDeletingProject = ref(false)
const projectOptions = ref([
  { label: 'No project selected', value: 'none' }
])

// Load available projects
async function loadAvailableProjects() {
  try {
    // Get list of projects from database/manifest files
    const { invoke } = await import('@tauri-apps/api/core')
    const projects = await invoke('get_available_projects') as { name: string; path: string; hash: string }[]
    
    projectOptions.value = [
      { label: 'No project selected', value: 'none' },
      ...projects.map(project => ({
        label: project.name || project.path.split('/').pop() || 'Unknown Project',
        value: project.hash
      }))
    ]
  } catch (error) {
    console.error('Error loading projects:', error)
    // Fallback to empty list if command doesn't exist yet
    projectOptions.value = [
      { label: 'No project selected', value: 'none' }
    ]
  }
}

// Delete selected project
async function handleDeleteProject() {
  if (!selectedProject.value || selectedProject.value === 'none') return
  
  const projectName = projectOptions.value.find(p => p.value === selectedProject.value)?.label || 'Unknown Project'
  
  // Use native Tauri dialog for project deletion confirmation
  const { ask } = await import('@tauri-apps/plugin-dialog')
  
  const confirmed = await ask(
    `Are you sure you want to delete the project "${projectName}"?\n\nThis will permanently remove all translations and project data. This action cannot be undone.`, 
    {
      title: 'Delete Project',
      kind: 'warning'
    }
  )
  
  if (!confirmed) return
  
  try {
    isDeletingProject.value = true
    
    // Delete project and all its translations
    const { invoke } = await import('@tauri-apps/api/core')
    await invoke('delete_project', { projectHash: selectedProject.value })
    
    // Reset selection and reload data
    selectedProject.value = 'none'
    await loadAvailableProjects()
    await loadTranslations()
    
    // Show success message
    showToast(
      'Project Deleted',
      `Project "${projectName}" has been successfully deleted.`,
      'success',
      5000,
      'i-lucide-check-circle'
    )
  } catch (error) {
    console.error('Error deleting project:', error)
    
    // Show error message
    showToast(
      'Error Deleting Project',
      error instanceof Error ? error.message : 'Failed to delete project',
      'error',
      7000,
      'i-lucide-alert-triangle'
    )
  } finally {
    isDeletingProject.value = false
  }
}

const columns: TableColumn<TextUnitRecord>[] = [
  // Selection column for bulk operations
  {
    id: 'select',
    header: ({ table }) =>
      h(UCheckbox, {
        modelValue: table.getIsSomePageRowsSelected()
          ? 'indeterminate'
          : table.getIsAllPageRowsSelected(),
        'onUpdate:modelValue': (value: boolean | 'indeterminate') =>
          table.toggleAllPageRowsSelected(!!value),
        'aria-label': 'Select all'
      }),
    cell: ({ row }) =>
      h(UCheckbox, {
        modelValue: row.getIsSelected(),
        'onUpdate:modelValue': (value: boolean | 'indeterminate') => row.toggleSelected(!!value),
        'aria-label': 'Select row'
      })
  },
  { accessorKey: 'source_text', header: 'Source Text' },
  { accessorKey: 'translated_text', header: 'Translation' },
  { accessorKey: 'status', header: 'Status' },
  { accessorKey: 'prompt_type', header: 'Type' },
  { accessorKey: 'field_type', header: 'Field Type' },
  { accessorKey: 'file_path', header: 'File' },
  {
    id: 'actions',
    header: 'Actions',
    enableSorting: false,
    cell: ({ row }) => {
      const UButton = resolveComponent('UButton') as Component
      return h('div', { class: 'flex gap-1' }, [
        h(UButton, {
          size: 'xs', 
          color: 'neutral', 
          variant: 'soft', 
          icon: 'i-lucide-pencil',
          disabled: modalOpen.value,
          onClick: () => openEdit(row.original.id!)
        }, { default: () => 'Edit' }),
        h(UButton, {
          size: 'xs', 
          color: 'error', 
          variant: 'soft', 
          icon: 'i-lucide-trash',
          disabled: modalOpen.value,
          onClick: () => handleDelete(row.original.id!)
        }, { default: () => 'Delete' })
      ])
    }
  }
]

// Modal management
const modalOpen = ref(false)
const activeTranslation = ref<TextUnitRecord | null>(null)

async function openEdit(id: number) {
  const translation = await getTranslation(id)
  if (translation) {
    activeTranslation.value = translation
    modalOpen.value = true
  }
}

async function handleDelete(id: number) {
  // Use native Tauri dialog for better reliability
  const { ask } = await import('@tauri-apps/plugin-dialog')
  
  const confirmed = await ask('Are you sure you want to delete this translation?', {
    title: 'Delete Translation',
    kind: 'warning'
  })
  
  if (confirmed) {
    await deleteTranslation(id)
  }
}

async function handleBulkDelete() {
  const selectedRows = getSelectedRows()
  const ids = selectedRows.map(row => row.id).filter(Boolean) as number[]
  if (ids.length === 0) return
  
  // Use native Tauri dialog for bulk delete confirmation
  const { ask } = await import('@tauri-apps/plugin-dialog')
  
  const confirmed = await ask(
    `Are you sure you want to delete ${ids.length} translation${ids.length > 1 ? 's' : ''}?`, 
    {
      title: 'Delete Translations',
      kind: 'warning'
    }
  )
  
  if (confirmed) {
    const deletedCount = await bulkDeleteTranslations(ids)
    if (deletedCount > 0) {
      // Clear selection after successful deletion
      clearSelection()
    }
  }
}

async function handleFormSave(id: number, translatedText: string, status?: string) {
  await updateTranslation(id, translatedText, status)
  modalOpen.value = false
}

// Load data on mount
onMounted(() => {
  loadTranslations()
  loadAvailableProjects()
})
</script>
