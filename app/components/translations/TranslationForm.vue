<template>
  <UModal
    v-model:open="open_"
    :dismissible="false"
    :ui="{ content: 'max-w-5xl' }"
  >
    <template #title>
      <div class="flex items-center gap-3">
        <div class="p-2 bg-primary-50 dark:bg-primary-900/20 rounded-lg">
          <UIcon name="i-lucide-edit-3" class="text-primary w-5 h-5" />
        </div>
        <div>
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white">Edit Translation</h3>
          <p class="text-sm text-gray-500 dark:text-gray-400">Update the translation and status</p>
        </div>
      </div>
    </template>
    <template #actions>
      <div class="flex items-center gap-2">
        <UBadge color="neutral" variant="soft" size="sm">{{ local.prompt_type }}</UBadge>
        <UBadge :color="getStatusColor(local.status) as any" size="sm">
          {{ getStatusLabel(local.status) }}
        </UBadge>
      </div>
    </template>

    <template #body>
      <div class="space-y-6">
        <!-- Two-column layout -->
        <div class="grid gap-6 lg:grid-cols-2">
          <!-- Source Text Card -->
          <UCard>
            <template #header>
              <div class="flex items-center justify-between">
                <div class="flex items-center gap-2">
                  <UIcon name="i-lucide-file-text" class="text-gray-500 w-4 h-4" />
                  <span class="font-medium text-gray-900 dark:text-white">Source Text</span>
                </div>
                <div class="flex items-center gap-2">
                  <UBadge color="neutral" variant="soft" size="sm">{{ sourceCharCount }} chars</UBadge>
                </div>
              </div>
            </template>
            <UFormField label="Original text">
              <UTextarea
                :model-value="local.source_text"
                readonly
                :rows="6"
                placeholder="Source text..."
                class="bg-gray-50 dark:bg-gray-800/50"
              />
            </UFormField>
          </UCard>

          <!-- Translation Card -->
          <UCard>
            <template #header>
              <div class="flex items-center justify-between">
                <div class="flex items-center gap-2">
                  <UIcon name="i-lucide-languages" class="text-primary w-4 h-4" />
                  <span class="font-medium text-gray-900 dark:text-white">Translation</span>
                </div>
                <div class="flex items-center gap-2">
                  <UBadge color="primary" variant="soft" size="sm">{{ translationCharCount }} chars</UBadge>
                </div>
              </div>
            </template>
            <UFormField label="Translated text">
              <UTextarea
                v-model="local.translated_text"
                :rows="6"
                placeholder="Enter translation..."
                @keydown.ctrl.enter.prevent="onSave"
                @keydown.meta.enter.prevent="onSave"
              />
            </UFormField>
          </UCard>
        </div>

        <!-- Metadata Section -->
        <UCard>
          <template #header>
            <div class="flex items-center gap-2">
              <UIcon name="i-lucide-info" class="text-gray-500 w-4 h-4" />
              <span class="font-medium text-gray-900 dark:text-white">Translation Metadata</span>
            </div>
          </template>
          
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
        </UCard>
      </div>
    </template>

    <template #footer>
      <div class="flex items-center justify-between w-full gap-4">
        <div class="flex items-center gap-3">
          <div class="text-xs text-gray-500 dark:text-gray-400">
            ID: {{ local.id || 'New' }} • {{ local.source_lang }} → {{ local.target_lang }}
          </div>
          <div class="text-xs text-gray-500 dark:text-gray-400">
            Ctrl/Cmd + Enter to save
          </div>
        </div>
        <div class="flex items-center gap-3">
          <UButton 
            color="neutral" 
            variant="outline" 
            @click="onCancel"
          >
            Cancel
          </UButton>
          <UButton 
            color="primary" 
            :disabled="!isValid"
            @click="onSave"
          >
            <UIcon name="i-lucide-check" class="w-4 h-4 mr-2" />
            Save Translation
          </UButton>
        </div>
      </div>
    </template>
  </UModal>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import type { TextUnitRecord } from '~/types/translation'
import { TranslationStatus, PromptType } from '~/types/translation'

interface Props {
  open: boolean
  translation?: TextUnitRecord | null
}

interface Emits {
  (e: 'update:open', value: boolean): void
  (e: 'save', id: number, translatedText: string, status?: TranslationStatus): void
}

const props = withDefaults(defineProps<Props>(), {
  translation: null
})

const emit = defineEmits<Emits>()

// Status options for the select
const statusOptions = [
  { label: 'Not Translated', value: TranslationStatus.NotTranslated },
  { label: 'Machine Translated', value: TranslationStatus.MachineTranslated },
  { label: 'Human Reviewed', value: TranslationStatus.HumanReviewed },
  { label: 'Ignored', value: TranslationStatus.Ignored }
]

// Utility functions
function getStatusLabel(status: TranslationStatus): string {
  switch (status) {
    case TranslationStatus.NotTranslated: return 'Not Translated'
    case TranslationStatus.MachineTranslated: return 'Machine Translated'
    case TranslationStatus.HumanReviewed: return 'Human Reviewed'
    case TranslationStatus.Ignored: return 'Ignored'
    default: return status
  }
}

function getStatusColor(status: TranslationStatus): string {
  switch (status) {
    case TranslationStatus.NotTranslated: return 'gray'
    case TranslationStatus.MachineTranslated: return 'yellow'
    case TranslationStatus.HumanReviewed: return 'green'
    case TranslationStatus.Ignored: return 'red'
    default: return 'gray'
  }
}

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

// Two-way binding for modal open state
const open_ = computed({
  get: () => props.open,
  set: (value: boolean) => emit('update:open', value)
})

// Character counts
const sourceCharCount = computed(() => local.value.source_text.length)
const translationCharCount = computed(() => local.value.translated_text?.length || 0)

// Validation
const isValid = computed(() => {
  return local.value.id !== undefined && local.value.translated_text && local.value.translated_text.trim().length > 0
})

// Watch for prop changes
watch(() => props.translation, (newTranslation) => {
  if (newTranslation) {
    local.value = { ...newTranslation }
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
  }
}, { immediate: true })

// Actions
function onSave() {
  if (!isValid.value || !local.value.id) return
  
  emit('save', local.value.id, local.value.translated_text || '', local.value.status)
}

function onCancel() {
  emit('update:open', false)
}
</script>
