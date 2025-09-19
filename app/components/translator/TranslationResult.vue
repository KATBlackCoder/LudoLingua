<template>
  <div class="space-y-4">
    <div class="flex items-center justify-between">
      <h3 class="text-lg font-semibold">Results</h3>
      <UInput v-model="search" icon="i-lucide-search" placeholder="Search source/translated/type/field…" />
    </div>
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
import TranslationEditor from '~/components/translator/TranslationEditor.vue'
import { useTranslator } from '~/composables/useTranslator'
import type { TableColumn } from '#ui/types'
import { useGlossary } from '~/composables/useGlossary'
import type { GlossaryTerm } from '~/types/glossary'
import { useLanguageStore } from '~/stores/language'
import { useAppToast } from '~/composables/useAppToast'
import { useEngineStore } from '~/stores/engine'

const props = defineProps<{ items: TextUnit[] }>()
const emit = defineEmits<{
  (e: 'save', payload: { id: string; translated_text: string; prompt_type?: string }): void
  (e: 'remove', id: string): void
}>()
const { isBusy, retranslate, saveEdit } = useTranslator()
const engineStore = useEngineStore()
const glossary = useGlossary()
const languageStore = useLanguageStore()
const { showToast } = useAppToast()

const promptTypeToCategory: Record<string, string> = {
  Character: 'Characters',
  Class: 'Mechanics',
  Skill: 'Mechanics',
  Equipment: 'Mechanics',
  State: 'Status Effects',
  System: 'Essential Terms',
  Dialogue: 'Essential Terms',
  Other: 'Essential Terms',
}

type Row = { id: string; source_text: string; translated_text: string; prompt_type: string; field_type: string }
const rows = computed<Row[]>(() => props.items.map(u => ({
  id: u.id,
  prompt_type: u.prompt_type,
  source_text: u.source_text,
  translated_text: u.translated_text ?? '',
  field_type: u.field_type
})))

const page = ref(1)
const pageSize = ref(25)
const search = ref('')
const filteredRows = computed(() => {
  const q = search.value.trim().toLowerCase()
  if (!q) return rows.value
  return rows.value.filter(r =>
    r.source_text.toLowerCase().includes(q) ||
    r.translated_text.toLowerCase().includes(q) ||
    String(r.prompt_type || '').toLowerCase().includes(q) ||
    String(r.field_type || '').toLowerCase().includes(q)
  )
})
const pageCount = computed(() => Math.max(1, Math.ceil(filteredRows.value.length / pageSize.value)))
const pagedRows = computed(() => {
  const start = (page.value - 1) * pageSize.value
  return filteredRows.value.slice(start, start + pageSize.value)
})

const columns: TableColumn<Row>[] = [
  { accessorKey: 'prompt_type', header: 'Type', enableSorting: true },
  { accessorKey: 'field_type', header: 'Field Type', enableSorting: true },
  { accessorKey: 'source_text', header: 'Source', enableSorting: false },
  { accessorKey: 'translated_text', header: 'Translated', enableSorting: false },
  {
    id: 'actions',
    header: 'Actions',
    enableSorting: false,
    cell: ({ row }) => {
      const UButton = resolveComponent('UButton') as Component
      return h('div', { class: 'flex gap-2' }, [
        h(UButton, {
          size: 'xs',
          color: 'primary',
          variant: 'soft',
          icon: 'i-lucide-refresh-cw',
          disabled: isBusy.value,
          onClick: async () => { await onRetranslate(row.original.id) }
        }, { default: () => 'Re-translate' }),
        h(UButton, {
          size: 'xs',
          color: 'warning',
          variant: 'soft',
          icon: 'i-lucide-plus',
          disabled: isBusy.value,
          onClick: async () => { await onAddToGlossary(row.original.id) }
        }, { default: () => 'Add to glossary' }),
        h(UButton, {
          size: 'xs',
          color: 'error',
          variant: 'soft',
          icon: 'i-lucide-trash-2',
          disabled: isBusy.value,
          onClick: () => { onRemove(row.original.id) }
        }, { default: () => 'Remove' }),
        h(UButton, {
          size: 'xs',
          color: 'neutral',
          icon: 'i-lucide-pencil',
          disabled: isBusy.value || editorOpen.value,
          onClick: () => openEditor(row.original.id)
        }, { default: () => 'Edit' })
      ])
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
  // Forward to store immediately and also emit for parent listeners
  saveEdit({ id: payload.id, translated_text: payload.translated_text })
  emit('save', payload)
  editorOpen.value = false
}

async function onRetranslate(id: string) {
  await retranslate(id)
}

async function onAddToGlossary(id: string) {
  const unit = props.items.find(u => u.id === id)
  if (!unit) return
  const src = languageStore.getLanguage.source_language?.id || 'en'
  const tgt = languageStore.getLanguage.target_language?.id || 'en'
  const category = promptTypeToCategory[unit.prompt_type] || 'Essential Terms'
  const term: GlossaryTerm = {
    id: 0,
    category,
    source_lang: src,
    target_lang: tgt,
    input: unit.source_text,
    output: unit.translated_text || '',
    enabled: true,
  }
  await glossary.save(term)
  showToast('Added to glossary', `${category}: "${term.input}" → "${term.output || '…'}"`, 'success', 2500, 'i-lucide-check-circle')
}

function onRemove(id: string) {
  // Optimistically remove from store; also notify parent
  const idx = engineStore.textUnits.findIndex(u => u.id === id)
  if (idx !== -1) engineStore.textUnits.splice(idx, 1)
  emit('remove', id)
}
</script>


