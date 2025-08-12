import { computed, ref } from 'vue'
import { storeToRefs } from 'pinia'
import { useEngineStore } from '~/stores/engine'
import { useTranslateStore } from '~/stores/translate'
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

export function useTranslation() {
  const engineStore = useEngineStore()
  const translateStore = useTranslateStore()

  const translatedItems = computed(() => engineStore.textUnits.filter((u) =>
    u.status === 'MachineTranslated' ||
    u.status === 'HumanReviewed' ||
    (!!u.translated_text && u.translated_text.trim() !== '')
  ))

  const hasTranslated = computed(() => translatedItems.value.length > 0)
  // Use storeToRefs to get reactive refs from the store (state + getters)
  const {
    isTranslationInProgress,
    translationProgress,
    translationTotal,
    failedTranslations,
  } = storeToRefs(translateStore)
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

  // If project has pre-translated units (e.g., loaded from manifest), jump to result view
  // Switch to Result mode when there are prefilled translations (e.g., loaded via manifest)
  if (engineStore.textUnits.some(u => u.translated_text && u.translated_text.trim() !== '')) {
    mode.value = 'result'
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
    await translateStore.startBatchTranslation(untranslated, (translatedUnit) => {
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
    const translated = await translateStore.translateTextUnit(unit)
    stopTimer()
    processRows.value[0]!.status = 'done'
    processRows.value[0]!.target_text = translated.translated_text ?? ''
    mode.value = 'result'
  }

  // Re-translate a single unit and return updated result
  const retranslate = async (id: string) => {
    const unit = engineStore.getTextUnitById(id)
    if (!unit) return
    const updated = await translateStore.translateTextUnit(unit)
    return updated
  }

  const reset = async () => {
    engineStore.textUnits.forEach((u) => { u.translated_text = ''; u.status = TranslationStatus.NotTranslated })
  }

  const saveEdit = (payload: { id: string; translated_text: string }) => {
    const unit = engineStore.getTextUnitById(payload.id)
    if (!unit) return
    unit.translated_text = payload.translated_text
    unit.status = TranslationStatus.HumanReviewed
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

    // actions
    startProcess,
    translateOne,
    retranslate,
    reset,
    saveEdit,

    // timing
    elapsedMs,
    remainingMs,
    elapsedText,
    remainingText,
  }
}


