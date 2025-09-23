import { computed, ref } from 'vue'
import { storeToRefs } from 'pinia'
import { useEngineStore } from '~/stores/engine'
import { useTranslatorStore } from '~/stores/translator'
import { TranslationStatus } from '~/types/translation'

type Mode = 'raw' | 'process' | 'result'
type ProcessRowStatus = 'pending' | 'processing' | 'done' | 'error'
export type ProcessRow = { id: string; source_text: string; target_text: string; status: ProcessRowStatus }

// Singleton reactive state shared across all component calls
const mode = ref<Mode>('raw')
const processRows = ref<ProcessRow[]>([])
const startTimestampMs = ref<number | null>(null)
const elapsedMs = ref<number>(0)
let intervalId: number | null = null

function formatDuration(ms: number): string {
  if (!Number.isFinite(ms) || ms < 0) ms = 0
  const totalSeconds = Math.floor(ms / 1000)
  const hours = Math.floor(totalSeconds / 3600)
  const minutes = Math.floor((totalSeconds % 3600) / 60)
  const seconds = totalSeconds % 60
  const hh = hours.toString().padStart(2, '0')
  const mm = minutes.toString().padStart(2, '0')
  const ss = seconds.toString().padStart(2, '0')
  return hours > 0 ? `${hh}:${mm}:${ss}` : `${mm}:${ss}`
}

function startTimer(): void {
  stopTimer()
  startTimestampMs.value = Date.now()
  elapsedMs.value = 0
  intervalId = window.setInterval(() => {
    if (startTimestampMs.value) {
      elapsedMs.value = Date.now() - startTimestampMs.value
    }
  }, 1000)
}

function stopTimer(): void {
  if (intervalId !== null) {
    clearInterval(intervalId)
    intervalId = null
  }
  startTimestampMs.value = null
}

