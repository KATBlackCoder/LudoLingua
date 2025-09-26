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
              {{ rows.length }} terms
            </UBadge>
          </div>
        </div>
      </template>

      <div class="space-y-4">
        <div class="grid grid-cols-1 md:grid-cols-5 gap-4">
          <!-- Search Filter -->
          <UFormField label="Search">
            <UInput 
              v-model="search" 
              icon="i-lucide-search" 
              placeholder="Search terms..."
              size="sm"
            />
          </UFormField>

          <!-- Category Filter -->
          <UFormField label="Category">
            <USelect 
              v-model="categoryFilter" 
              :items="categoryItems" 
              placeholder="All categories"
              size="sm"
              class="w-full"
            />
          </UFormField>

          <!-- Source Language Filter -->
          <UFormField label="Source Language">
            <USelect 
              v-model="sourceFilter" 
              :items="languageItems" 
              placeholder="Source language"
              size="sm"
              class="w-full"
            />
          </UFormField>

          <!-- Target Language Filter -->
          <UFormField label="Target Language">
            <USelect 
              v-model="targetFilter" 
              :items="languageItems" 
              placeholder="Target language"
              size="sm"
              class="w-full"
            />
          </UFormField>

          <!-- Enabled Filter -->
          <UFormField label="Status">
            <div class="flex items-center gap-2 p-2 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
              <USwitch v-model="onlyEnabled" size="sm" />
              <span class="text-sm text-gray-600 dark:text-gray-400">Only enabled</span>
            </div>
          </UFormField>
        </div>
      </div>
    </UCard>

    <!-- Glossary Table -->
    <UCard class="w-full">
      <template #header>
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <UIcon name="i-lucide-table" class="text-gray-500 w-4 h-4" />
            <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Glossary Terms</span>
          </div>
          <div class="flex items-center gap-2">
            <UButton 
              color="primary" 
              icon="i-lucide-plus" 
              size="sm"
              @click="openAdd"
            >
              Add Term
            </UButton>
            <UButton 
              color="neutral" 
              icon="i-lucide-refresh-cw" 
              :loading="isLoading" 
              size="sm"
              @click="reload"
            >
              Reload
            </UButton>
          </div>
        </div>
      </template>

      <div v-if="error" class="mb-4">
        <UAlert color="error" variant="soft" :title="error" />
      </div>

      <UTable :data="pagedRows" :columns="columns" :loading="isLoading" class="text-sm w-full min-w-full">
        <template #enabled-data="{ row }">
          <USwitch 
            :model-value="row.original.enabled" 
            size="sm"
            @update:model-value="(v:boolean) => toggleEnabled(row.original._id, v)" 
          />
        </template>
        <template #category-data="{ row }">
          <UBadge color="neutral" variant="soft" size="sm">
            {{ row.original.category }}
          </UBadge>
        </template>
      </UTable>

      <template #footer>
        <div class="flex items-center justify-between w-full">
          <div class="flex items-center gap-3">
            <div class="flex items-center gap-2">
              <UButton 
                size="xs" 
                variant="soft" 
                icon="i-lucide-download" 
                @click="exportJson"
              >
                Export JSON
              </UButton>
              <UButton 
                size="xs" 
                variant="soft" 
                icon="i-lucide-upload" 
                @click="importJson"
              >
                Import JSON
              </UButton>
            </div>
            <span class="text-xs text-gray-500 dark:text-gray-400">
              Showing {{ pagedRows.length }} of {{ rows.length }} terms
            </span>
          </div>
          <div class="flex items-center gap-3">
            <span class="text-xs text-gray-500 dark:text-gray-400">Page {{ page }} / {{ pageCount }}</span>
            <UPagination v-model:page="page" :total="rows.length" :items-per-page="pageSize" size="sm" />
          </div>
        </div>
      </template>
    </UCard>

    <GlossaryForm v-model:open="modalOpen" :term="activeTerm" heading="Glossary Term" @save="onSave" />
  </div>
  
</template>

<script setup lang="ts">
import { ref, watch, h, resolveComponent, onMounted, onUnmounted, type Component } from 'vue'
import type { TableColumn } from '#ui/types'
import GlossaryForm from '~/components/glossaries/GlossaryForm.vue'
import type { GlossaryTerm } from '~/types/glossary'
import { useGlossary } from '~/composables/useGlossary'

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

const {
  // state
  isLoading,
  error,
  // filters
  search,
  categoryItems,
  categoryFilter,
  languageItems,
  sourceFilter,
  targetFilter,
  onlyEnabled,
  // table
  rows,
  pagedRows,
  page,
  pageSize,
  pageCount,
  // actions
  reload,
  save,
  remove,
  toggleEnabled,
  exportJson,
  importJson,
  getTermById,
} = useGlossary()

type Row = { _id: number; enabled: boolean; category: string; input: string; output: string }

const columns: TableColumn<Row>[] = [
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
      return h('div', { class: 'flex gap-2' }, [
        h(UButton, {
          size: 'xs', color: 'neutral', variant: 'soft', icon: 'i-lucide-pencil',
          disabled: modalOpen.value,
          onClick: () => openEdit(row.original._id)
        }, { default: () => 'Edit' }),
        h(UButton, {
          size: 'xs', color: 'error', variant: 'soft', icon: 'i-lucide-trash',
          disabled: modalOpen.value,
          onClick: () => remove(row.original._id)
        }, { default: () => 'Delete' })
      ])
    }
  }
]

// reload provided

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

async function onSave(term: GlossaryTerm) { await save(term); modalOpen.value = false }

watch([categoryFilter, sourceFilter, targetFilter, onlyEnabled, search], () => { page.value = 1 })

// initial load
void reload()

// exportJson/importJson provided

// no local wrappers
</script>


