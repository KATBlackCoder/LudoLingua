<template>
  <div class="space-y-6">
    <!-- Header Section -->
    <UCard>
      <template #header>
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <div class="p-2 bg-primary-50 dark:bg-primary-900/20 rounded-lg">
              <UIcon name="i-lucide-list-checks" class="text-primary w-5 h-5" />
            </div>
            <div>
              <h3 class="text-lg font-semibold text-gray-900 dark:text-white">Translation Results</h3>
              <p class="text-sm text-gray-500 dark:text-gray-400">Manage and review your translations</p>
            </div>
          </div>
          <div class="flex items-center gap-2">
            <UBadge :color="hasTranslated ? 'success' : 'neutral'" variant="soft" size="sm">
              {{ filteredRows.length }} items
            </UBadge>
          </div>
        </div>
      </template>

      <!-- Filters Section -->
      <div class="space-y-4">
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
          <!-- Search Filter -->
          <div class="space-y-2">
            <label class="text-sm font-medium text-gray-700 dark:text-gray-300">Search</label>
            <UInput 
              v-model="search" 
              icon="i-lucide-search" 
              placeholder="Search translations..."
              size="sm"
            />
          </div>

          <!-- Placeholder Filter -->
          <div class="space-y-2">
            <label class="text-sm font-medium text-gray-700 dark:text-gray-300">Placeholder Type</label>
            <USelect 
              v-model="placeholderFilter" 
              :items="placeholderOptions" 
              placeholder="All placeholders"
              size="sm"
            />
          </div>

          <!-- Text Length Filter -->
          <div class="space-y-2">
            <label class="text-sm font-medium text-gray-700 dark:text-gray-300">
              Text Length: {{ textLengthRange[0] }}-{{ textLengthRange[1] }} chars
            </label>
            <USlider 
              v-model="textLengthRange" 
              :min="0" 
              :max="maxTextLength" 
              :step="5"
              size="sm"
            />
          </div>
        </div>
      </div>
    </UCard>

    <!-- Bulk Actions -->
    <UAlert
      v-if="selectedRows.length > 0"
      color="info"
      variant="soft"
      icon="i-lucide-check-square"
      :title="`${selectedRows.length} item(s) selected`"
      class="p-4"
    >
      <template #default>
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <UButton
              v-if="selectedRows.length >= 2"
              color="primary"
              variant="soft"
              icon="i-lucide-refresh-cw"
              :loading="isBulkRetranslating"
              :disabled="isBusy"
              size="sm"
              @click="onBulkRetranslate"
            >
              Re-translate Selected ({{ selectedRows.length }})
            </UButton>
            <UButton
              v-if="selectedRows.length >= 1"
              color="warning"
              variant="soft"
              icon="i-lucide-undo"
              :loading="isBulkReverting"
              :disabled="isBusy"
              size="sm"
              @click="onBulkRevert"
            >
              Revert to Raw ({{ selectedRows.length }})
            </UButton>
          </div>
          <UButton
            color="error"
            variant="ghost"
            size="sm"
            icon="i-lucide-x"
            @click="clearSelection"
          >
            Clear Selection
          </UButton>
        </div>
      </template>
    </UAlert>

    <!-- Results Table -->
    <UCard>
      <template #header>
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <UIcon name="i-lucide-table" class="text-gray-500 w-4 h-4" />
            <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Translation Results</span>
          </div>
          <div class="flex items-center gap-2">
            <span class="text-xs text-gray-500 dark:text-gray-400">
              Page {{ page }} of {{ pageCount }}
            </span>
            <UPagination 
              v-model:page="page" 
              :total="filteredRows.length" 
              :items-per-page="pageSize"
              size="sm"
            />
          </div>
        </div>
      </template>

      <UTable 
        ref="table"
        v-model:row-selection="rowSelection"
        v-model:sorting="sorting"
        :data="pagedRows" 
        :columns="columns" 
        class="text-sm" 
        @select="onSelect"
      />

      <template #footer>
        <div class="flex items-center justify-between text-xs text-gray-500 dark:text-gray-400">
          <span>
            {{ table?.tableApi?.getFilteredSelectedRowModel().rows.length || 0 }} of
            {{ table?.tableApi?.getFilteredRowModel().rows.length || 0 }} row(s) selected
          </span>
          <span>
            Showing {{ pagedRows.length }} of {{ filteredRows.length }} results
          </span>
        </div>
      </template>
    </UCard>

    <TranslationEditor v-model:open="editorOpen" :item="editingItem" @save="onSave" />
  </div>
</template>

