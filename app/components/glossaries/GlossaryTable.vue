<template>
  <div class="space-y-4">
    <UCard>
      <template #header>
        <div class="flex flex-col gap-3">
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-3">
              <h2 class="text-xl font-semibold">Glossary</h2>
              <UBadge variant="soft">{{ glossary.terms.length }} terms</UBadge>
            </div>
            <div class="flex gap-2">
              <UButton color="primary" icon="i-heroicons-plus" @click="openAdd">Add</UButton>
              <UButton color="neutral" icon="i-heroicons-arrow-path" :loading="isLoading" @click="reload">Reload</UButton>
            </div>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-5 gap-3">
            <UInput v-model="search" icon="i-heroicons-magnifying-glass" placeholder="Search input/output…" />

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
          <span class="text-xs text-muted">Page {{ page }} / {{ pageCount }}</span>
          <UPagination v-model:page="page" :total="rows.length" :items-per-page="pageSize" />
        </div>
      </template>
    </UCard>

    <GlossaryForm v-model:open="modalOpen" :term="activeTerm" heading="Glossary Term" @save="save" />
  </div>
  
</template>

<script setup lang="ts">
import { computed, ref, onMounted, h, resolveComponent, watch } from 'vue'
import type { Component } from 'vue'
import type { TableColumn } from '#ui/types'
import { useGlossaryStore } from '~/stores/glossary'
import { useLanguageStore } from '~/stores/language'
import type { GlossaryQuery, GlossaryTerm } from '~/types/glossary'
import GlossaryForm from '~/components/glossaries/GlossaryForm.vue'

const glossary = useGlossaryStore()
const language = useLanguageStore()

type Row = { _id: number; enabled: boolean; category: string; input: string; output: string }

const search = ref('')
const filteredTerms = computed(() => {
  const q = search.value.trim().toLowerCase()
  return glossary.terms.filter(t => {
    if (onlyEnabled.value && !t.enabled) return false
    if (categoryFilter.value && categoryFilter.value !== 'All' && t.category !== categoryFilter.value) return false
    if (t.source_lang !== sourceFilter.value) return false
    if (t.target_lang !== targetFilter.value) return false
    if (!q) return true
    const hay = `${t.input}\u0000${t.output}\u0000${t.category}`.toLowerCase()
    return hay.includes(q)
  })
})

const rows = computed<Row[]>(() => filteredTerms.value.map(t => ({
  _id: t.id,
  enabled: !!t.enabled,
  category: t.category,
  input: t.input,
  output: t.output
})))

const page = ref(1)
const pageSize = ref(25)
const pageCount = computed(() => Math.max(1, Math.ceil(rows.value.length / pageSize.value)))
const pagedRows = computed<Row[]>(() => {
  const start = (page.value - 1) * pageSize.value
  return rows.value.slice(start, start + pageSize.value)
})

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
          size: 'xs', color: 'neutral', variant: 'soft', icon: 'i-heroicons-pencil',
          disabled: modalOpen.value,
          onClick: () => openEdit(row.original._id)
        }, { default: () => 'Edit' }),
        h(UButton, {
          size: 'xs', color: 'error', variant: 'soft', icon: 'i-heroicons-trash',
          disabled: modalOpen.value,
          onClick: () => remove(row.original._id)
        }, { default: () => 'Delete' })
      ])
    }
  }
]

const isLoading = computed(() => glossary.isLoading)
const error = computed(() => glossary.error)

const categoryItems = ['All', 'Characters', 'Essential Terms', 'Status Effects', 'Mechanics', 'Translation Rules', 'Locations', 'Time & Weather']
const categoryFilter = ref<string>('All')
const languageItems = computed(() => language.languageOptions.map(l => ({ label: l.label, value: l.id })))
const sourceFilter = ref<string>(language.getLanguage.source_language?.id ?? 'en')
const targetFilter = ref<string>(language.getLanguage.target_language?.id ?? 'fr')
const onlyEnabled = ref<boolean>(false)

async function reload() {
  const q: GlossaryQuery = {
    source_lang: sourceFilter.value,
    target_lang: targetFilter.value,
    categories: categoryFilter.value && categoryFilter.value !== 'All' ? [categoryFilter.value] : [],
    prompt_types: [],
    only_enabled: onlyEnabled.value,
    limit: 500,
  }
  await glossary.fetchTerms(q)
  page.value = 1
}

const modalOpen = ref(false)
const activeTerm = ref<GlossaryTerm | null>(null)

function openAdd() {
  activeTerm.value = {
    id: 0,
    category: 'Characters',
    source_lang: language.getLanguage.source_language?.id ?? 'en',
    target_lang: language.getLanguage.target_language?.id ?? 'fr',
    input: '',
    output: '',
    enabled: true
  }
  modalOpen.value = true
}

function openEdit(id: number) {
  const t = glossary.terms.find(x => x.id === id)
  if (!t) return
  activeTerm.value = { ...t }
  modalOpen.value = true
}

async function save(term: GlossaryTerm) {
  await glossary.upsertTerm(term)
  modalOpen.value = false
  await reload()
}

async function remove(id: number) {
  await glossary.deleteTerm(id)
  await reload()
}

async function toggleEnabled(id: number, v: boolean) {
  const t = glossary.terms.find(x => x.id === id)
  if (!t) return
  await glossary.upsertTerm({ ...t, enabled: v })
  await reload()
}

watch([categoryFilter, sourceFilter, targetFilter, onlyEnabled, search], () => { page.value = 1 })

onMounted(() => { void reload() })
</script>


