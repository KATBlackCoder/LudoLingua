<template>
  <Modal
    v-model:open="open_"
    :dismissible="false"
    layout="two-column"
    title="Edit Translation"
    description="Update the translation and status"
    header-icon="i-lucide-edit-3"
    header-icon-color="primary"
    :status="getStatusLabel(local.status)"
    :status-color="getStatusColor(local.status) as any"
    :category="local.prompt_type"
    :source-text="local.source_text"
    source-title="Source Text"
    source-icon="i-lucide-file-text"
    source-icon-color="neutral"
    :show-character-count="true"
    :translation-value="local.translated_text"
    :translation-valid="!!isValid"
    :translation-error="!isValid && (local.translated_text?.length || 0) > 0"
    translation-validation-type="translation"
    :metadata-items="metadataItems"
    metadata-title="Translation Metadata"
    metadata-icon="i-lucide-info"
    metadata-icon-color="info"
    :show-cancel="true"
    cancel-label="Cancel"
    :show-save="true"
    :save-label="hasModifications ? 'Save Translation' : 'No Changes'"
    save-color="primary"
    :disabled="!isValid || !hasModifications"
    :status-info="`ID: ${local.id || 'New'} • ${local.source_lang} → ${local.target_lang}`"
    keyboard-shortcuts="Ctrl/Cmd + Enter to save"
    :modal-ui="{ content: 'max-w-5xl' }"
    :show-retranslation="true"
    :is-retranslating="isRetranslating"
    :retranslation-disabled="!local.source_text"
    @cancel="onCancel"
    @save="onSave"
    @retranslate="onRetranslate"
  >
    <!-- Translation content slot -->
    <template #translationContent>
      <UFormField label="Translated text">
        <UTextarea
          v-model="local.translated_text"
          :rows="6"
          placeholder="Enter translation..."
          @keydown.ctrl.enter.prevent="onSave"
          @keydown.meta.enter.prevent="onSave"
        />
      </UFormField>
    </template>

    <!-- Metadata content slot -->
    <template #metadataFooter>
      <div class="grid gap-4 sm:grid-cols-3">
        <UFormField label="Status">
          <USelect
            v-model="local.status"
            :items="statusOptions"
            placeholder="Select status"
            size="sm"
          />
        </UFormField>

        <UFormField label="File Path">
          <UInput
            :model-value="local.file_path"
            readonly
            class="bg-gray-50 dark:bg-gray-800/50"
            size="sm"
          />
        </UFormField>

        <UFormField label="Field Type">
          <UInput
            :model-value="local.field_type"
            readonly
            class="bg-gray-50 dark:bg-gray-800/50"
            size="sm"
          />
        </UFormField>
      </div>
    </template>
  </Modal>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import type { TextUnitRecord } from '~/types/translation'
import { TranslationStatus, PromptType } from '~/types/translation'
import Modal from '~/components/shared/modal/Modal.vue'
import { getStatusLabel, getStatusColor, statusOptions } from '~/utils/translation'

interface Props {
  open: boolean
  translation?: TextUnitRecord | null
}

interface Emits {
  (e: 'update:open', value: boolean): void
  (e: 'save', id: number, translatedText: string, status?: TranslationStatus): void
  (e: 'retranslate', id: number, sourceText: string): void
}

const props = withDefaults(defineProps<Props>(), {
  translation: null
})

const emit = defineEmits<Emits>()

// Status utilities imported directly from utils/translation

// Retranslation state
const isRetranslating = ref(false)

// Local state
const local = ref<TextUnitRecord>({
  id: undefined,
  project_path: '',
  file_path: '',
  field_type: '',
  source_text: '',
  translated_text: '',
  status: TranslationStatus.NotTranslated,
  prompt_type: PromptType.Other,
  source_lang: 'ja',
  target_lang: 'en'
})

// Original state to track changes
const original = ref<TextUnitRecord | null>(null)

// Two-way binding for modal open state
const open_ = computed({
  get: () => props.open,
  set: (value: boolean) => emit('update:open', value)
})

// Character counts (computed by the Modal component)

// Metadata items for the modal
const metadataItems = computed(() => [
  {
    label: 'File Path',
    value: local.value.file_path,
    icon: 'i-lucide-file',
    type: 'text' as const
  },
  {
    label: 'Field Type',
    value: local.value.field_type,
    icon: 'i-lucide-tag',
    type: 'text' as const
  },
  {
    label: 'Source Language',
    value: local.value.source_lang,
    icon: 'i-lucide-globe',
    type: 'text' as const
  },
  {
    label: 'Target Language',
    value: local.value.target_lang,
    icon: 'i-lucide-languages',
    type: 'text' as const
  }
])

// Check if there are any modifications
const hasModifications = computed(() => {
  if (!original.value) return false
  
  return (
    local.value.translated_text !== original.value.translated_text ||
    local.value.status !== original.value.status
  )
})

// Validation
const isValid = computed(() => {
  return local.value.id !== undefined && local.value.translated_text && local.value.translated_text.trim().length > 0
})

// Watch for prop changes
watch(() => props.translation, (newTranslation) => {
  if (newTranslation) {
    local.value = { ...newTranslation }
    original.value = { ...newTranslation } // Store original state
  } else {
    // Reset to default
    local.value = {
      id: undefined,
      project_path: '',
      file_path: '',
      field_type: '',
      source_text: '',
      translated_text: '',
      status: TranslationStatus.NotTranslated,
      prompt_type: PromptType.Other,
      source_lang: 'ja',
      target_lang: 'en'
    }
    original.value = null
  }
}, { immediate: true })

// Actions
function onSave() {
  if (!isValid.value || !local.value.id) return
  
  // Only save if there are modifications
  if (!hasModifications.value) {
    console.log('No modifications detected, skipping save')
    return
  }
  
  emit('save', local.value.id, local.value.translated_text || '', local.value.status)
}

function onCancel() {
  emit('update:open', false)
}

async function onRetranslate() {
  if (!local.value.id || !local.value.source_text || isRetranslating.value) return
  
  isRetranslating.value = true
  try {
    emit('retranslate', local.value.id, local.value.source_text)
  } finally {
    // Reset loading state after a short delay to show the loading state
    setTimeout(() => {
      isRetranslating.value = false
    }, 1000)
  }
}
</script>
