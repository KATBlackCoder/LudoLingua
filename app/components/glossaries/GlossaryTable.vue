<template>
  <div class="space-y-4">
    <div class="flex items-center justify-between">
      <h2 class="text-xl font-semibold">Glossary</h2>
      <div class="flex gap-2">
        <UButton color="primary" icon="i-heroicons-plus" @click="openAdd">Add</UButton>
        <UButton color="neutral" icon="i-heroicons-arrow-path" :loading="isLoading" @click="reload">Reload</UButton>
      </div>
    </div>

    <div v-if="error" class="text-red-500 text-sm">{{ error }}</div>

    <UTable :data="rows" :columns="columns" :loading="isLoading" class="text-base">
      <template #enabled-data="{ row }">
        <UBadge :color="row.original.enabled ? 'success' : 'neutral'">{{ row.original.enabled ? 'Yes' : 'No' }}</UBadge>
      </template>
    </UTable>

    <GlossaryForm v-model:open="modalOpen" :term="activeTerm" heading="Glossary Term" @save="save" />
  </div>
</template>

<script setup lang="ts">
import { computed, ref, onMounted, h, resolveComponent } from 'vue'
import type { Component } from 'vue'
import type { TableColumn } from '#ui/types'
import { useGlossaryStore } from '~/stores/glossary'
import { useLanguageStore } from '~/stores/language'
import type { GlossaryQuery, GlossaryTerm } from '~/types/glossary'
import GlossaryForm from '~/components/glossaries/GlossaryForm.vue'

const glossary = useGlossaryStore()
const language = useLanguageStore()

type Row = { _id: number; enabled: boolean; category: string; input: string; output: string }
const rows = computed<Row[]>(() => glossary.terms.map(t => ({
  _id: t.id,
  enabled: !!t.enabled,
  category: t.category,
  input: t.input,
  output: t.output
})))

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

async function reload() {
  const q: GlossaryQuery = {
    source_lang: language.getLanguage.source_language?.id ?? 'en',
    target_lang: language.getLanguage.target_language?.id ?? 'fr',
    categories: [],
    prompt_types: [],
    only_enabled: false,
    limit: 200,
  }
  await glossary.fetchTerms(q)
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

onMounted(() => { void reload() })
</script>


