<template>
  <UModal
    v-model:open="open_"
    :title="heading"
    :dismissible="false"
    :ui="{ content: 'max-w-xl' }"
  >
    <template #header>
      <div class="flex items-center justify-between gap-3 w-full">
        <div class="min-w-0">
          <h3 class="text-base font-semibold truncate">{{ heading }}</h3>
          <p class="text-xs text-muted truncate">Add or edit a glossary mapping</p>
        </div>
      </div>
    </template>

    <template #body>
      <div class="space-y-4">
        <div class="grid gap-4 sm:grid-cols-2">
          <UCard>
            <template #header>
              <div class="flex items-center justify-between">
                <span class="font-medium">Source</span>
              </div>
            </template>
            <UFormField>
              <UInput v-model="local.input" placeholder="源語 / Input term" />
            </UFormField>
          </UCard>

          <UCard>
            <template #header>
              <div class="flex items-center justify-between">
                <span class="font-medium">Target</span>
              </div>
            </template>
            <UFormField>
              <UInput v-model="local.output" placeholder="Translated term" />
            </UFormField>
          </UCard>
        </div>

        <div class="grid grid-cols-2 gap-4">
          <UFormField label="Category">
            <USelect v-model="local.category" :items="categoryItems" placeholder="Select category" />
          </UFormField>
          <UFormField label="Enabled">
            <USwitch v-model="local.enabled" />
          </UFormField>
        </div>

        <div class="grid grid-cols-2 gap-4">
          <UFormField label="Source Lang">
            <USelect v-model="local.source_lang" :items="languageItems" placeholder="Select source language" />
          </UFormField>
          <UFormField label="Target Lang">
            <USelect v-model="local.target_lang" :items="languageItems" placeholder="Select target language" />
          </UFormField>
        </div>
      </div>
    </template>

    <template #footer>
      <div class="flex items-center justify-between w-full gap-3">
        <div class="text-xs text-muted">Only Input and Output are required</div>
        <div class="flex gap-2">
          <UButton color="neutral" variant="soft" @click="emit('update:open', false)">Cancel</UButton>
          <UButton color="primary" :disabled="!canSave" @click="onSave">Save</UButton>
        </div>
      </div>
    </template>
  </UModal>
</template>

<script setup lang="ts">
import { computed, reactive, watchEffect } from 'vue'
import type { GlossaryTerm } from '~/types/glossary'
import { useLanguageStore } from '~/stores/language'

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

function onSave() {
  emit('save', { ...local })
  emit('update:open', false)
}

// Select items
const categoryItems = ['Characters', 'Essential Terms', 'Status Effects', 'Mechanics', 'Translation Rules', 'Locations', 'Time & Weather']
const language = useLanguageStore()
const languageItems = computed(() =>
  language.languageOptions.map(l => ({ label: l.label, value: l.id }))
)
</script>


