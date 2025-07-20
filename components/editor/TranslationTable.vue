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
          :disabled="engineStore.isLoading || !hasTranslatedUnits"
          @click="injectTranslations"
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
import { computed } from 'vue';
import { useEngineStore } from '~/stores/engine';
import { useTranslateStore } from '~/stores/translate';

const engineStore = useEngineStore();
const translateStore = useTranslateStore();

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
    const untranslatedUnits = engineStore.textUnits.filter(unit => unit.status === 'NotTranslated');

    if (untranslatedUnits.length === 0) {
      return;
    }

    await translateStore.startBatchTranslation(
      untranslatedUnits,
      [], // glossary terms (empty for now)
      (translatedUnit) => {
        // This callback is called for each translated unit
        // The engine store is already updated in the translate store
        console.log('Unit translated:', translatedUnit.id);
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
</script> 