<template>
  <div class="space-y-6 w-full">
    <!-- Filters Section -->
    <UCard>
      <template #header>
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <UIcon name="i-lucide-filter" class="text-gray-500 w-4 h-4" />
            <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Filters & Search</span>
          </div>
          <div class="flex items-center gap-2">
            <UBadge color="primary" variant="soft" size="sm">
              {{ filteredTranslations.length }} translations
            </UBadge>
          </div>
        </div>
      </template>

      <div class="space-y-4">
        <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
          <!-- Search Filter -->
          <UFormField label="Search">
            <UInput 
              v-model="search" 
              icon="i-lucide-search" 
              placeholder="Search translations..."
              size="sm"
            />
          </UFormField>

          <!-- Status Filter -->
          <UFormField label="Status">
            <USelect 
              v-model="statusFilter" 
              :items="statusOptions" 
              placeholder="All statuses"
              size="sm"
              class="w-full"
            />
          </UFormField>

          <!-- Type Filter -->
          <UFormField label="Type">
            <USelect 
              v-model="promptTypeFilter" 
              :items="promptTypeOptions" 
              placeholder="All types"
              size="sm"
              class="w-full"
            />
          </UFormField>

          <!-- Clear Filters -->
          <UFormField label="Actions">
            <UButton 
              variant="soft" 
              icon="i-lucide-x" 
              size="sm"
              @click="clearFilters"
            >
              Clear Filters
            </UButton>
          </UFormField>
        </div>
      </div>
    </UCard>

    <!-- Bulk Actions -->
    <UAlert
      v-if="selectedRowsCount > 0"
      color="warning"
      variant="soft"
      icon="i-lucide-alert-triangle"
      :title="`${selectedRowsCount} item(s) selected`"
      class="p-4"
    >
      <template #default>
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <UButton 
              color="error" 
              variant="soft"
              icon="i-lucide-trash" 
              size="sm"
              @click="handleBulkDelete"
            >
              Delete {{ selectedRowsCount }}
            </UButton>
          </div>
          <UButton 
            color="neutral" 
            variant="ghost" 
            size="sm"
            icon="i-lucide-x" 
            @click="clearSelection"
          >
            Clear Selection
          </UButton>
        </div>
      </template>
    </UAlert>

    <!-- Project Management Section -->
    <UCard>
      <template #header>
        <div class="flex items-center gap-3">
          <UIcon name="i-lucide-folder" class="text-gray-500 w-4 h-4" />
          <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Project Management</span>
        </div>
      </template>

      <div class="flex items-center gap-3">
        <USelect 
          v-model="selectedProject" 
          :items="projectOptions" 
          placeholder="Select a project to manage..."
          icon="i-lucide-folder"
          class="flex-1"
          size="sm"
        />
        
        <UButton 
          v-if="selectedProject && selectedProject !== 'none'" 
          color="error" 
          variant="soft"
          icon="i-lucide-trash-2" 
          :loading="isDeletingProject"
          size="sm"
          @click="handleDeleteProject"
        >
          Delete Project
        </UButton>
      </div>
    </UCard>

    <!-- Translations Table -->
    <UCard class="w-full">
      <template #header>
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <UIcon name="i-lucide-table" class="text-gray-500 w-4 h-4" />
            <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Translation Records</span>
          </div>
          <div class="flex items-center gap-2">
            <UButton 
              color="primary" 
              icon="i-lucide-refresh-cw" 
              :loading="isLoading" 
              size="sm"
              @click="() => loadTranslations()"
            >
              Reload
            </UButton>
          </div>
        </div>
      </template>

      <div v-if="error" class="mb-4">
        <UAlert color="error" variant="soft" :title="error" />
      </div>

      <UTable 
        ref="table"
        v-model:row-selection="rowSelection"
        :data="tableData" 
        :columns="columns" 
        :loading="isLoading" 
        class="text-sm w-full min-w-full"
        :empty-state="{ 
          icon: 'i-lucide-file-text', 
          label: 'No translations found', 
          description: 'Try loading a project first or adjusting your filters.' 
        }"
      >
        <template #source_text-data="{ row }">
          <div :class="isFullscreen ? 'max-w-md' : 'max-w-xs'" class="truncate" :title="row.original.source_text">
            {{ row.original.source_text }}
          </div>
        </template>

        <template #translated_text-data="{ row }">
          <div v-if="row.original.translated_text" :class="isFullscreen ? 'max-w-md' : 'max-w-xs'" class="truncate" :title="row.original.translated_text">
            {{ row.original.translated_text }}
          </div>
          <span v-else class="text-gray-400 italic">Not translated</span>
        </template>

        <template #status-data="{ row }">
          <UBadge :color="getStatusColor(row.original.status) as any" size="sm">
            {{ getStatusLabel(row.original.status) }}
          </UBadge>
        </template>
      </UTable>

      <template #footer>
        <div class="flex items-center justify-between w-full">
          <div class="flex items-center gap-4">
            <span class="text-xs text-gray-500 dark:text-gray-400">
              <template v-if="displayMode === 'paginated'">
                Showing {{ pagedTranslations.length }} of {{ filteredTranslations.length }} translations
              </template>
              <template v-else>
                Showing all {{ filteredTranslations.length }} translations
              </template>
            </span>
            <span class="text-xs text-gray-500 dark:text-gray-400">
              {{ selectedRowsCount }} of {{ tableData.length }} row(s) selected
            </span>
          </div>
          <div class="flex items-center gap-3">
            <template v-if="displayMode === 'paginated'">
              <span class="text-xs text-gray-500 dark:text-gray-400">Page {{ page }} / {{ pageCount }}</span>
              <UPagination v-model:page="page" :total="filteredTranslations.length" :items-per-page="pageSize" size="sm" />
            </template>
            <template v-else>
              <UBadge color="info" variant="soft" size="sm">
                <i-lucide-list class="w-3 h-3 mr-1" />
                All Data Mode
              </UBadge>
            </template>
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
import { ref, computed, onMounted, onUnmounted, watch, h, resolveComponent, type Component } from 'vue'
import type { TableColumn } from '#ui/types'
import TranslationForm from '~/components/translations/TranslationForm.vue'
import type { TextUnitRecord, TranslationStatus } from '~/types/translation'
import { useTranslations } from '~/composables/useTranslations'
import { useAppToast } from '~/composables/useAppToast'
import { promptTypeFilterOptions, statusFilterOptions } from '~/utils/translation'
// Tauri API will be imported dynamically when needed

