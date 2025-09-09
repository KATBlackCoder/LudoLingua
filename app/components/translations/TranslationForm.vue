<template>
  <UModal
    v-model:open="open_"
    :dismissible="false"
    :ui="{ content: 'max-w-4xl' }"
  >
    <template #title>
      Edit Translation
    </template>
    <template #description>
      Update the translation and status
    </template>
    <template #actions>
      <div class="flex items-center gap-2">
        <UBadge variant="soft">{{ local.prompt_type }}</UBadge>
        <UBadge :color="getStatusColor(local.status) as any" size="sm">
          {{ getStatusLabel(local.status) }}
        </UBadge>
      </div>
    </template>

    <template #body>
      <div class="space-y-4">
        <div class="grid gap-4 sm:grid-cols-2">
          <!-- Source Text (Read-only) -->
          <UCard class="overflow-hidden">
            <template #header>
              <div class="flex items-center justify-between">
                <span class="font-medium">Source Text</span>
                <div class="text-xs text-muted">{{ sourceCharCount }} chars</div>
              </div>
            </template>
            <UFormField label="Original text">
              <UTextarea
                :model-value="local.source_text"
                readonly
                :rows="4"
                placeholder="Source text..."
                class="bg-gray-50 dark:bg-gray-800"
              />
            </UFormField>
          </UCard>

          <!-- Translation (Editable) -->
          <UCard>
            <template #header>
              <div class="flex items-center justify-between">
                <span class="font-medium">Translation</span>
                <div class="text-xs text-muted">{{ translationCharCount }} chars</div>
              </div>
            </template>
            <UFormField label="Translated text">
              <UTextarea
                v-model="local.translated_text"
                :rows="4"
                placeholder="Enter translation..."
                @keydown.ctrl.enter.prevent="onSave"
                @keydown.meta.enter.prevent="onSave"
              />
            </UFormField>
          </UCard>
        </div>

        <!-- Metadata -->
        <div class="grid gap-4 sm:grid-cols-3">
          <UFormField label="Status">
            <USelect
              v-model="local.status"
              :items="statusOptions.filter(s => s !== 'All')"
              placeholder="Select status"
            />
          </UFormField>

          <UFormField label="File Path">
            <UInput
              :model-value="local.file_path"
              readonly
              class="bg-gray-50 dark:bg-gray-800"
            />
          </UFormField>

          <UFormField label="Field Type">
            <UInput
              :model-value="local.field_type"
              readonly
              class="bg-gray-50 dark:bg-gray-800"
            />
          </UFormField>
        </div>
      </div>
    </template>

    <template #footer>
      <div class="flex items-center justify-between w-full">
        <div class="text-xs text-muted">
          ID: {{ local.id || 'New' }} • {{ local.source_lang }} → {{ local.target_lang }}
        </div>
        <div class="flex items-center gap-2">
          <UButton color="neutral" variant="soft" @click="onCancel">Cancel</UButton>
          <UButton color="primary" @click="onSave" :disabled="!isValid">Save</UButton>
        </div>
      </div>
    </template>
  </UModal>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import type { TextUnitRecord } from '~/stores/translations'

interface Props {
  open: boolean
  translation?: TextUnitRecord | null
}

interface Emits {
  (e: 'update:open', value: boolean): void
  (e: 'save', id: number, translatedText: string, status?: string): void
}

const props = withDefaults(defineProps<Props>(), {
  translation: null
})

const emit = defineEmits<Emits>()

// Status options for the select
const statusOptions = [
  'NotTranslated',
  'MachineTranslated', 
  'HumanReviewed',
  'Ignored'
]

// Utility functions
function getStatusLabel(status: string): string {
  switch (status) {
    case 'NotTranslated': return 'Not Translated'
    case 'MachineTranslated': return 'Machine Translated'
    case 'HumanReviewed': return 'Human Reviewed'
    case 'Ignored': return 'Ignored'
    default: return status
  }
}

function getStatusColor(status: string): string {
  switch (status) {
    case 'NotTranslated': return 'gray'
    case 'MachineTranslated': return 'yellow'
    case 'HumanReviewed': return 'green'
    case 'Ignored': return 'red'
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
  status: 'NotTranslated',
  prompt_type: 'Other',
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
      status: 'NotTranslated',
      prompt_type: 'Other',
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