<script setup lang="ts">
import { computed, ref, h, resolveComponent, watch } from 'vue'
import type { Component } from 'vue'
import type { TextUnit } from '~/types/translation'
import TranslationEditor from '~/components/translator/TranslationEditor.vue'
import { useTranslator } from '~/composables/useTranslator'
import type { TableColumn, TableRow } from '#ui/types'
import { useGlossary } from '~/composables/useGlossary'
import type { GlossaryTerm } from '~/types/glossary'
import { useLanguageStore } from '~/stores/language'
import { useAppToast } from '~/composables/useAppToast'
import { useEngineStore } from '~/stores/engine'
import { useNotifications } from '~/composables/useNotifications'

const props = defineProps<{ items: TextUnit[] }>()
const emit = defineEmits<{
  (e: 'save', payload: { id: string; translated_text: string; prompt_type?: string }): void
  (e: 'remove', id: string): void
  (e: 'retranslate-selected', selectedRows: Row[]): void
}>()
const { isBusy, retranslate, saveEdit } = useTranslator()
const engineStore = useEngineStore()
const glossary = useGlossary()
const languageStore = useLanguageStore()
const { showToast } = useAppToast()
const { notify } = useNotifications()

// Row selection state
const rowSelection = ref<Record<string, boolean>>({})
const isBulkRetranslating = ref(false)
const isBulkReverting = ref(false)

// Sorting state
const sorting = ref([
  {
    id: 'prompt_type',
    desc: false
  }
])

const promptTypeToCategory: Record<string, string> = {
  Character: 'Characters',
  Class: 'Mechanics',
  Skill: 'Mechanics',
  Equipment: 'Mechanics',
  State: 'Status Effects',
  System: 'Essential Terms',
  Dialogue: 'Essential Terms',
  Other: 'Essential Terms',
}

type Row = { id: string; source_text: string; translated_text: string; prompt_type: string; field_type: string }
const rows = computed<Row[]>(() => props.items.map(u => ({
  id: u.id,
  prompt_type: u.prompt_type,
  source_text: u.source_text,
  translated_text: u.translated_text ?? '',
  field_type: u.field_type
})))

const page = ref(1)
const pageSize = ref(25)
const search = ref('')
const placeholderFilter = ref('all')

// Calculate max text length dynamically
const maxTextLength = computed(() => {
  if (rows.value.length === 0) return 200
  const maxSourceLength = Math.max(...rows.value.map(r => r.source_text.length))
  const maxTranslatedLength = Math.max(...rows.value.map(r => r.translated_text.length))
  return Math.max(maxSourceLength, maxTranslatedLength, 200)
})

const textLengthRange = ref([0, 200])

// Predefined placeholder types based on documentation
const placeholderOptions = computed(() => {
  // Get all unique placeholder types that actually exist in the current data
  const existingPlaceholders = new Set<string>()
  
  // Scan all text units for placeholder patterns
  rows.value.forEach(row => {
    const text = `${row.source_text} ${row.translated_text}`
    // Match patterns like [NEWLINE_1], [COLOR_6], [VARIABLE_16], etc.
    const matches = text.match(/\[([A-Z_]+)_\d+\]/g)
    if (matches) {
      matches.forEach(match => {
        const placeholderType = match.replace(/\[|\]/g, '').replace(/_\d+$/, '')
        existingPlaceholders.add(placeholderType)
      })
    }
  })
  
  // Comprehensive list of all possible placeholder types from documentation
  const allPlaceholderTypes = [
    // Common placeholders
    'ARG', 'NUM_PREFIX', 'FWSPC', 'SPC', 'TAB', 'NEWLINE', 'CARRIAGE_RETURN',
    'CTRL_DOT', 'CTRL_WAIT', 'CTRL_INSTANT', 'CTRL_INPUT',
    
    // RPG Maker placeholders
    'COLOR', 'NAME', 'VARIABLE', 'variable', 'SWITCH', 'ITEM', 'WEAPON', 
    'ARMOR', 'ACTOR', 'GOLD', 'CURRENCY', 'CONDITIONAL',
    
    // Wolf RPG placeholders
    'ICON', 'FONT', 'WOLF_END', 'RUBY_START', 'AT', 'SLOT', 'CSELF',
    
    // Additional patterns found in your data
    'AWSPC', 'BACKGROUND', 'BASE', 'BONE_CREAK', 'IWSPC', 'I_FSPC'
  ]
  
  // Filter to only show placeholders that exist in current data
  const availablePlaceholders = allPlaceholderTypes.filter(type => 
    existingPlaceholders.has(type)
  )
  
  // Convert to select options format
  const options = [
    { label: 'All placeholders', value: 'all' },
    ...availablePlaceholders.sort().map(placeholder => ({
      label: `[${placeholder}_*]`,
      value: placeholder
    }))
  ]
  
  return options
})