const { showToast } = useAppToast()

// Fullscreen detection
const isFullscreen = ref(false)

// Window resize handler
const handleResize = () => {
  isFullscreen.value = window.innerWidth >= 1920; // Consider 1920px+ as fullscreen
}

// Lifecycle hooks
onMounted(() => {
  handleResize()
  window.addEventListener('resize', handleResize)
})

onUnmounted(() => {
  window.removeEventListener('resize', handleResize)
})

// Use shared utilities for filter options
const statusOptions = statusFilterOptions
const promptTypeOptions = promptTypeFilterOptions

const {
  // State
  isLoading,
  error,
  
  // Filter options (using local constants from shared utilities)
  
  // Computed
  allTranslations,
  
  // Helper functions
  getFilteredData,
  getPagedData,
  getPageCount,
  getDefaultFilters,
  
  // Actions
  loadTranslations,
  updateTranslation,
  deleteTranslation,
  bulkDeleteTranslations,
  getTranslation,
  getStatusLabel,
  getStatusColor
} = useTranslations()

const UCheckbox = resolveComponent('UCheckbox') as Component

const table = useTemplateRef<{ tableApi?: { getFilteredSelectedRowModel: () => { rows: { original: TextUnitRecord }[] } } }>('table')
const rowSelection = ref({})

// Filter state (like TranslationRaw.vue)
const search = ref('')
const statusFilter = ref<string>('All')
const promptTypeFilter = ref<string>('All')

// Pagination state (like TranslationRaw.vue)
const page = ref(1)
const pageSize = ref(50)

// Display mode toggle
const displayMode = ref<'paginated' | 'all'>('paginated')

// Computed filtering
const filteredTranslations = computed(() => {
  return getFilteredData(allTranslations.value, search.value, statusFilter.value, promptTypeFilter.value)
})

// Computed pagination
const pagedTranslations = computed(() => {
  return getPagedData(filteredTranslations.value, page.value, pageSize.value)
})

const pageCount = computed(() => 
  getPageCount(filteredTranslations.value.length, pageSize.value)
)

// Computed data source based on display mode
const tableData = computed(() => {
  return displayMode.value === 'all' ? allTranslations.value : pagedTranslations.value
})

// Reset page when switching to all data mode
watch(displayMode, (newMode) => {
  if (newMode === 'all') {
    page.value = 1
  }
})

// Clear filters function
function clearFilters() {
  const defaults = getDefaultFilters()
  search.value = defaults.search
  statusFilter.value = defaults.statusFilter
  promptTypeFilter.value = defaults.promptTypeFilter
  page.value = 1
}

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
  { label: 'All Projects', value: 'none' }
])

// Load available projects
async function loadAvailableProjects() {
  try {
    // Get list of projects from database/manifest files
    const { invoke } = await import('@tauri-apps/api/core')
    const projects = await invoke('get_available_projects') as { name: string; path: string; hash: string }[]
    
    projectOptions.value = [
      { label: 'All Projects', value: 'none' },
      ...projects.map(project => ({
        label: project.name || project.path.split('/').pop() || 'Unknown Project',
        value: project.hash
      }))
    ]
  } catch (error) {
    console.error('Error loading projects:', error)
    // Fallback to empty list if command doesn't exist yet
    projectOptions.value = [
      { label: 'All Projects', value: 'none' }
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
  await updateTranslation(id, translatedText, status as TranslationStatus)
  modalOpen.value = false
}

// Load data on mount
onMounted(() => {
  loadTranslations()
  loadAvailableProjects()
})
</script>
