<template>
  <UCard>
    <template #header>
      <div class="flex items-center gap-3">
        <div class="p-2 bg-green-50 dark:bg-green-900/20 rounded-lg">
          <UIcon name="i-lucide-globe" class="text-green-500 w-5 h-5" />
        </div>
        <div>
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white">Languages</h3>
          <p class="text-sm text-gray-500 dark:text-gray-400">Set source and target languages for translation</p>
        </div>
      </div>
    </template>

    <div class="space-y-6">
      <!-- Language Selection -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <UFormField label="Source Language" description="Language of the original text" required>
          <USelectMenu 
            v-model="sourceLanguage" 
            :items="languageOptions" 
            value-key="id" 
            placeholder="Select source language" 
            class="w-full"
            icon="i-lucide-file-text"
          />
        </UFormField>

        <UFormField label="Target Language" description="Language to translate to" required>
          <USelectMenu 
            v-model="targetLanguage" 
            :items="languageOptions" 
            value-key="id" 
            placeholder="Select target language" 
            class="w-full"
            icon="i-lucide-languages"
          />
        </UFormField>
      </div>

      <!-- Swap Languages -->
      <div class="flex items-center justify-center">
        <UButton 
          size="sm" 
          icon="i-lucide-arrow-left-right" 
          variant="soft" 
          color="neutral"
          @click="swap"
        >
          Swap Languages
        </UButton>
      </div>

      <!-- Current Selection Display -->
      <div class="flex items-center gap-3 p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
        <UIcon name="i-lucide-info" class="text-gray-500 w-4 h-4" />
        <div class="flex-1">
          <div class="text-sm font-medium text-gray-900 dark:text-white">Current Translation Direction</div>
          <div class="text-xs text-gray-500 dark:text-gray-400">
            {{ sourceLanguage }} → {{ targetLanguage }}
          </div>
        </div>
        <div class="flex items-center gap-2">
          <UBadge color="primary" variant="soft" size="sm">{{ sourceLanguage }}</UBadge>
          <UIcon name="i-lucide-arrow-right" class="text-gray-400 w-3 h-3" />
          <UBadge color="success" variant="soft" size="sm">{{ targetLanguage }}</UBadge>
        </div>
      </div>
    </div>
  </UCard>
</template>

<script lang="ts" setup>
import { useLanguageStore } from "~/stores/language";

const languageStore = useLanguageStore();

const languageOptions = languageStore.languageOptions;
const sourceLanguage = ref(languageStore.currentSourceLanguage);
const targetLanguage = ref(languageStore.currentTargetLanguage);

// Keep language selections in sync with store
watch(
  () => languageStore.currentSourceLanguage,
  (newSource) => {
    sourceLanguage.value = newSource;
  },
  { immediate: true }
);

watch(
  () => languageStore.currentTargetLanguage,
  (newTarget) => {
    targetLanguage.value = newTarget;
  },
  { immediate: true }
);

// Update language store when selections change
watch(sourceLanguage, (newSource) => {
  if (newSource && newSource !== languageStore.currentSourceLanguage) {
    languageStore.setLanguage(newSource, targetLanguage.value);
  }
});

watch(targetLanguage, (newTarget) => {
  if (newTarget && newTarget !== languageStore.currentTargetLanguage) {
    languageStore.setLanguage(sourceLanguage.value, newTarget);
  }
});

function swap() {
  const src = sourceLanguage.value
  const tgt = targetLanguage.value
  sourceLanguage.value = tgt
  targetLanguage.value = src
  languageStore.setLanguage(sourceLanguage.value, targetLanguage.value)
}
</script>

<style></style>