const filteredRows = computed(() => {
  const q = search.value.trim().toLowerCase()
  const [minLength, maxLength] = textLengthRange.value
  const placeholderType = placeholderFilter.value
  
  let filtered = rows.value
  
  // Apply text length filter
  filtered = filtered.filter(r => {
    const sourceLength = r.source_text.length
    const translatedLength = r.translated_text.length
    // Show if either source or translated text falls within the range
    return (sourceLength >= minLength! && sourceLength <= maxLength!) ||
           (translatedLength >= minLength! && translatedLength <= maxLength!)
  })
  
  // Apply placeholder filter
  if (placeholderType && placeholderType !== 'all') {
    filtered = filtered.filter(r => {
      const text = `${r.source_text} ${r.translated_text}`
      // Check if text contains the selected placeholder type
      const placeholderPattern = new RegExp(`\\[${placeholderType}_\\d+\\]`, 'g')
      return placeholderPattern.test(text)
    })
  }
  
  // Apply search filter
  if (q) {
    filtered = filtered.filter(r =>
      r.source_text.toLowerCase().includes(q) ||
      r.translated_text.toLowerCase().includes(q) ||
      String(r.prompt_type || '').toLowerCase().includes(q) ||
      String(r.field_type || '').toLowerCase().includes(q)
    )
  }
  
  return filtered
})

// Reset page when filters change
watch([search, textLengthRange, placeholderFilter], () => {
  page.value = 1
}, { deep: true })

// Update text length range when max changes
watch(maxTextLength, (newMax) => {
  if (textLengthRange.value[1]! > newMax) {
    textLengthRange.value = [0, newMax]
  }
})

const pageCount = computed(() => Math.max(1, Math.ceil(filteredRows.value.length / pageSize.value)))
const pagedRows = computed(() => {
  const start = (page.value - 1) * pageSize.value
  return filteredRows.value.slice(start, start + pageSize.value)
})

const hasTranslated = computed(() => filteredRows.value.some(row => row.translated_text && row.translated_text.trim() !== ''))

// Table ref
const table = useTemplateRef('table')

// Selected rows computed using table API
const selectedRows = computed((): Row[] => {
  const selectedRowModel = table.value?.tableApi?.getFilteredSelectedRowModel()
  return selectedRowModel?.rows?.map((row: { original: Row }) => row.original) || []
})

const columns: TableColumn<Row>[] = [
  {
    id: 'select',
    header: ({ table }) => {
      const UCheckbox = resolveComponent('UCheckbox') as Component
      return h(UCheckbox, {
        modelValue: table.getIsSomePageRowsSelected()
          ? 'indeterminate'
          : table.getIsAllPageRowsSelected(),
        'onUpdate:modelValue': (value: boolean | 'indeterminate') =>
          table.toggleAllPageRowsSelected(!!value),
        'aria-label': 'Select all'
      })
    },
    cell: ({ row }) => {
      const UCheckbox = resolveComponent('UCheckbox') as Component
      return h(UCheckbox, {
        modelValue: row.getIsSelected(),
        'onUpdate:modelValue': (value: boolean | 'indeterminate') => row.toggleSelected(!!value),
        'aria-label': 'Select row'
      })
    }
  },
  { 
    accessorKey: 'prompt_type', 
    header: ({ column }) => {
      const isSorted = column.getIsSorted()
      const UButton = resolveComponent('UButton') as Component
      return h(UButton, {
        color: 'neutral',
        variant: 'ghost',
        label: 'Type',
        icon: isSorted
          ? isSorted === 'asc'
            ? 'i-lucide-arrow-up-narrow-wide'
            : 'i-lucide-arrow-down-wide-narrow'
          : 'i-lucide-arrow-up-down',
        class: '-mx-2.5',
        onClick: () => column.toggleSorting(column.getIsSorted() === 'asc')
      })
    },
    enableSorting: true 
  },
  { 
    accessorKey: 'field_type', 
    header: ({ column }) => {
      const isSorted = column.getIsSorted()
      const UButton = resolveComponent('UButton') as Component
      return h(UButton, {
        color: 'neutral',
        variant: 'ghost',
        label: 'Field Type',
        icon: isSorted
          ? isSorted === 'asc'
            ? 'i-lucide-arrow-up-narrow-wide'
            : 'i-lucide-arrow-down-wide-narrow'
          : 'i-lucide-arrow-up-down',
        class: '-mx-2.5',
        onClick: () => column.toggleSorting(column.getIsSorted() === 'asc')
      })
    },
    enableSorting: true 
  },
  { accessorKey: 'source_text', header: 'Source', enableSorting: false },
  { accessorKey: 'translated_text', header: 'Translated', enableSorting: false },
  { 
    id: 'raw_text',
    header: 'Raw Text',
    enableSorting: false,
    cell: ({ row }) => {
      const UButton = resolveComponent('UButton') as Component
      return h(UButton, {
        size: 'xs',
        color: 'neutral',
        variant: 'soft',
        icon: 'i-lucide-undo',
        disabled: isBusy.value,
        onClick: () => onRevertToRaw(row.original.id)
      }, { default: () => 'Revert' })
    }
  },
  {
    id: 'actions',
    header: 'Actions',
    enableSorting: false,
    cell: ({ row }) => {
      const UButton = resolveComponent('UButton') as Component
      return h('div', { class: 'flex gap-2' }, [
        h(UButton, {
          size: 'xs',
          color: 'primary',
          variant: 'soft',
          icon: 'i-lucide-refresh-cw',
          disabled: isBusy.value,
          onClick: async () => { await onRetranslate(row.original.id) }
        }, { default: () => 'Re-translate' }),
        h(UButton, {
          size: 'xs',
          color: 'warning',
          variant: 'soft',
          icon: 'i-lucide-plus',
          disabled: isBusy.value,
          onClick: async () => { await onAddToGlossary(row.original.id) }
        }, { default: () => 'Add to glossary' }),
        h(UButton, {
          size: 'xs',
          color: 'error',
          variant: 'soft',
          icon: 'i-lucide-trash-2',
          disabled: isBusy.value,
          onClick: () => { onRemove(row.original.id) }
        }, { default: () => 'Remove' }),
        h(UButton, {
          size: 'xs',
          color: 'neutral',
          icon: 'i-lucide-pencil',
          disabled: isBusy.value || editorOpen.value,
          onClick: () => openEditor(row.original.id)
        }, { default: () => 'Edit' })
      ])
    }
  }
]

