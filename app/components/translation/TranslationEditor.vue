<template>
  <UModal
    v-model:open="open"
    :dismissible="false"
    :ui="{ content: 'max-w-5xl' }"
  >
    <template #title>
      {{ modalTitle }}
    </template>
    <template #description>
      <span v-if="item?.field_type">{{ item?.field_type }}</span>
    </template>
    <template #actions>
      <UBadge :color="statusColor" variant="soft">{{ statusLabel }}</UBadge>
    </template>

    <template #body>
      <div class="space-y-4">
        <!-- Two-column layout on sm+ -->
        <div class="grid gap-4 sm:grid-cols-2">
          <UCard class="overflow-hidden">
            <template #header>
              <div class="flex items-center justify-between">
                <span class="font-medium">Source</span>
                <div class="text-xs text-muted">{{ sourceCharCount }} chars</div>
              </div>
            </template>
            <div class="text-sm whitespace-pre-wrap break-words">{{ item?.source_text }}</div>
          </UCard>

          <UCard>
            <template #header>
              <div class="flex items-center justify-between">
                <span class="font-medium">Translated</span>
                <div class="flex items-center gap-2">
                  <div class="text-xs text-muted">{{ draftCharCount }} chars</div>
                  <UButton
                    size="xs"
                    color="primary"
                    variant="soft"
                    icon="i-heroicons-sparkles"
                    :loading="isRetranslating"
                    @click="retranslate"
                  >Re-translate</UButton>
                </div>
              </div>
            </template>
            <div class="grid gap-3 sm:grid-cols-2 mb-2">
              <UFormField label="Prompt Type">
                <USelect v-model="promptTypeDraft" :items="promptTypeItems" />
              </UFormField>
            </div>
            <UFormField>
              <UTextarea
                v-model="draft"
                :rows="10"
                autofocus
                @keydown.ctrl.enter.prevent="save"
                @keydown.meta.enter.prevent="save"
              />
            </UFormField>
            <template #footer>
              <div class="flex items-center gap-2">
                <UButton size="xs" color="neutral" variant="subtle" icon="i-heroicons-document-duplicate" @click="copyFromSource">Copy source</UButton>
                <UButton size="xs" color="neutral" variant="subtle" icon="i-heroicons-backspace" @click="clearDraft">Clear</UButton>
              </div>
            </template>
          </UCard>
        </div>

        <UAlert
          icon="i-heroicons-information-circle"
          color="neutral"
          variant="soft"
          title="Placeholders"
          description="Text inside [ ... ] is preserved exactly. Do not edit it."
        />
      </div>
    </template>

    <template #footer>
      <div class="flex items-center justify-between w-full gap-3">
        <div class="text-xs text-muted">Ctrl/Cmd + Enter to save</div>
        <div class="flex gap-2">
          <UButton color="neutral" variant="soft" @click="cancel">Cancel</UButton>
          <UButton color="primary" :disabled="!canSave" @click="save">Save</UButton>
        </div>
      </div>
    </template>
  </UModal>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import type { PromptType, TextUnit } from '~/types/translation'
import { useTranslation } from '~/composables/useTranslation'
import type { SelectItem } from '#ui/types'
import { useEngineStore } from '~/stores/engine'

const props = defineProps<{ open: boolean; item: TextUnit | null }>()
const emit = defineEmits<{ (e: 'update:open', v: boolean): void; (e: 'save', payload: { id: string; translated_text: string; prompt_type?: string }): void }>()

const draft = ref('')
const promptTypeDraft = ref<string>('Other')
const engineStore = useEngineStore()
const isRetranslating = ref(false)
const { retranslate: retranslateOne } = useTranslation()
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
const draftCharCount = computed(() => draft.value.length)

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

const cancel = () => emit('update:open', false)
const save = () => {
  if (!props.item) return
  // update prompt type locally as well
  const unit = engineStore.getTextUnitById(props.item.id)
  if (unit) {
    unit.prompt_type = promptTypeDraft.value as PromptType
  }
  emit('save', { id: props.item.id, translated_text: draft.value, prompt_type: promptTypeDraft.value })
  emit('update:open', false)
}

function copyFromSource() {
  if (!props.item) return
  draft.value = props.item.source_text
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
    }
  } catch {
    // errors are already toasted in the store
  } finally {
    isRetranslating.value = false
  }
}

const promptTypeItems = [
  { label: 'Character', value: 'Character' },
  { label: 'Class', value: 'Class' },
  { label: 'Skill', value: 'Skill' },
  { label: 'Equipment', value: 'Equipment' },
  { label: 'State', value: 'State' },
  { label: 'System', value: 'System' },
  { label: 'Dialogue', value: 'Dialogue' },
  { label: 'Other', value: 'Other' },
] as SelectItem[]
</script>


