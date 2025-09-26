import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { 
  TranslationStatus, 
  TextUnit, 
  TranslationResult,
  TextUnitRecord,
  TextUnitQuery
} from '~/types/translation'
import { useEngineStore } from './engine'
import { useAppToast } from '~/composables/useAppToast'
import { useSettingsStore } from './settings'
import { useNotifications } from '~/composables/useNotifications'

/**
 * Unified Translation Store
 * 
 * Combines CRUD operations and translation workflow into a single store.
 * Manages translation data, progress tracking, and AI translation process.
 */
export const useTranslationStore = defineStore('translation', () => {
  const { showToast } = useAppToast()
  const settingsStore = useSettingsStore()
  const engineStore = useEngineStore()
  
  // ===== DATA STATE (from translations.ts) =====
  const translations = ref<TextUnitRecord[]>([])
  const isLoading = ref(false)
  const error = ref<string | null>(null)
  const stats = ref<Record<string, unknown> | null>(null)

  // ===== TRANSLATION PROCESS STATE (from translator.ts) =====
  const isTranslating = ref(false)
  const translationProgress = ref(0)
  const translationTotal = ref(0)
  const currentTranslatingUnit = ref<TextUnit | null>(null)
  const failedTranslations = ref<Array<{ unit: TextUnit; error: string }>>([])

  // ===== COMPUTED PROPERTIES =====
  const isTranslationInProgress = computed(() => isTranslating.value)
  const translationProgressPercentage = computed(() => {
    if (translationTotal.value === 0) return 0
    return Math.round((translationProgress.value / translationTotal.value) * 100)
  })
  const hasFailedTranslations = computed(() => failedTranslations.value.length > 0)

  // ===== CRUD OPERATIONS (from translations.ts) =====
  
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
    status?: TranslationStatus
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
      const data = await invoke<Record<string, unknown>>('get_translation_stats_cmd', { projectPath })
      stats.value = data
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to load stats'
      console.error('Error fetching stats:', e)
    }
  }

  // ===== TRANSLATION PROCESS OPERATIONS (from translator.ts) =====

  const translateTextUnit = async (textUnit: TextUnit): Promise<TextUnit> => {
    try {
      isTranslating.value = true
      currentTranslatingUnit.value = textUnit

      const unitPayload = engineStore.getTextUnitById(textUnit.id)
      const enginePayload = engineStore.projectInfo
      const translationResult = await invoke<TranslationResult>('translate_text_unit', {
        // snake_case (current backend)
        text_unit: unitPayload,
        engine_info: enginePayload,
        // camelCase (compat with older backend builds)
        textUnit: unitPayload,
        engineInfo: enginePayload,
        // common
        config: settingsStore.providerConfig,
      })

      // Debug log for inspection (raw output)
      console.debug('[MT][raw]', {
        id: translationResult.text_unit.id,
        prompt_type: translationResult.text_unit.prompt_type,
        source: translationResult.text_unit.source_text,
        target: translationResult.text_unit.translated_text,
      })

      // Update the engine store with the translated unit
      engineStore.updateTextUnit(translationResult.text_unit)

      // Remove from failed translations if it was there
      const failedIndex = failedTranslations.value.findIndex(f => f.unit.id === textUnit.id)
      if (failedIndex !== -1) {
        failedTranslations.value.splice(failedIndex, 1)
      }

      return translationResult.text_unit
    } catch (error) {
      console.error('Translation error:', error)
      
      // Add to failed translations
      const errorMessage = error instanceof Error ? error.message : 'Translation failed'
      const existingFailedIndex = failedTranslations.value.findIndex(f => f.unit.id === textUnit.id)
      if (existingFailedIndex !== -1) {
        failedTranslations.value[existingFailedIndex] = { unit: textUnit, error: errorMessage }
      } else {
        failedTranslations.value.push({ unit: textUnit, error: errorMessage })
      }

      showToast('Translation Failed', `Failed to translate "${textUnit.source_text.substring(0, 50)}...": ${errorMessage}`, 'error', 1500)

      throw error
    } finally {
      isTranslating.value = false
      currentTranslatingUnit.value = null
    }
  }

  const startBatchTranslation = async (
    textUnits: TextUnit[],
    onUnitTranslated?: (unit: TextUnit) => void
  ) => {
    if (isTranslating.value) {
      throw new Error('Translation already in progress')
    }

    try {
      isTranslating.value = true
      translationTotal.value = textUnits.length
      translationProgress.value = 0
      failedTranslations.value = []

      showToast('Batch Translation Started', `Translating ${textUnits.length} text units`, 'info', 1200)

      for (const textUnit of textUnits) {
        try {
          const translatedUnit = await translateTextUnit(textUnit)

          translationProgress.value++
          onUnitTranslated?.(translatedUnit)

          // Small delay to prevent overwhelming the API
          await new Promise(resolve => setTimeout(resolve, 100))
        } catch (error) {
          console.error(`Failed to translate unit ${textUnit.id}:`, error)
          // Abort batch on fatal provider quota errors for clarity
          const message = error instanceof Error ? error.message : String(error)
          const msgLower = message.toLowerCase()
          const isFatalQuota = msgLower.includes('insufficient_quota') || msgLower.includes('exceeded your current quota')
          if (isFatalQuota) {
            showToast('Provider Quota Exceeded', 'Provider reports insufficient quota. Check your plan/billing.', 'error', 2500)
            break
          }
          // Otherwise continue to next unit
        }
      }

      const successCount = translationProgress.value
      const failedCount = failedTranslations.value.length
      const totalCount = translationTotal.value

      showToast('Batch Translation Completed', `Successfully translated ${successCount} units${failedCount > 0 ? `, ${failedCount} failed` : ''}`, failedCount > 0 ? 'warning' : 'success', 1500)

      // Send notification for batch translation completion
      const { notifyTranslationComplete } = useNotifications()
      await notifyTranslationComplete(successCount, totalCount, failedCount)

    } catch (error) {
      console.error('Batch translation error:', error)
      showToast('Batch Translation Failed', error instanceof Error ? error.message : 'Batch translation failed', 'error', 1500)
      throw error
    } finally {
      isTranslating.value = false
      currentTranslatingUnit.value = null
    }
  }

  const reset = () => {
    isTranslating.value = false
    translationProgress.value = 0
    translationTotal.value = 0
    currentTranslatingUnit.value = null
    failedTranslations.value = []
  }

  // ===== RETURN STORE API =====
  return {
    // State
    translations,
    isLoading,
    error,
    stats,
    isTranslating,
    translationProgress,
    translationTotal,
    currentTranslatingUnit,
    failedTranslations,

    // Computed
    isTranslationInProgress,
    translationProgressPercentage,
    hasFailedTranslations,

    // CRUD Actions
    fetchTranslations,
    getTranslation,
    updateTranslation,
    deleteTranslation,
    bulkDeleteTranslations,
    fetchStats,

    // Translation Process Actions
    translateTextUnit,
    startBatchTranslation,
    reset,
  }
})