const editorOpen = ref(false)
const editingItem = ref<TextUnit | null>(null)

const openEditor = (id: string) => {
  const unit = props.items.find(u => u.id === id) || null
  editingItem.value = unit
  editorOpen.value = !!unit
}

async function onSave(payload: { id: string; translated_text: string }) {
  // Forward to store immediately and also emit for parent listeners
  await saveEdit({ id: payload.id, translated_text: payload.translated_text })
  emit('save', payload)
  editorOpen.value = false
}

async function onRetranslate(id: string) {
  await retranslate(id)
  // Notify when single retranslation is complete
  await notify('Translation Complete', 'Single translation completed successfully')
}

async function onAddToGlossary(id: string) {
  const unit = props.items.find(u => u.id === id)
  if (!unit) return
  const src = languageStore.getLanguage.source_language?.id || 'en'
  const tgt = languageStore.getLanguage.target_language?.id || 'en'
  const category = promptTypeToCategory[unit.prompt_type] || 'Essential Terms'
  const term: GlossaryTerm = {
    id: 0,
    category,
    source_lang: src,
    target_lang: tgt,
    input: unit.source_text,
    output: unit.translated_text || '',
    enabled: true,
  }
  await glossary.save(term)
  showToast('Added to glossary', `${category}: "${term.input}" → "${term.output || '…'}"`, 'success', 2500, 'i-lucide-check-circle')
}

function onRemove(id: string) {
  // Optimistically remove from store; also notify parent
  const idx = engineStore.textUnits.findIndex(u => u.id === id)
  if (idx !== -1) engineStore.textUnits.splice(idx, 1)
  emit('remove', id)
}

// Row selection handlers following Nuxt UI v4 pattern
function onSelect(row: TableRow<Row>) {
  row.toggleSelected(!row.getIsSelected())
}

function clearSelection() {
  rowSelection.value = {}
}

// Bulk retranslation - send to process view
async function onBulkRetranslate() {
  if (selectedRows.value.length < 2) return
  
  try {
    isBulkRetranslating.value = true
    
    // Clear selection first
    clearSelection()
    
    // Emit selected rows to parent (translator.vue) to pass to TranslationProcess
    emit('retranslate-selected', selectedRows.value)
    
  } finally {
    isBulkRetranslating.value = false
  }
}

// Revert single text unit to raw (source text)
async function onRevertToRaw(id: string) {
  const unit = props.items.find(u => u.id === id)
  if (!unit) return
  
  // Update the translated text to be the same as source text
  const payload = { id, translated_text: unit.source_text }
  await saveEdit(payload)
  emit('save', payload)
  
  showToast('Reverted to raw', `Text reverted to source: "${unit.source_text}"`, 'warning', 2000, 'i-lucide-undo')
}

// Bulk revert selected rows to raw (source text)
async function onBulkRevert() {
  if (selectedRows.value.length === 0) return
  
  try {
    isBulkReverting.value = true
    
    // Revert each selected row to its source text
    for (const row of selectedRows.value) {
      const payload = { id: row.id, translated_text: row.source_text }
      await saveEdit(payload)
      emit('save', payload)
    }
    
    // Clear selection
    clearSelection()
    
    showToast('Bulk revert complete', `Reverted ${selectedRows.value.length} items to raw text`, 'warning', 3000, 'i-lucide-undo')
    
  } finally {
    isBulkReverting.value = false
  }
}
</script>


