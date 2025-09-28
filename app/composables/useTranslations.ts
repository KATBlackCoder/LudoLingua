import { computed } from 'vue'
import { useTranslationStore } from '~/stores/translation'
import { useAppToast } from '~/composables/useAppToast'
import type { TextUnitRecord, TextUnitQuery, TranslationStatus } from '~/types/translation'

export function useTranslations() {
  const translationsStore = useTranslationStore()
  const { showToast } = useAppToast()

  // Raw data access (components handle their own filtering)
  const allTranslations = computed(() => translationsStore.translations)

  // Filtering helper function (components handle their own filter state)
  const getFilteredData = (
    data: TextUnitRecord[], 
    search: string, 
    statusFilter: string, 
    promptTypeFilter: string
  ) => {
    let filtered = data

    // Text search
    if (search) {
      const searchLower = search.toLowerCase()
      filtered = filtered.filter(t => 
        t.source_text.toLowerCase().includes(searchLower) ||
        (t.translated_text && t.translated_text.toLowerCase().includes(searchLower)) ||
        (t.field_type && t.field_type.toLowerCase().includes(searchLower))
      )
    }

    // Status filter
    if (statusFilter !== 'All') {
      filtered = filtered.filter(t => t.status === statusFilter)
    }

    // Prompt type filter
    if (promptTypeFilter !== 'All') {
      filtered = filtered.filter(t => t.prompt_type === promptTypeFilter)
    }

    return filtered
  }

  // Pagination helper functions (components handle their own pagination state)
  const getPagedData = <T>(data: T[], page: number, pageSize: number): T[] => {
    const start = (page - 1) * pageSize
    const end = start + pageSize
    return data.slice(start, end)
  }

  const getPageCount = (totalItems: number, pageSize: number) => 
    Math.ceil(totalItems / pageSize)

  // Actions
  async function loadTranslations(query: TextUnitQuery = {}) {
    await translationsStore.fetchTranslations(query)
  }

  async function updateTranslation(
    id: number, 
    translatedText: string, 
    status?: TranslationStatus
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
    // Use native Tauri dialog for deletion confirmation
    const { ask } = await import('@tauri-apps/plugin-dialog')
    
    const confirmed = await ask('Are you sure you want to delete this translation?', {
      title: 'Delete Translation',
      kind: 'warning'
    })
    
    if (!confirmed) return false
    
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
    
    // Use native Tauri dialog for bulk deletion confirmation
    const { ask } = await import('@tauri-apps/plugin-dialog')
    
    const confirmed = await ask(
      `Are you sure you want to delete ${ids.length} translation${ids.length > 1 ? 's' : ''}?`, 
      {
        title: 'Delete Translations',
        kind: 'warning'
      }
    )
    
    if (!confirmed) return 0
    
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

  // Project management functions
  async function loadAvailableProjects(): Promise<Array<{ label: string; value: string }>> {
    try {
      // Get list of projects from database/manifest files
      const { invoke } = await import('@tauri-apps/api/core')
      const projects = await invoke('get_available_projects') as { name: string; path: string; hash: string }[]
      
      return [
        { label: 'All Projects', value: 'none' },
        ...projects.map(project => ({
          label: project.name || project.path.split('/').pop() || 'Unknown Project',
          value: project.hash
        }))
      ]
    } catch (error) {
      console.error('Error loading projects:', error)
      // Fallback to empty list if command doesn't exist yet
      return [
        { label: 'All Projects', value: 'none' }
      ]
    }
  }

  async function deleteProject(projectHash: string, projectName: string): Promise<boolean> {
    // Use native Tauri dialog for project deletion confirmation
    const { ask } = await import('@tauri-apps/plugin-dialog')
    
    const confirmed = await ask(
      `Are you sure you want to delete the project "${projectName}"?\n\nThis will permanently remove all translations and project data. This action cannot be undone.`, 
      {
        title: 'Delete Project',
        kind: 'warning'
      }
    )
    
    if (!confirmed) return false
    
    try {
      // Delete project and all its translations
      const { invoke } = await import('@tauri-apps/api/core')
      await invoke('delete_project', { projectHash })
      
      // Show success message
      showToast(
        'Project Deleted',
        `Project "${projectName}" has been successfully deleted.`,
        'success',
        5000,
        'i-lucide-check-circle'
      )
      
      return true
    } catch (error) {
      console.error('Error deleting project:', error)
      
      // Show error message
      showToast(
        'Error Deleting Project',
        error instanceof Error ? error.message : 'Failed to delete project',
        'error',
        7000,
        'i-lucide-alert-triangle'
      )
      
      return false
    }
  }

  // Clear filters helper (components handle their own filter state)
  const getDefaultFilters = () => ({
    search: '',
    statusFilter: 'All',
    promptTypeFilter: 'All'
  })

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

    // Filter options moved to shared utilities

    // Computed
    allTranslations,

    // Helper functions
    getFilteredData,
    getPagedData,
    getPageCount,
    getDefaultFilters,

    // Actions
    loadTranslations,
    updateTranslation,
    deleteTranslation,
    bulkDeleteTranslations,
    getTranslation,
    getStatusLabel,
    getStatusColor,
    
    // Project management
    loadAvailableProjects,
    deleteProject
  }
}
