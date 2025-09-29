<template>
  <div class="space-y-6 w-full">
    <!-- Error Display -->
    <div v-if="error" class="mb-4">
      <UAlert color="error" variant="soft" :title="error" />
    </div>

    <!-- DataTable Component -->
    <DataTable
      :data="rows"
      :columns="columns"
      :loading="isLoading"
      title="Glossary Terms"
      icon="i-lucide-book-open"
      :show-filters="true"
      :show-search="true"
      :show-category-filter="true"
      :show-selection="false"
      :show-bulk-actions="false"
      :show-pagination="true"
      :show-row-count="true"
      :custom-filters="true"
      :category-filter-options="categoryFilterOptions"
      :header-actions="headerActions"
      :initial-page-size="25"
      :page-size-options="[
        { label: '10', value: 10 },
        { label: '25', value: 25 },
        { label: '50', value: 50 },
        { label: '100', value: 100 }
      ]"
      :filter-function="customFilterFunction"
    >
      <!-- Custom filters slot -->
      <template #filters>
        <div class="space-y-1">
          <label class="text-sm font-medium text-gray-700 dark:text-gray-300">Status</label>
          <div class="flex items-center gap-2 p-2 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
            <USwitch v-model="onlyEnabled" size="sm" />
            <span class="text-sm text-gray-600 dark:text-gray-400">Only enabled</span>
          </div>
        </div>
      </template>
      <!-- Custom cell slots -->
      <template #enabled-data="{ row }">
        <USwitch 
          :model-value="(row.original as Row).enabled" 
          size="sm"
          @update:model-value="(v:boolean) => toggleEnabled((row.original as Row)._id, v)" 
        />
      </template>
      
      <template #category-data="{ row }">
        <UBadge color="neutral" variant="soft" size="sm">
          {{ (row.original as Row).category }}
        </UBadge>
      </template>
    </DataTable>

    <GlossaryForm v-model:open="modalOpen" :term="activeTerm" heading="Glossary Term" @save="onSave" />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, h, resolveComponent, type Component } from 'vue'
import type { TableColumn } from '#ui/types'
import DataTable from '~/components/shared/DataTable.vue'
import GlossaryForm from '~/components/glossaries/GlossaryForm.vue'
import type { GlossaryTerm } from '~/types/glossary'
import { useGlossary } from '~/composables/useGlossary'
import type { ActionButton } from '~/components/shared/ActionButtonGroup.vue'

const {
  // state
  isLoading,
  error,
  // filters
  categoryItems,
  sourceFilter,
  targetFilter,
  onlyEnabled,
  // table
  rows,
  // actions
  reload,
  save,
  remove,
  toggleEnabled,
  exportJson,
  importJson,
  getTermById,
} = useGlossary()

type Row = { _id: number; enabled: boolean; category: string; input: string; output: string; source_lang: string; target_lang: string }

const columns: TableColumn<unknown>[] = [
  { accessorKey: 'enabled', header: 'On', enableSorting: false },
  { accessorKey: 'category', header: 'Category' },
  { accessorKey: 'input', header: 'Input' },
  { accessorKey: 'output', header: 'Output' },
  {
    id: 'actions',
    header: 'Actions',
    enableSorting: false,
    cell: ({ row }) => {
      const UButton = resolveComponent('UButton') as Component
      const rowData = row.original as Row
      return h('div', { class: 'flex gap-2' }, [
        h(UButton, {
          size: 'xs', color: 'neutral', variant: 'soft', icon: 'i-lucide-pencil',
          disabled: modalOpen.value,
          onClick: () => openEdit(rowData._id)
        }, { default: () => 'Edit' }),
        h(UButton, {
          size: 'xs', color: 'error', variant: 'soft', icon: 'i-lucide-trash',
          disabled: modalOpen.value,
          onClick: () => remove(rowData._id)
        }, { default: () => 'Delete' })
      ])
    }
  }
]

// Convert filter options to the expected format
const categoryFilterOptions = computed(() => 
  categoryItems.map(item => ({ label: item, value: item }))
)


// Header actions for the DataTable
const headerActions = computed((): ActionButton[] => [
  {
    id: 'add-term',
    label: 'Add Term',
    icon: 'i-lucide-plus',
    color: 'primary',
    variant: 'solid',
    onClick: openAdd
  },
  {
    id: 'reload',
    label: 'Reload',
    icon: 'i-lucide-refresh-cw',
    color: 'info',
    variant: 'soft',
    loading: isLoading,
    onClick: reload
  },
  {
    id: 'export',
    label: 'Export JSON',
    icon: 'i-lucide-download',
    color: 'success',
    variant: 'soft',
    onClick: exportJson
  },
  {
    id: 'import',
    label: 'Import JSON',
    icon: 'i-lucide-upload',
    color: 'warning',
    variant: 'soft',
    onClick: importJson
  }
])

// Custom filter function to handle the "only enabled" filter
// Note: This function is called AFTER DataTable's built-in filters, so we only need to handle our custom logic
const customFilterFunction = (data: unknown[], _searchQuery: string) => {
  let filtered = data as Row[]

  // Apply "only enabled" filter
  if (onlyEnabled.value) {
    filtered = filtered.filter(item => item.enabled)
  }

  return filtered
}

const modalOpen = ref(false)
const activeTerm = ref<GlossaryTerm | null>(null)

function openAdd() {
  activeTerm.value = {
    id: 0,
    category: 'Characters',
    source_lang: sourceFilter.value || 'en',
    target_lang: targetFilter.value || 'fr',
    input: '',
    output: '',
    enabled: true
  }
  modalOpen.value = true
}

function openEdit(id: number) {
  const t = getTermById(id)
  if (!t) return
  activeTerm.value = { ...t }
  modalOpen.value = true
}

async function onSave(term: GlossaryTerm) { 
  await save(term)
  modalOpen.value = false 
}

// Initial load
void reload()
</script>


