<template>
  <div class="space-y-4">
    <h3 class="text-lg font-semibold">Results</h3>
    <UTable :data="pagedRows" :columns="columns" class="text-base" />
    <div class="flex items-center justify-between">
      <span class="text-xs text-muted">Page {{ page }} / {{ pageCount }}</span>
      <UPagination v-model:page="page" :total="rows.length" :items-per-page="pageSize" />
    </div>

    <TranslationEditor v-model:open="editorOpen" :item="editingItem" @save="onSave" />
  </div>
 </template>

<script setup lang="ts">
import { computed, ref, h, resolveComponent } from 'vue'
import type { Component } from 'vue'
import type { TextUnit } from '~/types/translation'
import TranslationEditor from '~/components/translation/TranslationEditor.vue'
import { useTranslation } from '~/composables/useTranslation'
import type { TableColumn } from '#ui/types'

const props = defineProps<{ items: TextUnit[] }>()
const emit = defineEmits<{ (e: 'save', payload: { id: string; translated_text: string }): void }>()
const { isBusy } = useTranslation()

type Row = { id: string; source_text: string; translated_text: string }
const rows = computed<Row[]>(() => props.items.map(u => ({
  id: u.id,
  prompt_type: u.prompt_type,
  source_text: u.source_text,
  translated_text: u.translated_text ?? ''
})))

const page = ref(1)
const pageSize = ref(25)
const pageCount = computed(() => Math.max(1, Math.ceil(rows.value.length / pageSize.value)))
const pagedRows = computed(() => {
  const start = (page.value - 1) * pageSize.value
  return rows.value.slice(start, start + pageSize.value)
})

const columns: TableColumn<Row>[] = [
  { accessorKey: 'prompt_type', header: 'Type', enableSorting: true },
  { accessorKey: 'source_text', header: 'Source', enableSorting: false },
  { accessorKey: 'translated_text', header: 'Translated', enableSorting: false },
  {
    id: 'actions',
    header: 'Actions',
    enableSorting: false,
    cell: ({ row }) => {
      const UButton = resolveComponent('UButton') as Component
      return h(UButton, {
        size: 'xs',
        color: 'neutral',
        icon: 'i-heroicons-pencil',
        disabled: isBusy.value || editorOpen.value,
        onClick: () => openEditor(row.original.id)
      }, { default: () => 'Edit' })
    }
  }
]

const editorOpen = ref(false)
const editingItem = ref<TextUnit | null>(null)

const openEditor = (id: string) => {
  const unit = props.items.find(u => u.id === id) || null
  editingItem.value = unit
  editorOpen.value = !!unit
}

function onSave(payload: { id: string; translated_text: string }) {
  emit('save', payload)
  editorOpen.value = false
}
</script>


