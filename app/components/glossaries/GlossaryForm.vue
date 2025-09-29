<template>
  <Modal
    v-model:open="open_"
    :dismissible="false"
    layout="custom"
    :title="heading"
    description="Add or edit a glossary mapping"
    header-icon="i-lucide-book-open"
    header-icon-color="primary"
    header-icon-background="primary"
    :status="local.category"
    :metadata="`${local.source_lang} → ${local.target_lang}`"
    metadata-color="primary"
    :show-footer="true"
    :show-cancel="true"
    cancel-label="Cancel"
    :show-copy="true"
    :copy-label="'Copy Input'"
    :copy-color="'neutral'"
    copy-variant="ghost"
    :copy-icon="'i-lucide-copy'"
    :copy-text="local.input"
    :show-save="true"
    save-label="Save Term"
    save-color="primary"
    :disabled="!canSave"
    status-info="Ctrl/Cmd + Enter to save — Only Input and Output are required"
    keyboard-shortcuts="Ctrl/Cmd + Enter to save"
    @cancel="emit('update:open', false)"
    @save="onSave"
  >
    <template #content>
      <div class="space-y-6">
        <!-- Two-column layout -->
        <div class="grid gap-6 lg:grid-cols-2">
          <!-- Source Term Card -->
          <UCard>
            <template #header>
              <div class="flex items-center justify-between">
                <div class="flex items-center gap-2">
                  <UIcon name="i-lucide-file-text" class="text-gray-500 w-4 h-4" />
                  <span class="font-medium text-gray-900 dark:text-white">Source Term</span>
                </div>
                <div class="flex items-center gap-2">
                  <UBadge color="neutral" variant="soft" size="sm">{{ inputCharCount }} chars</UBadge>
                </div>
              </div>
            </template>
            <UFormField label="Input term">
              <UInput
                v-model="local.input"
                placeholder="源語 / Input term"
                @keydown.ctrl.enter.prevent="onSave"
                @keydown.meta.enter.prevent="onSave"
              />
            </UFormField>
          </UCard>

          <!-- Target Term Card -->
          <UCard>
            <template #header>
              <div class="flex items-center justify-between">
                <div class="flex items-center gap-2">
                  <UIcon name="i-lucide-languages" class="text-primary w-4 h-4" />
                  <span class="font-medium text-gray-900 dark:text-white">Target Term</span>
                </div>
                <div class="flex items-center gap-2">
                  <UBadge color="primary" variant="soft" size="sm">{{ outputCharCount }} chars</UBadge>
                </div>
              </div>
            </template>
            <UFormField label="Translated term">
              <UInput
                v-model="local.output"
                placeholder="Translated term"
                @keydown.ctrl.enter.prevent="onSave"
                @keydown.meta.enter.prevent="onSave"
              />
            </UFormField>
            <template #footer>
              <div class="flex items-center justify-between">
                <div class="flex items-center gap-2">
                  <UButton 
                    size="xs" 
                    color="neutral" 
                    variant="ghost" 
                    icon="i-lucide-x" 
                    @click="clearOutput"
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

        <!-- Configuration Section -->
        <UCard>
          <template #header>
            <div class="flex items-center gap-2">
              <UIcon name="i-lucide-settings" class="text-gray-500 w-4 h-4" />
              <span class="font-medium text-gray-900 dark:text-white">Term Configuration</span>
            </div>
          </template>
          
          <div class="space-y-4">
            <div class="grid grid-cols-2 gap-4">
              <UFormField label="Category">
                <USelect 
                  v-model="local.category" 
                  :items="categoryItems" 
                  placeholder="Select category"
                  size="sm"
                />
              </UFormField>
              <UFormField label="Enabled">
                <div class="flex items-center gap-2 p-2 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
                  <USwitch v-model="local.enabled" size="sm" />
                  <span class="text-sm text-gray-600 dark:text-gray-400">Enable this term</span>
                </div>
              </UFormField>
            </div>

            <div class="grid grid-cols-2 gap-4">
              <UFormField label="Source Language">
                <USelect 
                  v-model="local.source_lang" 
                  :items="languageItems" 
                  placeholder="Select source language"
                  size="sm"
                />
              </UFormField>
              <UFormField label="Target Language">
                <USelect 
                  v-model="local.target_lang" 
                  :items="languageItems" 
                  placeholder="Select target language"
                  size="sm"
                />
              </UFormField>
            </div>

            <div class="flex justify-end">
              <UButton 
                size="sm" 
                icon="i-lucide-arrow-left-right" 
                variant="soft" 
                @click="swapLangs"
              >
                Swap Languages
              </UButton>
            </div>
          </div>
        </UCard>
      </div>
    </template>
  </Modal>
</template>

<script setup lang="ts">
import { computed, reactive, watchEffect } from 'vue'
import type { GlossaryTerm } from '~/types/glossary'
import { useLanguageStore } from '~/stores/language'
import { useGlossary } from '~/composables/useGlossary'
import Modal from '~/components/shared/modal/Modal.vue'

const props = defineProps<{ term: GlossaryTerm | null; heading?: string; open?: boolean }>()
const emit = defineEmits<{ (e: 'save', term: GlossaryTerm): void; (e: 'update:open', v: boolean): void }>()

const heading = computed(() => props.heading || (props.term && props.term.id > 0 ? 'Edit Glossary Term' : 'Add Glossary Term'))

const open_ = computed({
  get: () => !!props.open,
  set: (v: boolean) => emit('update:open', v)
})

const local = reactive<GlossaryTerm>({
  id: 0,
  category: 'Characters',
  source_lang: 'en',
  target_lang: 'fr',
  input: '',
  output: '',
  enabled: true
})

watchEffect(() => {
  const t = props.term
  if (t) {
    Object.assign(local, t)
  }
})

const canSave = computed(() => local.input.trim().length > 0 && local.output.trim().length > 0)

const inputCharCount = computed(() => local.input.length)
const outputCharCount = computed(() => local.output.length)

function onSave() {
  emit('save', { ...local })
  emit('update:open', false)
}


function clearOutput() {
  local.output = ''
}

function swapLangs() {
  const src = local.source_lang
  const tgt = local.target_lang
  local.source_lang = tgt
  local.target_lang = src
}

// Select items
const { categoryItems } = useGlossary()
const language = useLanguageStore()
const languageItems = computed(() =>
  language.languageOptions.map(l => ({ label: l.label, value: l.id }))
)
</script>


