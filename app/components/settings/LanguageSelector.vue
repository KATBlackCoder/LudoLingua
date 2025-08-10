<template>
  <div class="space-y-4">
    <div class="flex flex-wrap items-start gap-4">
      <UFormField label="Source Language" name="source_language" description="Language of the original text" required>
        <USelectMenu v-model="sourceLanguage" :items="languageOptions" value-key="id" placeholder="Select source language" class="w-56" />
      </UFormField>

      <UFormField label="Target Language" name="target_language" description="Language to translate to" required>
        <USelectMenu v-model="targetLanguage" :items="languageOptions" value-key="id" placeholder="Select target language" class="w-56" />
      </UFormField>

      <UFormField label="Direction" name="direction" description="Swap source and target">
        <UButtonGroup>
          <UButton size="xs" icon="i-heroicons-arrows-right-left" variant="soft" @click="swap">Swap</UButton>
        </UButtonGroup>
      </UFormField>
    </div>
  </div>
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
