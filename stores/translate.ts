import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { TextUnit } from '../types/translation';
import { useProviderStore } from './provider';
import { useEngineStore } from './engine';

export const useTranslateStore = defineStore('translate', () => {
  const toast = useToast();
  const providerStore = useProviderStore();
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
    textUnit: TextUnit,
    glossaryTerms: [string, string][] = []
  ): Promise<TextUnit> => {
    try {
      isTranslating.value = true;
      currentTranslatingUnit.value = textUnit;

      //console.log('Translating text unit:', textUnit.id);
      const translatedUnit = await invoke<TextUnit>('translate_text_unit', {
        textUnit: engineStore.getTextUnitById(textUnit.id),
        config: providerStore.currentProviderConfig,
        engineInfo: engineStore.projectInfo,
        glossaryTerms: glossaryTerms.length > 0 ? glossaryTerms : null,
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

      toast.add({
        title: 'Translation Failed',
        description: `Failed to translate "${textUnit.source_text.substring(0, 50)}...": ${errorMessage}`,
        color: 'error',
      });

      throw error;
    } finally {
      isTranslating.value = false;
      currentTranslatingUnit.value = null;
    }
  };

  const startBatchTranslation = async (
    textUnits: TextUnit[],
    glossaryTerms: [string, string][] = [],
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

      toast.add({
        title: 'Batch Translation Started',
        description: `Translating ${textUnits.length} text units`,
        color: 'info',
      });

      for (const textUnit of textUnits) {
        try {
          const translatedUnit = await translateTextUnit(
            textUnit,
            glossaryTerms
          );

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

      toast.add({
        title: 'Batch Translation Completed',
        description: `Successfully translated ${successCount} units${failedCount > 0 ? `, ${failedCount} failed` : ''}`,
        color: failedCount > 0 ? 'warning' : 'success',
      });

    } catch (error) {
      console.error('Batch translation error:', error);
      toast.add({
        title: 'Batch Translation Failed',
        description: error instanceof Error ? error.message : 'Batch translation failed',
        color: 'error',
      });
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
      
      toast.add({
        title: 'Translation Cancelled',
        description: 'Batch translation has been cancelled',
        color: 'warning',
      });
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