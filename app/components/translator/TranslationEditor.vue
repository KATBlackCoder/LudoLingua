<template>
  <Modal
    v-model:open="open"
    :dismissible="false"
    layout="two-column"
    :title="modalTitle"
    :description="item?.field_type"
    :header-icon="'i-lucide-edit-3'"
    :header-icon-color="'primary'"
    :header-icon-background="'primary'"
    :status="statusLabel"
    :status-color="statusColor"
    :category="item?.prompt_type"
    :category-variant="'soft'"
    :source-text="item?.source_text"
    :source-title="'Source Text'"
    :source-icon="'i-lucide-file-text'"
    :source-icon-color="'neutral'"
    :show-character-count="true"
    :translation-title="'Translation'"
    :translation-icon="'i-lucide-languages'"
    :translation-icon-color="'primary'"
    :translation-validation-type="'translation'"
    :translation-value="draft"
    :show-retranslation="true"
    :is-retranslating="isRetranslating"
    :retranslation-label="'AI Retranslation'"
    :retranslation-description="'Generate new translation using AI'"
    :retranslation-icon="'i-lucide-refresh-cw'"
    :retranslation-button-icon="'i-lucide-sparkles'"
    :retranslation-button-text="'Re-translate'"
    :retranslation-loading-text="'Retranslating...'"
    :alert-message="'Text inside [ ... ] brackets contains game formatting codes. These must be preserved exactly as they are in your translation.'"
    :alert-title="'Placeholder Guidelines'"
    :alert-color="'info'"
    :alert-icon="'i-lucide-info'"
    :save-label="'Save Translation'"
    :save-color="'primary'"
    :loading="isSaving"
    :disabled="!canSave"
    :show-copy="true"
    :copy-label="'Copy Source'"
    :copy-color="'neutral'"
    :copy-variant="'ghost'"
    :copy-icon="'i-lucide-copy'"
    :copy-text="item?.source_text"
    :status-info="statusInfo"
    :keyboard-shortcuts="'Ctrl/Cmd + Enter to save'"
    @save="save"
    @cancel="cancel"
    @retranslate="retranslate"
  >
    <!-- Translation content slot -->
    <template #translationContent>
      <div class="space-y-4">
        <!-- Prompt Type Selection -->
        <UFormField label="Prompt Type">
          <USelect v-model="promptTypeDraft" :items="promptTypeItems" />
        </UFormField>
        
        <!-- Translation Textarea -->
        <UFormField label="Translation">
          <UTextarea
            v-model="draft"
            :rows="12"
            placeholder="Enter your translation here..."
            autofocus
            @keydown.ctrl.enter.prevent="save"
            @keydown.meta.enter.prevent="save"
          />
        </UFormField>
      </div>
    </template>

    <!-- Source footer with character count -->
    <template #sourceFooter>
      <div class="flex items-center gap-2">
        <UBadge color="neutral" variant="soft" size="sm">{{ sourceCharCount }} chars</UBadge>
      </div>
    </template>

    <!-- Additional footer actions -->
    <template #footerActions>
      <UButton 
        size="xs" 
        color="neutral" 
        variant="ghost" 
        icon="i-lucide-x" 
        @click="clearDraft"
      >
        Clear
      </UButton>
    </template>
  </Modal>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import type { PromptType, TextUnit } from '~/types/translation'
import { useTranslator } from '~/composables/useTranslator'
import type { SelectItem } from '#ui/types'
import { useEngineStore } from '~/stores/engine'
import { useNotifications } from '~/composables/useNotifications'
import Modal from '~/components/shared/modal/Modal.vue'
import { promptTypeOptions } from '~/utils/translation'

const props = defineProps<{ open: boolean; item: TextUnit | null }>()
const emit = defineEmits<{ (e: 'update:open', v: boolean): void; (e: 'save', payload: { id: string; translated_text: string; prompt_type?: string }): void }>()

const draft = ref('')
const promptTypeDraft = ref<string>('Other')
const engineStore = useEngineStore()
const isRetranslating = ref(false)
const isSaving = ref(false)
const lastSaved = ref<string | null>(null)
const { retranslate: retranslateOne } = useTranslator()
const { notify } = useNotifications()
const open = computed({
  get: () => props.open,
  set: (v: boolean) => emit('update:open', v)
})

watch(() => props.item, (val) => {
  draft.value = val?.translated_text ?? ''
  promptTypeDraft.value = String(val?.prompt_type || 'Other')
}, { immediate: true })

const modalTitle = computed(() => props.item ? `Edit Translation – ${props.item.id}` : 'Edit Translation')

const sourceCharCount = computed(() => props.item?.source_text?.length ?? 0)

const canSave = computed(() => {
  if (!props.item) return false
  return draft.value !== (props.item.translated_text ?? '')
})

const statusLabel = computed(() => props.item?.status ?? 'Unknown')
const statusColor = computed(() => {
  const s = String(props.item?.status || '')
  if (s.includes('Human')) return 'success'
  if (s.includes('Machine')) return 'info'
  if (s.includes('Ignored')) return 'warning'
  return 'neutral'
})

const statusInfo = computed(() => {
  if (isSaving.value) {
    return 'Saving to database...'
  }
  if (lastSaved.value) {
    return `Saved ${lastSaved.value}`
  }
  return ''
})

const cancel = () => emit('update:open', false)
const save = async () => {
  if (!props.item) return

  try {
    isSaving.value = true

    // update prompt type locally as well
    const unit = engineStore.getTextUnitById(props.item.id)
    if (unit) {
      unit.prompt_type = promptTypeDraft.value as PromptType
    }

    // Simulate database save time (would be handled by the store)
    await new Promise(resolve => setTimeout(resolve, 500))

    emit('save', { id: props.item.id, translated_text: draft.value, prompt_type: promptTypeDraft.value })

    // Update last saved time
    lastSaved.value = new Date().toLocaleTimeString()

    emit('update:open', false)
  } catch (error) {
    console.error('Save error:', error)
  } finally {
    isSaving.value = false
  }
}


function clearDraft() {
  draft.value = ''
}

async function retranslate() {
  if (!props.item || isRetranslating.value) return
  try {
    isRetranslating.value = true
    
    // Update prompt type before retranslating
    const unit = engineStore.getTextUnitById(props.item.id)
    if (unit) {
      unit.prompt_type = promptTypeDraft.value as PromptType
    }
    
    const updated = await retranslateOne(props.item.id)
    if (updated) {
      draft.value = updated.translated_text || ''
      // Notify when retranslation is complete
      await notify('Translation Complete', 'Re-translation completed successfully')
    }
  } catch {
    // errors are already toasted in the store
  } finally {
    isRetranslating.value = false
  }
}

// Use shared prompt type options from translation utils
const promptTypeItems = promptTypeOptions as SelectItem[]
</script>


