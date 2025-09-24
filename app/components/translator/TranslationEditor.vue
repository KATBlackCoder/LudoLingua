<template>
  <UModal
    v-model:open="open"
    :dismissible="false"
    :ui="{ content: 'max-w-6xl' }"
  >
    <template #title>
      <div class="flex items-center gap-3">
        <div class="p-2 bg-primary-50 dark:bg-primary-900/20 rounded-lg">
          <UIcon name="i-lucide-edit-3" class="text-primary w-5 h-5" />
        </div>
        <div>
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white">{{ modalTitle }}</h3>
          <p v-if="item?.field_type" class="text-sm text-gray-500 dark:text-gray-400">{{ item?.field_type }}</p>
        </div>
      </div>
    </template>
    <template #actions>
      <div class="flex items-center gap-2">
        <UBadge :color="statusColor" variant="soft" size="sm">{{ statusLabel }}</UBadge>
        <UBadge v-if="item?.prompt_type" color="neutral" variant="soft" size="sm">{{ item?.prompt_type }}</UBadge>
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
            <div class="text-sm whitespace-pre-wrap break-words bg-gray-50 dark:bg-gray-800/50 p-4 rounded-lg border">
              {{ item?.source_text }}
            </div>
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
                  <UBadge color="primary" variant="soft" size="sm">{{ draftCharCount }} chars</UBadge>
                  <UButton
                    size="xs"
                    color="primary"
                    variant="soft"
                    :loading="isRetranslating"
                    @click="retranslate"
                  >
                    <UIcon v-if="!isRetranslating" name="i-lucide-sparkles" class="w-3 h-3 mr-1" />
                    Re-translate
                  </UButton>
                </div>
              </div>
            </template>
            
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

            <template #footer>
              <div class="flex items-center justify-between">
                <div class="flex items-center gap-2">
                  <UButton 
                    size="xs" 
                    color="neutral" 
                    variant="ghost" 
                    icon="i-lucide-copy" 
                    @click="copyFromSource"
                  >
                    Copy Source
                  </UButton>
                  <UButton 
                    size="xs" 
                    color="neutral" 
                    variant="ghost" 
                    icon="i-lucide-x" 
                    @click="clearDraft"
                  >
                    Clear
                  </UButton>
                </div>
                <div class="text-xs text-gray-500 dark:text-gray-400">
                  Ctrl/Cmd + Enter to save
                </div>
              </div>
            </template>
          </UCard>
        </div>

        <!-- Placeholder Information -->
        <UAlert
          icon="i-lucide-info"
          color="info"
          variant="soft"
          title="Placeholder Guidelines"
          description="Text inside [ ... ] brackets contains game formatting codes. These must be preserved exactly as they are in your translation."
        />
      </div>
    </template>

    <template #footer>
      <div class="flex items-center justify-between w-full gap-4">
        <div class="flex items-center gap-3">
          <UBadge v-if="isSaving" color="primary" variant="soft" size="sm">
            <UIcon name="i-lucide-loader-2" class="w-3 h-3 mr-1 animate-spin" />
            Saving to database...
          </UBadge>
          <UBadge v-else-if="lastSaved" color="success" variant="soft" size="sm">
            <UIcon name="i-lucide-check-circle" class="w-3 h-3 mr-1" />
            Saved {{ lastSaved }}
          </UBadge>
          <div v-else class="text-xs text-gray-500 dark:text-gray-400">
            Ctrl/Cmd + Enter to save
          </div>
        </div>
        <div class="flex gap-3">
          <UButton 
            color="neutral" 
            variant="outline" 
            @click="cancel"
          >
            Cancel
          </UButton>
          <UButton 
            color="primary" 
            :disabled="!canSave" 
            :loading="isSaving" 
            @click="save"
          >
            <UIcon v-if="!isSaving" name="i-lucide-check" class="w-4 h-4 mr-2" />
            Save Translation
          </UButton>
        </div>
      </div>
    </template>
  </UModal>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import type { PromptType, TextUnit } from '~/types/translation'
import { useTranslator } from '~/composables/useTranslator'
import type { SelectItem } from '#ui/types'
import { useEngineStore } from '~/stores/engine'
import { useNotifications } from '~/composables/useNotifications'

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
      // Notify when retranslation is complete
      await notify('Translation Complete', 'Re-translation completed successfully')
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


