<template>
  <div v-if="tableData.length > 0">
    <!-- Header with Translate All and Inject buttons -->
    <div class="flex items-center justify-between mb-4">
      <h3 class="text-lg font-semibold">Translation Units</h3>
      <div class="flex gap-2">
        <UButton
          label="Translate All"
          icon="i-heroicons-arrow-right"
          color="primary"
          :loading="translateStore.isTranslationInProgress"
          :disabled="translateStore.isTranslationInProgress || !hasUntranslatedUnits"
          @click="translateAll"
        />
        <UButton
          label="Inject"
          icon="i-heroicons-document-arrow-down"
          color="success"
          :loading="engineStore.isLoading"
          :disabled="engineStore.isLoading || !hasTranslatedUnits || !isTranslationFinished"
          @click="injectTranslations"
        />
        <UButton
          label="Reset"
          icon="i-heroicons-arrow-path"
          color="neutral"
          :loading="engineStore.isLoading"
          :disabled="engineStore.isLoading || !hasTranslatedUnits"
          @click="resetTranslations"
        />
        <UButton
          label="Export JSON"
          icon="i-heroicons-document-arrow-up"
          color="warning"
          :disabled="!hasTranslatedUnits"
          @click="exportToJson"
        />
      </div>
    </div>

    <!-- Progress indicator -->
    <div v-if="translateStore.isTranslationInProgress" class="mb-4">
      <UProgress 
        :value="translateStore.translationProgressPercentage" 
        :max="100"
        class="mb-2"
      />
      <p class="text-sm text-gray-600 dark:text-gray-400">
        Translating {{ translateStore.translationProgress }} of {{ translateStore.translationTotal }} units...
      </p>
    </div>

    <!-- Inline connection error -->
    <div v-if="inlineError" class="mb-3 text-sm text-red-600 dark:text-red-400">
      {{ inlineError }}
    </div>

    <!-- Translation Table -->
    <UTable 
      :data="tableData" 
      :loading="engineStore.isLoading"
      class="w-full"
    />
  </div>
  
  <div v-else class="py-8 text-center text-gray-500">
    No translation units found.
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import { useEngineStore } from '~/stores/engine';
import { useTranslateStore } from '~/stores/translate';
import { useProviderStore } from '~/stores/provider';
import { TranslationStatus } from '~/types/translation';

const engineStore = useEngineStore();
const translateStore = useTranslateStore();
const providerStore = useProviderStore();
const inlineError = ref<string | null>(null)

// Convert all text units to table data format
const tableData = computed(() => {
  return engineStore.textUnits.map(unit => ({
    id: unit.id,
    status: getStatusLabel(unit.status),
    source_text: unit.source_text,
    translated_text: unit.translated_text || 'Not translated',
    prompt_type: unit.prompt_type,
  }));
});
// Check if there are untranslated units
const hasUntranslatedUnits = computed(() => {
  return engineStore.textUnits.some(unit => unit.status === 'NotTranslated');
});

// Check if there are translated units
const hasTranslatedUnits = computed(() => {
  return engineStore.textUnits.some(unit => 
    unit.status === 'MachineTranslated' || 
    unit.status === 'HumanReviewed' ||
    (unit.status === 'NotTranslated' && unit.translated_text && unit.translated_text.trim() !== '')
  );
});

// Check if translation is finished (no untranslated units remaining)
const isTranslationFinished = computed(() => {
  return !engineStore.textUnits.some(unit => unit.status === 'NotTranslated');
});

const getStatusLabel = (status: string) => {
  switch (status) {
    case 'NotTranslated':
      return 'Not Translated';
    case 'MachineTranslated':
      return 'Machine Translated';
    case 'HumanReviewed':
      return 'Human Reviewed';
    case 'Ignored':
      return 'Ignored';
    default:
      return status;
  }
};

// Translate all untranslated units
const translateAll = async () => {
  try {
    inlineError.value = null
    const ok = await providerStore.ensureConnected()
    if (!ok) {
      inlineError.value = 'Connection not available. Test connection and try again.'
      return
    }
    const untranslatedUnits = engineStore.textUnits.filter(unit => unit.status === 'NotTranslated');

    if (untranslatedUnits.length === 0) {
      return;
    }

    await translateStore.startBatchTranslation(
      untranslatedUnits,
      (translatedUnit) => {
        // This callback is called for each translated unit
        // The engine store is already updated in the translate store
        console.log('Unit translated:', translatedUnit.id);
        console.log('Original:', translatedUnit.source_text);
        console.log('Translated:', translatedUnit.translated_text);
        console.log('Prompt Type:', translatedUnit.prompt_type);
        console.log('---');
      }
    );
  } catch (error) {
    console.error('Translation failed:', error);
  }
};

// Inject translated text units back into project files
const injectTranslations = async () => {
  try {
    if (!engineStore.projectInfo) {
      console.error('No project loaded');
      return;
    }

    await engineStore.saveProject();
    console.log('Translations injected successfully');
  } catch (error) {
    console.error('Failed to inject translations:', error);
  }
};

// Reset all translations back to original state
const resetTranslations = async () => {
  try {
    if (!engineStore.projectInfo) {
      console.error('No project loaded');
      return;
    }

    // Reset all text units to their original state
    engineStore.textUnits.forEach(unit => {
      unit.translated_text = '';
      unit.status = TranslationStatus.NotTranslated;
    });

    console.log('Translations reset successfully');
  } catch (error) {
    console.error('Failed to reset translations:', error);
  }
};

// Export finished translations to JSON file
const exportToJson = () => {
  try {
    // Filter only translated units
    const translatedUnits = engineStore.textUnits.filter(unit => 
      unit.status === 'MachineTranslated' || 
      unit.status === 'HumanReviewed' ||
      (unit.status === 'NotTranslated' && unit.translated_text && unit.translated_text.trim() !== '')
    );

    if (translatedUnits.length === 0) {
      console.warn('No translated units to export');
      return;
    }

    // Create export data
    const exportData = {
      project_name: engineStore.projectInfo?.name || 'Unknown Project',
      export_date: new Date().toISOString(),
      total_units: engineStore.textUnits.length,
      translated_units: translatedUnits.length,
      translations: translatedUnits.map(unit => ({
        id: unit.id,
        source_text: unit.source_text,
        translated_text: unit.translated_text,
        prompt_type: unit.prompt_type,
        status: unit.status,
        field_type: unit.field_type
      }))
    };

    // Convert to JSON string with pretty formatting
    const jsonString = JSON.stringify(exportData, null, 2);

    // Create and download the file
    const blob = new Blob([jsonString], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.download = `translations_${engineStore.projectInfo?.name || 'project'}_${new Date().toISOString().split('T')[0]}.json`;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    URL.revokeObjectURL(url);

    console.log(`Exported ${translatedUnits.length} translations to JSON`);
  } catch (error) {
    console.error('Failed to export translations:', error);
  }
};
</script> 