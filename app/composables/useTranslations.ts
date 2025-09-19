import { ref, computed } from 'vue'
import { useTranslationsStore } from '~/stores/translations'
import { useAppToast } from '~/composables/useAppToast'
import type { TextUnitRecord, TextUnitQuery } from '~/stores/translations'

export function useTranslations() {
  const translationsStore = useTranslationsStore()
  const { showToast } = useAppToast()

  // UI State
  const search = ref('')
  const statusFilter = ref<string>('All')
  const promptTypeFilter = ref<string>('All')
  const page = ref(1)
  const pageSize = ref(50)

  // Filter options
  const statusOptions = [
    'All',
    'NotTranslated',
    'MachineTranslated', 
    'HumanReviewed',
    'Ignored'
  ]

  const promptTypeOptions = [
    'All',
    'Character',
    'Dialogue',
    'System',
    'Equipment',
    'Skill',
    'Class',
    'State',
    'Other'
  ]

  // Computed properties
  const filteredTranslations = computed(() => {
    let filtered = translationsStore.translations

    // Text search
    if (search.value) {
      const searchLower = search.value.toLowerCase()
      filtered = filtered.filter(t => 
        t.source_text.toLowerCase().includes(searchLower) ||
        (t.translated_text && t.translated_text.toLowerCase().includes(searchLower)) ||
        (t.field_type && t.field_type.toLowerCase().includes(searchLower))
      )
    }

    // Status filter
    if (statusFilter.value !== 'All') {
      filtered = filtered.filter(t => t.status === statusFilter.value)
    }

    // Prompt type filter
    if (promptTypeFilter.value !== 'All') {
      filtered = filtered.filter(t => t.prompt_type === promptTypeFilter.value)
    }

    return filtered
  })

  const pagedTranslations = computed(() => {
    const start = (page.value - 1) * pageSize.value
    const end = start + pageSize.value
    return filteredTranslations.value.slice(start, end)
  })

  const pageCount = computed(() => 
    Math.ceil(filteredTranslations.value.length / pageSize.value)
  )

  // Actions
  async function loadTranslations(query: TextUnitQuery = {}) {
    await translationsStore.fetchTranslations(query)
  }

  async function updateTranslation(
    id: number, 
    translatedText: string, 
    status?: string
  ): Promise<boolean> {
    const success = await translationsStore.updateTranslation(id, translatedText, status)
    if (success) {
      showToast('Translation updated successfully', 'success')
    } else {
      showToast('Failed to update translation', 'error')
    }
    return success
  }

  async function deleteTranslation(id: number): Promise<boolean> {
    const success = await translationsStore.deleteTranslation(id)
    if (success) {
      showToast('Translation deleted successfully', 'success')
    } else {
      showToast('Failed to delete translation', 'error')
    }
    return success
  }

  async function bulkDeleteTranslations(ids: number[]): Promise<number> {
    if (ids.length === 0) return 0
    
    const count = await translationsStore.bulkDeleteTranslations(ids)
    if (count > 0) {
      showToast(`Deleted ${count} translations successfully`, 'success')
    } else {
      showToast('Failed to delete translations', 'error')
    }
    return count
  }

  async function getTranslation(id: number): Promise<TextUnitRecord | null> {
    return await translationsStore.getTranslation(id)
  }

  function clearFilters() {
    search.value = ''
    statusFilter.value = 'All'
    promptTypeFilter.value = 'All'
    page.value = 1
  }

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

  return {
    // State
    translations: translationsStore.translations,
    isLoading: translationsStore.isLoading,
    error: translationsStore.error,
    stats: translationsStore.stats,

    // UI State
    search,
    statusFilter,
    promptTypeFilter,
    page,
    pageSize,

    // Filter options
    statusOptions,
    promptTypeOptions,

    // Computed
    filteredTranslations,
    pagedTranslations,
    pageCount,

    // Actions
    loadTranslations,
    updateTranslation,
    deleteTranslation,
    bulkDeleteTranslations,
    getTranslation,
    clearFilters,
    getStatusLabel,
    getStatusColor
  }
}
