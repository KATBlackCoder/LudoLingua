<template>
  <UModal
    v-model:open="open_"
    :dismissible="false"
    :ui="{ content: 'max-w-6xl' }"
  >
    <template #title>
      <div class="flex items-center gap-3">
        <div class="p-2 bg-primary-50 dark:bg-primary-900/20 rounded-lg">
          <UIcon name="i-lucide-book-open" class="text-primary w-5 h-5" />
        </div>
        <div>
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white">{{ heading }}</h3>
          <p class="text-sm text-gray-500 dark:text-gray-400">Add or edit a glossary mapping</p>
        </div>
      </div>
    </template>
    <template #actions>
      <div class="flex items-center gap-2">
        <UBadge color="neutral" variant="soft" size="sm">{{ local.category }}</UBadge>
        <UBadge color="primary" variant="soft" size="sm">{{ local.source_lang }} → {{ local.target_lang }}</UBadge>
      </div>
    </template>

    <template #body>
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
                    icon="i-lucide-copy" 
                    @click="copyFromInput"
                  >
                    Copy input
                  </UButton>
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

    <template #footer>
      <div class="flex items-center justify-between w-full gap-4">
        <div class="flex items-center gap-3">
          <div class="text-xs text-gray-500 dark:text-gray-400">
            Ctrl/Cmd + Enter to save — Only Input and Output are required
          </div>
        </div>
        <div class="flex items-center gap-3">
          <UButton 
            color="neutral" 
            variant="outline" 
            @click="emit('update:open', false)"
          >
            Cancel
          </UButton>
          <UButton 
            color="primary" 
            :disabled="!canSave" 
            @click="onSave"
          >
            <UIcon name="i-lucide-check" class="w-4 h-4 mr-2" />
            Save Term
          </UButton>
        </div>
      </div>
    </template>
  </UModal>
</template>

<script setup lang="ts">
import { computed, reactive, watchEffect } from 'vue'
import type { GlossaryTerm } from '~/types/glossary'
import { useLanguageStore } from '~/stores/language'
import { useGlossary } from '~/composables/useGlossary'

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

function copyFromInput() {
  local.output = local.input
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


