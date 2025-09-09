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
                v-if="selectedRows.length > 0" 
                color="error" 
                icon="i-heroicons-trash" 
                @click="handleBulkDelete"
              >
                Delete {{ selectedRows.length }}
              </UButton>
              <UButton color="primary" icon="i-heroicons-arrow-path" :loading="isLoading" @click="() => loadTranslations()">
                Reload
              </UButton>
            </div>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-4 gap-3">
            <UInput 
              v-model="search" 
              icon="i-heroicons-magnifying-glass" 
              placeholder="Search source/translated text…" 
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
              icon="i-heroicons-x-mark" 
              @click="clearFilters"
            >
              Clear Filters
            </UButton>
          </div>
        </div>
      </template>

      <div v-if="error" class="text-red-500 text-sm mb-2">{{ error }}</div>

      <UTable 
        v-model:selected="selectedRows"
        :data="pagedTranslations" 
        :columns="columns" 
        :loading="isLoading" 
        class="text-base"
        :empty-state="{ 
          icon: 'i-heroicons-document-text', 
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

        <template #actions-data="{ row }">
          <div class="flex gap-1">
            <UButton 
              size="xs" 
              color="neutral" 
              variant="soft" 
              icon="i-heroicons-pencil" 
              @click="openEdit(row.original.id!)"
            >
              Edit
            </UButton>
            <UButton 
              size="xs" 
              color="error" 
              variant="soft" 
              icon="i-heroicons-trash" 
              @click="handleDelete(row.original.id!)"
            >
              Delete
            </UButton>
          </div>
        </template>
      </UTable>

      <template #footer>
        <div class="flex items-center justify-between w-full">
          <span class="text-xs text-muted">
            Showing {{ pagedTranslations.length }} of {{ filteredTranslations.length }} translations
          </span>
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
import { ref, onMounted } from 'vue'
import type { TableColumn } from '#ui/types'
import TranslationForm from '~/components/translations/TranslationForm.vue'
import type { TextUnitRecord } from '~/stores/translations'
import { useTranslations } from '~/composables/useTranslations'

const {
  // State
  translations,
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

const selectedRows = ref<TextUnitRecord[]>([])

const columns: TableColumn<TextUnitRecord>[] = [
  { accessorKey: 'source_text', header: 'Source Text' },
  { accessorKey: 'translated_text', header: 'Translation' },
  { accessorKey: 'status', header: 'Status' },
  { accessorKey: 'prompt_type', header: 'Type' },
  { accessorKey: 'file_path', header: 'File' },
  { accessorKey: 'actions', header: 'Actions', enableSorting: false }
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
  if (confirm('Are you sure you want to delete this translation?')) {
    await deleteTranslation(id)
  }
}

async function handleBulkDelete() {
  const ids = selectedRows.value.map(row => row.id).filter(Boolean) as number[]
  if (ids.length === 0) return
  
  if (confirm(`Are you sure you want to delete ${ids.length} translations?`)) {
    await bulkDeleteTranslations(ids)
    selectedRows.value = []
  }
}

async function handleFormSave(id: number, translatedText: string, status?: string) {
  await updateTranslation(id, translatedText, status)
  modalOpen.value = false
}

// Load data on mount
onMounted(() => {
  loadTranslations()
})
</script>
