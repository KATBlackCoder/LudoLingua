import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// Types matching the backend TextUnitRecord
export interface TextUnitRecord {
  id?: number
  project_path: string
  file_path: string
  field_type: string
  source_text: string
  translated_text?: string
  status: string
  prompt_type: string
  source_lang: string
  target_lang: string
  manifest_hash?: string
  created_at?: string
  updated_at?: string
}

export interface TextUnitQuery {
  manifest_hash?: string
  status?: string
  prompt_type?: string
  source_lang?: string
  target_lang?: string
  search_text?: string
  limit?: number
  offset?: number
}

export const useTranslationsStore = defineStore('translations', () => {
  const translations = ref<TextUnitRecord[]>([])
  const isLoading = ref(false)
  const error = ref<string | null>(null)
  const stats = ref<any>(null)

  async function fetchTranslations(query: TextUnitQuery = {}) {
    try {
      isLoading.value = true
      error.value = null
      const data = await invoke<TextUnitRecord[]>('list_translations_cmd', { query })
      translations.value = data
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to load translations'
      console.error('Error fetching translations:', e)
    } finally {
      isLoading.value = false
    }
  }

  async function getTranslation(id: number): Promise<TextUnitRecord | null> {
    try {
      const data = await invoke<TextUnitRecord>('get_translation_cmd', { id })
      return data
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to get translation'
      console.error('Error getting translation:', e)
      return null
    }
  }

  async function updateTranslation(
    id: number, 
    translatedText: string, 
    status?: string
  ): Promise<boolean> {
    try {
      const success = await invoke<boolean>('update_translation_cmd', { 
        id, 
        translatedText, 
        status 
      })
      if (success) {
        // Refresh the list to show updated data
        await fetchTranslations()
      }
      return success
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to update translation'
      console.error('Error updating translation:', e)
      return false
    }
  }

  async function deleteTranslation(id: number): Promise<boolean> {
    try {
      const success = await invoke<boolean>('delete_translation_cmd', { id })
      if (success) {
        // Remove from local state
        translations.value = translations.value.filter(t => t.id !== id)
      }
      return success
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to delete translation'
      console.error('Error deleting translation:', e)
      return false
    }
  }

  async function bulkDeleteTranslations(ids: number[]): Promise<number> {
    try {
      const deletedCount = await invoke<number>('bulk_delete_translations_cmd', { ids })
      if (deletedCount > 0) {
        // Remove from local state
        translations.value = translations.value.filter(t => !ids.includes(t.id!))
      }
      return deletedCount
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to delete translations'
      console.error('Error bulk deleting translations:', e)
      return 0
    }
  }

  async function fetchStats(projectPath?: string) {
    try {
      const data = await invoke<any>('get_translation_stats_cmd', { projectPath })
      stats.value = data
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to load stats'
      console.error('Error fetching stats:', e)
    }
  }

  return {
    translations,
    isLoading,
    error,
    stats,
    fetchTranslations,
    getTranslation,
    updateTranslation,
    deleteTranslation,
    bulkDeleteTranslations,
    fetchStats
  }
})
