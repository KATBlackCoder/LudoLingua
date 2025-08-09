<template>
  <UModal v-model:open="open" title="Edit Translation" :close="false" :dismissible="false">
    <template #body>
      <div class="space-y-3">
        <div>
          <label class="text-sm text-gray-600 dark:text-gray-400">Source</label>
          <p class="whitespace-pre-wrap">{{ item?.source_text }}</p>
        </div>
        <div>
          <label class="text-sm text-gray-600 dark:text-gray-400">Translated</label>
          <UTextarea v-model="draft" :rows="6" />
        </div>
      </div>
    </template>
    <template #footer>
      <div class="flex justify-end gap-2">
        <UButton color="neutral" @click="cancel">Cancel</UButton>
        <UButton color="primary" @click="save">Save</UButton>
      </div>
    </template>
  </UModal>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import type { TextUnit } from '~/types/translation'

const props = defineProps<{ open: boolean; item: TextUnit | null }>()
const emit = defineEmits<{ (e: 'update:open', v: boolean): void; (e: 'save', payload: { id: string; translated_text: string }): void }>()

const draft = ref('')
const open = computed({
  get: () => props.open,
  set: (v: boolean) => emit('update:open', v)
})

watch(() => props.item, (val) => { draft.value = val?.translated_text ?? '' }, { immediate: true })

const cancel = () => emit('update:open', false)
const save = () => {
  if (!props.item) return
  emit('save', { id: props.item.id, translated_text: draft.value })
  emit('update:open', false)
}
</script>


