import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { TextUnit } from '../types/translation';
// import { useProviderStore } from './provider';
import { useEngineStore } from './engine';
import { useAppToast } from '~/composables/useAppToast';
import { useSettingsStore } from './settings';

/**
 * Translate store
 *
 * Orchestrates single-unit and batch translation flows. Tracks progress,
 * current unit, and failures. Uses the provider and engine stores for
 * configuration and source data, and calls the `translate_text_unit`
 * Tauri command to perform translations.
 */
export const useTranslateStore = defineStore('translate', () => {
  const { showToast } = useAppToast();
  // const providerStore = useProviderStore();
  const settingsStore = useSettingsStore();
  const engineStore = useEngineStore();
  
  // State
  const isTranslating = ref(false);
  const translationProgress = ref(0);
  const translationTotal = ref(0);
  const currentTranslatingUnit = ref<TextUnit | null>(null);
  const failedTranslations = ref<Array<{ unit: TextUnit; error: string }>>([]);
  
  // Computed
  const isTranslationInProgress = computed(() => isTranslating.value);
  const translationProgressPercentage = computed(() => {
    if (translationTotal.value === 0) return 0;
    return Math.round((translationProgress.value / translationTotal.value) * 100);
  });
  const hasFailedTranslations = computed(() => failedTranslations.value.length > 0);

  // Actions
  const translateTextUnit = async (
    textUnit: TextUnit
  ): Promise<TextUnit> => {
    try {
      isTranslating.value = true;
      currentTranslatingUnit.value = textUnit;

      //console.log('Translating text unit:', textUnit.id);
      const unitPayload = engineStore.getTextUnitById(textUnit.id)
      const enginePayload = engineStore.projectInfo
      const translatedUnit = await invoke<TextUnit>('translate_text_unit', {
        // snake_case (current backend)
        text_unit: unitPayload,
        engine_info: enginePayload,
        // camelCase (compat with older backend builds)
        textUnit: unitPayload,
        engineInfo: enginePayload,
        // common
        config: settingsStore.providerConfig,
      });

      // Debug log for inspection (raw output)
      console.debug('[MT][raw]', {
        id: translatedUnit.id,
        prompt_type: translatedUnit.prompt_type,
        source: translatedUnit.source_text,
        target: translatedUnit.translated_text,
      });

      // Update the engine store with the translated unit
      engineStore.updateTextUnit(translatedUnit);

      // Remove from failed translations if it was there
      const failedIndex = failedTranslations.value.findIndex(f => f.unit.id === textUnit.id);
      if (failedIndex !== -1) {
        failedTranslations.value.splice(failedIndex, 1);
      }

      //console.log('Translation completed for:', textUnit.id);
      return translatedUnit;
    } catch (error) {
      console.error('Translation error:', error);
      
      // Add to failed translations
      const errorMessage = error instanceof Error ? error.message : 'Translation failed';
      const existingFailedIndex = failedTranslations.value.findIndex(f => f.unit.id === textUnit.id);
      if (existingFailedIndex !== -1) {
        failedTranslations.value[existingFailedIndex] = { unit: textUnit, error: errorMessage };
      } else {
        failedTranslations.value.push({ unit: textUnit, error: errorMessage });
      }

      showToast('Translation Failed', `Failed to translate "${textUnit.source_text.substring(0, 50)}...": ${errorMessage}`, 'error', 1500)

      throw error;
    } finally {
      isTranslating.value = false;
      currentTranslatingUnit.value = null;
    }
  };


  const startBatchTranslation = async (
    textUnits: TextUnit[],
    onUnitTranslated?: (unit: TextUnit) => void
  ) => {
    if (isTranslating.value) {
      throw new Error('Translation already in progress');
    }

    try {
      isTranslating.value = true;
      translationTotal.value = textUnits.length;
      translationProgress.value = 0;
      failedTranslations.value = [];

      showToast('Batch Translation Started', `Translating ${textUnits.length} text units`, 'info', 1200)

      for (const textUnit of textUnits) {
        //if (!isTranslating.value) break
        try {
          const translatedUnit = await translateTextUnit(textUnit);

          translationProgress.value++;
          onUnitTranslated?.(translatedUnit);

          // Small delay to prevent overwhelming the API
          await new Promise(resolve => setTimeout(resolve, 100));
        } catch (error) {
          console.error(`Failed to translate unit ${textUnit.id}:`, error);
          // Error is already handled in translateTextUnit
        }
      }

      const successCount = translationProgress.value;
      const failedCount = failedTranslations.value.length;

      showToast('Batch Translation Completed', `Successfully translated ${successCount} units${failedCount > 0 ? `, ${failedCount} failed` : ''}`, failedCount > 0 ? 'warning' : 'success', 1500)

    } catch (error) {
      console.error('Batch translation error:', error);
      showToast('Batch Translation Failed', error instanceof Error ? error.message : 'Batch translation failed', 'error', 1500)
      throw error;
    } finally {
      isTranslating.value = false;
      currentTranslatingUnit.value = null;
    }
  };

  const cancelBatchTranslation = () => {
    if (isTranslating.value) {
      isTranslating.value = false;
      currentTranslatingUnit.value = null;
      
      showToast('Translation Cancelled', 'Batch translation has been cancelled', 'warning', 1200)
    }
  };

  const reset = () => {
    isTranslating.value = false;
    translationProgress.value = 0;
    translationTotal.value = 0;
    currentTranslatingUnit.value = null;
    failedTranslations.value = [];
  };

  return {
    // State
    isTranslating,
    translationProgress,
    translationTotal,
    currentTranslatingUnit,
    failedTranslations,

    // Computed
    isTranslationInProgress,
    translationProgressPercentage,
    hasFailedTranslations,

    // Actions
    translateTextUnit,
    startBatchTranslation,
    cancelBatchTranslation,
    reset,
  };
}); 