export function useTranslator() {
  const engineStore = useEngineStore()
  const translatorStore = useTranslatorStore()

  const translatedItems = computed(() => engineStore.textUnits.filter((u) =>
    u.status === 'MachineTranslated' ||
    u.status === 'HumanReviewed' ||
    u.status === 'Ignored' ||
    (!!u.translated_text && u.translated_text.trim() !== '')
  ))
  // Use storeToRefs to get reactive refs from the store (state + getters)
  const {
    isTranslationInProgress,
    translationProgress,
    translationTotal,
    failedTranslations,
  } = storeToRefs(translatorStore)
  const isBusy = isTranslationInProgress
  const failedCount = computed(() => failedTranslations.value.length)

  const elapsedText = computed(() => formatDuration(elapsedMs.value))
  const remainingMs = computed(() => {
    const progress = translationProgress.value
    const total = translationTotal.value
    if (!startTimestampMs.value || progress <= 0) return 0
    const perUnit = elapsedMs.value / progress
    const remainingUnits = Math.max(0, total - progress)
    return Math.floor(perUnit * remainingUnits)
  })
  const remainingText = computed(() => formatDuration(remainingMs.value))

  // mode/processRows are module-level singletons

  const textUnits = computed(() => engineStore.textUnits)

  // Enhanced auto-switch logic for intelligent workflow routing
  const hasNotTranslated = computed(() => textUnits.value.some(u => u.status === 'NotTranslated'))
  const hasTranslated = computed(() => textUnits.value.some(u =>
    (u.status === 'MachineTranslated' || u.status === 'HumanReviewed' || u.status === 'Ignored') &&
    u.translated_text && u.translated_text.trim() !== ''
  ))

  // Auto-determine initial mode based on project state
  const determineInitialMode = () => {
    if (textUnits.value.length === 0) {
      return 'raw' // No units loaded yet
    }

    if (!hasNotTranslated.value && hasTranslated.value) {
      // All units are translated - go directly to results
      return 'result'
    } else if (hasNotTranslated.value && !hasTranslated.value) {
      // Only raw units - go to raw view
      return 'raw'
    } else {
      // Mixed state - user can choose, but start with raw for translation workflow
      return 'raw'
    }
  }

  // Set initial mode when text units are loaded
  if (textUnits.value.length > 0 && mode.value === 'raw') {
    mode.value = determineInitialMode()
  }

  const startProcess = async () => {
    const untranslated = engineStore.textUnits.filter((u) => u.status === 'NotTranslated')
    if (!untranslated.length) {
      // If nothing to translate, jump directly to result if there are any translations
      mode.value = translatedItems.value.length ? 'result' : 'raw'
      return
    }
    processRows.value = untranslated.map((u): ProcessRow => ({ id: u.id, source_text: u.source_text, target_text: '', status: 'pending' }))
    mode.value = 'process'
    if (processRows.value.length) {
      processRows.value[0]!.status = 'processing'
    }
    startTimer()
    await translatorStore.startBatchTranslation(untranslated, (translatedUnit) => {
      const currentIndex = processRows.value.findIndex(r => r.status === 'processing')
      const rowIndex = processRows.value.findIndex(r => r.id === translatedUnit.id)
      if (rowIndex !== -1) {
        processRows.value[rowIndex]!.status = 'done'
        processRows.value[rowIndex]!.target_text = translatedUnit.translated_text ?? ''
        const nextIndex = currentIndex >= 0 ? currentIndex + 1 : rowIndex + 1
        if (processRows.value[nextIndex]) processRows.value[nextIndex]!.status = 'processing'
      }
    })
    stopTimer()
    mode.value = 'result'
  }

  const translateOne = async (id: string) => {
    const unit = engineStore.getTextUnitById(id)
    if (!unit) return

    // Initialize process view with single row
    processRows.value = [{ id: unit.id, source_text: unit.source_text, target_text: '', status: 'processing' }]
    mode.value = 'process'
    startTimer()
    const translated = await translatorStore.translateTextUnit(unit)
    stopTimer()
    processRows.value[0]!.status = 'done'
    processRows.value[0]!.target_text = translated.translated_text ?? ''
    mode.value = 'result'
  }

  // Re-translate a single unit and return updated result
  const retranslate = async (id: string) => {
    const unit = engineStore.getTextUnitById(id)
    if (!unit) return
    const updated = await translatorStore.translateTextUnit(unit)
    return updated
  }

  // Start bulk retranslation process with selected rows (similar to startProcess but for specific items)
  const startBulkRetranslation = async (selectedRows: { id: string; source_text: string; translated_text: string; prompt_type: string; field_type: string }[]) => {
    if (!selectedRows.length) return

    // Convert selected rows to ProcessRow format
    processRows.value = selectedRows.map((row): ProcessRow => ({ 
      id: row.id, 
      source_text: row.source_text, 
      target_text: '', 
      status: 'pending' 
    }))
    
    mode.value = 'process'
    if (processRows.value.length) {
      processRows.value[0]!.status = 'processing'
    }
    
    startTimer()
    
    // Get the actual text units for translation
    const unitsToTranslate = selectedRows.map(row => engineStore.getTextUnitById(row.id)).filter((unit): unit is NonNullable<typeof unit> => unit !== null && unit !== undefined)
    
    await translatorStore.startBatchTranslation(unitsToTranslate, (translatedUnit) => {
      const currentIndex = processRows.value.findIndex(r => r.status === 'processing')
      const rowIndex = processRows.value.findIndex(r => r.id === translatedUnit.id)
      if (rowIndex !== -1) {
        processRows.value[rowIndex]!.status = 'done'
        processRows.value[rowIndex]!.target_text = translatedUnit.translated_text ?? ''
        const nextIndex = currentIndex >= 0 ? currentIndex + 1 : rowIndex + 1
        if (processRows.value[nextIndex]) processRows.value[nextIndex]!.status = 'processing'
      }
    })
    
    stopTimer()
    mode.value = 'result'
  }

  const reset = async () => {
    engineStore.textUnits.forEach((u) => { u.translated_text = ''; u.status = TranslationStatus.NotTranslated })
  }

  const saveEdit = async (payload: { id: string; translated_text: string }) => {
    const unit = engineStore.getTextUnitById(payload.id)
    if (!unit) return
    
    // Update in-memory store immediately for UI responsiveness
    unit.translated_text = payload.translated_text
    unit.status = TranslationStatus.HumanReviewed
    
    // Also persist to database
    try {
      // Convert string ID to number for database (assuming the ID is numeric)
      const numericId = parseInt(payload.id, 10)
      if (!isNaN(numericId)) {
        // Use dynamic import to avoid build-time dependency issues
        const { invoke } = await import('@tauri-apps/api/core')
        await invoke('update_translation_cmd', {
          id: numericId,
          translatedText: payload.translated_text,
          status: 'HumanReviewed'
        })
      }
    } catch (error) {
      console.error('Failed to save translation to database:', error)
      // Note: We don't revert the in-memory change since the user expects it to be saved
      // The database will be synced on next project load
    }
  }



  return {
    // state
    mode,
    processRows,
    textUnits,
    translatedItems,
    hasTranslated,
    isBusy,
    translationProgress,
    translationTotal,
    failedCount,

    // enhanced workflow state
    hasNotTranslated,
    determineInitialMode,

    // actions
    startProcess,
    translateOne,
    retranslate,
    startBulkRetranslation,
    reset,
    saveEdit,

    // timing
    elapsedMs,
    remainingMs,
    elapsedText,
    remainingText,
  }
}


