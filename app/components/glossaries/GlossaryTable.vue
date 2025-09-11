<template>
  <div class="space-y-4">
    <UCard>
      <template #header>
        <div class="flex flex-col gap-3">
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-3">
              <h2 class="text-xl font-semibold">Glossary</h2>
              <UBadge variant="soft">{{ rows.length }} terms</UBadge>
            </div>
            <div class="flex gap-2">
              <UButton color="primary" icon="i-lucide-plus" @click="openAdd">Add</UButton>
              <UButton color="neutral" icon="i-lucide-refresh-cw" :loading="isLoading" @click="reload">Reload</UButton>
            </div>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-5 gap-3">
            <UInput v-model="search" icon="i-lucide-search" placeholder="Search input/output…" />

            <USelect v-model="categoryFilter" :items="categoryItems" placeholder="All categories" />

            <USelect v-model="sourceFilter" :items="languageItems" placeholder="Source language" />

            <USelect v-model="targetFilter" :items="languageItems" placeholder="Target language" />

            <div class="flex items-center gap-2">
              <USwitch v-model="onlyEnabled" />
              <span class="text-sm">Only enabled</span>
            </div>
          </div>
        </div>
      </template>

      <div v-if="error" class="text-red-500 text-sm mb-2">{{ error }}</div>

      <UTable :data="pagedRows" :columns="columns" :loading="isLoading" class="text-base">
        <template #enabled-data="{ row }">
          <USwitch :model-value="row.original.enabled" @update:model-value="(v:boolean) => toggleEnabled(row.original._id, v)" />
        </template>
      </UTable>

      <template #footer>
            <div class="flex items-center justify-between w-full">
              <div class="flex items-center gap-2">
                <UButton size="xs" variant="soft" icon="i-lucide-download" @click="exportJson">Export JSON</UButton>
                <UButton size="xs" variant="soft" icon="i-lucide-upload" @click="importJson">Import JSON</UButton>
              </div>
              <div class="flex items-center gap-3">
                <span class="text-xs text-muted">Page {{ page }} / {{ pageCount }}</span>
                <UPagination v-model:page="page" :total="rows.length" :items-per-page="pageSize" />
              </div>
            </div>
      </template>
    </UCard>

    <GlossaryForm v-model:open="modalOpen" :term="activeTerm" heading="Glossary Term" @save="onSave" />
  </div>
  
</template>

<script setup lang="ts">
import { ref, watch, h, resolveComponent, type Component } from 'vue'
import type { TableColumn } from '#ui/types'
import GlossaryForm from '~/components/glossaries/GlossaryForm.vue'
import type { GlossaryTerm } from '~/types/glossary'
import { useGlossary } from '~/composables/useGlossary'

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


