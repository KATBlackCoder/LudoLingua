<template>
  <div class="space-y-3">
    <h3 class="text-base font-semibold">Results</h3>
    <UTable :data="rows">
      <template #actions-data="{ row }">
        <UButton size="xs" icon="i-heroicons-pencil" :disabled="isBusy || editorOpen" @click="openEditor(row.original.id)">Edit</UButton>
      </template>
    </UTable>

    <TranslationEditor v-model:open="editorOpen" :item="editingItem" @save="onSave" />
  </div>
 </template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import type { TextUnit } from '~/types/translation'
import TranslationEditor from '~/components/translation/TranslationEditor.vue'
import { useTranslation } from '~/composables/useTranslation'

const props = defineProps<{ items: TextUnit[] }>()
const emit = defineEmits<{ (e: 'save', payload: { id: string; translated_text: string }): void }>()
const { isBusy } = useTranslation()

const rows = computed(() => props.items.map(u => ({
  id: u.id,
  source_text: u.source_text,
  translated_text: u.translated_text ?? ''
})))

const editorOpen = ref(false)
const editingItem = ref<TextUnit | null>(null)

function openEditor(id: string) {
  const unit = props.items.find(u => u.id === id) || null
  editingItem.value = unit
  editorOpen.value = !!unit
}

function onSave(payload: { id: string; translated_text: string }) {
  emit('save', payload)
  editorOpen.value = false
}
</script>


